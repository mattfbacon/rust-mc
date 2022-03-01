use super::super::position::UnpackedPosition;
use super::super::varint::VarLong;
use super::super::wrappers::{bitvec::PrefixedBitVec, std::*};
use encde::{Decode, Encode, Result as EResult};
use std::io::Write;

#[derive(Encode, Decode)]
pub struct LightUpdateCommon {
	trust_edges: bool,
	sky_light_mask: PrefixedBitVec,
	block_light_mask: PrefixedBitVec,
	empty_sky_light_mask: PrefixedBitVec,
	empty_block_light_mask: PrefixedBitVec,
	sky_light_array: PrefixedVec<SkyLightData>,
	block_light_array: PrefixedVec<BlockLightData>,
}

// TODO implement
// #[derive(Encode, Decode)]
pub struct HeightMaps(());

// TODO implement
// #[derive(Encode, Decode)]
pub struct Blocks(());

// TODO implement
// #[derive(Encode, Decode)]
pub struct BlockEntities(());

/// 2048 u8 backing items, 4096 4-bit entries
#[derive(Encode, Decode)]
pub struct SkyLightData(PrefixedBitVec<u8>);

/// 2048 u8 backing items, 4096 4-bit entries
#[derive(Encode, Decode)]
pub struct BlockLightData(PrefixedBitVec<u8>);

#[derive(Encode, Decode)]
pub struct Position<T: Encode + Decode> {
	pub x: T,
	pub z: T,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct SectionPosition(UnpackedPosition<i32>);

impl Encode for SectionPosition {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let encoded: u64 = ((((self.0.x as u32) & 0x3fffff) as u64) << 42) | (((self.0.y as u32) & 0xfffff) as u64) | ((((self.0.z as u32) & 0x3fffff) as u64) << 20);
		encoded.encode(writer)
	}
}

/// A position within a chunk
#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq)]
pub struct WithinPosition(UnpackedPosition<u8>);

impl WithinPosition {
	fn to_repr(self) -> u16 {
		((self.0.x as u16) << 8) | ((self.0.y as u16) << 4) | (self.0.z as u16)
	}
	fn from_repr(repr: u16) -> Self {
		Self(UnpackedPosition {
			x: ((repr >> 8) & 0xff) as u8,
			y: ((repr >> 4) & 0xff) as u8,
			z: (repr & 0xff) as u8,
		})
	}
}
impl serde::Serialize for WithinPosition {
	fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_repr().serialize(serializer)
	}
}
impl<'de> serde::Deserialize<'de> for WithinPosition {
	fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		serde::Deserialize::deserialize(deserializer).map(Self::from_repr)
	}
}
impl std::hash::Hash for WithinPosition {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
		self.to_repr().hash(hasher)
	}
}

pub struct MultiBlockChangeEntry {
	relative_position: WithinPosition,
	new_block_state: i32,
}

impl Encode for MultiBlockChangeEntry {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		let encoded: u64 = ((self.new_block_state as u64) << 12) | (self.relative_position.to_repr() as u64);
		VarLong(encoded as i64).encode(writer)
	}
}
