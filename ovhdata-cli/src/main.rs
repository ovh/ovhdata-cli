use std::fs::OpenOptions;
use std::io::stdout;
use std::process::exit;

use clap::error::ErrorKind;
use clap::Parser;
use crossterm::tty::IsTty;
use tracing::{error, info};
use tracing_subscriber::fmt::writer::Tee;
use tracing_subscriber::EnvFilter;

use ovhdata_common::ovhapi::OVHapiV6Client;
use ovhdata_common::BUG;

use crate::command::auth;
use crate::command::completion::CompletionCommand;
use crate::command::config::ConfigCommand;
use crate::command::debug::DebugCommand;
use crate::command::di::DiCommand;
use crate::command::me::MeCommand;
use crate::command::upgrade;

use crate::options::*;

use crate::config::{Config, Context, CLI_NAME};
use crate::logging::SESSION_ID;
use crate::utils::ui::printer::{Printer, HELP_NO_AUTH_HOW_TO, HELP_NO_SERVICE_NAME_HOW_TO, NO_COLOR, NO_SPINNER};

mod command;
mod config;
mod logging;
mod options;
mod utils;

#[tokio::main]
async fn main() {
    // Parse command line
    let result: clap::error::Result<Opts> = Opts::try_parse();

    if let Err(error) = result {
        println!("{}", error);
        if error.kind() == ErrorKind::DisplayVersion {
            exit(EXIT_CODE_SUCCESS)
        }

        exit(EXIT_CODE_ERROR)
    }

    let opts = result.unwrap();
    let verbose = opts.verbose;

    // Initialize logging
    init_log(verbose, opts.json_log);
    info!("Command: {}", std::env::args().collect::<Vec<_>>().join(" "));

    // Disable color if needed, or if stdout is a tty
    *NO_COLOR.write().expect(BUG) = opts.no_color || !stdout().is_tty();

    *NO_SPINNER.write().expect(BUG) = opts.no_spinner || !stdout().is_tty();

    // Auto upgrade on startup except for the upgrade command ;-)
    match opts.subcmd {
        SubCommand::Upgrade(_) => {}
        _ => auto_upgrade().await,
    }

    // Execute command
    let command_result = execute_command(opts).await;

    unwrap_or_exit(command_result, verbose);
}

async fn auto_upgrade() {
    let auto_upgrade = Context::get().features.auto_upgrade;
    let confirm_before_upgrade = Context::get().features.confirm_before_upgrade;

    if auto_upgrade &&
        // Either no-confirm or in a TTY
        (stdout().is_tty() || !confirm_before_upgrade) &&
        // No version check in last hour
        upgrade::Upgrade::release_cache_expired()
    {
        // Try to auto-upgrade
        if let Err(err) = upgrade::Upgrade::new().upgrade(false, confirm_before_upgrade, true).await {
            Printer::eprintln_fail(&err.to_string());
        }
    } else if stdout().is_tty() {
        // Only display banner on break changes
        if let Err(err) = upgrade::Upgrade::new().check_upgrade().await {
            Printer::eprintln_fail(&err.to_string());
        }
    }
}

fn init_log(verbosity: u8, json: bool) {
    // Dont write log file with the level 0
    if verbosity == 0 {
        return;
    }

    let log_file_path = logging::LOG_FILE.clone();
    // Create log directory if needed
    let mut log_directory = log_file_path.clone();
    log_directory.pop();
    if let Err(error) = std::fs::create_dir_all(log_directory.as_path()) {
        eprintln!("unable to create log directory {:?}: {:?}", log_directory, error);
        return;
    }
    // Configure tracing subscriber to append on the log file
    match OpenOptions::new().create(true).write(true).append(true).open(log_file_path.as_path()) {
        Ok(log_file) => {
            let result = match verbosity {
                0 => ovhdata_common::log::init_subscriber(log_file, json, EnvFilter::new("info")),
                other => {
                    let directive = match other {
                        1 => "info",
                        2 => "debug",
                        _ => "trace",
                    };
                    ovhdata_common::log::init_subscriber(Tee::new(log_file, std::io::stderr), json, EnvFilter::new(directive))
                }
            };
            if let Err(error) = result {
                eprintln!("Logger error: {:?}", error);
            }
        }
        Err(error) => {
            eprintln!("error opening {:?} error: {:?}", log_file_path, error);
        }
    }
}

