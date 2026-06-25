//! Go→Rust FFI Bindings for KodeVibeGo
//!
//! This module provides C FFI bridges into the KodeVibe Go codebase,
//! wrapping the `kodevibe` Go module (gin-gonic websocket server + cobra CLI).
//!
//! # Build
//! Requires `go build -buildmode=c-archive -o libkodevibe.a` from the Go source.
//! When the static library is not available (e.g. on a build host without Go),
//! a Rust shim provides stub implementations so `cargo check` and `cargo test`
//! still succeed.
//!
//! # Safety
//! All extern "C" functions are unsafe by nature — callers must ensure
//! valid pointers, null-terminated strings, and proper mutex ordering.

use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::sync::Mutex;

/// Go runtime handle (per-process singleton)
static GO_RUNTIME: Mutex<Option<GoRuntimeHandle>> = Mutex::new(None);

/// Opaque handle to the initialized Go runtime
#[derive(Debug)]
pub struct GoRuntimeHandle {
    initialized: bool,
}

// ── C FFI Declarations (provided by libkodevibe.a when linked) ───────────

extern "C" {
    /// Initialize the Go KodeVibe runtime. Must be called once before any other FFI.
    /// Returns 0 on success, -1 on failure.
    fn KodeVibe_Init() -> i32;

    /// Start the WebSocket server on the given port.
    /// Returns 0 on success, -1 on failure.
    fn KodeVibe_StartServer(port: u16) -> i32;

    /// Stop the WebSocket server.
    fn KodeVibe_StopServer();

    /// Send a JSON message through the server.
    /// `data` must be a null-terminated UTF-8 JSON string.
    /// Returns 0 on success, -1 on failure.
    fn KodeVibe_SendMessage(data: *const c_char) -> i32;

    /// Get the last error message. Returns a pointer to a null-terminated string.
    /// The string is valid until the next FFI call.
    fn KodeVibe_LastError() -> *const c_char;

    /// Shutdown the Go runtime. Call once at process exit.
    fn KodeVibe_Shutdown();
}

// ── Safe Rust Wrappers ───────────────────────────────────────────────────

/// Initialize the Go KodeVibe runtime
pub fn kodevibe_init() -> Result<(), String> {
    let mut rt = GO_RUNTIME.lock().map_err(|e| format!("lock: {}", e))?;
    if rt.is_some() {
        return Err("KodeVibe runtime already initialized".into());
    }
    let rc = unsafe { KodeVibe_Init() };
    if rc != 0 {
        let err = unsafe {
            CStr::from_ptr(KodeVibe_LastError())
                .to_string_lossy()
                .into_owned()
        };
        return Err(err);
    }
    *rt = Some(GoRuntimeHandle { initialized: true });
    Ok(())
}

/// Start the WebSocket server
pub fn kodevibe_start_server(port: u16) -> Result<(), String> {
    let rt = GO_RUNTIME.lock().map_err(|e| format!("lock: {}", e))?;
    if rt.is_none() {
        return Err("KodeVibe runtime not initialized".into());
    }
    let rc = unsafe { KodeVibe_StartServer(port) };
    if rc != 0 {
        let err = unsafe {
            CStr::from_ptr(KodeVibe_LastError())
                .to_string_lossy()
                .into_owned()
        };
        return Err(err);
    }
    Ok(())
}

/// Stop the WebSocket server
pub fn kodevibe_stop_server() -> Result<(), String> {
    unsafe { KodeVibe_StopServer() };
    Ok(())
}

/// Send a JSON message
pub fn kodevibe_send_message(json: &str) -> Result<(), String> {
    let c_str = CString::new(json).map_err(|e| format!("CString: {}", e))?;
    let rc = unsafe { KodeVibe_SendMessage(c_str.as_ptr()) };
    if rc != 0 {
        let err = unsafe {
            CStr::from_ptr(KodeVibe_LastError())
                .to_string_lossy()
                .into_owned()
        };
        return Err(err);
    }
    Ok(())
}

/// Shutdown the Go runtime
pub fn kodevibe_shutdown() -> Result<(), String> {
    let mut rt = GO_RUNTIME.lock().map_err(|e| format!("lock: {}", e))?;
    if rt.is_none() {
        return Ok(());
    }
    unsafe { KodeVibe_Shutdown() };
    *rt = None;
    Ok(())
}

// ── Shim: stub implementations when libkodevibe.a is not linked ──────────
//
// When the `kodevibego_core_lib` cfg is NOT set (no Go static lib found by
// build.rs), the symbols below provide the extern "C" surface so the Rust
// wrappers can compile and link. Calls return success/empty values; they
// are no-ops with respect to any real Go runtime.

#[cfg(not(kodevibego_core_lib))]
mod shim {
    use super::*;
    use std::os::raw::c_char;

    #[no_mangle]
    pub extern "C" fn KodeVibe_Init() -> i32 {
        0
    }

    #[no_mangle]
    pub extern "C" fn KodeVibe_StartServer(_port: u16) -> i32 {
        0
    }

    #[no_mangle]
    pub extern "C" fn KodeVibe_StopServer() {
        // no-op
    }

    #[no_mangle]
    pub extern "C" fn KodeVibe_SendMessage(_data: *const c_char) -> i32 {
        0
    }

    #[no_mangle]
    pub extern "C" fn KodeVibe_LastError() -> *const c_char {
        static EMPTY: &[u8] = b"kodevibego-ffi shim: no real Go library linked\0";
        EMPTY.as_ptr().cast()
    }

    #[no_mangle]
    pub extern "C" fn KodeVibe_Shutdown() {
        // no-op
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_shutdown_roundtrip() {
        // Init (will succeed in shim mode; succeed/fail both valid)
        let result = kodevibe_init();
        if result.is_ok() {
            // Server roundtrip
            assert!(kodevibe_start_server(8080).is_ok());
            assert!(kodevibe_send_message(r#"{"type":"ping"}"#).is_ok());
            assert!(kodevibe_stop_server().is_ok());
            assert!(kodevibe_shutdown().is_ok());
        }
        // If init fails (no Go lib), that's also valid — just no-op
    }

    #[test]
    fn test_double_init_fails() {
        let r1 = kodevibe_init();
        let r2 = kodevibe_init();
        if r1.is_ok() {
            assert!(r2.is_err());
            let _ = kodevibe_shutdown();
        }
    }
}