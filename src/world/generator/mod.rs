use super::util::{Chunk, ChunkPosition, Entity};
use std::collections::HashMap;
use uuid::Uuid;

pub mod flat;
pub mod void;

pub trait Generator {
	fn new(config: figment::Figment) -> anyhow::Result<Box<dyn Generator>>
	where
		Self: Sized;

	fn generate_chunk(&self, position: ChunkPosition) -> (Chunk, HashMap<Uuid, Entity>);
}
