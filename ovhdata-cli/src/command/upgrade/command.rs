use std::fs;
use std::path::PathBuf;
#[cfg(not(target_os = "windows"))]
use std::process::Command;

use chrono::{Duration, Utc};
use clap::crate_version;
use crossterm::style::Stylize;
use descriptor::Descriptor;
use filetime::FileTime;
use futures::StreamExt;
use regex::Regex;
use reqwest::header::USER_AGENT;
use reqwest::Client;
use semver::Version;
use serde::Deserialize;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

use ovhdata_common::BUG;

use crate::config::{config_dir, Config, Context, Toggle, CLI_NAME};
use crate::utils::ui::printer::{stderr, Printer, HELP_UPGRADE, HELP_UPGRADE_MANDATORY};
use crate::utils::{Error, Result};

#[cfg(target_os = "macos")]
static OS: &str = "darwin";
#[cfg(target_os = "linux")]
static OS: &str = "linux";
#[cfg(target_os = "windows")]
static OS: &str = "windows";

pub struct Upgrade {}

#[derive(Debug, Clone, Deserialize, Descriptor)]
pub struct GithubAsset {
    pub name: String,
    pub browser_download_url: String,
}

#[derive(Debug, Clone, Deserialize, Descriptor)]
pub struct GithubRelease {
    pub tag_name: String,
    pub created_at: Option<String>,
    pub published_at: Option<String>,
    pub assets: Vec<GithubAsset>,
}

impl Upgrade {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn check_upgrade(&self) -> Result<()> {
        // Check if the CLI is already up to date
        let last_version = Self::get_last_version().await?;
        let current_version = Version::parse(crate_version!()).expect(BUG);

        // Banner on break changes
        if current_version.minor < last_version.minor || current_version.major < last_version.major {
            Printer::print_help(HELP_UPGRADE_MANDATORY, Toggle::NoToggle);
        }

        Ok(())
    }

    /// Only write to stderr
    /// If quiet is true will only write if an upgrade happen
    pub async fn upgrade(&self, force: bool, confirm: bool, quiet: bool) -> Result<()> {
        // Check if the CLI is already up to date
        let last_version = Self::get_last_version().await?;
        let current_version = Version::parse(crate_version!()).expect(BUG);

        if !force && last_version <= current_version {
            if !quiet {
                Printer::println_success(&mut stderr(), "Your CLI is already up to date");
            }
            return Ok(());
        }

        // Banner on break changes
        if !quiet && (current_version.minor < last_version.minor || current_version.major < last_version.major) {
            Printer::print_help(HELP_UPGRADE, Toggle::NoToggle);
        }

        // Ask for confirmation
        if confirm {
            let message = format!("Version {} is available. Proceed with upgrade", last_version);
            let choices = ["Yes", "No", "Always", "Never"];
            match Printer::ask_select(&message, &choices, 0) {
                Ok(choice) => match choices[choice] {
                    "No" => {
                        return Ok(());
                    }
                    "Always" => {
                        let mut context = Context::get();
                        context.features.confirm_before_upgrade = false;
                        context.save()?;
                    }
                    "Never" => {
                        let mut context = Context::get();
                        context.features.auto_upgrade = false;
                        context.save()?;
                        return Ok(());
                    }
                    _ => {}
                },
                Err(_) => {
                    return Ok(());
                }
            };
        }
        // Upgrade CLI
        self.upgrade_os(quiet, &last_version).await
    }

    #[cfg(target_os = "windows")]
    pub async fn upgrade_os(&self, quiet: bool, _last_version: &Version) -> Result<()> {
        if !quiet {
            eprintln!("Auto-upgrade is not available for Windows yet.");
            eprintln!("Please download the new version of the CLI at {}", Self::get_binary_url().await?);

            Printer::eprintln_fail("Upgrade canceled.");
        }
        Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    pub async fn upgrade_os(&self, quiet: bool, last_version: &Version) -> Result<()> {
        use std::os::unix::process::CommandExt;

        let spinner = Printer::start_spinner(&format!("Downloading new version {}", last_version.to_string().green()));

        let mut tmp_dir = std::env::temp_dir();
        tmp_dir.push(format!("{}.{}.tmp", CLI_NAME, last_version));
        let mut file = File::create(tmp_dir.clone()).await?;

        let response = reqwest::get(&Upgrade::get_binary_url().await?).await?;

        let mut stream = response.bytes_stream();
        while let Some(bytes) = stream.next().await {
            let bytes = bytes.map_err(Error::custom)?;
            file.write_all(&bytes).await?;
        }
        Printer::stop_spinner(spinner);

        // Replace current executable
        let current_exe_path = std::env::current_exe()?;

        let perms = fs::metadata(&current_exe_path)?.permissions();
        file.set_permissions(perms).await?;

        fs::remove_file(&current_exe_path)?;
        fs::copy(&tmp_dir, &current_exe_path)?;

        Printer::println_success(&mut stderr(), "New version installed");

        // Execute updated executable (program stops there)
        // If quiet is false the command is 'upgrade', so no need to launch the command again
        if quiet {
            Err(Command::new(&current_exe_path).args(std::env::args().skip(1)).exec())?;
        }

        Ok(())
    }

    fn release_cache_file_path() -> PathBuf {
        let mut path = config_dir();
        path.push("RELEASE");
        path
    }

    pub fn release_cache_expired() -> bool {
        let path = Self::release_cache_file_path();
        let metadata = fs::metadata(path.as_path());
        if let Ok(metadata) = metadata {
            let mtime = FileTime::from_last_modification_time(&metadata);
            if mtime.unix_seconds() > (Utc::now() - Duration::hours(1)).timestamp() {
                return false;
            }
        }
        true
    }

    async fn get_binary_url() -> Result<String> {
        let re = Regex::new(&format!(r"({})", OS)).unwrap();

        // get last version from cache
        let github_release = Self::get_last_release().await?;

        // Return the download url corresponding to the OS
        github_release
            .assets
            .iter()
            .filter(|asset| re.is_match(&asset.name))
            .map(|asset| asset.browser_download_url.clone())
            .last()
            .ok_or(Error::Custom(format!("Asset not found OS={}", OS)))
    }

    async fn get_last_release() -> Result<GithubRelease> {
        let url = format!("{}/latest", &Config::get().cli_release_url);

        // get last release from cache
        let path = Self::release_cache_file_path();
        if path.exists() && !Self::release_cache_expired() {
            let last_github_release = fs::read_to_string(path.as_path());
            if let Ok(last_github_release) = last_github_release {
                let release: GithubRelease = serde_json::from_str(&last_github_release).unwrap();
                return Ok(release);
            }
        }

        // get last version from remote
        let client = Client::builder().build()?.get(url).header(USER_AGENT, "request");
        let body_response = client.send().await?.text().await?;

        // parse version
        let last_release = serde_json::from_str(&body_response).unwrap();

        // save to cache
        let err = fs::write(path.as_path(), body_response);
        if let Err(err) = err {
            eprintln!("{}", err);
        }

        Ok(last_release)
    }

    async fn get_last_version() -> Result<Version> {
        // get last version from cache
        let last_release = Self::get_last_release().await?;

        match Version::parse(&last_release.tag_name) {
            Ok(valid_remote_version) => Ok(valid_remote_version),
            Err(_) => Err(Error::custom("failed to get last version")),
        }
    }
}
