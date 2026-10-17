use serde_json::Value;
use std::fs;
use zkperf::da51::{AtlasCoord, Da51Address};

#[test]
fn shared_manifest_atlas_addresses_are_canonical_and_unique() {
    let raw = fs::read_to_string("fixtures/da51_v1.json").unwrap();
    let payload: Value = serde_json::from_str(&raw).unwrap();
    let addresses = payload["atlas_addresses"].as_array().unwrap();

    assert_eq!(addresses.len(), 23);
    let mut seen = std::collections::BTreeSet::new();
    for (serial, value) in addresses.iter().enumerate() {
        let text = value.as_str().unwrap();
        let address: Da51Address = text.parse().unwrap();
        let coord = AtlasCoord::decode(address).unwrap();
        assert_eq!(coord.serial as usize, serial);
        assert!(seen.insert(address.raw()));
    }
}

#[test]
fn shared_manifest_max_serial_round_trips() {
    let raw = fs::read_to_string("fixtures/da51_v1.json").unwrap();
    let payload: Value = serde_json::from_str(&raw).unwrap();
    let address: Da51Address = payload["max_serial_address"].as_str().unwrap().parse().unwrap();
    let coord = AtlasCoord::decode(address).unwrap();

    assert_eq!(coord.serial, 4095);
    assert_eq!(coord.address(), address);
}
