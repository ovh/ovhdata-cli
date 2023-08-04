use std::collections::{HashMap, HashSet};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::fs::{create_dir_all, metadata, set_permissions, File};
use std::io::{BufReader, BufWriter};
use std::ops::Deref;
use std::path::PathBuf;
use std::sync::{Mutex, MutexGuard};

use ovhdata_common::config::{AllConfig, ConfigName};
use ovhdata_common::model::di::common::EnsureSecret;
use ovhdata_macros::PrintObjectCompletely;

use crossterm::style::Stylize;
use descriptor::Descriptor;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize, Serializer};
use thiserror::Error as ThisError;
use tracing::error;
use uuid::Uuid;

lazy_static! {
    static ref CONTEXT: Mutex<Context> = Mutex::new(Context::load(None).expect("Unexpected error"));
    static ref CONFIG: Mutex<AllConfig> = Mutex::new(Config::load().expect("Unexpected error"));
}

pub const CLI_NAME: &str = "ovhdata-cli";
const DEFAULT_REGION: Region = Region::EU;
const CONFIG_EU: &str = include_str!("../config/eu.json");
const CONFIG_CA: &str = include_str!("../config/ca.json");

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Region {
    EU,
    CA,
}

impl Display for Region {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Region::EU => f.write_str("OVH-EU"),
            Region::CA => f.write_str("OVH-CA"),
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Ovhapiv6Credentials {
    pub application_key: Option<String>,
    pub application_secret: Option<String>,
    pub consumer_key: Option<String>,
}

impl EnsureSecret<Ovhapiv6Credentials> for Ovhapiv6Credentials {
    fn hide_secrets(&self) -> Ovhapiv6Credentials {
        let mut creds = self.clone();
        creds.application_secret = Some("[hidden_secret]".to_string());
        creds
    }
}

#[derive(Deserialize, Serialize)]
pub struct Context {
    // UUID use to identify uniquely a context log file (so that different user can work in parallel on the same machine)
    #[serde(default)]
    pub uuid: Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ovhapi_credentials: Option<HashMap<ConfigName, Ovhapiv6Credentials>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_names: Option<HashMap<ConfigName, String>>,
    #[serde(default)]
    toggles: HashSet<Toggle>,
    #[serde(default)]
    pub features: Features,
    #[serde(skip)]
    pub config_path: PathBuf,
}

#[derive(PartialEq, Clone, Serialize, Deserialize, Descriptor)]
pub struct RuntimeContext {
    pub service_name: Option<String>,
}

#[derive(Deserialize, Serialize)]
pub struct Features {
    #[serde(default = "default_as_true")]
    pub auto_upgrade: bool,
    #[serde(default = "default_as_true")]
    pub confirm_before_upgrade: bool,
    #[serde(default = "default_as_true")]
    pub app_beta_banner: bool,
}

impl Default for Features {
    fn default() -> Self {
        Self {
            auto_upgrade: true,
            confirm_before_upgrade: true,
            app_beta_banner: true,
        }
    }
}

impl Context {
    /// Get global context
    pub fn get<'a>() -> MutexGuard<'a, Context> {
        CONTEXT.lock().expect("Unexpected error")
    }

    pub fn get_runtime_context(&self, config_name: &ConfigName) -> RuntimeContext {
        RuntimeContext {
            service_name: self.get_service_name(config_name),
        }
    }

    pub fn get_ovhapi_credentials(&self) -> Option<Ovhapiv6Credentials> {
        let config_name = Config::get_config_name();
        self.ovhapi_credentials.as_ref().and_then(|x| x.get(&config_name).cloned())
    }

    pub fn set_ovhapi_credentials(&mut self, creds: Ovhapiv6Credentials) {
        let config_name = Config::get_config_name();
        match &mut self.ovhapi_credentials {
            Some(map) => {
                map.insert(config_name, creds);
            }
            None => {
                let mut map = HashMap::new();
                map.insert(config_name, creds);
                self.ovhapi_credentials = Some(map);
            }
        }
    }

