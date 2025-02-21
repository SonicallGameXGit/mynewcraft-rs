use std::{hash::{Hash, Hasher}, ops::{Add, AddAssign, Sub}};

use cgmath::Vector3;

use crate::game::server::world::chunk::Chunk;

pub type ChunkAxis = i32;
pub type BlockAxis = i64;
pub type CoordAxis = f64;

pub type LocalCoordAxis = f32;
pub type LocalBlockAxis = usize;

#[derive(Clone)]
pub struct ChunkPos {
    pub x: ChunkAxis,
    pub z: ChunkAxis,
}
impl ChunkPos {
    pub fn new(x: ChunkAxis, z: ChunkAxis) -> Self {
        Self { x, z }
    }

    pub fn to_block_xz_pos(&self) -> (BlockAxis, BlockAxis) {
        (
            self.x as BlockAxis * Chunk::WIDTH as BlockAxis,
            self.z as BlockAxis * Chunk::LENGTH as BlockAxis,
        )
    }
}
impl Add<ChunkPos> for &ChunkPos {
    type Output = ChunkPos;
    
    fn add(self, rhs: ChunkPos) -> Self::Output {
        ChunkPos::new(self.x + rhs.x, self.z + rhs.z)
    }
}
impl Sub<&ChunkPos> for &ChunkPos {
    type Output = ChunkPos;
    
    fn sub(self, rhs: &ChunkPos) -> Self::Output {
        ChunkPos::new(self.x - rhs.x, self.z - rhs.z)
    }
}
impl PartialEq for ChunkPos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.z == other.z
    }
}
impl Eq for ChunkPos {}
impl Hash for ChunkPos {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.z.hash(state);
    }
}

pub struct BlockPos {
    pub x: BlockAxis,
    pub y: BlockAxis,
    pub z: BlockAxis,
}
impl BlockPos {
    pub fn new(x: BlockAxis, y: BlockAxis, z: BlockAxis) -> Self {
        Self { x, y, z }
    }

    pub fn to_chunk_pos(&self) -> ChunkPos {
        ChunkPos::new(
            self.x.div_euclid(Chunk::WIDTH as BlockAxis) as ChunkAxis,
            self.z.div_euclid(Chunk::WIDTH as BlockAxis) as ChunkAxis,
        )
    }
}

pub struct LocalBlockPos {
    pub x: LocalBlockAxis,
    pub y: LocalBlockAxis,
    pub z: LocalBlockAxis,
}
impl LocalBlockPos {
    pub fn new(x: LocalBlockAxis, y: LocalBlockAxis, z: LocalBlockAxis) -> Self {
        Self { x, y, z }
    }
}

pub struct Coord {
    chunk_x: ChunkAxis, chunk_z: ChunkAxis,
    frac_x: LocalCoordAxis, frac_z: LocalCoordAxis,

    y: CoordAxis,
}

impl Coord {
    pub fn new(x: CoordAxis, y: CoordAxis, z: CoordAxis) -> Self {
        let chunk_x = (x / Chunk::WIDTH as f64).floor() as ChunkAxis;
        let chunk_z = (z / Chunk::LENGTH as f64).floor() as ChunkAxis;

        Self {
            chunk_x, chunk_z,

            frac_x: (x - (chunk_x as BlockAxis * Chunk::WIDTH as BlockAxis) as CoordAxis) as LocalCoordAxis,
            frac_z: (z - (chunk_z as BlockAxis * Chunk::LENGTH as BlockAxis) as CoordAxis) as LocalCoordAxis,

            y,
        }
    }

    pub fn get_block_x(&self) -> BlockAxis {
        self.chunk_x as BlockAxis * Chunk::WIDTH as BlockAxis + self.frac_x as BlockAxis
    }
    pub fn get_block_y(&self) -> BlockAxis {
        self.y as BlockAxis
    }
    pub fn get_block_z(&self) -> BlockAxis {
        self.chunk_z as BlockAxis * Chunk::LENGTH as BlockAxis + self.frac_z as BlockAxis
    }

    pub fn get_frac_x(&self) -> LocalCoordAxis {
        self.frac_x.fract()
    }
    pub fn get_frac_y(&self) -> LocalCoordAxis {
        self.y.fract() as LocalCoordAxis
    }
    pub fn get_frac_z(&self) -> LocalCoordAxis {
        self.frac_z.fract()
    }

    pub fn get_world_x(&self) -> CoordAxis {
        self.chunk_x as CoordAxis * Chunk::WIDTH as CoordAxis + self.frac_x as CoordAxis
    }
    pub fn get_world_y(&self) -> CoordAxis {
        self.y
    }
    pub fn get_world_z(&self) -> CoordAxis {
        self.chunk_z as CoordAxis * Chunk::LENGTH as CoordAxis + self.frac_z as CoordAxis
    }

    pub fn get_local_x(&self) -> LocalCoordAxis {
        self.frac_x
    }
    pub fn get_local_z(&self) -> LocalCoordAxis {
        self.frac_z
    }

    pub fn get_chunk_x(&self) -> ChunkAxis {
        self.chunk_x
    }
    pub fn get_chunk_z(&self) -> ChunkAxis {
        self.chunk_z
    }
}

impl AddAssign<Vector3<CoordAxis>> for Coord {
    fn add_assign(&mut self, rhs: Vector3<CoordAxis>) {
        let new_x = self.frac_x as CoordAxis + rhs.x;
        let new_z = self.frac_z as CoordAxis + rhs.z;
        
        let chunk_dx = (new_x / Chunk::WIDTH as CoordAxis).floor() as ChunkAxis;
        let chunk_dz = (new_z / Chunk::LENGTH as CoordAxis).floor() as ChunkAxis;

        self.chunk_x += chunk_dx;
        self.chunk_z += chunk_dz;

        self.frac_x = (new_x - chunk_dx as CoordAxis * Chunk::WIDTH as CoordAxis) as LocalCoordAxis;
        self.frac_z = (new_z - chunk_dz as CoordAxis * Chunk::LENGTH as CoordAxis) as LocalCoordAxis;

        self.y += rhs.y;
    }
}