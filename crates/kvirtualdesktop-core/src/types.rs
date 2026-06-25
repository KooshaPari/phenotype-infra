//! Core MCP Protocol Types
//! 
//! This module defines all the core types used in the MCP protocol,
//! including messages, tools, resources, and prompts.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use url::Url;

/// MCP Message ID
pub type MessageId = String;

/// MCP Request ID
pub type RequestId = String;

/// MCP Session ID
pub type SessionId = Uuid;

/// MCP Message base structure
///
/// The envelope for every request, notification, and response sent over an
/// MCP transport. Fields are populated based on direction:
///
/// - **Request**: `id`, `method`, `params`, `session_id`, `timestamp`.
/// - **Notification**: `method`, `params`, `session_id`, `timestamp` (no `id`).
/// - **Response**: `id`, `result` (success) or `error` (failure),
///   `session_id`, `timestamp`.
///
/// ```
/// use kvirtualdesktop_core::Message;
/// use chrono::Utc;
///
/// let req = Message {
///     id: "req-1".into(),
///     method: "tools/list".into(),
///     params: None,
///     result: None,
///     error: None,
///     session_id: None,
///     timestamp: Utc::now(),
/// };
/// assert_eq!(req.method, "tools/list");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Wire-format message identifier. Omit for notifications.
    pub id: MessageId,
    /// MCP method name (e.g. `"tools/list"`, `"resources/read"`). See
    /// [`crate::protocol::McpMethods`] for the canonical set.
    pub method: String,
    /// JSON-encoded method parameters, if any.
    pub params: Option<serde_json::Value>,
    /// JSON-encoded result, populated on success responses.
    pub result: Option<serde_json::Value>,
    /// MCP error, populated on failure responses.
    pub error: Option<McpError>,
    /// Session this message belongs to, if a session has been established.
    pub session_id: Option<SessionId>,
    /// Wall-clock timestamp at which the sender constructed the message.
    pub timestamp: DateTime<Utc>,
}

/// MCP Error structure
///
/// A JSON-RPC-style error block. The numeric `code` is wire-stable;
/// see [`McpError::to_error_code`](crate::error::McpError::to_error_code)
/// for the canonical mapping.
///
/// ```
/// use kvirtualdesktop_core::types::McpError;
///
/// let e = McpError {
///     code: -32601,
///     message: "Method not found: foo/bar".into(),
///     data: None,
/// };
/// assert_eq!(e.code, -32601);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    /// Numeric MCP error code (negative integers reserved by spec).
    pub code: i32,
    /// Human-readable error message; safe to log but not necessarily safe
    /// to surface to end users verbatim.
    pub message: String,
    /// Optional structured payload (e.g. validation details).
    pub data: Option<serde_json::Value>,
}

/// MCP Tool Definition
///
/// Declares a callable tool: its JSON-Schema input/output, what capabilities
/// it requires, and any security gates. Tools are advertised via
/// `tools/list` and invoked via `tools/call`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    /// Stable tool identifier (e.g. `"screenshot.capture"`).
    pub name: String,
    /// One-sentence description surfaced to MCP clients.
    pub description: String,
    /// JSON Schema describing accepted arguments.
    pub input_schema: ToolInputSchema,
    /// Optional JSON Schema describing successful results.
    pub output_schema: Option<ToolOutputSchema>,
    /// Capabilities the tool needs to function. The runtime must verify
    /// the host can satisfy these before invocation.
    pub capabilities: Vec<ToolCapability>,
    /// Security requirements enforced before the tool may be called.
    pub security_requirements: Option<SecurityRequirements>,
}

/// Tool Input Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInputSchema {
    pub r#type: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub required: Vec<String>,
    pub additional_properties: Option<bool>,
}

/// Tool Output Schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutputSchema {
    pub r#type: String,
    pub properties: HashMap<String, serde_json::Value>,
    pub required: Vec<String>,
}

/// Tool Capability
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ToolCapability {
    #[serde(rename = "ui_automation")]
    UiAutomation,
    #[serde(rename = "cli_access")]
    CliAccess,
    #[serde(rename = "file_system")]
    FileSystem,
    #[serde(rename = "network_access")]
    NetworkAccess,
    #[serde(rename = "vm_control")]
    VmControl,
    #[serde(rename = "container_control")]
    ContainerControl,
    #[serde(rename = "screen_recording")]
    ScreenRecording,
    #[serde(rename = "screenshot")]
    Screenshot,
    #[serde(rename = "audio_tts")]
    AudioTts,
    #[serde(rename = "credential_management")]
    CredentialManagement,
}

/// MCP Resource Definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resource {
    pub uri: Url,
    pub name: String,
    pub description: String,
    pub mime_type: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    pub access_control: Option<AccessControl>,
}

/// Access Control for Resources
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessControl {
    pub read: bool,
    pub write: bool,
    pub execute: bool,
    pub delete: bool,
    pub required_scopes: Vec<String>,
}

/// MCP Prompt Template
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptTemplate {
    pub name: String,
    pub description: String,
    pub arguments: Vec<PromptArgument>,
    pub template: String,
    pub category: PromptCategory,
}

