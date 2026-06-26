# B14: OCI Helpers Deduplication Report

**Date:** 2026-06-25
**Unit:** B14
**Type:** dedup
**Repo:** KooshaPari/phenotype-infra
**Branch:** dag-B14-2026-06-25

## Summary

This unit audits and completes the extraction of duplicated OCI helper code
across `iac/oci-lottery` and `iac/oci-post-acquire` into a shared
`iac/oci-helpers` crate. The shared crate was created previously (2026-06-24);
this batch completes remaining deduplication opportunities and documents the
final state.

## Shared Crate: `iac/oci-helpers`

**Location:** `iac/oci-helpers/`
**Workspace member:** yes (listed in `iac/Cargo.toml` members)

### Public API

| Function | Signature | Description |
|----------|-----------|-------------|
| `expand_home` | `fn(p: &str) -> PathBuf` | Resolves `~` prefix to `$HOME` |
| `home_dir` | `fn() -> Option<PathBuf>` | Returns `$HOME` or `None` |
| `home_or_fallback` | `fn() -> PathBuf` | Returns `$HOME` or `/tmp` |
| `load_json_or` | `async fn(PathBuf, T) -> Result<T>` | Load JSON or return default |
| `save_json` | `async fn(PathBuf, &T) -> Result<()>` | Save JSON, creating parents |
| `which_on_path` | `async fn(&str) -> bool` | Check binary on `$PATH` via `command -v` |

## Consumer Usage

### `oci-lottery` (Cargo.toml at `iac/oci-lottery/Cargo.toml`)

- **Dependency:** `oci-helpers = { path = "../oci-helpers" }` — **YES, already present**
- **Module usage:**
  - `src/main.rs` — `use oci_helpers::home_or_fallback;`
  - `src/config.rs` — `use oci_helpers::home_or_fallback;`
  - `src/hooks.rs` — `use oci_helpers::which_on_path;`

### `oci-post-acquire` (Cargo.toml at `iac/oci-post-acquire/Cargo.toml`)

- **Dependency:** `oci-helpers = { path = "../oci-helpers" }` — **YES, already present**
- **Module usage:**
  - `src/main.rs` — `use oci_helpers::expand_home;`
  - `src/hooks.rs` — `use oci_helpers::expand_home;`
  - `src/cf.rs` — `use oci_helpers::expand_home;`
  - `src/mesh.rs` — `use oci_helpers::expand_home;`

## Deduplication Completed (this batch)

### 1. `oci-lottery/src/config.rs` — `Config::load_or_default()` → `oci_helpers::load_json_or`

**Before:** 10-line inline implementation using `tokio::fs::try_exists` +
`read_to_string` + `serde_json::from_str`

**After:** Single line delegating to `oci_helpers::load_json_or`

### 2. `oci-lottery/src/state.rs` — `LotteryState::load()` / `save()` / `write_acquired()` → `oci_helpers::load_json_or` / `save_json`

**Before:** Three separate inline implementations duplicating the same
read-json/create-dirs-write-json pattern (27 lines total)

**After:** Three single-line delegations to the shared helpers

### 3. `oci-lottery/src/hooks.rs` — Removed inline `mod which`

**Before:** A private `mod which` (12 lines) providing `which::which()` for
finding the imessage CLI binary. It was redundant with
`oci_helpers::which_on_path`.

**After:** Replaced `which::which(...)` call with
`oci_helpers::which_on_path(...)` and removed the dead module.

### 4. `oci-lottery/src/hooks.rs` — `imessage_relay()` now uses `oci_helpers::which_on_path`

**Before:** `if which::which(&bin).is_err()` using the private mod

**After:** `if !oci_helpers::which_on_path(&bin).await`

## Remaining Duplication (intentional)

The following code exists in both crates but is intentionally **not** shared:

| Pattern | Present in | Reason not to extract |
|---------|-----------|----------------------|
| `imessage_send` (oci-post-acquire) vs `imessage_relay` (oci-lottery) | Both | Different invocation style: post-acquire uses direct `agent-imessage` CLI; lottery uses configurable `AGENT_IMESSAGE_CLI` env var. Different structure. |
| SSH wait loop (`TcpStream::connect` polling) | `oci-post-acquire/src/main.rs` | Single use, tightly coupled to post-acquire's provisioning logic. |
| Ansible runner | `oci-post-acquire/src/main.rs` | Single use, specific to post-acquire. |
| Cloudflare DNS upsert | `oci-post-acquire/src/cf.rs` | Single use, Cloudflare-specific logic. |
| Tailscale enrollment | `oci-post-acquire/src/tailscale.rs` | Single use, Tailscale-specific. |
| OCI CLI wrappers (`list_availability_domains`, `try_launch`, `instance_public_ip`) | `oci-lottery/src/oci.rs` | Lottery-specific OCI CLI interaction. |

## Files Modified (this batch)

| File | Change |
|------|--------|
| `iac/oci-lottery/src/config.rs` | Replaced inline JSON load with `oci_helpers::load_json_or` |
| `iac/oci-lottery/src/state.rs` | Replaced inline JSON I/O with `oci_helpers::load_json_or` / `save_json` |
| `iac/oci-lottery/src/hooks.rs` | Replaced private `mod which` with `oci_helpers::which_on_path`; removed dead module |

## Lines of Duplication Removed

| Location | Lines removed |
|----------|--------------|
| `oci-lottery/src/config.rs` | 7 lines |
| `oci-lottery/src/state.rs` | 24 lines |
| `oci-lottery/src/hooks.rs` | 13 lines |
| **Total** | **44 lines** |

## Static Verification

All changes are mechanically equivalent:
- `load_json_or(path, default)` has the same signature and semantics as the
  inline `try_exists` → `read_to_string` → `serde_json::from_str` path.
- `save_json(path, value)` has the same signature and semantics as the inline
  `create_dir_all` → `to_string_pretty` → `write` path.
- `which_on_path(bin)` is async (unlike the sync `mod which`), so the caller
  needed an `.await` addition — this was done correctly.

## Workspace Configuration

The `iac/Cargo.toml` workspace correctly lists `"oci-helpers"` under members.
Both `oci-lottery` and `oci-post-acquire` have the path dependency. No changes
to `Cargo.toml` or workspace structure were needed.
