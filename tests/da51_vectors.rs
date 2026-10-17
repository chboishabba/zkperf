use serde_json::Value;
use std::fs;
use zkperf::da51::{AtlasCoord, Da51Address};

#[test]
fn shared_manifest_vectors_match_decoder_fields() {
    let text = fs::read_to_string("fixtures/da51_v1.json").unwrap();
    let payload: Value = serde_json::from_str(&text).unwrap();
    let vectors = payload["atlas_vectors"].as_array().unwrap();
    assert_eq!(vectors.len(), 23);

    for vector in vectors {
        let address = Da51Address::parse_hex(vector["address"].as_str().unwrap()).unwrap();
        let coord = AtlasCoord::decode(address).unwrap();
        assert_eq!(address.type_code(), vector["type"].as_u64().unwrap() as u8);
        assert_eq!(coord.level as u8, vector["level"].as_u64().unwrap() as u8);
        assert_eq!(coord.biome as u8, vector["biome"].as_u64().unwrap() as u8);
        assert_eq!(coord.status as u8, vector["status"].as_u64().unwrap() as u8);
        assert_eq!(coord.serial, vector["serial"].as_u64().unwrap() as u16);
        assert_eq!(address.data() & 0xFFFFF, vector["reserved"].as_u64().unwrap());
    }
}

#[test]
fn shared_manifest_declares_identity_authority_boundary() {
    let text = fs::read_to_string("fixtures/da51_v1.json").unwrap();
    let payload: Value = serde_json::from_str(&text).unwrap();
    let authority = &payload["authority"];
    assert_eq!(authority["address_is_identity_only"], true);
    assert_eq!(authority["provenance_is_separate"], true);
    assert_eq!(authority["status_nibble_is_not_external_authority"], true);
}