    pub fn get_current_service_name(&self) -> Option<String> {
        let config_name = Config::get_config_name();
        self.get_service_name(&config_name)
    }

    fn get_service_name(&self, config_name: &ConfigName) -> Option<String> {
        self.service_names.as_ref().and_then(|x| x.get(config_name).cloned())
    }

    pub fn set_service_name(&mut self, service_name: String) {
        let config_name = Config::get_config_name();
        match &mut self.service_names {
            Some(map) => {
                map.insert(config_name, service_name);
            }
            None => {
                let mut map = HashMap::new();
                map.insert(config_name, service_name);
                self.service_names = Some(map);
            }
        }
    }

    pub fn logout(&mut self) {
        let config_name = Config::get_config_name();
        match &mut self.ovhapi_credentials {
            Some(map) => {
                map.remove(&config_name);
            }
            None => {}
        }
        match &mut self.service_names {
            Some(map) => {
                map.remove(&config_name);
            }
            None => {}
        }
    }

    /// Load context from a file, return default if it does not exist yet
    fn load(custom_path: Option<PathBuf>) -> Result<Self> {
        // Context path
        let path = if let Some(cp) = custom_path { cp } else { default_context_path() };
        Context::create_context_file(path.clone())?;

        // Read context
        let mut context = match File::open(&path) {
            Ok(file) => {
                let mut ctx: Context = serde_json::from_reader(BufReader::new(file))?;
                ctx.config_path = path.clone();
                ctx
            }
            Err(_) => Context::default(),
        };

        // If the context uuid is the default one it means that it hasn't been set yet
        if context.uuid == Uuid::default() {
            context.uuid = Uuid::new_v4();
            if let Err(err) = context.save() {
                eprintln!("unable to save context on {:?}: {:?}", path, err);
            }
        }

        Ok(context)
    }

    /// create context file specified as parameter along with all its tree.
    /// if it does not already exist, it will be initialized with a serialized version of Context::default()
    fn create_context_file(context_file: PathBuf) -> Result<()> {
        let mut context_parent_dir = context_file.clone();
        context_parent_dir.pop();
        // Context path
        create_dir_all(&context_parent_dir)?;

        if File::open(&context_file).is_err() {
            let context_file_handle = File::create(&context_file)?;

            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                let metadata = context_file_handle.metadata()?;
                let mut permissions = metadata.permissions();
                permissions.set_mode(0o600); // Read/write user only.
                set_permissions(&context_file, permissions)?;
            }
            serde_json::to_writer_pretty(BufWriter::new(context_file_handle), &Context::default())?;
        } else {
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;

                let mode = metadata(&context_file)?.permissions().mode();
                if mode != 0o100600 {
                    eprintln!("{}", format!("WARNING: {} file permissions are incorrect for a file that holds sensitive information. Please manually run `chmod 600 {}` (read/write for user only).", &context_file.to_str().unwrap(), &context_file.to_str().unwrap()).yellow());
                    {}
                }
            }
        }
        Ok(())
    }

    /// Save context into a file
    pub fn save(&self) -> Result<()> {
        Self::create_context_file(self.config_path.clone())?;

        // Write context
        serde_json::to_writer_pretty(BufWriter::new(File::create(&self.config_path)?), self)?;

        Ok(())
    }

    /// Return true if the flag is toggled
    pub fn is_toggle(&self, toggle: Toggle) -> bool {
        self.toggles.contains(&toggle)
    }

    /// Toggle a flag and save
    pub fn _toggle(&mut self, toggle: Toggle) {
        if !self.toggles.contains(&toggle) {
            self.toggles.insert(toggle);
            self.save().unwrap();
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        Self {
            uuid: Uuid::new_v4(),
            ovhapi_credentials: None,
            service_names: None,
            toggles: HashSet::new(),
            features: Features::default(),
            config_path: default_context_path(),
        }
    }
}

