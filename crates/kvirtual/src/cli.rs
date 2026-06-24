//! CLI runner for KVirtualDesktop (`kvd`)
//!
//! Provides the command execution layer that dispatches CLI subcommands
//! to the appropriate handler modules.

use crate::config::Config;
use crate::session::SessionManager;

/// Shared context carried through CLI command execution.
pub struct CommandContext {
    pub config: Config,
    pub session_manager: SessionManager,
    pub current_session: Option<String>,
}

impl CommandContext {
    pub fn new(config: Config, session_manager: SessionManager, session: Option<String>) -> Self {
        Self {
            config,
            session_manager,
            current_session: session,
        }
    }
}

/// CLI runner that dispatches subcommands.
pub struct CliRunner {
    ctx: CommandContext,
}

impl CliRunner {
    pub fn new(ctx: CommandContext) -> Self {
        Self { ctx }
    }

    pub async fn run_repl(&mut self) -> anyhow::Result<()> {
        println!("KVirtual REPL — type 'help' for commands");
        // REPL loop would be wired here
        Ok(())
    }

    pub async fn run_script(&mut self, _file: &str, _args: Vec<String>) -> anyhow::Result<()> {
        tracing::info!("Script execution not yet implemented");
        Ok(())
    }

    pub async fn handle_desktop_action(
        &mut self,
        action: crate::DesktopAction,
    ) -> anyhow::Result<()> {
        tracing::info!("Desktop action: {:?}", action);
        Ok(())
    }

    pub async fn handle_container_action(
        &mut self,
        action: crate::ContainerAction,
    ) -> anyhow::Result<()> {
        tracing::info!("Container action: {:?}", action);
        Ok(())
    }

    pub async fn handle_vm_action(
        &mut self,
        action: crate::VmAction,
    ) -> anyhow::Result<()> {
        tracing::info!("VM action: {:?}", action);
        Ok(())
    }

    pub async fn handle_record_action(
        &mut self,
        action: crate::RecordAction,
    ) -> anyhow::Result<()> {
        tracing::info!("Record action: {:?}", action);
        Ok(())
    }

    pub async fn handle_session_action(
        &mut self,
        action: crate::SessionAction,
    ) -> anyhow::Result<()> {
        tracing::info!("Session action: {:?}", action);
        Ok(())
    }

    pub async fn handle_config_action(
        &mut self,
        action: crate::ConfigAction,
    ) -> anyhow::Result<()> {
        tracing::info!("Config action: {:?}", action);
        Ok(())
    }

    pub async fn handle_creds_action(
        &mut self,
        action: crate::CredAction,
    ) -> anyhow::Result<()> {
        tracing::info!("Creds action: {:?}", action);
        Ok(())
    }
}
