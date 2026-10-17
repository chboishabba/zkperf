# DA51 address ABI

`zkperf` is a runtime producer for the shared DASHI `0xDA51` 64-bit address envelope. The normative formal owner is `dashi_lean4/DA51/Core.lean`.

```text
[prefix:16][type:4][data:44]
prefix = 0xDA51
```

Type codes `0`–`7` are frozen historical assignments. Type `8` is the theory-atlas coordinate payload:

```text
[level:4][biome:4][status:4][serial:12][reserved:20]
```

Canonical type-8 addresses require zero reserved bits and recognized field codes. The Rust codec lives in `src/da51.rs`; `zkperf-da51` inspects a literal address; and `fixtures/da51_v1.json` is the cross-repository semantic fixture manifest.

## Identity, semantics and provenance

DA51 has three distinct layers:

1. the immutable 64-bit address identity;
2. type-specific interpretation of the 44 data bits;
3. producer-owned provenance naming an artifact about that address.

`src/provenance.rs` defines `Da51EvidenceRef { address, producer, artifact, kind }` for the third layer. Evidence kinds are proof, trace, benchmark, source and receipt. The envelope keeps provenance explicit instead of overloading the address bits.

## Authority boundary

A DA51 value or provenance edge is **not** a proof result. It does not itself establish theorem truth, literature truth, ZK proof validity, security severity, exploitability, or authorization. Existing zkperf witness/CBOR/proof payloads remain authoritative for their own claims.

Historical DA51 trace identities are retained as legacy identities. They are never reinterpreted as atlas coordinates unless their type nibble is exactly `8` and the canonical type-8 decoder accepts the payload.

The shared fixture additionally pins decoded vectors for all 23 atlas addresses and explicitly declares that address identity, provenance and external authority remain separate.
