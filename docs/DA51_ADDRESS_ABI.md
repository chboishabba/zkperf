# DA51 address ABI

`zkperf` uses the shared DASHI `0xDA51` 64-bit address envelope:

```text
[prefix:16][type:4][data:44]
prefix = 0xDA51
```

Type codes `0`–`7` are frozen historical assignments. Type `8` is the theory-atlas coordinate payload:

```text
[level:4][biome:4][status:4][serial:12][reserved:20]
```

Canonical type-8 addresses require zero reserved bits and recognized field codes. The Rust implementation lives in `src/da51.rs`; `zkperf-da51` inspects a literal address; and `fixtures/da51_v1.json` is the byte-for-byte cross-repository fixture manifest.

## Authority boundary

A DA51 value is an **identity/envelope**, not a proof result. A trace/shard carrying a DA51 identity does not thereby establish theorem truth, ZK proof validity, security severity, exploitability, or authorization. Existing zkperf witness/CBOR/proof payloads remain authoritative for their own claims.

Historical DA51 trace identities are retained as legacy identities. They are never reinterpreted as atlas coordinates unless their type nibble is exactly `8` and the canonical type-8 decoder accepts the payload.
