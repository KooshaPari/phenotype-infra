//! KVirtualDesktop MCP Server
//! 
//! This is the main MCP server implementation for KVirtualDesktop,
//! providing desktop automation capabilities through the MCP protocol.

use clap::{Arg, Command};
use kvirtualdesktop_core::*;
use kvirtualdesktop_mcp_server::*;
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    let app = Command::new("kvirtualdesktop-mcp-server")
        .version(env!("CARGO_PKG_VERSION"))
        .author("KVirtualDesktop Team")
        .about("MCP server for KVirtualDesktop desktop automation")
        .arg(
            Arg::new("transport")
                .short('t')
                .long("transport")
                .value_name("TRANSPORT")
                .help("Transport mechanism (stdio, http)")
                .default_value("stdio")
        )
        .arg(
            Arg::new("host")
                .short('h')
                .long("host")
                .value_name("HOST")
                .help("Host to bind to (for HTTP transport)")
                .default_value("localhost")
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Port to bind to (for HTTP transport)")
                .default_value("8080")
        )
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("CONFIG")
                .help("Path to configuration file")
        )
        .arg(
            Arg::new("log-level")
                .short('l')
                .long("log-level")
                .value_name("LEVEL")
                .help("Log level (trace, debug, info, warn, error)")
                .default_value("info")
        );

    let matches = app.get_matches();

    let transport = matches.get_one::<String>("transport").unwrap();
    let host = matches.get_one::<String>("host").unwrap();
    let port = matches.get_one::<String>("port").unwrap().parse::<u16>()?;
    let config_path = matches.get_one::<String>("config").map(PathBuf::from);

    info!("Starting KVirtualDesktop MCP Server v{}", env!("CARGO_PKG_VERSION"));
    info!("Transport: {}", transport);

    // Load configuration
    let config = if let Some(path) = config_path {
        ServerConfig::from_file(path)?
    } else {
        ServerConfig::default()
    };

    // Create and start server
    let mut server = match transport.as_str() {
        "stdio" => {
            info!("Using stdio transport");
            McpServer::new_stdio(config)?
        }
        "http" => {
            info!("Using HTTP transport on {}:{}", host, port);
            McpServer::new_http(config, host.clone(), port)?
        }
        _ => {
            error!("Invalid transport: {}", transport);
            return Err("Invalid transport".into());
        }
    };

    // Start server
    if let Err(e) = server.start().await {
        error!("Failed to start server: {}", e);
        return Err(e.into());
    }

    info!("Server started successfully");

    // Run server
    server.run().await?;

    Ok(())
}