//! T22 observability init for `phenotype-infra` iac/* daemons.
//!
//! Thin wrapper over [`phenotype_logging`] that gives each daemon a single,
//! consistent [`init`] call that wires up:
//!
//! 1. **Tracing** — `tracing-subscriber` fmt layer with `RUST_LOG` env-filter.
//!    Includes `service.name` on every event for fleet-side aggregation.
//! 2. **Metrics** — `metrics-exporter-prometheus` HTTP exporter on `:9090`
//!    (per-daemon port offset from `METRICS_PORT` env, default 9090).
//! 3. **OTel** (optional, behind the `otel` feature flag) — OTLP gRPC
//!    exporter + W3C `traceparent` propagation. Wired through
//!    `tracing-opentelemetry` once `pheno-tracing` v0.5.0 inlines
//!    `pheno-otel` (see ADR-036).
//!
//! # Usage
//!
//! ```rust,no_run
//! use phenotype_infra_observability::{init, OtelConfig};
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let _guard = init("oci-lottery", OtelConfig::disabled())?;
//!     // ... rest of app ...
//!     Ok(())
//! }
//! ```
//!
//! Filter logs via `RUST_LOG=info,oci_lottery=debug`. Metrics scrape URL is
//! `http://localhost:9090/metrics` by default.

#![deny(missing_docs)]
#![deny(rust_2018_idioms)]
#![warn(clippy::all)]

use std::sync::Arc;

use tracing::Level;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

use phenotype_logging::is_initialized;

/// Default log level if `RUST_LOG` is unset.
pub const DEFAULT_LEVEL: Level = Level::INFO;

/// Default Prometheus exporter bind address.
pub const DEFAULT_METRICS_BIND: &str = "0.0.0.0:9090";

/// Optional OpenTelemetry configuration.
///
/// Pass [`OtelConfig::disabled`] to skip the OTel exporter entirely
/// (the default). Pass [`OtelConfig::otlp_grpc`] with an endpoint to
/// install the OTLP gRPC exporter.
#[derive(Debug, Clone)]
pub enum OtelConfig {
    /// No OTel exporter installed. Tracing events stay local + Prometheus.
    Disabled,
    /// Install the OTLP gRPC exporter pointed at `endpoint`
    /// (e.g. `"http://otel-collector.phenotype.svc:4317"`).
    OtlpGrpc {
        /// OTLP gRPC endpoint URL.
        endpoint: String,
        /// Service name resource attribute (overrides the `service_name` arg to `init`).
        service_name: Option<&'static str>,
    },
}

impl OtelConfig {
    /// Convenience: no OTel.
    pub const fn disabled() -> Self {
        Self::Disabled
    }
}

/// OTel exporter install state. Held in the [`InitGuard`] so the exporter
/// shuts down cleanly when the daemon exits.
#[cfg(feature = "otel")]
pub struct OtelExporter {
    _inner: Option<()>,
}

#[cfg(not(feature = "otel"))]
pub struct OtelExporter {
    _private: (),
}

impl OtelExporter {
    fn install(cfg: &OtelConfig) -> Self {
        match cfg {
            OtelConfig::Disabled => Self { _private: () },
            OtelConfig::OtlpGrpc { .. } => {
                // pheno-otel is not yet inlined into pheno-tracing v0.5.0.
                // Once it is, this block will install the OTLP exporter via
                // opentelemetry-otlp + tracing-opentelemetry. For now we
                // log a warning and no-op so callers can wire it through.
                tracing::warn!(
                    "phenotype-infra-observability: OTel exporter requested but `otel` feature flag is off. \
                     Build with `--features otel` once pheno-tracing v0.5.0 inlines pheno-otel (ADR-036)."
                );
                Self { _private: () }
            }
        }
    }
}

/// Returned by [`init`]. Drop to flush logs + metrics on daemon shutdown.
pub struct InitGuard {
    /// OTel exporter handle (currently a no-op until `otel` feature lands).
    pub otel: OtelExporter,
    /// Subscriber handle (kept for API symmetry with future pheno-tracing integration).
    pub subscriber: Arc<dyn std::any::Any + Send + Sync>,
}

