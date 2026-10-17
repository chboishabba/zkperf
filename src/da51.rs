//! Canonical 0xDA51 address envelope.
//!
//! Wire ABI: `[prefix:16][type:4][data:44]`, prefix `0xDA51`.
//! Historical type codes 0-7 are frozen; type 8 is the theory-atlas payload.
//! A DA51 address is an identity/envelope only.  It does not confer proof,
//! ZK-validity, literature truth, security severity, or authorization.

use std::fmt;
use std::str::FromStr;

pub const PREFIX: u16 = 0xDA51;
pub const DATA_MASK: u64 = 0x0000_0FFF_FFFF_FFFF;
pub const TYPE_MASK: u64 = 0x0000_F000_0000_0000;
pub const PREFIX_MASK: u64 = 0xFFFF_0000_0000_0000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Da51Type {
    MonsterWalk = 0,
    AstNode = 1,
    MonsterProtocol = 2,
    NestedCid = 3,
    HarmonicPath = 4,
    ShardId = 5,
    EigenspaceAddr = 6,
    Hauptmodul = 7,
    AtlasEntry = 8,
}

impl Da51Type {
    pub const fn code(self) -> u8 {
        self as u8
    }

    pub fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            0 => Self::MonsterWalk,
            1 => Self::AstNode,
            2 => Self::MonsterProtocol,
            3 => Self::NestedCid,
            4 => Self::HarmonicPath,
            5 => Self::ShardId,
            6 => Self::EigenspaceAddr,
            7 => Self::Hauptmodul,
            8 => Self::AtlasEntry,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Da51Address(pub u64);

impl Da51Address {
    pub const fn new(type_code: u8, data: u64) -> Self {
        Self(((PREFIX as u64) << 48) | (((type_code as u64) & 0xF) << 44) | (data & DATA_MASK))
    }

    pub const fn from_type(kind: Da51Type, data: u64) -> Self {
        Self::new(kind.code(), data)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }

    pub const fn prefix(self) -> u16 {
        (self.0 >> 48) as u16
    }

    pub const fn type_code(self) -> u8 {
        ((self.0 >> 44) & 0xF) as u8
    }

    pub const fn data(self) -> u64 {
        self.0 & DATA_MASK
    }

    pub const fn has_valid_prefix(self) -> bool {
        self.prefix() == PREFIX
    }

    pub fn known_type(self) -> Option<Da51Type> {
        Da51Type::from_code(self.type_code())
    }

    /// Parse a 64-bit hexadecimal DA51 identity.  Both `0xDA51...` and the
    /// historical `da51...` spelling are accepted.
    pub fn parse_hex(input: &str) -> Result<Self, Da51Error> {
        let body = input
            .trim()
            .strip_prefix("0x")
            .or_else(|| input.trim().strip_prefix("0X"))
            .unwrap_or(input.trim());
        if body.len() != 16 || !body.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err(Da51Error::InvalidHex);
        }
        let raw = u64::from_str_radix(body, 16).map_err(|_| Da51Error::InvalidHex)?;
        let address = Self(raw);
        if !address.has_valid_prefix() {
            return Err(Da51Error::WrongPrefix(address.prefix()));
        }
        Ok(address)
    }

    pub fn canonical_hex(self) -> String {
        format!("0x{:016X}", self.0)
    }

    /// Decode type 8 when present.  Legacy types are returned as `Ok(None)`;
    /// malformed type-8 payloads are rejected rather than reinterpreted.
    pub fn atlas_coord(self) -> Result<Option<AtlasCoord>, Da51Error> {
        if !self.has_valid_prefix() {
            return Err(Da51Error::WrongPrefix(self.prefix()));
        }
        if self.type_code() != Da51Type::AtlasEntry.code() {
            return Ok(None);
        }
        AtlasCoord::decode(self).map(Some)
    }
}

impl fmt::Display for Da51Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "0x{:016X}", self.0)
    }
}

