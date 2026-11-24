# ARTEX defensive observation / zkPerf witness efficacy roadmap

Status: planned, 2026-11-24. Companion to [zkSEC M15](https://github.com/chboishabba/zkSEC/blob/docs/artex-defensive-efficacy-20261124/docs/MILESTONE_15.md).

## P0 deliverables
- WP15-A: map local sanitized observation IDs to stable run_id, t_index, pid/tid, operation and provenance fields of zkSEC TraceProposalFact; retain unresolved linkage rather than infer identity.
- WP15-B: independently measure benign synthetic workloads and validate trace provenance, capture boundaries, missing events and measurement overhead. Reuse src/witness.rs, zkperf-witness, examples/zkperf-chain.py and existing witness fixtures; audit their actual semantics before promotion.
- WP15-C: paired observer collision and incremental evidence benchmark, with ground-truth labels held independently of producer outputs.
- WP15-D: export deterministic replay receipts and uncertainty/coverage ledger for zkSEC M15-G. No severity or authority conclusions from trace data.
- WP15-E (P1): selective disclosure of bounded invariant claims, bind artifact identity, capture scope and policy version, and verify negative controls. Distinguish commitment validity, proof validity and measurement trust.

## Independent efficacy protocol (P0)
Adapt the dual-graph, iterative observation and coverage methodology described in [ARTEX Global](https://github.com/jiwoochris/artex-global/blob/main/README.md#system-architecture) and its [defensive guide](https://github.com/jiwoochris/artex-global/blob/main/docs/defense-en.md). Attribution: upstream ARTEX / Autumn-27 and global edition / jiwoochris, AGPL-3.0. This is methodology reuse, not imported code or a claim of equivalent capability.

Use only offline synthetic fixtures and prerecorded sanitized observations; no autonomous offensive execution, live targets, real credentials or outbound network access. Independent fixture oracle declares expected invariants and labels before evaluation. Freeze corpus, seeds, producer versions, budget and expected outcomes; separate development and held-out sets. Compare fixed baseline, observation-only, witness-only, grounded fusion and full policy evaluation on the same fixtures. Iterative rounds may select additional pre-existing fixture evidence, never execute external tools. Record both graphs, anchors, coverage frontier and unresolved residuals at every round.

Report counts and denominators: true/false positives, true/false negatives, unresolved positives/negatives separately; precision, recall (including unresolved positives in denominator), abstention, coverage, decision latency, witness overhead, reproducibility and evidence completeness. Undefined ratios remain undefined. Compare paired cases and report uncertainty; no superiority claim without measured results. Include benign matched controls, missing/conflicting/duplicate evidence, stale timestamps, invalid commitments, identity ambiguity, and unsupported authority assertions. Mandatory acceptance: zero unauthorized promotions, no network/process execution introduced into zkSEC, deterministic replay, no silent residual deletion, and no false claims of complete coverage. Detection accuracy targets to be preregistered before held-out evaluation.

Observer collisions: if O(s1)=O(s2) for different oracle labels, test whether independent W separates the pair; if not, preserve unresolved. Report collision counts before/after fusion rather than assuming additional telemetry implies knowledge. H(t,k) retains asset identities, events, witnesses, grounding and residuals; t denotes source time, k refinement round, distinct from ingest time. Signed verdict -1 violation, 0 unresolved, +1 admissible applies only to a named invariant and scope; retain typed zero reasons. A proof over a commitment does not prove capture authenticity or completeness.

## Boundary
No live offensive tooling or external target traffic in this benchmark. No blockchain publication or remote transfer required. Existing proof artifacts are donors, not proof of efficacy. No tests executed by this roadmap update.
