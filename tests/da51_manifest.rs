use serde_json::Value;
use std::fs;
use zkperf::da51::{AtlasCoord, Da51Address, Da51Error};

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
fn shared_manifest_negative_addresses_are_rejected() {
    let raw = fs::read_to_string("fixtures/da51_v1.json").unwrap();
    let payload: Value = serde_json::from_str(&raw).unwrap();
    let negative = payload["negative"].as_object().unwrap();

    for (name, value) in negative {
        let text = value.as_str().unwrap();
        let expected = match name.as_str() {
            "invalid_biome" => Da51Error::InvalidAtlasBiome(9),
            "invalid_level" => Da51Error::InvalidAtlasLevel(6),
            "invalid_status" => Da51Error::InvalidAtlasStatus(5),
            "nonzero_reserved" => Da51Error::NonZeroReserved(1),
            "unknown_type" => Da51Error::NotAtlas(9),
            "wrong_prefix" => Da51Error::WrongPrefix(0xCAFE),
            _ => panic!("unexpected negative fixture: {name}"),
        };
        let actual = match text.parse::<Da51Address>() {
            Err(error) => error,
            Ok(address) => AtlasCoord::decode(address).unwrap_err(),
        };
        assert_eq!(actual, expected, "negative case {name}");
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
