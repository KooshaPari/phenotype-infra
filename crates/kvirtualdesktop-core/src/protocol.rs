//! MCP Protocol Messages and Methods
//!
//! This module defines the core MCP protocol messages and methods
//! for client-server communication.

use crate::types::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::Url;

/// MCP Protocol Methods
pub struct McpMethods;

impl McpMethods {
    // Core Protocol Methods
    pub const INITIALIZE: &'static str = "initialize";
    pub const INITIALIZED: &'static str = "initialized";
    pub const SHUTDOWN: &'static str = "shutdown";
    pub const PING: &'static str = "ping";
    pub const PONG: &'static str = "pong";

    // Tool Methods
    pub const TOOLS_LIST: &'static str = "tools/list";
    pub const TOOLS_CALL: &'static str = "tools/call";
    pub const TOOLS_RESULT: &'static str = "tools/result";

    // Resource Methods
    pub const RESOURCES_LIST: &'static str = "resources/list";
    pub const RESOURCES_READ: &'static str = "resources/read";
    pub const RESOURCES_WRITE: &'static str = "resources/write";
    pub const RESOURCES_SUBSCRIBE: &'static str = "resources/subscribe";
    pub const RESOURCES_UNSUBSCRIBE: &'static str = "resources/unsubscribe";

    // Prompt Methods
    pub const PROMPTS_LIST: &'static str = "prompts/list";
    pub const PROMPTS_GET: &'static str = "prompts/get";

    // Session Methods
    pub const SESSION_CREATE: &'static str = "session/create";
    pub const SESSION_DESTROY: &'static str = "session/destroy";
    pub const SESSION_LIST: &'static str = "session/list";

    // Desktop Automation Methods
    pub const DESKTOP_CAPTURE_SCREENSHOT: &'static str = "desktop/capture_screenshot";
    pub const DESKTOP_START_RECORDING: &'static str = "desktop/start_recording";
    pub const DESKTOP_STOP_RECORDING: &'static str = "desktop/stop_recording";
    pub const DESKTOP_CLICK: &'static str = "desktop/click";
    pub const DESKTOP_TYPE: &'static str = "desktop/type";
    pub const DESKTOP_KEY_PRESS: &'static str = "desktop/key_press";
    pub const DESKTOP_DRAG_DROP: &'static str = "desktop/drag_drop";
    pub const DESKTOP_WINDOW_LIST: &'static str = "desktop/window_list";
    pub const DESKTOP_WINDOW_FOCUS: &'static str = "desktop/window_focus";
    pub const DESKTOP_WINDOW_RESIZE: &'static str = "desktop/window_resize";
    pub const DESKTOP_WINDOW_MOVE: &'static str = "desktop/window_move";

    // VM/Container Methods
    pub const VM_LIST: &'static str = "vm/list";
    pub const VM_START: &'static str = "vm/start";
    pub const VM_STOP: &'static str = "vm/stop";
    pub const VM_PAUSE: &'static str = "vm/pause";
    pub const VM_RESUME: &'static str = "vm/resume";
    pub const VM_SNAPSHOT: &'static str = "vm/snapshot";
    pub const VM_RESTORE: &'static str = "vm/restore";
    pub const VM_STATUS: &'static str = "vm/status";

    // CLI Methods
    pub const CLI_EXECUTE: &'static str = "cli/execute";
    pub const CLI_EXECUTE_ASYNC: &'static str = "cli/execute_async";
    pub const CLI_KILL: &'static str = "cli/kill";
    pub const CLI_LIST_PROCESSES: &'static str = "cli/list_processes";

    // TTS Methods
    pub const TTS_SPEAK: &'static str = "tts/speak";
    pub const TTS_STOP: &'static str = "tts/stop";
    pub const TTS_LIST_VOICES: &'static str = "tts/list_voices";
    pub const TTS_SET_CONFIG: &'static str = "tts/set_config";

    // Credential Methods
    pub const CREDENTIALS_LIST: &'static str = "credentials/list";
    pub const CREDENTIALS_GET: &'static str = "credentials/get";
    pub const CREDENTIALS_STORE: &'static str = "credentials/store";
    pub const CREDENTIALS_DELETE: &'static str = "credentials/delete";
}

/// Initialize Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeRequest {
    pub protocol_version: String,
    pub client_info: ClientInfo,
    pub capabilities: ClientCapabilities,
}

/// Initialize Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitializeResponse {
    pub protocol_version: String,
    pub server_info: ServerInfo,
    pub capabilities: ServerCapabilities,
}

/// Server Information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub homepage: Option<Url>,
    pub documentation: Option<Url>,
}

/// Tools List Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsListRequest {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

/// Tools List Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsListResponse {
    pub tools: Vec<Tool>,
    pub next_cursor: Option<String>,
}

/// Tool Call Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
    pub session_id: Option<SessionId>,
}

/// Tool Call Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResponse {
    pub content: Vec<ToolCallContent>,
    pub is_error: bool,
    pub session_id: Option<SessionId>,
}

