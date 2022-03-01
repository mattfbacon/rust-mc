use super::{Chunk, ChunkPosition, Entity, Generator};
use std::collections::HashMap;
use uuid::Uuid;

pub struct Void;

impl Generator for Void {
	fn new(_figment: figment::Figment) -> anyhow::Result<Box<dyn Generator>> {
		Ok(Box::new(Self))
	}
	fn generate_chunk(&self, position: ChunkPosition) -> (Chunk, HashMap<Uuid, Entity>) {
		(Chunk { position, blocks: HashMap::new() }, HashMap::new())
	}
}
