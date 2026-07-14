# Execution substrate matrix

This inventory records runtime adapters used by the Linux/Apple foundation. It is
not a provider registry and does not authorize any adapter to own cloud state.

| Substrate | Owner | Composition input | State owner | Evidence required |
|---|---|---|---|---|
| NanoVMS | NanoVMS | NanoVMS plan + immutable composition digest | NanoVMS runtime lifecycle only | tier, engine, instance id, lifecycle status |
| Podman | NanoVMS/PhenoCompose adapter | Docker Compose / OCI | local runtime only | backend, host, container id, health |
| Apple Containers | NanoVMS/PhenoCompose adapter | Docker-compatible OCI plan | local Apple runtime only | backend, host, container id, health |
| WSL Containers (first-party extension) | NanoVMS/PhenoCompose adapter | Docker-compatible OCI plan | local WSL runtime only | backend, distro/host, container id, health |

BytePort may record the selected substrate as opaque deployment metadata, but it
remains the only owner of cloud/IaC state. PhenoCompose owns validation and target
rendering. NanoVMS owns runtime selection and lifecycle. substrate/sharecli/phenodag
may orchestrate these handoffs but must not persist provider or runtime state.

Every substrate handoff must carry `Owner`, `Source`, `Verified`, and `Evidence`
fields in the organization inventory. Credentials and provider handles never enter
composition plans or runtime metadata.
