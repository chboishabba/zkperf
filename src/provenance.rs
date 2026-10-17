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
        Ok(Self {
            address,
            producer,
            artifact,
            kind,
        })
    }

    pub fn identity_token(&self) -> String {
        format!("da51:{}", self.address.canonical_hex().to_ascii_lowercase())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProvenanceError {
    EmptyProducer,
    EmptyArtifact,
}

impl std::fmt::Display for ProvenanceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyProducer => write!(f, "DA51 evidence producer must not be empty"),
            Self::EmptyArtifact => write!(f, "DA51 evidence artifact must not be empty"),
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
        let evidence = Da51EvidenceRef::new(
            address,
            "zkperf",
            "sha256:0123456789abcdef",
            EvidenceKind::Trace,
        )
        .unwrap();

        assert_eq!(evidence.address, address);
        assert_eq!(evidence.producer, "zkperf");
        assert_eq!(evidence.artifact, "sha256:0123456789abcdef");
        assert_eq!(evidence.identity_token(), "da51:0xda51853000a00000");
    }

    #[test]
    fn provenance_requires_explicit_producer_and_artifact() {
        let address = Da51Address::parse_hex("0xDA51853000A00000").unwrap();
        assert_eq!(
            Da51EvidenceRef::new(address, "", "receipt:1", EvidenceKind::Receipt),
            Err(ProvenanceError::EmptyProducer)
        );
        assert_eq!(
            Da51EvidenceRef::new(address, "zkperf", "", EvidenceKind::Receipt),
            Err(ProvenanceError::EmptyArtifact)
        );
    }
}