impl FromStr for Da51Address {
    type Err = Da51Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::parse_hex(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasLevel {
    Delta0 = 0,
    Pi1 = 1,
    Sigma1 = 2,
    Pi2 = 3,
    Sigma2 = 4,
    Lean = 5,
}

impl AtlasLevel {
    fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            0 => Self::Delta0,
            1 => Self::Pi1,
            2 => Self::Sigma1,
            3 => Self::Pi2,
            4 => Self::Sigma2,
            5 => Self::Lean,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasBiome {
    MoonshineHighlands = 0,
    EisensteinPlains = 1,
    SpectralSea = 2,
    BridgeMarsh = 3,
    OggArchipelago = 4,
    ScaffoldFoothills = 5,
    GalleryPeaks = 6,
    MachineWorks = 7,
    HaltingPens = 8,
}

impl AtlasBiome {
    fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            0 => Self::MoonshineHighlands,
            1 => Self::EisensteinPlains,
            2 => Self::SpectralSea,
            3 => Self::BridgeMarsh,
            4 => Self::OggArchipelago,
            5 => Self::ScaffoldFoothills,
            6 => Self::GalleryPeaks,
            7 => Self::MachineWorks,
            8 => Self::HaltingPens,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtlasStatus {
    Open = 0,
    ProvedTrue = 1,
    ProvedFalse = 2,
    TrueInLiterature = 3,
    FalseInLiterature = 4,
}

impl AtlasStatus {
    fn from_code(code: u8) -> Option<Self> {
        Some(match code {
            0 => Self::Open,
            1 => Self::ProvedTrue,
            2 => Self::ProvedFalse,
            3 => Self::TrueInLiterature,
            4 => Self::FalseInLiterature,
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtlasCoord {
    pub level: AtlasLevel,
    pub biome: AtlasBiome,
    pub status: AtlasStatus,
    pub serial: u16,
}

impl AtlasCoord {
    pub fn new(
        level: AtlasLevel,
        biome: AtlasBiome,
        status: AtlasStatus,
        serial: u16,
    ) -> Result<Self, Da51Error> {
        if serial >= 4096 {
            return Err(Da51Error::SerialOutOfRange(serial));
        }
        Ok(Self {
            level,
            biome,
            status,
            serial,
        })
    }

    pub const fn data(self) -> u64 {
        ((self.level as u64) << 40)
            | ((self.biome as u64) << 36)
            | ((self.status as u64) << 32)
            | ((self.serial as u64) << 20)
    }

    pub const fn address(self) -> Da51Address {
        Da51Address::from_type(Da51Type::AtlasEntry, self.data())
    }

    pub fn decode(address: Da51Address) -> Result<Self, Da51Error> {
        if !address.has_valid_prefix() {
            return Err(Da51Error::WrongPrefix(address.prefix()));
        }
        if address.type_code() != Da51Type::AtlasEntry.code() {
            return Err(Da51Error::NotAtlas(address.type_code()));
        }
        let data = address.data();
        if data & 0xFFFFF != 0 {
            return Err(Da51Error::NonZeroReserved((data & 0xFFFFF) as u32));
        }
        let level_code = ((data >> 40) & 0xF) as u8;
        let biome_code = ((data >> 36) & 0xF) as u8;
        let status_code = ((data >> 32) & 0xF) as u8;
        let serial = ((data >> 20) & 0xFFF) as u16;
        Ok(Self {
            level: AtlasLevel::from_code(level_code).ok_or(Da51Error::InvalidAtlasLevel(level_code))?,
            biome: AtlasBiome::from_code(biome_code).ok_or(Da51Error::InvalidAtlasBiome(biome_code))?,
            status: AtlasStatus::from_code(status_code).ok_or(Da51Error::InvalidAtlasStatus(status_code))?,
            serial,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Da51Error {
    InvalidHex,
    WrongPrefix(u16),
    NotAtlas(u8),
    InvalidAtlasLevel(u8),
    InvalidAtlasBiome(u8),
    InvalidAtlasStatus(u8),
    SerialOutOfRange(u16),
    NonZeroReserved(u32),
}

impl fmt::Display for Da51Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHex => write!(f, "expected exactly 16 hexadecimal digits"),
            Self::WrongPrefix(p) => write!(f, "wrong DA51 prefix: 0x{p:04X}"),
            Self::NotAtlas(t) => write!(f, "DA51 type {t} is not atlas type 8"),
            Self::InvalidAtlasLevel(v) => write!(f, "invalid atlas level code {v}"),
            Self::InvalidAtlasBiome(v) => write!(f, "invalid atlas biome code {v}"),
            Self::InvalidAtlasStatus(v) => write!(f, "invalid atlas status code {v}"),
            Self::SerialOutOfRange(v) => write!(f, "atlas serial {v} is not below 4096"),
            Self::NonZeroReserved(v) => write!(f, "atlas reserved bits are nonzero: 0x{v:05X}"),
        }
    }
}

impl std::error::Error for Da51Error {}

#[cfg(test)]
mod tests {
    use super::*;

    const GOLDEN: &[(u64, AtlasLevel, AtlasBiome, AtlasStatus, u16)] = &[
        (0xDA51800100000000, AtlasLevel::Delta0, AtlasBiome::MoonshineHighlands, AtlasStatus::ProvedTrue, 0),
        (0xDA51800100100000, AtlasLevel::Delta0, AtlasBiome::MoonshineHighlands, AtlasStatus::ProvedTrue, 1),
        (0xDA51800100200000, AtlasLevel::Delta0, AtlasBiome::MoonshineHighlands, AtlasStatus::ProvedTrue, 2),
        (0xDA51808100300000, AtlasLevel::Delta0, AtlasBiome::HaltingPens, AtlasStatus::ProvedTrue, 3),
        (0xDA51818000400000, AtlasLevel::Pi1, AtlasBiome::HaltingPens, AtlasStatus::Open, 4),
        (0xDA51811100500000, AtlasLevel::Pi1, AtlasBiome::EisensteinPlains, AtlasStatus::ProvedTrue, 5),
        (0xDA51813000600000, AtlasLevel::Pi1, AtlasBiome::BridgeMarsh, AtlasStatus::Open, 6),
        (0xDA51813400700000, AtlasLevel::Pi1, AtlasBiome::BridgeMarsh, AtlasStatus::FalseInLiterature, 7),
        (0xDA51831100800000, AtlasLevel::Pi2, AtlasBiome::EisensteinPlains, AtlasStatus::ProvedTrue, 8),
        (0xDA51838000900000, AtlasLevel::Pi2, AtlasBiome::HaltingPens, AtlasStatus::Open, 9),
        (0xDA51853000A00000, AtlasLevel::Lean, AtlasBiome::BridgeMarsh, AtlasStatus::Open, 10),
        (0xDA51853000B00000, AtlasLevel::Lean, AtlasBiome::BridgeMarsh, AtlasStatus::Open, 11),
        (0xDA51852000C00000, AtlasLevel::Lean, AtlasBiome::SpectralSea, AtlasStatus::Open, 12),
        (0xDA51851100D00000, AtlasLevel::Lean, AtlasBiome::EisensteinPlains, AtlasStatus::ProvedTrue, 13),
        (0xDA51851100E00000, AtlasLevel::Lean, AtlasBiome::EisensteinPlains, AtlasStatus::ProvedTrue, 14),
        (0xDA51852100F00000, AtlasLevel::Lean, AtlasBiome::SpectralSea, AtlasStatus::ProvedTrue, 15),
        (0xDA51852101000000, AtlasLevel::Lean, AtlasBiome::SpectralSea, AtlasStatus::ProvedTrue, 16),
        (0xDA51852101100000, AtlasLevel::Lean, AtlasBiome::SpectralSea, AtlasStatus::ProvedTrue, 17),
        (0xDA51850101200000, AtlasLevel::Lean, AtlasBiome::MoonshineHighlands, AtlasStatus::ProvedTrue, 18),
        (0xDA51850101300000, AtlasLevel::Lean, AtlasBiome::MoonshineHighlands, AtlasStatus::ProvedTrue, 19),
        (0xDA51850101400000, AtlasLevel::Lean, AtlasBiome::MoonshineHighlands, AtlasStatus::ProvedTrue, 20),
        (0xDA51850301500000, AtlasLevel::Lean, AtlasBiome::MoonshineHighlands, AtlasStatus::TrueInLiterature, 21),
        (0xDA51850301600000, AtlasLevel::Lean, AtlasBiome::MoonshineHighlands, AtlasStatus::TrueInLiterature, 22),
    ];

    #[test]
    fn header_extractors_match_16_4_44_layout() {
        let a = Da51Address(0xDA51853000A00000);
        assert_eq!(a.prefix(), 0xDA51);
        assert_eq!(a.type_code(), 8);
        assert_eq!(a.data(), 0x53000A00000);
    }

    #[test]
    fn atlas_golden_addresses_encode_and_decode() {
        for &(raw, level, biome, status, serial) in GOLDEN {
            let coord = AtlasCoord::new(level, biome, status, serial).unwrap();
            assert_eq!(coord.address().raw(), raw);
            assert_eq!(AtlasCoord::decode(Da51Address(raw)).unwrap(), coord);
        }
    }

    #[test]
    fn reserved_bits_are_rejected() {
        let err = AtlasCoord::decode(Da51Address(0xDA51800100000001)).unwrap_err();
        assert_eq!(err, Da51Error::NonZeroReserved(1));
    }

    #[test]
    fn invalid_atlas_fields_are_rejected() {
        assert_eq!(AtlasCoord::decode(Da51Address(0xDA51860100000000)).unwrap_err(), Da51Error::InvalidAtlasLevel(6));
        assert_eq!(AtlasCoord::decode(Da51Address(0xDA51809100000000)).unwrap_err(), Da51Error::InvalidAtlasBiome(9));
        assert_eq!(AtlasCoord::decode(Da51Address(0xDA51800500000000)).unwrap_err(), Da51Error::InvalidAtlasStatus(5));
    }

    #[test]
    fn legacy_trace_identity_is_accepted_without_atlas_reinterpretation() {
        let legacy = Da51Address::parse_hex("da5113c01b0a55f4").unwrap();
        assert_eq!(legacy.type_code(), 1);
        assert_eq!(legacy.known_type(), Some(Da51Type::AstNode));
        assert_eq!(legacy.atlas_coord().unwrap(), None);
    }

    #[test]
    fn unknown_types_remain_unassigned() {
        let a = Da51Address::parse_hex("0xDA51900000000000").unwrap();
        assert_eq!(a.type_code(), 9);
        assert_eq!(a.known_type(), None);
    }

    #[test]
    fn serial_4095_round_trips() {
        let c = AtlasCoord::new(
            AtlasLevel::Lean,
            AtlasBiome::HaltingPens,
            AtlasStatus::FalseInLiterature,
            4095,
        )
        .unwrap();
        assert_eq!(AtlasCoord::decode(c.address()).unwrap(), c);
    }

    #[test]
    fn canonical_hex_is_fixed_width_uppercase() {
        let a = Da51Address::parse_hex("da51800100000000").unwrap();
        assert_eq!(a.canonical_hex(), "0xDA51800100000000");
    }
}