/// Prompt Argument
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptArgument {
    pub name: String,
    pub description: String,
    pub required: bool,
    pub r#type: String,
    pub default: Option<serde_json::Value>,
}

/// Prompt Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptCategory {
    #[serde(rename = "desktop_automation")]
    DesktopAutomation,
    #[serde(rename = "vm_management")]
    VmManagement,
    #[serde(rename = "ui_testing")]
    UiTesting,
    #[serde(rename = "workflow_automation")]
    WorkflowAutomation,
    #[serde(rename = "system_monitoring")]
    SystemMonitoring,
}

/// Security Requirements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityRequirements {
    pub authentication: AuthenticationMethod,
    pub authorization: AuthorizationMethod,
    pub required_scopes: Vec<String>,
    pub resource_indicators: Vec<Url>,
}

/// Authentication Method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthenticationMethod {
    #[serde(rename = "oauth2")]
    OAuth2 {
        authorization_server: Url,
        token_endpoint: Url,
        client_id: String,
    },
    #[serde(rename = "api_key")]
    ApiKey {
        header_name: String,
        prefix: Option<String>,
    },
    #[serde(rename = "none")]
    None,
}

/// Authorization Method
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthorizationMethod {
    #[serde(rename = "scope_based")]
    ScopeBased,
    #[serde(rename = "role_based")]
    RoleBased,
    #[serde(rename = "resource_based")]
    ResourceBased,
}

/// MCP Server Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerCapabilities {
    pub tools: bool,
    pub resources: bool,
    pub prompts: bool,
    pub session_management: bool,
    pub streaming: bool,
    pub notifications: bool,
    pub security: SecurityCapabilities,
}

/// Security Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityCapabilities {
    pub oauth2: bool,
    pub resource_indicators: bool,
    pub token_introspection: bool,
    pub pkce: bool,
}

/// MCP Client Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub name: String,
    pub version: String,
    pub capabilities: ClientCapabilities,
    pub user_agent: Option<String>,
}

/// Client Capabilities
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientCapabilities {
    pub supports_notifications: bool,
    pub supports_streaming: bool,
    pub supports_sessions: bool,
    pub max_concurrent_requests: Option<u32>,
}

/// Desktop Automation Context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopAutomationContext {
    pub session_id: SessionId,
    pub vm_id: Option<String>,
    pub container_id: Option<String>,
    pub screen_resolution: Option<(u32, u32)>,
    pub active_windows: Vec<WindowInfo>,
    pub environment_variables: HashMap<String, String>,
}

/// Window Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowInfo {
    pub id: String,
    pub title: String,
    pub process_name: String,
    pub bounds: WindowBounds,
    pub is_active: bool,
    pub is_visible: bool,
}

/// Window Bounds
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindowBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// VM/Container State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineState {
    pub id: String,
    pub name: String,
    pub state: VmState,
    pub cpu_usage: Option<f64>,
    pub memory_usage: Option<f64>,
    pub disk_usage: Option<f64>,
    pub network_interfaces: Vec<NetworkInterface>,
    pub snapshots: Vec<VmSnapshot>,
}

/// VM State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum VmState {
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "stopped")]
    Stopped,
    #[serde(rename = "paused")]
    Paused,
    #[serde(rename = "suspended")]
    Suspended,
    #[serde(rename = "unknown")]
    Unknown,
}

/// Network Interface
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterface {
    pub name: String,
    pub mac_address: String,
    pub ip_addresses: Vec<String>,
    pub state: NetworkInterfaceState,
}

/// Network Interface State
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkInterfaceState {
    #[serde(rename = "up")]
    Up,
    #[serde(rename = "down")]
    Down,
    #[serde(rename = "unknown")]
    Unknown,
}

/// VM Snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmSnapshot {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub size: u64,
}

/// Recording Session
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordingSession {
    pub id: String,
    pub name: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub duration: Option<u64>,
    pub file_path: Option<String>,
    pub file_size: Option<u64>,
    pub format: RecordingFormat,
    pub quality: RecordingQuality,
}

/// Recording Format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordingFormat {
    #[serde(rename = "mp4")]
    Mp4,
    #[serde(rename = "webm")]
    WebM,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "png_sequence")]
    PngSequence,
}

/// Recording Quality
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RecordingQuality {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "lossless")]
    Lossless,
}

/// TTS Configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsConfig {
    pub voice: String,
    pub language: String,
    pub speed: f32,
    pub pitch: f32,
    pub volume: f32,
    pub output_format: AudioFormat,
}

/// Audio Format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AudioFormat {
    #[serde(rename = "wav")]
    Wav,
    #[serde(rename = "mp3")]
    Mp3,
    #[serde(rename = "ogg")]
    Ogg,
    #[serde(rename = "flac")]
    Flac,
}

/// Credential Store Entry
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialEntry {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub credential_type: CredentialType,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Credential Type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CredentialType {
    #[serde(rename = "username_password")]
    UsernamePassword,
    #[serde(rename = "api_key")]
    ApiKey,
    #[serde(rename = "oauth2_token")]
    OAuth2Token,
    #[serde(rename = "ssh_key")]
    SshKey,
    #[serde(rename = "certificate")]
    Certificate,
    #[serde(rename = "custom")]
    Custom,
}