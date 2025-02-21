use cgmath::{Matrix4, Vector3};
use gl::types::{GLsizei, GLsizeiptr, GLuint};

use crate::game::{common::coords::{ChunkAxis, ChunkPos, LocalBlockPos}, server::world::chunk::Chunk};

use super::resources::{BlockRegistry, TerrainAtlas, TerrainShader};

pub struct NextChunks<'a> {
    left: Option<&'a Chunk>,
    right: Option<&'a Chunk>,
    back: Option<&'a Chunk>,
    front: Option<&'a Chunk>,
}

impl<'a> NextChunks<'a> {
    pub fn create(
        left: Option<&'a Chunk>,
        right: Option<&'a Chunk>,
        back: Option<&'a Chunk>,
        front: Option<&'a Chunk>,
    ) -> Self {
        Self { left, right, back, front }
    }
}

#[repr(C)]
struct Vertex {
    pub data: u32,
}

#[repr(u8)]
enum Face {
    Left, Right,
    Bottom, Top,
    Back, Front,
}

impl Vertex {
    pub fn create(position: &LocalBlockPos, uv: (usize, usize), face: Face) -> Self {
        Self {
            data: 
                 (position.x &   0x1F) as u32        |
                ((position.y &  0x1FF) as u32) <<  5 |
                ((position.z &   0x1F) as u32) << 14 |
                ((      uv.0 &   0x1F) as u32) << 19 |
                ((      uv.1 &   0x1F) as u32) << 24 |
                 (face as u32 &   0x5)         << 29
        }
    }
}

pub struct ChunkData {
    vertices: Vec<Vertex>,
}

impl ChunkData {
    pub fn create() -> Self {
        Self {
            vertices: vec![]
        }
    }

    fn put_vertex(&mut self, vertex: Vertex) {
        self.vertices.push(vertex);
    }

