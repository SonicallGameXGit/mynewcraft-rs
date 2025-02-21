use std::collections::HashMap;

use cgmath::{MetricSpace, Point2};
use crate::game::{common::{coords::{ChunkPos, Coord}, world::mapping::ChunkPosHasherBuilder}, server::{world::chunk::Chunk, ServerWorld}};
use super::{chunk_mesh::{ChunkData, ChunkMesh, NextChunks}, resources::BlockRegistry};

pub struct ClientWorld {
    chunk_meshes: HashMap<ChunkPos, ChunkMesh, ChunkPosHasherBuilder>,
}

impl ClientWorld {
    pub fn create() -> Self {
        Self { chunk_meshes: HashMap::default() }
    }

    pub fn update(&mut self, block_registry: &BlockRegistry, view_position: &Coord, view_distance: usize, world: &mut ServerWorld) {
        let chunk_view_pos = Point2::new(
            view_position.get_chunk_x() as f64 + (view_position.get_local_x() as f64 / Chunk::WIDTH as f64).round(),
            view_position.get_chunk_z() as f64 + (view_position.get_local_z() as f64 / Chunk::LENGTH as f64).round(),
        );

        let mut chunk_meshes_to_remove: Vec<ChunkPos> = vec![];
        for position in self.chunk_meshes.keys() {
            let chunk = world.get_chunk(position);
            if chunk.is_some() && !chunk.unwrap().is_dirty() {
                continue;
            }

            if chunk_view_pos.distance(Point2::new(position.x as f64, position.z as f64)) > view_distance as f64 {
                chunk_meshes_to_remove.push(position.clone());
            }
        }
        for position in chunk_meshes_to_remove {
            self.chunk_meshes.remove(&position);
        }

        let chunks_to_update: Vec<ChunkPos> = world
            .get_all_chunks()
            .iter()
            .filter(|(position, chunk)| chunk_view_pos.distance(
                Point2::new(position.x as f64, position.z as f64)) <= view_distance as f64 &&
                chunk.is_dirty()
            )
            .map(|(position, _)| position.clone())
            .collect();
        
        if let Some(position) = chunks_to_update.first() {
            if let Some(chunk) = world.get_chunk(position) {
                let mut chunk_data = ChunkData::create();
                chunk_data.build(block_registry, chunk, &NextChunks::create(
                    world.get_chunk(&ChunkPos::new(position.x - 1, position.z)),
                    world.get_chunk(&ChunkPos::new(position.x + 1, position.z)),
                    world.get_chunk(&ChunkPos::new(position.x, position.z - 1)),
                    world.get_chunk(&ChunkPos::new(position.x, position.z + 1)),
                ));
    
                if self.chunk_meshes.contains_key(position) {
                    let chunk_mesh = self.chunk_meshes.get_mut(position).unwrap();
                    chunk_mesh.build(&chunk_data);
                } else {
                    let mut chunk_mesh = ChunkMesh::create();
                    chunk_mesh.build(&chunk_data);
    
                    self.chunk_meshes.insert(ChunkPos::new(position.x, position.z), chunk_mesh);
                }
            }
            if let Some(chunk) = world.get_chunk_mut(position) {
                chunk.mark_clean();
            }
        }
    }

    pub fn get_all_meshes(&self) -> &HashMap<ChunkPos, ChunkMesh, ChunkPosHasherBuilder> {
        &self.chunk_meshes
    }
}