/// Tool Call Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallContent {
    pub r#type: String,
    pub text: Option<String>,
    pub data: Option<serde_json::Value>,
    pub mime_type: Option<String>,
}

/// Resources List Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesListRequest {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

/// Resources List Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourcesListResponse {
    pub resources: Vec<Resource>,
    pub next_cursor: Option<String>,
}

/// Resource Read Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceReadRequest {
    pub uri: Url,
    pub range: Option<ReadRange>,
}

/// Read Range
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadRange {
    pub start: u64,
    pub end: Option<u64>,
}

/// Resource Read Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceReadResponse {
    pub contents: Vec<ResourceContent>,
}

/// Resource Content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceContent {
    pub r#type: String,
    pub text: Option<String>,
    pub data: Option<Vec<u8>>,
    pub mime_type: Option<String>,
}

/// Prompts List Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptsListRequest {
    pub cursor: Option<String>,
    pub limit: Option<u32>,
}

/// Prompts List Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptsListResponse {
    pub prompts: Vec<PromptTemplate>,
    pub next_cursor: Option<String>,
}

/// Prompt Get Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptGetRequest {
    pub name: String,
    pub arguments: HashMap<String, serde_json::Value>,
}

/// Prompt Get Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptGetResponse {
    pub description: String,
    pub messages: Vec<PromptMessage>,
}

/// Prompt Message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptMessage {
    pub role: String,
    pub content: String,
}

/// Session Create Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCreateRequest {
    pub name: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Session Create Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCreateResponse {
    pub session_id: SessionId,
    pub created_at: DateTime<Utc>,
}

/// Desktop Screenshot Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopScreenshotRequest {
    pub session_id: Option<SessionId>,
    pub region: Option<ScreenRegion>,
    pub format: Option<String>,
    pub quality: Option<u8>,
}

/// Screen Region
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// Desktop Screenshot Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopScreenshotResponse {
    pub image_data: Vec<u8>,
    pub format: String,
    pub width: u32,
    pub height: u32,
    pub timestamp: DateTime<Utc>,
}

/// Desktop Click Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopClickRequest {
    pub session_id: Option<SessionId>,
    pub x: i32,
    pub y: i32,
    pub button: MouseButton,
    pub click_count: Option<u32>,
}

/// Mouse Button
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MouseButton {
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "middle")]
    Middle,
}

/// Desktop Type Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopTypeRequest {
    pub session_id: Option<SessionId>,
    pub text: String,
    pub delay: Option<u64>,
}

/// Desktop Key Press Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesktopKeyPressRequest {
    pub session_id: Option<SessionId>,
    pub key: String,
    pub modifiers: Vec<KeyModifier>,
}

/// Key Modifier
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum KeyModifier {
    #[serde(rename = "ctrl")]
    Ctrl,
    #[serde(rename = "alt")]
    Alt,
    #[serde(rename = "shift")]
    Shift,
    #[serde(rename = "cmd")]
    Cmd,
    #[serde(rename = "meta")]
    Meta,
}

/// VM List Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmListRequest {
    pub include_stopped: bool,
}

/// VM List Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmListResponse {
    pub vms: Vec<VirtualMachineState>,
}

/// VM Start Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VmStartRequest {
    pub vm_id: String,
    pub wait_for_boot: bool,
}

/// CLI Execute Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliExecuteRequest {
    pub session_id: Option<SessionId>,
    pub command: String,
    pub args: Vec<String>,
    pub working_directory: Option<String>,
    pub environment: Option<HashMap<String, String>>,
    pub timeout: Option<u64>,
}

/// CLI Execute Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliExecuteResponse {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
    pub execution_time: u64,
}

/// TTS Speak Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsSpeakRequest {
    pub text: String,
    pub config: Option<TtsConfig>,
    pub session_id: Option<SessionId>,
}

/// TTS Speak Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TtsSpeakResponse {
    pub audio_data: Option<Vec<u8>>,
    pub duration: f64,
    pub format: AudioFormat,
}

/// Credentials List Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsListRequest {
    pub filter: Option<String>,
    pub credential_type: Option<CredentialType>,
}

/// Credentials List Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsListResponse {
    pub credentials: Vec<CredentialEntry>,
}

/// Credentials Get Request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsGetRequest {
    pub id: String,
    pub decrypt: bool,
}

/// Credentials Get Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialsGetResponse {
    pub credential: CredentialEntry,
    pub secret_data: Option<HashMap<String, String>>,
}

/// Notification types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NotificationType {
    #[serde(rename = "resource_changed")]
    ResourceChanged,
    #[serde(rename = "tool_progress")]
    ToolProgress,
    #[serde(rename = "session_expired")]
    SessionExpired,
    #[serde(rename = "vm_state_changed")]
    VmStateChanged,
    #[serde(rename = "recording_completed")]
    RecordingCompleted,
}

/// Notification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Notification {
    pub r#type: NotificationType,
    pub data: serde_json::Value,
    pub timestamp: DateTime<Utc>,
    pub session_id: Option<SessionId>,
}

/// Standard MCP Response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StandardResponse {
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<serde_json::Value>,
}
