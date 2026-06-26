// SPDX-License-Identifier: MIT OR Apache-2.0
// Build script for kodevibego-ffi
//
// Orchestrates the link chain between Rust FFI bindings and the KodeVibeGo
// Go Core (formerly tools/kodevibego Go analysis engine).
//
//   Mode A: Go static lib exists → link against libkodevibe.a,
//            set `cfg(kodevibego_core_lib)` so the Rust shim is disabled.
//   Mode B: Go static lib absent  → no external link; Rust shim module
//            provides stub `extern "C"` implementations (supports
//            `cargo check` and `cargo test` without a Go toolchain).
//
// Expected static lib location (built by `make kodevibe-c-archive`):
//   target/libkodevibe.a

use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    // Declare custom cfg so rustc's `unexpected_cfgs` lint (stable since 1.80) stays silent.
    println!("cargo:rustc-check-cfg=cfg(kodevibego_core_lib)");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Possible locations for the Go static library
    let target_dir = manifest_dir
        .ancestors()
        .find(|p| p.join("Cargo.toml").exists())
        .map(|p| p.join("target"))
        .unwrap_or_else(|| manifest_dir.join("../target"));

    let lib_paths = [target_dir.join("libkodevibe.a")];

    // Try to find the Go toolchain
    let go_available = Command::new("go").arg("version").output().is_ok();

    // Check if any pre-built static lib exists
    let lib_exists = lib_paths.iter().any(|p| p.exists());

    if lib_exists {
        // Mode A: Link against the real Go static library
        for path in &lib_paths {
            if path.exists() {
                let dir = path.parent().unwrap();
                println!("cargo:rustc-link-search=native={}", dir.display());
                println!("cargo:rustc-link-lib=static=kodevibe");
                println!("cargo:rustc-cfg=kodevibego_core_lib");
                println!(
                    "cargo:warning=Linking against real KodeVibeGo core at {}",
                    path.display()
                );
                break;
            }
        }
    } else if go_available {
        // Mode B: Go toolchain available but no pre-built lib
        // Try to build it on-the-fly from tools/kodevibego
        let go_src = manifest_dir.join("../../tools/kodevibego");
        if go_src.exists() {
            let output_path = out_dir.join("libkodevibe.a");

            let status = Command::new("go")
                .args([
                    "build",
                    "-buildmode=c-archive",
                    "-o",
                    &output_path.to_string_lossy(),
                    "./cmd/kodevibed",
                ])
                .current_dir(&go_src)
                .output();

            match status {
                Ok(output) if output.status.success() => {
                    println!("cargo:rustc-link-search=native={}", out_dir.display());
                    println!("cargo:rustc-link-lib=static=kodevibe");
                    println!("cargo:rustc-cfg=kodevibego_core_lib");
                    println!(
                        "cargo:warning=KodeVibeGo core built on-the-fly at {}",
                        output_path.display()
                    );
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    println!("cargo:warning=Go build failed: {}", stderr);
                    println!("cargo:warning=Falling back to Rust shim (stub) implementation");
                }
                Err(e) => {
                    println!("cargo:warning=Go build error: {}", e);
                    println!("cargo:warning=Falling back to Rust shim (stub) implementation");
                }
            }
        } else {
            println!("cargo:warning=Go source not found at {}", go_src.display());
            println!("cargo:warning=Falling back to Rust shim (stub) implementation");
        }
    } else {
        // Mode C: No Go toolchain and no pre-built lib
        // The Rust shim module provides stub implementations
        println!("cargo:warning=No Go toolchain found — using Rust shim (stub) for KodeVibeGo FFI");
        println!(
            "cargo:warning=Install Go and run `make kodevibe-c-archive` for real KodeVibeGo linkage"
        );
    }

    // Re-run if our own source changes
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/ffi.rs");
}
