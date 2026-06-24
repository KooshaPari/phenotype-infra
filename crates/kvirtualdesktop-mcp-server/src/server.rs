//! Core MCP Server Implementation
//! 
//! This module provides the main MCP server implementation that handles
//! protocol communication and dispatches requests to appropriate handlers.

use crate::config::ServerConfig;
use crate::handlers::*;
use crate::session::SessionManager;
use crate::middleware::SecurityMiddleware;
use kvirtualdesktop_core::*;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn, debug};
use uuid::Uuid;
use chrono::Utc;
use url::Url;

/// MCP Server
pub struct McpServer {
    config: ServerConfig,
    transport: Box<dyn Transport>,
    session_manager: Arc<RwLock<SessionManager>>,
    security_middleware: Arc<SecurityMiddleware>,
    handlers: Arc<RequestHandlers>,
    server_info: ServerInfo,
    capabilities: ServerCapabilities,
    running: Arc<RwLock<bool>>,
}

impl McpServer {
    /// Create new MCP server with stdio transport
    pub fn new_stdio(config: ServerConfig) -> Result<Self, McpError> {
        let transport = TransportFactory::create_stdio_transport();
        Self::new_with_transport(config, transport)
    }
    
    /// Create new MCP server with HTTP transport
    pub fn new_http(config: ServerConfig, host: String, port: u16) -> Result<Self, McpError> {
        let base_url = Url::parse(&format!("http://{}:{}", host, port))
            .map_err(|e| McpError::Internal(format!("Invalid URL: {}", e)))?;
        
        let transport = TransportFactory::create_http_sse_transport(base_url);
        Self::new_with_transport(config, transport)
    }
    
    /// Create new MCP server with custom transport
    pub fn new_with_transport(
        config: ServerConfig,
        transport: Box<dyn Transport>,
    ) -> Result<Self, McpError> {
        let session_manager = Arc::new(RwLock::new(SessionManager::new()));
        let security_middleware = Arc::new(SecurityMiddleware::new(config.security.clone()));
        let handlers = Arc::new(RequestHandlers::new(config.clone()));
        
        let server_info = ServerInfo {
            name: "KVirtualDesktop MCP Server".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            description: Some("Desktop automation server with VM/container support".to_string()),
            homepage: Some(Url::parse("https://github.com/kvirtualdesktop/kvirtualdesktop").unwrap()),
            documentation: Some(Url::parse("https://docs.kvirtualdesktop.com").unwrap()),
        };
        
        let capabilities = ServerCapabilities {
            tools: true,
            resources: true,
            prompts: true,
            session_management: true,
            streaming: true,
            notifications: true,
            security: SecurityCapabilities {
                oauth2: config.security.oauth2_enabled,
                resource_indicators: true,
                token_introspection: true,
                pkce: true,
            },
        };
        
        Ok(Self {
            config,
            transport,
            session_manager,
            security_middleware,
            handlers,
            server_info,
            capabilities,
            running: Arc::new(RwLock::new(false)),
        })
    }
    
    /// Start the server
    pub async fn start(&mut self) -> Result<(), McpError> {
        info!("Starting MCP server...");
        
        // Start transport
        self.transport.start().await
            .map_err(|e| McpError::Transport(e))?;
        
        // Initialize handlers
        self.handlers.initialize().await?;
        
        // Mark as running
        *self.running.write().await = true;
        
        info!("MCP server started successfully");
        Ok(())
    }
    
    /// Stop the server
    pub async fn stop(&mut self) -> Result<(), McpError> {
        info!("Stopping MCP server...");
        
        // Mark as not running
        *self.running.write().await = false;
        
        // Stop transport
        self.transport.stop().await
            .map_err(|e| McpError::Transport(e))?;
        
        // Cleanup handlers
        self.handlers.cleanup().await?;
        
        info!("MCP server stopped");
        Ok(())
    }
    
    /// Run the server main loop
    pub async fn run(&mut self) -> Result<(), McpError> {
        info!("Running MCP server main loop");
        
        while *self.running.read().await {
            match self.transport.receive_message().await {
                Ok(message) => {
                    debug!("Received message: {:?}", message);
                    
                    // Process message in background
                    let server_clone = self.clone_for_handler();
                    tokio::spawn(async move {
                        if let Err(e) = server_clone.handle_message(message).await {
                            error!("Error handling message: {}", e);
                        }
                    });
                }
                Err(McpTransportError::ConnectionClosed) => {
                    info!("Connection closed");
                    break;
                }
                Err(e) => {
                    error!("Transport error: {}", e);
                    // Continue running unless it's a fatal error
                }
            }
        }
        
        info!("Server main loop ended");
        Ok(())
    }
    
