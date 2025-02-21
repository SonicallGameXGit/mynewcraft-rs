use cgmath::{Deg, Matrix4, Point3, Quaternion, Rotation, Rotation3, SquareMatrix, Vector3, VectorSpace};
use crate::{engine::{timer::Timer, window::Window}, game::common::coords::Coord};

pub struct Camera {
    pub position: Coord,
    pub rotation: Vector3<f32>,
    pub velocity: Vector3<f32>,

    front: Vector3<f32>,
    up: Vector3<f32>,
    right: Vector3<f32>,

    project_matrix: Matrix4<f32>,
    view_matrix: Matrix4<f32>,
    project_view_matrix: Matrix4<f32>,
}

impl Camera {
    pub fn create() -> Self {
        Self {
            position: Coord::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            velocity: Vector3::new(0.0, 0.0, 0.0),

            front: -Vector3::unit_z(),
            up: Vector3::unit_y(),
            right: Vector3::unit_x(),

            project_matrix: Matrix4::identity(),
            view_matrix: Matrix4::identity(),
            project_view_matrix: Matrix4::identity(),
        }
    }

    pub fn fly(&mut self, window: &Window, timer: &Timer) {
        const SENSITIVITY: f32 = 0.05;
        const SPEED: f32 = 4.13;
        const FAST_SPEED: f32 = 67.6;

        const ACCELERATION: f32 = 2.0;
        const FAST_ACCELERATION: f32 = 10.0;

        self.rotation.y -= window.get_mouse_dx() * SENSITIVITY;
        self.rotation.y -= (self.rotation.y / 360.0).floor() * 360.0;

        self.rotation.x -= window.get_mouse_dy() * SENSITIVITY;
        self.rotation.x = self.rotation.x.clamp(-90.0, 90.0);

        let mut wish_velocity = Vector3::new(0.0, 0.0, 0.0);

        if window.is_key_pressed(glfw::Key::W) {
            wish_velocity += self.front;
        }
        if window.is_key_pressed(glfw::Key::S) {
            wish_velocity -= self.front;
        }
        if window.is_key_pressed(glfw::Key::D) {
            wish_velocity += self.right;
        }
        if window.is_key_pressed(glfw::Key::A) {
            wish_velocity -= self.right;
        }
        if window.is_key_pressed(glfw::Key::E) {
            wish_velocity += self.up;
        }
        if window.is_key_pressed(glfw::Key::Q) {
            wish_velocity -= self.up;
        }

        let running = window.is_key_pressed(glfw::Key::LeftControl);
        wish_velocity *= if running { FAST_SPEED } else { SPEED };

        self.velocity = Vector3::lerp(
            self.velocity,
            wish_velocity,
            if running { ACCELERATION } else { FAST_ACCELERATION } * timer.get_delta()
        );
        self.position += self.velocity.map(|v| v as f64) * timer.get_delta() as f64;
    }
    pub fn update(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
        let quaternion_pitch = Quaternion::from_angle_x(Deg(self.rotation.x));
        let quaternion_yaw = Quaternion::from_angle_y(Deg(self.rotation.y));
        let quaternion_roll = Quaternion::from_angle_z(Deg(self.rotation.z));

        let quaternion_all = quaternion_yaw * quaternion_pitch * quaternion_roll;
        
        self.front = quaternion_all.rotate_vector(-Vector3::unit_z());
        self.up = quaternion_all.rotate_vector(Vector3::unit_y());
        self.right = quaternion_all.rotate_vector(Vector3::unit_x());

        let chunk_frac_position = Point3::new(
            self.position.get_local_x(),
            self.position.get_world_y() as f32,
            self.position.get_local_z(),
        );
        
        self.project_matrix = cgmath::perspective(Deg(fov), aspect, near, far);
        self.view_matrix = Matrix4::look_at_rh(
            chunk_frac_position,
            chunk_frac_position + self.front,
            self.up
        );
        self.project_view_matrix = self.project_matrix * self.view_matrix;
    }

    pub fn get_front(&self) -> &Vector3<f32> {
        &self.front
    }
    // pub fn get_up(&self) -> &Vector3<f32> {
    //     &self.up
    // }
    // pub fn get_right(&self) -> &Vector3<f32> {
    //     &self.right
    // }

    // pub fn get_project_matrix(&self) -> &Matrix4<f32> {
    // 	return &self.project_matrix;
    // }
    // pub fn get_view_matrix(&self) -> &Matrix4<f32> {
    // 	return &self.view_matrix;
    // }
    pub fn get_project_view_matrix(&self) -> &Matrix4<f32> {
        &self.project_view_matrix
    }
}