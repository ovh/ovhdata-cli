use clap::{Command, CommandFactory};
use clap_complete::{generate, Shell};
use std::io::stdout;

use crate::config::CLI_NAME;
use crate::opts::Opts;
use crate::utils::Result;

pub struct CompletionCommand {}

impl CompletionCommand {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn generate(&self, shell: Shell) -> Result<()> {
        let mut app: Command = Opts::command();
        generate(shell, &mut app, CLI_NAME, &mut stdout());
        Ok(())
    }
}