    /// Handle incoming message
    async fn handle_message(&self, message: Message) -> Result<(), McpError> {
        // Apply security middleware
        let security_context = self.security_middleware.process_message(&message).await?;
        
        // Route message to appropriate handler
        let response = match message.method.as_str() {
            McpMethods::INITIALIZE => self.handle_initialize(message).await?,
            McpMethods::INITIALIZED => self.handle_initialized(message).await?,
            McpMethods::SHUTDOWN => self.handle_shutdown(message).await?,
            McpMethods::PING => self.handle_ping(message).await?,
            
            // Tool methods
            McpMethods::TOOLS_LIST => self.handlers.handle_tools_list(message, &security_context).await?,
            McpMethods::TOOLS_CALL => self.handlers.handle_tools_call(message, &security_context).await?,
            
            // Resource methods
            McpMethods::RESOURCES_LIST => self.handlers.handle_resources_list(message, &security_context).await?,
            McpMethods::RESOURCES_READ => self.handlers.handle_resources_read(message, &security_context).await?,
            McpMethods::RESOURCES_WRITE => self.handlers.handle_resources_write(message, &security_context).await?,
            
            // Prompt methods
            McpMethods::PROMPTS_LIST => self.handlers.handle_prompts_list(message, &security_context).await?,
            McpMethods::PROMPTS_GET => self.handlers.handle_prompts_get(message, &security_context).await?,
            
            // Session methods
            McpMethods::SESSION_CREATE => self.handle_session_create(message, &security_context).await?,
            McpMethods::SESSION_DESTROY => self.handle_session_destroy(message, &security_context).await?,
            McpMethods::SESSION_LIST => self.handle_session_list(message, &security_context).await?,
            
            // Desktop automation methods
            method if method.starts_with("desktop/") => {
                self.handlers.handle_desktop_automation(message, &security_context).await?
            }
            
            // VM/Container methods
            method if method.starts_with("vm/") => {
                self.handlers.handle_vm_operations(message, &security_context).await?
            }
            
            // CLI methods
            method if method.starts_with("cli/") => {
                self.handlers.handle_cli_operations(message, &security_context).await?
            }
            
            // TTS methods
            method if method.starts_with("tts/") => {
                self.handlers.handle_tts_operations(message, &security_context).await?
            }
            
            // Credential methods
            method if method.starts_with("credentials/") => {
                self.handlers.handle_credential_operations(message, &security_context).await?
            }
            
            _ => {
                warn!("Unknown method: {}", message.method);
                self.create_error_response(
                    message.id,
                    McpError::MethodNotFound(message.method.clone()),
                )?
            }
        };
        
        // Send response
        self.send_message(response).await?;
        
        Ok(())
    }
    
    /// Handle initialize request
    async fn handle_initialize(&self, message: Message) -> Result<Message, McpError> {
        let request: InitializeRequest = serde_json::from_value(
            message.params.unwrap_or_default()
        ).map_err(|e| McpError::InvalidParams(e.to_string()))?;
        
        info!("Client connecting: {} v{}", request.client_info.name, request.client_info.version);
        
        let response = InitializeResponse {
            protocol_version: MCP_VERSION.to_string(),
            server_info: self.server_info.clone(),
            capabilities: self.capabilities.clone(),
        };
        
        Ok(Message {
            id: message.id,
            method: message.method,
            params: None,
            result: Some(serde_json::to_value(response)?),
            error: None,
            session_id: None,
            timestamp: Utc::now(),
        })
    }
    
    /// Handle initialized notification
    async fn handle_initialized(&self, message: Message) -> Result<Message, McpError> {
        info!("Client initialization complete");
        
        // Create acknowledgment response
        Ok(Message {
            id: message.id,
            method: message.method,
            params: None,
            result: Some(serde_json::json!({"status": "acknowledged"})),
            error: None,
            session_id: None,
            timestamp: Utc::now(),
        })
    }
    