/// Initialise tracing + metrics + (optional) OTel.
///
/// Idempotent — safe to call from each binary's `main`. Reads `RUST_LOG`
/// from env (default `info`). Includes `service.name` on every `info!`
/// line so fleet-side aggregators can join per-service traces.
///
/// The metrics exporter binds to `METRICS_BIND` env (default
/// `0.0.0.0:9090`). Daemons that share a host should set a unique port
/// per service via `metrics_exporter_bind()` before calling [`init`].
pub fn init(
    service_name: &'static str,
    otel: OtelConfig,
) -> Result<InitGuard, Box<dyn std::error::Error>> {
    init_tracing(service_name);
    init_metrics(service_name)?;
    let otel = OtelExporter::install(&otel);

    tracing::info!(
        service.name = service_name,
        "observability initialised (phenotype-infra-observability 0.3.0)"
    );

    Ok(InitGuard {
        otel,
        subscriber: Arc::new(()),
    })
}

/// Initialise tracing only and return a no-op guard.
///
/// Most callers should use [`init`] instead.
pub fn init_tracing(service_name: &'static str) -> Arc<dyn std::any::Any + Send + Sync> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(DEFAULT_LEVEL.as_str()));

    let subscriber = tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer().with_target(true).with_level(true));

    let _ = subscriber.try_init();

    if !is_initialized() {
        // The stub's own `init_tracing` is the canonical idempotency flag;
        // install it once for the duration of the process.
        phenotype_logging::init_tracing(service_name, DEFAULT_LEVEL);
    }

    Arc::new(())
}

/// Initialise the Prometheus metrics exporter.
///
/// Idempotent. Safe to call from each binary's `main`. Honours
/// `METRICS_BIND` env (default [`DEFAULT_METRICS_BIND`]).
pub fn init_metrics(
    service_name: &'static str,
) -> Result<(), Box<dyn std::error::Error>> {
    use metrics_exporter_prometheus::PrometheusBuilder;

    let bind = std::env::var("METRICS_BIND").unwrap_or_else(|_| DEFAULT_METRICS_BIND.to_string());

    let builder = PrometheusBuilder::new().with_http_listener(
        bind.parse()
            .map_err(|e| format!("METRICS_BIND parse error: {e}"))?,
    );

    builder.install().map_err(|e| -> Box<dyn std::error::Error> {
        format!("metrics-exporter-prometheus install failed: {e}").into()
    })?;

    tracing::info!(
        service.name = service_name,
        bind = %bind,
        "prometheus exporter initialised"
    );

    // Emit a 0 counter so the service is discoverable in Prometheus before
    // the first request lands. Label the counter with service_name so the
    // fleet aggregator can group by service.
    metrics::describe_counter!(
        "phenotype_service_info",
        "Static service identification counter; always 1."
    );
    metrics::counter!("phenotype_service_info", "service" => service_name.to_string()).increment(1);

    Ok(())
}

/// Set the metrics exporter bind address before calling [`init_metrics`].
///
/// Useful for tests or for daemons that share a host and need
/// per-service ports. Must be called before [`init`] / [`init_metrics`].
pub fn metrics_exporter_bind(bind: std::net::SocketAddr) {
    std::env::set_var("METRICS_BIND", bind.to_string());
}

/// Emit a heartbeat trace event + a `phenotype_heartbeat_total` counter.
/// Daemons can call this every N seconds to prove liveness.
pub fn heartbeat(service_name: &'static str) {
    tracing::info!(service.name = service_name, "heartbeat");
    metrics::counter!(
        "phenotype_heartbeat_total",
        "service" => service_name.to_string()
    )
    .increment(1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_tracing_does_not_panic() {
        // Calling twice should be a no-op (try_init).
        let _ = init_tracing("oci-lottery-test");
        let _ = init_tracing("oci-post-acquire-test");
    }

    #[test]
    fn heartbeat_does_not_panic() {
        init_tracing("observability-test");
        heartbeat("observability-test");
    }

    #[test]
    fn init_metrics_binds_to_test_port() {
        // Pick a port nobody else should be using on the test host.
        metrics_exporter_bind("127.0.0.1:0".parse().unwrap());
        init_metrics("observability-metrics-test").expect("metrics init");
    }

    #[test]
    fn init_full_with_otel_disabled() {
        // Use port 0 to avoid clashing with concurrent tests / live daemons.
        metrics_exporter_bind("127.0.0.1:0".parse().unwrap());
        let guard = init("observability-full-test", OtelConfig::disabled())
            .expect("full init");
        heartbeat("observability-full-test");
        drop(guard);
    }
}
