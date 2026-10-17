//! Authority-neutral DA51 provenance envelopes.
//!
//! These records bind a DA51 identity to a producer-owned artifact. They do
//! not assert theorem truth, ZK validity, security severity, exploitability,
//! authorization, or literature authority.

use crate::da51::Da51Address;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EvidenceKind {
    Proof,
    Trace,
    Benchmark,
    Source,
    Receipt,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Da51EvidenceRef {
    pub address: Da51Address,
    pub producer: String,
    pub artifact: String,
    pub kind: EvidenceKind,
}

impl Da51EvidenceRef {
    pub fn new(
        address: Da51Address,
        producer: impl Into<String>,
        artifact: impl Into<String>,
        kind: EvidenceKind,
    ) -> Result<Self, ProvenanceError> {
        let producer = producer.into().trim().to_owned();
        let artifact = artifact.into().trim().to_owned();
        if producer.is_empty() {
            return Err(ProvenanceError::EmptyProducer);
        }
        if artifact.is_empty() {
            return Err(ProvenanceError::EmptyArtifact);
        }
        Ok(Self { address, producer, artifact, kind })
    }

    pub fn identity_token(&self) -> String {
        format!("da51:{}", self.address.canonical_hex().to_ascii_lowercase())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RuntimeEvidenceLink {
    pub evidence: Da51EvidenceRef,
    pub demand_ref: Option<String>,
    pub obligation_ref: Option<String>,
    pub artifact_revision: Option<String>,
    pub statement_digest: Option<String>,
}

impl RuntimeEvidenceLink {
    pub fn new(
        evidence: Da51EvidenceRef,
        demand_ref: Option<String>,
        obligation_ref: Option<String>,
        artifact_revision: Option<String>,
        statement_digest: Option<String>,
    ) -> Result<Self, ProvenanceError> {
        for value in [&demand_ref, &obligation_ref, &artifact_revision, &statement_digest] {
            if value.as_deref().is_some_and(|item| item.trim().is_empty()) {
                return Err(ProvenanceError::EmptyOptionalLink);
            }
        }
        Ok(Self {
            evidence,
            demand_ref: demand_ref.map(|item| item.trim().to_owned()),
            obligation_ref: obligation_ref.map(|item| item.trim().to_owned()),
            artifact_revision: artifact_revision.map(|item| item.trim().to_owned()),
            statement_digest: statement_digest.map(|item| item.trim().to_owned()),
        })
    }

    #[must_use]
    pub const fn establishes_formal_proof(&self) -> bool { false }

    #[must_use]
    pub const fn establishes_security_conclusion(&self) -> bool { false }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceError {
    EmptyProducer,
    EmptyArtifact,
    EmptyOptionalLink,
}

impl std::fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyProducer => write!(f, "DA51 evidence producer must not be empty"),
            Self::EmptyArtifact => write!(f, "DA51 evidence artifact must not be empty"),
            Self::EmptyOptionalLink => write!(f, "DA51 evidence optional link must not be empty when present"),
        }
    }
}

impl std::error::Error for ProvenanceError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn provenance_keeps_identity_and_artifact_separate() {
        let address = Da51Address::parse_hex("0xDA51853000A00000").unwrap();
        let evidence = Da51EvidenceRef::new(address, "zkperf", "sha256:0123456789abcdef", EvidenceKind::Trace).unwrap();
        assert_eq!(evidence.address, address);
        assert_eq!(evidence.producer, "zkperf");
        assert_eq!(evidence.artifact, "sha256:0123456789abcdef");
        assert_eq!(evidence.identity_token(), "da51:0xda51853000a00000");
    }

    #[test]
    fn provenance_requires_explicit_producer_and_artifact() {
        let address = Da51Address::parse_hex("0xDA51853000A00000").unwrap();
        assert_eq!(Da51EvidenceRef::new(address, "", "receipt:1", EvidenceKind::Receipt), Err(ProvenanceError::EmptyProducer));
        assert_eq!(Da51EvidenceRef::new(address, "zkperf", "", EvidenceKind::Receipt), Err(ProvenanceError::EmptyArtifact));
    }

    #[test]
    fn benchmark_linked_to_formal_demand_is_not_formal_proof_or_security_conclusion() {
        let address = Da51Address::parse_hex("0xDA51853000A00000").unwrap();
        let evidence = Da51EvidenceRef::new(address, "zkperf", "run:benchmark:1", EvidenceKind::Benchmark).unwrap();
        let link = RuntimeEvidenceLink::new(
            evidence,
            Some("formal-demand:1".into()),
            Some("proof-obligation:1".into()),
            Some("run-revision:1".into()),
            Some("sha256:statement-a".into()),
        ).unwrap();
        assert!(!link.establishes_formal_proof());
        assert!(!link.establishes_security_conclusion());
    }

    #[test]
    fn trace_linked_to_same_da51_object_remains_observational() {
        let address = Da51Address::parse_hex("0xDA51853000A00000").unwrap();
        let evidence = Da51EvidenceRef::new(address, "zkperf", "trace:1", EvidenceKind::Trace).unwrap();
        let link = RuntimeEvidenceLink::new(evidence, None, None, None, None).unwrap();
        assert!(!link.establishes_formal_proof());
        assert!(!link.establishes_security_conclusion());
    }
}
