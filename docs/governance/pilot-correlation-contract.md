# Cross-repository pilot correlation contract

This contract defines the audit-only handoff identifiers for a
PhenoCompose -> BytePort -> NanoVMS run. It does not create a second control
plane and does not move credentials or mutable provider/runtime state into
phenotype-infra.

```yaml
render_digest: <PhenoCompose deterministic render digest>
workload_id: <BytePort opaque workload identifier>
sandbox_id: <NanoVMS opaque sandbox identifier>
verified_utc: <YYYY-MM-DDThh:mm:ssZ>
status: not-run | rendered | submitted | running | succeeded | failed
```

Handoff invariants:

1. `render_digest` is produced by PhenoCompose and is immutable for the run.
2. BytePort records the `render_digest` as an opaque request reference and
   returns `workload_id`; it remains authoritative for infrastructure state.
3. NanoVMS receives the workload's execution request and returns `sandbox_id`;
   it remains authoritative for lifecycle and health state.
4. A missing identifier means that handoff was not run or failed. It must not
   be inferred from a neighboring receipt.
5. Receipts may include URLs or commit SHAs, but never credentials, provider
   state snapshots, or mutable runtime records.

The live pilot is not considered complete until one append-only receipt carries
all three identifiers, exact component heads, authenticated request outcomes,
and UTC timestamps for each handoff.
