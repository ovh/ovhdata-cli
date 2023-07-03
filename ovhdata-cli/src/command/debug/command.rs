use futures::StreamExt;
use tokio::fs::File;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::config::CLI_NAME;
use crate::utils::Result;
use crate::Context;

pub struct DebugCommand {}

impl DebugCommand {
    pub fn new() -> Self {
        Self {}
    }

    /// Print logs of a command run in the past
    pub async fn log(&self, session_id: String) -> Result<()> {
        // Get log file
        let mut logfile = std::env::temp_dir();
        {
            let context = Context::get();
            logfile.push(context.uuid.to_string());
            logfile.push(format!("{}-{}.log", session_id, CLI_NAME));
        }
        let output_log_file = logfile.clone();

        let file = File::open(logfile).await?;
        FramedRead::new(file, LinesCodec::new())
            .for_each(|line| async move {
                println!("{}", line.unwrap());
            })
            .await;

        println!("\nDebug file path={:#?}", output_log_file);

        Ok(())
    }
}