    // TODO: Replace face building on CPU with GPU (just put the block position for each vertex and then calculate faces on GPU using gl_VertexID)
    pub fn build(&mut self, block_registry: &BlockRegistry, chunk: &Chunk, next_chunks: &NextChunks) {
        self.vertices.clear();

        for x in 0..Chunk::WIDTH {
            for y in 0..Chunk::HEIGHT {
                for z in 0..Chunk::LENGTH {
                    let block = chunk.get_block(&LocalBlockPos::new(x, y, z));
                    if block == 0 {
                        continue;
                    }

                    let block = block_registry.get(block as usize);

                    if if x == 0 {
                        next_chunks.left.is_none() || next_chunks.left.unwrap().get_block(&LocalBlockPos::new(Chunk::WIDTH - 1, y, z)) == 0
                    } else {
                        chunk.get_block(&LocalBlockPos::new(x - 1, y, z)) == 0
                    } {
                        let (u0, v0) = TerrainAtlas::get_uv(block.left_texture());
                        let (u1, v1) = (u0 + 1, v0 + 1);
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z), (u0, v0), Face::Left));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z + 1), (u1, v0), Face::Left));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z), (u0, v1), Face::Left));
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z + 1), (u1, v1), Face::Left));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z), (u0, v1), Face::Left));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z + 1), (u1, v0), Face::Left));
                    }
                    
                    if if x >= Chunk::WIDTH - 1 {
                        next_chunks.right.is_none() || next_chunks.right.unwrap().get_block(&LocalBlockPos::new(0, y, z)) == 0
                    } else {
                        chunk.get_block(&LocalBlockPos::new(x + 1, y, z)) == 0
                    } {
                        let (u0, v0) = TerrainAtlas::get_uv(block.right_texture());
                        let (u1, v1) = (u0 + 1, v0 + 1);
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z + 1), (u0, v0), Face::Right));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z), (u1, v0), Face::Right));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z + 1), (u0, v1), Face::Right));
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z), (u1, v1), Face::Right));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z + 1), (u0, v1), Face::Right));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z), (u1, v0), Face::Right));
                    }
                    
                    if y == 0 || chunk.get_block(&LocalBlockPos::new(x, y - 1, z)) == 0 {
                        let (u0, v0) = TerrainAtlas::get_uv(block.bottom_texture());
                        let (u1, v1) = (u0 + 1, v0 + 1);
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z), (u0, v0), Face::Bottom));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z), (u1, v0), Face::Bottom));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z + 1), (u0, v1), Face::Bottom));
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z + 1), (u1, v1), Face::Bottom));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z + 1), (u0, v1), Face::Bottom));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z), (u1, v0), Face::Bottom));
                    }
                    
                    if y >= Chunk::HEIGHT - 1 || chunk.get_block(&LocalBlockPos::new(x, y + 1, z)) == 0 {
                        let (u0, v0) = TerrainAtlas::get_uv(block.top_texture());
                        let (u1, v1) = (u0 + 1, v0 + 1);
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z + 1), (u0, v0), Face::Top));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z + 1), (u1, v0), Face::Top));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z), (u0, v1), Face::Top));
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z), (u1, v1), Face::Top));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z), (u0, v1), Face::Top));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z + 1), (u1, v0), Face::Top));
                    }
                    
                    if if z == 0 {
                        next_chunks.back.is_none() || next_chunks.back.unwrap().get_block(&LocalBlockPos::new(x, y, Chunk::LENGTH - 1)) == 0
                    } else {
                        chunk.get_block(&LocalBlockPos::new(x, y, z - 1)) == 0
                    } {
                        let (u0, v0) = TerrainAtlas::get_uv(block.back_texture());
                        let (u1, v1) = (u0 + 1, v0 + 1);
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z), (u1, v0), Face::Back));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z), (u0, v0), Face::Back));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z), (u1, v1), Face::Back));
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z), (u0, v1), Face::Back));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z), (u1, v1), Face::Back));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z), (u0, v0), Face::Back));
                    }
                    
                    if if z >= Chunk::LENGTH - 1 {
                        next_chunks.front.is_none() || next_chunks.front.unwrap().get_block(&LocalBlockPos::new(x, y, 0)) == 0
                    } else {
                        chunk.get_block(&LocalBlockPos::new(x, y, z + 1)) == 0
                    } {
                        let (u0, v0) = TerrainAtlas::get_uv(block.front_texture());
                        let (u1, v1) = (u0 + 1, v0 + 1);
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y, z + 1), (u0, v0), Face::Front));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z + 1), (u1, v0), Face::Front));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z + 1), (u0, v1), Face::Front));
                    
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y + 1, z + 1), (u1, v1), Face::Front));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x, y + 1, z + 1), (u0, v1), Face::Front));
                        self.put_vertex(Vertex::create(&LocalBlockPos::new(x + 1, y, z + 1), (u1, v0), Face::Front));
                    }
                }
            }
        }
    }
}

pub struct ChunkMesh {
    vao: GLuint,
    vbo: GLuint,
    num_vertices: GLsizeiptr,
}

impl ChunkMesh {
    pub fn create() -> Self {
        unsafe {
            let mut vao: GLuint = 0;
            gl::CreateVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            
            let mut vbo: GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribIPointer(0, 1, gl::UNSIGNED_INT, size_of::<Vertex>() as GLsizei, std::ptr::null());

            Self {
                vao,
                vbo,
                num_vertices: 0,
            }
        }
    }

    pub fn render(&self, position: &ChunkPos, project_view_matrix: &Matrix4<f32>, shader: &TerrainShader) {
        if self.num_vertices <= 0 {
            return;
        }
        
        shader.set_mvp_matrix(&(
            project_view_matrix *
            Matrix4::from_translation(Vector3::new(
                (position.x * Chunk::WIDTH as ChunkAxis) as f32,
                0.0,
                (position.z * Chunk::LENGTH as ChunkAxis) as f32
            ))
        ));

        unsafe {
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::TRIANGLES, 0, self.num_vertices as GLsizei);
        }
    }

    pub fn build(&mut self, chunk_data: &ChunkData) {
        self.num_vertices = chunk_data.vertices.len() as GLsizeiptr;

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (chunk_data.vertices.len() * size_of::<Vertex>()) as GLsizeiptr,
                chunk_data.vertices.as_ptr() as *const std::ffi::c_void,
                gl::STATIC_DRAW
            );
        }
    }
}

impl Drop for ChunkMesh {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}