mod engine;
mod game;
mod camera;

use std::f32;

use camera::Camera;
use cgmath::InnerSpace;
use cgmath::Point3;
use cgmath::Vector3;
use cgmath::Zero;
use engine::texture::Texture;
use engine::timer::Timer;
use engine::window::*;
use engine::shader::*;
use game::client::debug::LineDebug;
use game::client::resources::Block;
use game::client::resources::BlockRegistry;
use game::client::resources::LineShader;
use game::client::resources::TerrainAtlas;
use game::client::resources::TerrainShader;
use game::client::world::ClientWorld;
use game::common::coords::BlockPos;
use game::common::coords::ChunkPos;
use game::common::coords::Coord;
use game::common::world::block_types::BlockTypes;
use game::server::ServerWorld;
use game::server::world::worldgen::WorldGen;

struct RayHitInfo {
    position: Point3<i64>,
    normal: Vector3<i64>,
}

fn raycast(world: &ServerWorld, origin: &Coord, direction: &Vector3<f32>, max_distance: f32) -> Option<RayHitInfo> {
    let step = direction.map(|v| v.signum() as i64);
    let delta = direction.map(|v| {
        if v != 0.0 {
            (1.0 / v).abs()
        } else {
            f32::INFINITY
        }
    });

    let mut position = Point3::new(origin.get_block_x(), origin.get_block_y(), origin.get_block_z());
    let mut tmax = Point3::new(
        if step.x > 0 { 1.0 - origin.get_frac_x() } else { origin.get_frac_x() } * delta.x,
        if step.y > 0 { 1.0 - origin.get_frac_y() } else { origin.get_frac_y() } * delta.y,
        if step.z > 0 { 1.0 - origin.get_frac_z() } else { origin.get_frac_z() } * delta.z
    );

    let mut traveled_distance = 0.0f32;
    let mut normal = Vector3::<f32>::zero();

    while traveled_distance < max_distance {
        if position.y >= u16::MIN as i64 && position.y <= u16::MAX as i64 && world.get_block(&BlockPos::new(position.x, position.y, position.z)) != BlockTypes::Air as u8 {
            return Some(RayHitInfo {
                position,
                normal: normal.normalize().map(|v| v as i64),
            })
        }

        if tmax.x < tmax.y {
            if tmax.x < tmax.z {
                position.x += step.x;
                normal = Vector3::unit_x() * -step.x as f32;

                traveled_distance = tmax.x;
                tmax.x += delta.x;
            } else {
                position.z += step.z;
                normal = Vector3::unit_z() * -step.z as f32;

                traveled_distance = tmax.z;
                tmax.z += delta.z;
            }
        } else if tmax.y < tmax.z {
            position.y += step.y;
            normal = Vector3::unit_y() * -step.y as f32;

            traveled_distance = tmax.y;
            tmax.y += delta.y;
        } else {
            position.z += step.z;
            normal = Vector3::unit_z() * -step.z as f32;

            traveled_distance = tmax.z;
            tmax.z += delta.z;
        }
    }

    None
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let vsync = args.contains(&"--vsync".to_string());
    if vsync { println!("Using vsync."); }

    let mut max_fps = WindowBuilder::NO_MAX_FPS;
    for arg in args {
        if let Some(max_fps_arg) = arg.strip_prefix("--max-fps=") {
            if let Ok(max_fps_eval) = meval::eval_str(max_fps_arg) {
                max_fps = max_fps_eval as u32;
            }
        }
    }

    if max_fps != WindowBuilder::NO_MAX_FPS { println!("Max fps set to: {}.", max_fps) }

    let mut window = WindowBuilder::new()
        .with_title("MyFirstOpenGLGame")
        .with_size(1920, 1080)
        .with_vsync(vsync)
        .with_max_fps(max_fps)
        .build();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);
        gl::Enable(gl::CULL_FACE);

        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

        gl::LineWidth(5.0);

        // gl::PolygonMode(gl::FRONT_AND_BACK, gl::LINE);
        gl::Enable(gl::POLYGON_OFFSET_FILL);
        gl::PolygonOffset(2.5, 1.0);
    }

    let mut terrain_shader = TerrainShader::create();
    let mut line_shader = LineShader::create();

    let mut line_debug = LineDebug::new();
    let terrain_atlas = TerrainAtlas::new();

    let mut block_registry = BlockRegistry::create();
    block_registry.register(Block::all(1)); // BlockTypes::Dirt
    block_registry.register(Block::side(3, 2, 1)); // BlockTypes::GrassBlock
    block_registry.register(Block::all(4)); // BlockTypes::Stone
    block_registry.register(Block::all(5)); // BlockTypes::Cobblestone
    block_registry.register(Block::all(6)); // BlockTypes::Bedrock
    block_registry.register(Block::all(7)); // BlockTypes::Gravel
    block_registry.register(Block::all(8)); // BlockTypes::Sand

    let mut server_world = ServerWorld::create(WorldGen::random_seed());
    let mut client_world = ClientWorld::create();

    const VIEW_DISTANCE: usize = 24;

    const SKY_COLOR: Vector3<f32> = Vector3::new(0.08, 0.47, 0.8);
    const SUN_COLOR: Vector3<f32> = Vector3::new(1.23, 1.18, 1.1);

    let mut camera = Camera::create();
    // camera.position += Vector3::new(0.0, 0.0, 30000000.0);
    unsafe { gl::ClearColor(SKY_COLOR.x, SKY_COLOR.y, SKY_COLOR.z, 1.0); }

    let mut timer = Timer::create();
    
    let mut fps_timer = 0.0f32;
    let mut fps_counter = 0u64;

    window.grab_mouse();
    while window.is_running() {
        window.poll_events();
        timer.update();

        fps_timer += timer.get_delta();
        fps_counter += 1;

        if fps_timer >= 1.0 {
            println!("FPS: {}.", fps_counter);

            fps_timer = 0.0;
            fps_counter = 0;
        }

        if window.is_key_just_pressed(glfw::Key::Escape) {
            window.toggle_mouse();
        }
        if window.is_key_just_pressed(glfw::Key::R) {
            terrain_shader = TerrainShader::create();
            line_shader = LineShader::create();
        }

        if window.is_mouse_grabbed() {
            camera.fly(&window, &timer);
        }
        camera.update(90.0, window.get_aspect(), 0.01, 1500.0);

        // TODO: Add world serialization/deserialization
        const MAX_REACH_DISTANCE: f32 = 5.0;
        if let Some(hit_info) = raycast(&server_world, &camera.position, camera.get_front(), MAX_REACH_DISTANCE) {
            // Draw outline

            // let time = timer.get_time() * f32::consts::PI;
            // let red = ((time.sin() + 1.0) * 127.5) as u8;  // Oscillates between 0 and 255
            // let green = (((time + 2.0).sin() + 1.0) * 127.5) as u8;
            // let blue = (((time + 4.0).sin() + 1.0) * 127.5) as u8;
            // line_debug.color_rgbai((red, green, blue, 255));

            line_debug.color_hex(0x00000088);
            line_debug.cube(
                &Coord::new(hit_info.position.x as f64, hit_info.position.y as f64, hit_info.position.z as f64),
                &(1.0, 1.0, 1.0),
            );

            // Break / Place blocks
            if window.is_mouse_button_just_pressed(glfw::MouseButton::Left) {
                server_world.set_block(&BlockPos::new(hit_info.position.x, hit_info.position.y, hit_info.position.z), BlockTypes::Air as u8);
            }
            if window.is_mouse_button_just_pressed(glfw::MouseButton::Right) {
                if let Some(mut hit_info) = raycast(&server_world, &camera.position, camera.get_front(), MAX_REACH_DISTANCE) {
                    if !hit_info.normal.is_zero() {
                        hit_info.position += hit_info.normal;
                        server_world.set_block(&BlockPos::new(hit_info.position.x, hit_info.position.y, hit_info.position.z), BlockTypes::Cobblestone as u8);
                    }
                };
            }
        };

        server_world.load_region(&ChunkPos::new(camera.position.get_chunk_x(), camera.position.get_chunk_z()), VIEW_DISTANCE / 2);
        client_world.update(&block_registry, &camera.position, VIEW_DISTANCE / 2, &mut server_world);

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        
        terrain_shader.bind();
        terrain_shader.set_sun_direction(&Vector3::new(-0.3, -1.0, 0.6).normalize());
        terrain_shader.set_sky_color(&SKY_COLOR);
        terrain_shader.set_sun_color(&SUN_COLOR);

        terrain_atlas.texture().bind(TerrainShader::COLOR_TEXTURE_SLOT);

        for (position, chunk_mesh) in client_world.get_all_meshes() {
            chunk_mesh.render(
                &ChunkPos::new(position.x - camera.position.get_chunk_x(), position.z - camera.position.get_chunk_z()),
                camera.get_project_view_matrix(),
                &terrain_shader,
            );
        }

        Texture::unbind();

        line_shader.bind();
        line_shader.set_project_view_matrix(camera.get_project_view_matrix());
        line_shader.set_render_offset(&ChunkPos::new(camera.position.get_chunk_x(), camera.position.get_chunk_z()));

        line_debug.render_all();
        Shader::unbind();

        window.swap_buffers();
    }
}