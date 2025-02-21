use cgmath::{Matrix4, Vector3};

use crate::{engine::{shader::Shader, texture::Texture}, game::{common::coords::{ChunkAxis, ChunkPos}, server::world::chunk::Chunk}};

pub struct TerrainAtlas {
    texture: Texture,
}

impl TerrainAtlas {
    pub const NUM_ITEMS_X: usize = 16;
    pub const NUM_ITEMS_Y: usize = 16;

    pub const ITEM_UV: (f32, f32) = (
        1.0 / Self::NUM_ITEMS_X as f32,
        1.0 / Self::NUM_ITEMS_Y as f32
    );

    pub fn new() -> Self {
        Self {
            texture: Texture::load_from_file(
                "./assets/textures/terrain.png",
                gl::NEAREST,
                gl::CLAMP_TO_EDGE
            ),
        }
    }

    pub fn get_uv(item_id: usize) -> (usize, usize) {
        (
            item_id % Self::NUM_ITEMS_X,
            item_id / Self::NUM_ITEMS_X,
        )
    }

    pub fn texture(&self) -> &Texture {
        &self.texture
    }
}

pub struct Block {
    left_texture: usize,
    right_texture: usize,
    bottom_texture: usize,
    top_texture: usize,
    back_texture: usize,
    front_texture: usize,
}

impl Block {
    pub fn all(texture: usize) -> Self {
        Self {
            left_texture: texture,
            right_texture: texture,
            bottom_texture: texture,
            top_texture: texture,
            back_texture: texture,
            front_texture: texture,
        }
    }
    pub fn side(side_texture: usize, top_texture: usize, bottom_texture: usize) -> Self {
        Self {
            left_texture: side_texture,
            right_texture: side_texture,
            top_texture,
            bottom_texture,
            back_texture: side_texture,
            front_texture: side_texture,
        }
    }
    // pub fn each(
    //     left_texture: usize, right_texture: usize,
    //     bottom_texture: usize, top_texture: usize,
    //     back_texture: usize, front_texture: usize
    // ) -> Self {
    //     Self {
    //         left_texture, right_texture,
    //         bottom_texture, top_texture,
    //         back_texture, front_texture,
    //     }
    // }

    pub fn left_texture(&self) -> usize {
        self.left_texture
    }
    pub fn right_texture(&self) -> usize {
        self.right_texture
    }
    pub fn bottom_texture(&self) -> usize {
        self.bottom_texture
    }
    pub fn top_texture(&self) -> usize {
        self.top_texture
    }
    pub fn back_texture(&self) -> usize {
        self.back_texture
    }
    pub fn front_texture(&self) -> usize {
        self.front_texture
    }
}

pub struct BlockRegistry {
    blocks: Vec<Block>,
}

impl BlockRegistry {
    pub fn create() -> Self {
        Self { blocks: vec![Block::all(0)] }
    }

    pub fn register(&mut self, block: Block) {
        self.blocks.push(block);
    }
    pub fn get(&self, id: usize) -> &Block {
        let block = self.blocks.get(id);
        match block {
            Some(block) => { block },
            None => { &self.blocks[0] }
        }
    }
}

pub struct TerrainShader {
    base: Shader,
}
impl TerrainShader {
    pub const COLOR_TEXTURE_SLOT: u32 = 0;

    pub fn create() -> Self {
        let base = Shader::create("./assets/shaders/terrain.vert", "./assets/shaders/terrain.frag");
        base.bind();
        base.set_int("u_ColorSampler", 0);
        base.set_vec2("u_AtlasScalar", TerrainAtlas::ITEM_UV);
        Shader::unbind();

        Self { base }
    }

    pub fn bind(&self) {
        self.base.bind();
    }
    
    pub fn set_mvp_matrix(&self, matrix: &Matrix4<f32>) {
        self.base.set_mat4("u_MVPMatrix", matrix);
    }
    pub fn set_sun_direction(&self, direction: &Vector3<f32>) {
        self.base.set_vec3("u_SunDirection", (direction.x, direction.y, direction.z));
    }
    pub fn set_sky_color(&self, color: &Vector3<f32>) {
        self.base.set_vec3("u_SkyColor", (color.x, color.y, color.z));
    }
    pub fn set_sun_color(&self, color: &Vector3<f32>) {
        self.base.set_vec3("u_SunColor", (color.x, color.y, color.z));
    }
}

pub struct LineShader {
    base: Shader,
}
impl LineShader {
    pub fn create() -> Self {
        Self { base: Shader::create("./assets/shaders/line.vert", "./assets/shaders/line.frag") }
    }

    pub fn bind(&self) {
        self.base.bind();
    }
    
    pub fn set_project_view_matrix(&self, matrix: &Matrix4<f32>) {
        self.base.set_mat4("u_ProjectViewMatrix", matrix);
    }
    pub fn set_render_offset(&self, offset: &ChunkPos) {
        self.base.set_ivec2("u_RenderOffset", (offset.x * Chunk::WIDTH as ChunkAxis, offset.z * Chunk::LENGTH as ChunkAxis));
    }
}