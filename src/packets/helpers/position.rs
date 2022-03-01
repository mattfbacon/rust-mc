use encde::{Decode, Encode, Result as EResult};
use std::io::{Read, Write};

#[derive(Encode, Decode, PartialEq, Eq, Hash, Clone, Copy)]
pub struct UnpackedPosition<T> {
	pub x: T,
	pub y: T,
	pub z: T,
}
pub type F32Position = UnpackedPosition<f32>;
pub type F64Position = UnpackedPosition<f64>;

// XXX is it better to store the position as packed or unpacked?
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct PackedPosition {
	x: i32,
	y: i16,
	z: i32,
}
impl PackedPosition {
	pub fn new(x: i32, y: i16, z: i32) -> Self {
		Self { x, y, z }
	}
	pub fn to_repr(self) -> u64 {
		// 26 MSBs = x
		// 26 middle bits = z
		// 12 LSBs = y
		(((self.x as u32 & 0x3ffffff) as u64) << 38) | (((self.z as u32 & 0x3fffff) as u64) << 12) | ((self.y as u16 & 0xfff) as u64)
	}
	pub fn from_repr(repr: u64) -> Self {
		Self {
			x: (repr >> 38) as i32,
			y: (repr & 0xfff) as i16,
			z: ((repr >> 12) & 0x3ffffff) as i32,
		}
	}
}
impl serde::Serialize for PackedPosition {
	fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
		self.to_repr().serialize(serializer)
	}
}
impl<'de> serde::Deserialize<'de> for PackedPosition {
	fn deserialize<D: serde::de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
		serde::Deserialize::deserialize(deserializer).map(Self::from_repr)
	}
}
impl std::hash::Hash for PackedPosition {
	fn hash<H: std::hash::Hasher>(&self, hasher: &mut H) {
		self.to_repr().hash(hasher)
	}
}
impl Encode for PackedPosition {
	fn encode(&self, writer: &mut dyn Write) -> EResult<()> {
		self.to_repr().encode(writer)
	}
}
impl Decode for PackedPosition {
	fn decode(reader: &mut dyn Read) -> EResult<Self> {
		Decode::decode(reader).map(Self::from_repr)
	}
}

/// coordinates are in eighths
/// TODO write constructor once use case is known
#[derive(Encode, Decode)]
pub struct EffectPosition(UnpackedPosition<i32>);
