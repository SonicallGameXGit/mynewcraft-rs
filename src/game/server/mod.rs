pub mod world;

use std::collections::HashMap;
use world::{chunk::Chunk, worldgen::WorldGen};

use super::common::{coords::{BlockAxis, BlockPos, ChunkAxis, ChunkPos, CoordAxis, LocalBlockAxis, LocalBlockPos}, world::{block_types::BlockTypes, mapping::ChunkPosHasherBuilder}};

pub struct ServerWorld {
    chunks: HashMap<ChunkPos, Chunk, ChunkPosHasherBuilder>,
    worldgen: WorldGen,
}

impl ServerWorld {
    pub fn create(seed: u32) -> Self {
        Self {
            chunks: HashMap::default(),
            worldgen: WorldGen::create(seed)
        }
    }

    // TODO: Move "load_region" to client side and just ask for server to load certain chunk.
    pub fn load_region(&mut self, position: &ChunkPos, radius: usize) {
        let iradius = radius as ChunkAxis;

        let mut chunks_to_remove: Vec<ChunkPos> = vec![];
        for chunk in self.chunks.keys() {
            let difference = chunk - position;
            if ((difference.x * difference.x + difference.z * difference.z) as CoordAxis).sqrt() > radius as CoordAxis {
                chunks_to_remove.push(chunk.clone());
            }
        }

        for chunk_pos in chunks_to_remove {
            if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x - 1, chunk_pos.z)) { chunk.mark_dirty(); }
            if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x + 1, chunk_pos.z)) { chunk.mark_dirty(); }
            if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x, chunk_pos.z - 1)) { chunk.mark_dirty(); }
            if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x, chunk_pos.z + 1)) { chunk.mark_dirty(); }
            
            self.chunks.remove(&chunk_pos);
        }

        for x in -iradius..iradius + 1 {
            for z in -iradius..iradius + 1 {
                if ((x * x + z * z) as f32).sqrt() > radius as f32 {
                    continue;
                }

                let chunk_pos = position + ChunkPos::new(x, z);
                if self.chunks.contains_key(&chunk_pos) {
                    continue;
                }

                if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x - 1,        chunk_pos.z)) { chunk.mark_dirty(); }
                if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x + 1,        chunk_pos.z)) { chunk.mark_dirty(); }
                if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(       chunk_pos.x, chunk_pos.z - 1)) { chunk.mark_dirty(); }
                if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(       chunk_pos.x, chunk_pos.z + 1)) { chunk.mark_dirty(); }

                self.chunks.insert(chunk_pos.clone(), Chunk::create(&self.worldgen, &chunk_pos));
            }
        }
    }

    pub fn get_chunk(&self, position: &ChunkPos) -> Option<&Chunk> {
        self.chunks.get(position)
    }
    pub fn get_chunk_mut(&mut self, position: &ChunkPos) -> Option<&mut Chunk> {
        self.chunks.get_mut(position)
    }

    pub fn get_all_chunks(&self) -> &HashMap<ChunkPos, Chunk, ChunkPosHasherBuilder> {
        &self.chunks
    }

    pub fn set_block(&mut self, position: &BlockPos, block: u8) -> bool {
        let chunk_pos = position.to_chunk_pos();

        match self.chunks.get_mut(&chunk_pos) {
            Some(chunk) => {
                let local_x = (position.x - chunk_pos.x as BlockAxis * Chunk::WIDTH as BlockAxis) as LocalBlockAxis;
                let local_z = (position.z - chunk_pos.z as BlockAxis * Chunk::LENGTH as BlockAxis) as LocalBlockAxis;

                if !chunk.set_block(&LocalBlockPos::new(local_x, position.y as LocalBlockAxis, local_z), block) {
                    return false;
                }

                if local_x == 0 {
                    if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x - 1, chunk_pos.z)) { chunk.mark_dirty(); }
                } else if local_x >= Chunk::WIDTH - 1 {
                    if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x + 1, chunk_pos.z)) { chunk.mark_dirty(); }
                }

                if local_z == 0 {
                    if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x, chunk_pos.z - 1)) { chunk.mark_dirty(); }
                } else if local_z >= Chunk::LENGTH - 1 {
                    if let Some(chunk) = self.chunks.get_mut(&ChunkPos::new(chunk_pos.x, chunk_pos.z + 1)) { chunk.mark_dirty(); }
                }

                true
            },
            None => { false },
        }
    }
    pub fn get_block(&self, position: &BlockPos) -> u8 {
        let chunk_pos = position.to_chunk_pos();

        match self.chunks.get(&chunk_pos) {
            Some(chunk) => {
                chunk.get_block(&LocalBlockPos::new(
                    (position.x - chunk_pos.x as BlockAxis * Chunk::WIDTH as i64) as LocalBlockAxis,
                    position.y as LocalBlockAxis,
                    (position.z - chunk_pos.z as BlockAxis * Chunk::LENGTH as i64) as LocalBlockAxis,
                ))
            },
            None => { BlockTypes::Air as u8 },
        }
    }
}