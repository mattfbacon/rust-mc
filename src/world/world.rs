use anyhow::Result;
use std::collections::HashMap;
use uuid::Uuid;

use super::generator::Generator;
use super::storage::Storage;
use super::util::{Chunk, ChunkPosition, Entity, IsDirty};

pub struct World {
	chunks: HashMap<ChunkPosition, IsDirty<Chunk>>,
	entities: HashMap<Uuid, IsDirty<Option<Entity>>>,
	generator: Box<dyn Generator>,
	underlying: Box<dyn Storage>,
}

impl World {
	pub fn new(underlying: Box<dyn Storage>, generator: Box<dyn Generator>) -> Self {
		Self {
			chunks: Default::default(),
			entities: Default::default(),
			generator,
			underlying,
		}
	}

	/// This function does not set the dirty flag, to allow for correct `get_chunk` behavior.
	fn _get_chunk(&mut self, position: ChunkPosition) -> Result<&mut IsDirty<Chunk>> {
		if !self.chunks.contains_key(&position) {
			let chunk = self.underlying.get_chunk(position)?;
			if let Some(chunk) = chunk {
				self.chunks.insert(position, IsDirty { is_dirty: false, data: chunk });
			} else {
				let (chunk, entities) = self.generator.generate_chunk(position);
				self.chunks.insert(position, IsDirty { is_dirty: true, data: chunk });
				for (entity_id, entity) in entities.into_iter() {
					self.entities.insert(entity_id, IsDirty { is_dirty: true, data: Some(entity) });
				}
			}
		}
		Ok(self.chunks.get_mut(&position).unwrap())
	}
	pub fn get_chunk_mut(&mut self, position: ChunkPosition) -> Result<&mut Chunk> {
		let chunk = self._get_chunk(position)?;
		chunk.is_dirty = true;
		Ok(&mut chunk.data)
	}
	pub fn get_chunk(&mut self, position: ChunkPosition) -> Result<&Chunk> {
		Ok(&self._get_chunk(position)?.data)
	}

	/// This function does not set the dirty flag, to allow for correct `get_entity` behavior.
	fn _get_entity(&mut self, uuid: Uuid) -> Result<Option<&mut IsDirty<Option<Entity>>>> {
		if self.entities.contains_key(&uuid) {
			Ok(Some(self.entities.get_mut(&uuid).unwrap()))
		} else if let Some(entity) = self.underlying.get_entity(uuid)? {
			self.entities.insert(uuid, IsDirty { is_dirty: false, data: Some(entity) });
			Ok(Some(self.entities.get_mut(&uuid).unwrap()))
		} else {
			Ok(None)
		}
	}
	pub fn get_entity(&mut self, uuid: Uuid) -> Result<Option<&Entity>> {
		Ok(match self._get_entity(uuid)?.map(|entity| &entity.data) {
			None => None,
			Some(maybe_entity) => maybe_entity.as_ref(),
		})
	}
	pub fn get_entity_mut(&mut self, uuid: Uuid) -> Result<Option<&mut Entity>> {
		match self._get_entity(uuid)? {
			None => Ok(None),
			Some(IsDirty { ref mut data, ref mut is_dirty }) => {
				*is_dirty = true;
				Ok(data.as_mut())
			}
		}
	}

	pub fn flush(&mut self) -> Result<()> {
		for (&uuid, IsDirty { ref mut is_dirty, data: entity }) in self.entities.iter_mut() {
			self.underlying.set_entity(uuid, entity.as_ref())?;
			*is_dirty = false;
		}
		for (&position, IsDirty { data: chunk, is_dirty }) in self.chunks.iter_mut() {
			self.underlying.set_chunk(position, &chunk)?;
			*is_dirty = false;
		}
		Ok(())
	}
}

impl Drop for World {
	fn drop(&mut self) {
		self.flush().expect("Flushing world data");
	}
}
