use crate::game::common::{coords::{BlockAxis, ChunkPos, LocalBlockPos}, world::block_types::BlockTypes};
use super::worldgen::WorldGen;

// TODO: Move common chunk content to common module and make a ServerChunk class here that will inherit common Chunk class.
pub struct Chunk {
    blocks: Box<[u8; Self::VOLUME]>,
    is_dirty: bool,
}

impl Chunk {
    pub const WIDTH: usize = 16;
    pub const HEIGHT: usize = 256;
    pub const LENGTH: usize = 16;

    pub const VOLUME: usize = Self::WIDTH * Self::HEIGHT * Self::LENGTH;

    pub fn create(worldgen: &WorldGen, position: &ChunkPos) -> Self {
        let mut chunk = Self {
            blocks: Box::new([0; Self::VOLUME]),
            is_dirty: true,
        };

        for x in 0..Self::WIDTH {
            for z in 0..Self::LENGTH {
                let block_xz_pos = position.to_block_xz_pos();
                let world_x = x as BlockAxis + block_xz_pos.0;
                let world_z = z as BlockAxis + block_xz_pos.1;

                let height = worldgen.get_height(world_x, world_z).max(0.0) as usize;
                let randoms = [
                    worldgen.get_random(world_x, world_z),
                    worldgen.get_random(world_x + 3824, world_z - 9324),
                ];
                
                for y in 0..height {
                    let mut block = BlockTypes::Stone;
                    let height_influence = y.saturating_sub(60) as f64 / 100.0 * 2.0;

                    if y == 0 {
                        block = BlockTypes::Bedrock;
                    } else if randoms[0].powf(3.0) < height_influence {
                        // Keep block = BlockTypes::Stone
                    } else if height - y <= 1 {
                        block = if randoms[1].powf(4.0) < height_influence { BlockTypes::Dirt } else { BlockTypes::GrassBlock };
                    } else if height - y <= 4 && randoms[0] >= (height - y) as f64 / 4.0 {
                        block = BlockTypes::Dirt;
                    }

                    chunk.set_block(&LocalBlockPos::new(x, y, z), block as u8);
                }
            }
        }

        chunk
    }

    pub fn set_block(&mut self, position: &LocalBlockPos, block: u8) -> bool {
        if position.x >= Self::WIDTH || position.y >= Self::HEIGHT || position.z >= Self::LENGTH {
            return false;
        }

        self.blocks[position.x + position.z * Self::WIDTH + position.y * Self::WIDTH * Self::LENGTH] = block;
        self.mark_dirty();

        true
    }

    pub fn get_block(&self, position: &LocalBlockPos) -> u8 {
        if position.x >= Self::WIDTH || position.y >= Self::HEIGHT || position.z >= Self::LENGTH {
            return 0;
        }

        self.blocks[position.x + position.z * Self::WIDTH + position.y * Self::WIDTH * Self::LENGTH]
    }

    pub fn mark_dirty(&mut self) {
        self.is_dirty = true;
    }
    pub fn mark_clean(&mut self) {
        self.is_dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.is_dirty
    }
}