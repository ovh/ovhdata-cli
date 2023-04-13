use chrono::Utc;
use lazy_static::lazy_static;
use std::path::PathBuf;

use crate::config::CLI_NAME;
use crate::Context;

lazy_static! {
    // Unique ID for this command
    pub static ref SESSION_ID: String = format!("{}", Utc::now().timestamp_millis());
    // Temporary file to log to
    pub static ref LOG_FILE: PathBuf = {
        let mut file = std::env::temp_dir();
        let context = Context::get();
        file.push(context.uuid.to_string());
        file.push(format!("{}-{}.log", SESSION_ID.to_string(), CLI_NAME));
        file
    };
}
