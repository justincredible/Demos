use glam::{Mat4, Vec3};
use winit::event::{ElementState, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey::Code};
use std::f32::consts::TAU;

pub struct CameraState {
    aspect_ratio: f32,
    position: Vec3,
    direction: Vec3,

    moving_up: bool,
    moving_left: bool,
    moving_down: bool,
    moving_right: bool,
    moving_forward: bool,
    moving_backward: bool,
}

impl CameraState {
    pub fn new() -> CameraState {
        CameraState {
            aspect_ratio: 4.0 / 3.0,
            position: Vec3::new(0.1, 0.1, 1.0),
            direction: -Vec3::Z,
            moving_up: false,
            moving_left: false,
            moving_down: false,
            moving_right: false,
            moving_forward: false,
            moving_backward: false,
        }
    }

    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
    }

    pub fn set_position(&mut self, pos: Vec3) {
        self.position = pos;
    }

    pub fn set_direction(&mut self, dir: Vec3) {
        self.direction = dir;
    }

    pub fn get_perspective(&self) -> Mat4 {
        let fov: f32 = TAU / 8.0;
        let zfar = 1024.0;
        let znear = 0.1;

        Mat4::perspective_lh(fov, self.aspect_ratio, znear, zfar)
    }

    pub fn get_view(&self) -> Mat4 {
        let f = self.direction.normalize();
        let up = if f.cross(Vec3::Y).length() < 0.001 {
            -Vec3::Z
        } else {
            Vec3::Y
        };
        let s = f.cross(up).normalize();
        let u = s.cross(f).normalize();

        Mat4::from_cols_array(&[
            s.x,
            u.x,
            f.x,
            0.0, // c0
            s.y,
            u.y,
            f.y,
            0.0, // c1
            s.z,
            u.z,
            f.z,
            0.0, // c2
            -s.dot(self.position),
            -u.dot(self.position),
            -f.dot(self.position),
            1.0,
        ])
    }

    pub fn update(&mut self) {
        let f = {
            let f = self.direction;
            let mut len = f.x * f.x + f.y * f.y + f.z * f.z;
            len = len.sqrt();
            (f.x / len, f.y / len, f.z / len)
        };

        let up = (0.0, 1.0, 0.0);

        let s = (
            f.1 * up.2 - f.2 * up.1,
            f.2 * up.0 - f.0 * up.2,
            f.0 * up.1 - f.1 * up.0,
        );

        let s = {
            let len = s.0 * s.0 + s.1 * s.1 + s.2 * s.2;
            let len = len.sqrt();
            (s.0 / len, s.1 / len, s.2 / len)
        };

        let u = (
            s.1 * f.2 - s.2 * f.1,
            s.2 * f.0 - s.0 * f.2,
            s.0 * f.1 - s.1 * f.0,
        );

        const DELTA: f32 = 0.03f32;

        if self.moving_up {
            self.position.x += u.0 * DELTA;
            self.position.y += u.1 * DELTA;
            self.position.z += u.2 * DELTA;
        }

        if self.moving_left {
            self.position.x -= s.0 * DELTA;
            self.position.y -= s.1 * DELTA;
            self.position.z -= s.2 * DELTA;
        }

        if self.moving_down {
            self.position.x -= u.0 * DELTA;
            self.position.y -= u.1 * DELTA;
            self.position.z -= u.2 * DELTA;
        }

        if self.moving_right {
            self.position.x += s.0 * DELTA;
            self.position.y += s.1 * DELTA;
            self.position.z += s.2 * DELTA;
        }

        if self.moving_forward {
            self.position.x += f.0 * DELTA;
            self.position.y += f.1 * DELTA;
            self.position.z += f.2 * DELTA;
        }

        if self.moving_backward {
            self.position.x -= f.0 * DELTA;
            self.position.y -= f.1 * DELTA;
            self.position.z -= f.2 * DELTA;
        }
    }

    pub fn process_input(&mut self, event: &WindowEvent) {
        let event = match event {
            WindowEvent::KeyboardInput { event, .. } => event,
            _ => return,
        };
        match event {
            KeyEvent { state, physical_key: Code(KeyCode::ArrowUp), .. } =>
                self.moving_up = *state == ElementState::Pressed,
            KeyEvent { state, physical_key: Code(KeyCode::ArrowDown), .. } =>
                self.moving_down = *state == ElementState::Pressed,
            KeyEvent { state, physical_key: Code(KeyCode::KeyA), .. } =>
                self.moving_left = *state == ElementState::Pressed,
            KeyEvent { state, physical_key: Code(KeyCode::KeyD), .. } =>
                self.moving_right = *state == ElementState::Pressed,
            KeyEvent { state, physical_key: Code(KeyCode::KeyW), .. } =>
                self.moving_forward = *state == ElementState::Pressed,
            KeyEvent { state, physical_key: Code(KeyCode::KeyS), .. } =>
                self.moving_backward = *state == ElementState::Pressed,
            _ => (),
        };
    }
}