/// Execute a command
async fn execute_command(opts: Opts) -> Result<()> {
    // Use service name given if set
    if let Some(service_name) = opts.service_name {
        let mut context = Context::get();
        context.set_service_name(service_name);
    };

    match opts.subcmd {
        // Upgrade
        SubCommand::Upgrade(Upgrade { force }) => upgrade::Upgrade::new().upgrade(force, true, false).await?,
        // Login
        SubCommand::Login(login) => {
            let command = auth::Auth::new();
            command
                .login(
                    login.application_key,
                    login.consumer_key,
                    login.secret,
                    login.output.unwrap_or_default().into(),
                )
                .await?
        }

        // Logout
        SubCommand::Logout(_logout) => auth::Auth::new().logout().await?,

        // Debug
        SubCommand::Debug(Debug { session_id }) => DebugCommand::new().log(session_id).await?,

        // Me
        SubCommand::Me(me) => {
            let command = MeCommand::new(build_ovhapi_client().await?);
            command.me(me.output.unwrap_or_default().into()).await?
        }

        // Data Integration
        SubCommand::Di(DiShim { subcmd }) => {
            let command = DiCommand::new(build_ovhapi_cloud_client().await?);
            command.execute_command(subcmd).await?
        }

        // Config
        SubCommand::Config(ConfigShim { subcmd }) => {
            let command = ConfigCommand::new(build_ovhapi_cloud_client().await?);
            command.execute_command(subcmd).await?
        }

        // Completion
        SubCommand::Completion(completion) => {
            let command = CompletionCommand::new();
            command.generate(completion.shell).await?
        }
    };

    Ok(())
}

/// Build an OVHapi v6 cloud client (with service_name)
async fn build_ovhapi_cloud_client() -> Result<OVHapiV6Client> {
    let apiv6client = build_ovhapi_client().await?;

    let context = Context::get();
    let service_name = context.get_current_service_name();

    // No ovh api creds exit
    if service_name.is_none() {
        eprintln!();
        let help = Printer::gen_help(HELP_NO_SERVICE_NAME_HOW_TO);
        eprintln!("{}", help);
        exit(EXIT_CODE_ERROR);
    }

    Ok(apiv6client)
}

/// Build an OVHapi v6 client
async fn build_ovhapi_client() -> Result<OVHapiV6Client> {
    let context = Context::get();
    let ovhapicreds_option = context.get_ovhapi_credentials();

    // No ovh api creds exit
    if ovhapicreds_option.is_none() {
        eprintln!();
        let help = Printer::gen_help(HELP_NO_AUTH_HOW_TO);
        eprintln!("{}", help);
        exit(EXIT_CODE_ERROR);
    }

    let ovhapicreds = ovhapicreds_option.unwrap();
    let ovhapiv6_client = OVHapiV6Client::new(
        Config::get().ovhapiv6.endpoint_url.clone(),
        ovhapicreds.application_key.unwrap(),
        ovhapicreds.application_secret.unwrap(),
        ovhapicreds.consumer_key.unwrap(),
    );
    Ok(ovhapiv6_client)
}

/// Unwrap the result or print and error and exit
fn unwrap_or_exit<T>(result: Result<T>, verbosity: u8) -> T {
    match result {
        Ok(ok) => ok,
        Err(err) => {
            error!("{}", err);
            eprintln!();
            Printer::eprintln_fail(&format!("{}", err));
            eprintln!();
            if verbosity > 0 {
                eprintln!("To print the full logs of this command:");
                eprintln!("> {} debug {}", CLI_NAME, *SESSION_ID);
            } else {
                eprintln!("You may use -v option. Useful for debugging and seeing what's is going on \"under the hood\".");
            }
            exit(EXIT_CODE_ERROR);
        }
    }
}

pub type Result<T> = std::result::Result<T, utils::Error>;

pub type ExitCode = i32;
pub const EXIT_CODE_SUCCESS: ExitCode = 0;
pub const EXIT_CODE_ERROR: ExitCode = 1;
pub const EXIT_CODE_SKIPPED: ExitCode = 10;
