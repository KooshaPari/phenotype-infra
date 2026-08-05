# NanoVMS - Nano Virtual Machine Services

Lightweight, headless VM abstraction for agents — supports desktop, mobile simulators, and emerging form factors.

> **Canonical workspace:** NanoVMS is maintained in
> [`crates/nanovms-core`](../../../). The former
> `KooshaPari/nanovms` checkout is historical provenance. The nested Go module
> keeps its `github.com/kooshapari/nanovms` path for compatibility; that path
> does not change the source-of-truth repository.

## Features

- **Multi-Platform Support**: macOS, Windows, Linux + mobile simulators (iOS, Android, tvOS, watchOS, VisionOS)
- **Headless IDE Support**: Run Android Studio / Xcode in VMs for agent use
- **Multi-Tier VM Architecture**: Native VMs → Container/WSL → MicroVMs (Firecracker)
- **Sandbox Isolation**: gVisor, landlock, seccomp, WASM runtime layers
- **Simulator Abstraction**: Unified interface for iOS Simulator, Android Emulator, tvOS, watchOS, VisionOS

## Quick Start

```bash
# Clone the canonical workspace
git clone https://github.com/KooshaPari/phenotype-infra.git
cd phenotype-infra/crates/nanovms-core

# Build
go build ./cmd/nanovms

# Run
./nanovms --help
```

## Architecture

NanoVMS uses a hexagonal (ports and adapters) architecture:

```
┌─────────────────────────────────────────────────────────────┐
│                      Application Core                        │
│  ┌─────────────────────────────────────────────────────┐  │
│  │                 Domain (Sandbox)                      │  │
│  │  - Sandbox entity                                    │  │
│  │  - Lifecycle management                               │  │
│  │  - Configuration                                     │  │
│  └─────────────────────────────────────────────────────┘  │
│  ┌─────────────────────────────────────────────────────┐  │
│  │                   Ports (Interfaces)                 │  │
│  │  - RuntimePort                                      │  │
│  │  - FilesystemPort                                   │  │
│  │  - NetworkPort                                      │  │
│  └─────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
                          │
        ┌─────────────────┼─────────────────┐
        ▼                 ▼                 ▼
┌───────────────┐ ┌───────────────┐ ┌───────────────┐
│   Mac Adapter │ │ Windows Adapt │ │ Linux Adapter │
│  (Lima/vz)   │ │   (WSL2/gVis)│ │  (gVisor)    │
└───────────────┘ └───────────────┘ └───────────────┘
```

## Platform Support

| Platform | Primary Runtime | Isolation | Status |
|----------|----------------|-----------|--------|
| macOS | Lima/Colima + vz | Namespace | Stable |
| Windows | WSL2 + gVisor | Syscall interception | Stable |
| Linux | gVisor/crun | Syscall filtering | Stable |
| iOS Simulator | Via macOS host | Lima VM | Stable |
| Android Emulator | Headless mode | Via Lima | Stable |

## License

MIT
