use crate::packets::helpers::game::chunk::WithinPosition;
use crate::packets::helpers::position::PackedPosition;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type Seed = u64;

/// TODO
#[derive(Serialize, Deserialize)]
pub struct Entity {}

/// May be increased to u64 at some point
pub type BlockStateId = u32;

pub type BlockPosition = crate::packets::helpers::position::UnpackedPosition<i32>;
pub type ChunkPosition = PackedPosition;

pub struct Chunk {
	pub position: ChunkPosition,
	pub blocks: HashMap<WithinPosition, BlockStateId>,
}

pub struct IsDirty<T> {
	pub(super) is_dirty: bool,
	pub(super) data: T,
}
