//! TUI runner for interactive mode

use crate::cli::CommandContext;

pub struct TuiRunner {
    ctx: CommandContext,
}

impl TuiRunner {
    pub fn new(ctx: CommandContext) -> Self {
        Self { ctx }
    }

    pub async fn run(&mut self) -> anyhow::Result<()> {
        tracing::info!("TUI mode — interactive interface");
        println!("KVirtual TUI — interactive mode");
        // TUI would be initialized with ratatui/crossterm here
        Ok(())
    }
}
