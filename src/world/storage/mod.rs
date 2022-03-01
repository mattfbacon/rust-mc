use anyhow::Result;
use std::path::PathBuf;
use uuid::Uuid;

mod naive;

use super::util::{Chunk, ChunkPosition, Entity};

/// Implementations should not buffer any data.
/// All data should be directly read from and written to the underlying storage.
/// Buffering is handled by `World`.
pub trait Storage {
	fn open(root: PathBuf) -> Result<Box<dyn Storage>>
	where
		Self: Sized;

	fn get_entity(&self, uuid: Uuid) -> Result<Option<Entity>>;
	fn set_entity(&mut self, uuid: Uuid, entity: Option<&Entity>) -> Result<()>;

	// YAGNI
	// fn get_block(&self, position: BlockPosition) -> Result<Option<Block>>;
	// fn set_block(&mut self, position: BlockPosition, block: Block) -> Result<()>;

	fn get_chunk(&self, position: ChunkPosition) -> Result<Option<Chunk>>;
	fn set_chunk(&mut self, position: ChunkPosition, data: &Chunk) -> Result<()>;
}