#[derive(Clone, Copy, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Toggle {
    NoToggle,
    HelpLogin,
}

#[derive(Clone, Serialize, Deserialize, Descriptor, PrintObjectCompletely)]
#[descriptor(default_headers = ["name"])]
pub struct Config {
    #[descriptor(rename_header = "NAME")]
    pub name: ConfigName,
    pub config: ovhdata_common::config::Config,
    pub context: Option<RuntimeContext>,
}

#[derive(Descriptor)]
#[descriptor(default_headers = ["item.name", "item.config.ovhapiv6.endpoint_url"], map=map_row_selectable_item)]
pub struct SelectableItem {
    pub item: Config,
    pub is_selected: bool,
}

fn map_row_selectable_item(selectable_item: &SelectableItem, cell: String) -> String {
    match selectable_item.is_selected {
        true => cell.green(),
        false => cell.reset(),
    }
    .to_string()
}

impl SelectableItem {
    pub fn new(item: Config, is_selected: bool) -> Self {
        SelectableItem { item, is_selected }
    }
}

impl Deref for Config {
    type Target = ovhdata_common::config::Config;
    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl Serialize for SelectableItem {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<<S as Serializer>::Ok, <S as Serializer>::Error>
    where
        S: Serializer,
    {
        self.item.serialize(serializer)
    }
}

impl Config {
    pub fn new<N>(name: N, config: ovhdata_common::config::Config, context: Option<RuntimeContext>) -> Self
    where
        N: Into<ConfigName>,
    {
        Config {
            name: name.into(),
            config,
            context,
        }
    }

    pub fn get() -> Self {
        let all_config = Self::get_all();
        match all_config.get_current_config() {
            Ok(config) => Config::new(all_config.current_config_name.as_str(), config.clone(), None),
            Err(error) => panic!("{}", error.to_string()),
        }
    }

    pub fn get_config_name() -> ConfigName {
        Self::get().name
    }

    /// Get global config
    pub fn get_all<'a>() -> MutexGuard<'a, AllConfig> {
        CONFIG.lock().expect("Unexpected error")
    }

    /// Load config
    fn load() -> Result<AllConfig> {
        let default_configs = maplit::hashmap! {
        ConfigName::from(Region::EU.to_string()) => Self::load_region_default(Region::EU)?,
        ConfigName::from(Region::CA.to_string()) => Self::load_region_default(Region::CA)?,
        };
        match AllConfig::try_from(custom_config_path()) {
            Ok(all_config) => {
                let mut configs = all_config.configs.clone();
                configs.extend(default_configs);
                Ok(AllConfig {
                    current_config_name: all_config.current_config_name,
                    configs,
                })
            }
            Err(_) => Ok(AllConfig {
                current_config_name: ConfigName::from(DEFAULT_REGION.to_string()),
                configs: default_configs,
            }),
        }
    }

    fn load_region_default(region: Region) -> Result<ovhdata_common::config::Config> {
        let _config_val = serde_json::from_str(match region {
            Region::CA => CONFIG_CA,
            Region::EU => CONFIG_EU,
        })?;

        Ok(serde_json::from_value::<ovhdata_common::config::Config>(_config_val)
            .unwrap_or_else(|_| panic!("Unable to load default configuration for region {}", &region)))
    }
}

pub fn config_dir() -> PathBuf {
    let mut path = dirs::home_dir().expect("Cannot get home directory");
    path.push(".config");
    path.push(CLI_NAME);
    path
}

fn default_context_path() -> PathBuf {
    let mut path = config_dir();
    path.push("context.json");
    path
}

pub fn custom_config_path() -> PathBuf {
    let mut path = config_dir();
    path.push("config.json");
    path
}

fn default_as_true() -> bool {
    true
}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    ConfigError(ovhdata_common::config::Error),
}

impl From<ovhdata_common::config::Error> for Error {
    fn from(err: ovhdata_common::config::Error) -> Self {
        Error::ConfigError(err)
    }
}
