# Execution substrate capability probe

`tools/probe-execution-substrates.ps1` is the read-only discovery boundary for
local runtime adapters. It reports installed command surfaces and a bounded
version/health result without starting a VM, creating a container, inspecting
credentials, contacting a cloud provider, or changing runtime state.

Run it from the repository root:

```powershell
pwsh -NoProfile -File tools/probe-execution-substrates.ps1
pwsh -NoProfile -File tools/probe-execution-substrates.ps1 -Json > substrate-capabilities.json
```

The JSON output is schema
`phenotype.infra/execution-substrate-capability/v1` and contains:

| Field | Meaning |
|---|---|
| `observed_utc` | Timestamp for this observation, not a lease or deployment time |
| `host` | Non-secret host identity and architecture |
| `read_only` | Must be `true`; the probe has no lifecycle side effects |
| `credentials_inspected` | Must be `false`; secrets never enter probe output |
| `substrates[]` | One result per Podman, Apple Containers/WSL Containers, and WSL host surface |

Each substrate result uses these statuses:

* `available`: the command answered its bounded probe successfully;
* `installed_unavailable`: a command or shim exists but cannot be used by this
  host without an explicit lifecycle action;
* `missing`: no command surface was found;
* `timeout` or `error`: the probe itself could not establish health;
* `not_applicable`: the host does not expose that substrate (for example WSL on
  macOS).

The result is evidence for adapter selection only. It does not grant an
adapter ownership of container state, provider state, credentials, or
composition semantics. PhenoCompose renders the target, NanoVMS owns runtime
lifecycle, and BytePort owns provider-neutral desired state and mesh inventory.