    /// Handle shutdown request
    async fn handle_shutdown(&self, message: Message) -> Result<Message, McpError> {
        info!("Client requesting shutdown");
        
        // Mark server as not running
        *self.running.write().await = false;
        
        Ok(Message {
            id: message.id,
            method: message.method,
            params: None,
            result: Some(serde_json::json!({"status": "shutting_down"})),
            error: None,
            session_id: None,
            timestamp: Utc::now(),
        })
    }
    
    /// Handle ping request
    async fn handle_ping(&self, message: Message) -> Result<Message, McpError> {
        Ok(Message {
            id: message.id,
            method: McpMethods::PONG.to_string(),
            params: None,
            result: Some(serde_json::json!({"timestamp": Utc::now()})),
            error: None,
            session_id: message.session_id,
            timestamp: Utc::now(),
        })
    }
    
    /// Handle session create request
    async fn handle_session_create(
        &self,
        message: Message,
        security_context: &SecurityContext,
    ) -> Result<Message, McpError> {
        let request: SessionCreateRequest = serde_json::from_value(
            message.params.unwrap_or_default()
        ).map_err(|e| McpError::InvalidParams(e.to_string()))?;
        
        let session_id = self.session_manager.write().await.create_session(
            request.name,
            request.metadata,
            security_context,
        ).await?;
        
        let response = SessionCreateResponse {
            session_id,
            created_at: Utc::now(),
        };
        
        Ok(Message {
            id: message.id,
            method: message.method,
            params: None,
            result: Some(serde_json::to_value(response)?),
            error: None,
            session_id: Some(session_id),
            timestamp: Utc::now(),
        })
    }
    
    /// Handle session destroy request
    async fn handle_session_destroy(
        &self,
        message: Message,
        security_context: &SecurityContext,
    ) -> Result<Message, McpError> {
        if let Some(session_id) = message.session_id {
            self.session_manager.write().await.destroy_session(session_id).await?;
        }
        
        Ok(Message {
            id: message.id,
            method: message.method,
            params: None,
            result: Some(serde_json::json!({"status": "destroyed"})),
            error: None,
            session_id: None,
            timestamp: Utc::now(),
        })
    }
    
    /// Handle session list request
    async fn handle_session_list(
        &self,
        message: Message,
        security_context: &SecurityContext,
    ) -> Result<Message, McpError> {
        let sessions = self.session_manager.read().await.list_sessions().await?;
        
        Ok(Message {
            id: message.id,
            method: message.method,
            params: None,
            result: Some(serde_json::to_value(sessions)?),
            error: None,
            session_id: message.session_id,
            timestamp: Utc::now(),
        })
    }
    
    /// Create error response
    fn create_error_response(&self, message_id: String, error: McpError) -> Result<Message, McpError> {
        let mcp_error = crate::types::McpError {
            code: error.to_error_code(),
            message: error.to_string(),
            data: None,
        };
        
        Ok(Message {
            id: message_id,
            method: String::new(),
            params: None,
            result: None,
            error: Some(mcp_error),
            session_id: None,
            timestamp: Utc::now(),
        })
    }
    
    /// Send message through transport
    async fn send_message(&self, message: Message) -> Result<(), McpError> {
        // Note: We need to make transport mutable for sending
        // In a real implementation, we'd use interior mutability or channels
        debug!("Sending message: {:?}", message);
        Ok(())
    }
    
    /// Clone server for handler (simplified for this example)
    fn clone_for_handler(&self) -> ServerHandle {
        ServerHandle {
            session_manager: self.session_manager.clone(),
            security_middleware: self.security_middleware.clone(),
            handlers: self.handlers.clone(),
            server_info: self.server_info.clone(),
            capabilities: self.capabilities.clone(),
        }
    }
}

/// Handle for background processing
#[derive(Clone)]
pub struct ServerHandle {
    session_manager: Arc<RwLock<SessionManager>>,
    security_middleware: Arc<SecurityMiddleware>,
    handlers: Arc<RequestHandlers>,
    server_info: ServerInfo,
    capabilities: ServerCapabilities,
}

impl ServerHandle {
    pub async fn handle_message(&self, message: Message) -> Result<(), McpError> {
        // Implementation similar to McpServer::handle_message
        // but using the cloned handles
        Ok(())
    }
}