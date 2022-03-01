use super::Storage;
use crate::world::util::{Chunk, ChunkPosition, Entity};
use anyhow::Result;
use std::fs::File;
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct Naive {
	root: PathBuf,
}

impl Naive {
	fn entity_path(&self, uuid: Uuid) -> PathBuf {
		self.root.join("entities").join(format!("{}.nbt", uuid))
	}
	fn chunk_path(&self, position: ChunkPosition) -> PathBuf {
		self.root.join("chunks").join(format!("{}.nbt", position.to_repr()))
	}
	fn scaffold(path: &Path) -> Result<()> {
		std::fs::create_dir_all(path.join("chunks"))?;
		std::fs::create_dir_all(path.join("entities"))?;
		Ok(())
	}
}

impl Storage for Naive {
	fn open(root: PathBuf) -> Result<Box<dyn Storage>> {
		Self::scaffold(&root)?;
		Ok(Box::new(Self { root }))
	}

	fn get_entity(&self, uuid: Uuid) -> Result<Option<Entity>> {
		let path = self.entity_path(uuid);
		if path.exists() {
			Ok(Some(nbt::from_reader(File::open(path)?)?))
		} else {
			Ok(None)
		}
	}
	fn set_entity(&mut self, uuid: Uuid, entity: Option<&Entity>) -> Result<()> {
		let path = self.entity_path(uuid);
		match entity {
			Some(data) => nbt::to_writer(&mut File::create(path)?, data, None)?,
			None => std::fs::remove_file(path)?,
		}
		Ok(())
	}

	fn get_chunk(&self, position: ChunkPosition) -> Result<Option<Chunk>> {
		let path = self.chunk_path(position);
		if path.exists() {
			Ok(Some(Chunk {
				position,
				blocks: nbt::from_reader(File::open(path)?)?,
			}))
		} else {
			Ok(None)
		}
	}
	fn set_chunk(&mut self, position: ChunkPosition, data: &Chunk) -> Result<()> {
		let path = self.chunk_path(position);
		nbt::to_writer(&mut File::create(path)?, &data.blocks, None)?;
		Ok(())
	}
}
