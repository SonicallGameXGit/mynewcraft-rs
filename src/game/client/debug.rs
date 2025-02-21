use std::{ffi::c_void, mem::offset_of};
use gl::types::{GLsizei, GLsizeiptr, GLuint};

use crate::game::common::coords::{Coord, CoordAxis};

#[repr(C)]
struct Vertex {
    position: [f32; 3],
    color: u32,
}

pub struct LineDebug {
    // vertices: HashMap<ChunkPos, Vec<Vertex>, ChunkPosHasherBuilder>,
    vertices: Vec<Vertex>,
    
    vao: GLuint,
    vbo: GLuint,

    picked_color: u32,
}

impl LineDebug {
    pub fn new() -> Self {
        unsafe {
            let mut vao: GLuint = 0;
            gl::CreateVertexArrays(1, &mut vao);
            gl::BindVertexArray(vao);
            
            let mut vbo: GLuint = 0;
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(
                0, 3,
                gl::FLOAT, gl::FALSE,
                size_of::<Vertex>() as GLsizei,
                std::ptr::null(),
            );

            gl::EnableVertexAttribArray(1);
            gl::VertexAttribIPointer(
                1, 1,
                gl::UNSIGNED_INT,
                size_of::<Vertex>() as GLsizei,
                offset_of!(Vertex, color) as *const c_void,
            );

            Self { vertices: vec![], vao, vbo, picked_color: 0x000000ff }
        }
    }

    pub fn color_hex(&mut self, color: u32) {
        self.picked_color = color;
    }
    // pub fn color_rgbaf(&mut self, color: (f32, f32, f32, f32)) {
    //     let red = (color.0.clamp(0.0, 1.0) * 255.0) as u32;
    //     let green = (color.1.clamp(0.0, 1.0) * 255.0) as u32;
    //     let blue = (color.2.clamp(0.0, 1.0) * 255.0) as u32;
    //     let alpha = (color.3.clamp(0.0, 1.0) * 255.0) as u32;

    //     self.picked_color = (red << 24) | (green << 16) | (blue << 8) | alpha;
    // }
    // pub fn color_rgbai(&mut self, color: (u8, u8, u8, u8)) {
    //     self.picked_color = ((color.0 as u32) << 24) | ((color.1 as u32) << 16) | ((color.2 as u32) << 8) | color.3 as u32;
    // }
    // pub fn color_rgbf(&mut self, color: (f32, f32, f32)) {
    //     let red = (color.0.clamp(0.0, 1.0) * 255.0) as u32;
    //     let green = (color.1.clamp(0.0, 1.0) * 255.0) as u32;
    //     let blue = (color.2.clamp(0.0, 1.0) * 255.0) as u32;

    //     self.picked_color = (red << 24) | (green << 16) | (blue << 8) | u8::MAX as u32;
    // }
    // pub fn color_rgbi(&mut self, color: (u8, u8, u8)) {
    //     self.picked_color = (*color.0 as u32) << 24) | ((color.1 as u32) << 16) | ((color.2 as u32) << 8) | u8::MAX as u32;
    // }

    // TODO: Start using chunk position for hash-mapping and local positions for offset
    fn basic_line(&mut self, from: &(CoordAxis, CoordAxis, CoordAxis), to: &(CoordAxis, CoordAxis, CoordAxis)) {
        self.vertices.push(Vertex { position: [from.0 as f32, from.1 as f32, from.2 as f32], color: self.picked_color });
        self.vertices.push(Vertex { position: [to.0 as f32, to.1 as f32, to.2 as f32], color: self.picked_color });
    }
    
    // pub fn line(&mut self, from: &Coord, to: &Coord) {
    //     self.basic_line(
    //         &(from.get_world_x(), from.get_world_y(), from.get_world_z()),
    //         &(to.get_world_x(), to.get_world_y(), to.get_world_z()),
    //     );
    // }
    pub fn cube(&mut self, position: &Coord, size: &(CoordAxis, CoordAxis, CoordAxis)) {
        let x0 = position.get_world_x();
        let y0 = position.get_world_y();
        let z0 = position.get_world_z();

        let x1 = x0 + size.0;
        let y1 = y0 + size.1;
        let z1 = z0 + size.2;

        self.basic_line(&(x0, y0, z0), &(x1, y0, z0));
        self.basic_line(&(x1, y0, z0), &(x1, y1, z0));
        self.basic_line(&(x1, y1, z0), &(x0, y1, z0));
        self.basic_line(&(x0, y1, z0), &(x0, y0, z0));

        self.basic_line(&(x0, y0, z1), &(x1, y0, z1));
        self.basic_line(&(x1, y0, z1), &(x1, y1, z1));
        self.basic_line(&(x1, y1, z1), &(x0, y1, z1));
        self.basic_line(&(x0, y1, z1), &(x0, y0, z1));

        self.basic_line(&(x0, y0, z0), &(x0, y0, z1));
        self.basic_line(&(x1, y0, z0), &(x1, y0, z1));
        self.basic_line(&(x1, y1, z0), &(x1, y1, z1));
        self.basic_line(&(x0, y1, z0), &(x0, y1, z1));
    }
    pub fn render_all(&mut self) {
        if self.vertices.is_empty() {
            return;
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER,
                (self.vertices.len() * size_of::<Vertex>()) as GLsizeiptr,
                self.vertices.as_ptr() as *const std::ffi::c_void,
                gl::DYNAMIC_DRAW
            );
            
            gl::BindVertexArray(self.vao);
            gl::DrawArrays(gl::LINES, 0, self.vertices.len() as GLsizei);
        }

        self.vertices.clear();
    }
}

impl Drop for LineDebug {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteBuffers(1, &self.vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}