extern crate cgmath;

use std::f64::consts::PI;

use glium::glutin::{ElementState, VirtualKeyCode, WindowEvent};
use cgmath::{Deg, Point3, Vector3, Matrix4};

use util::types::Float;
use util::math::clamp;

const MOUSE_SENSITIVITY: Float = 4.0;
const MOVEMENT_SPEED: Float = 0.10;

const MIN_ANGLE: Float = -PI as Float / 2.0 + 0.01;
const MAX_ANGLE: Float = PI as Float / 2.0 - 0.01;

#[derive(Debug)]
pub struct CameraState {
    pub fov: Float,
    pub aspect_ratio: Float,
    pub znear: Float,
    pub zfar: Float,

    pub position: Point3<Float>,
    pub direction: Vector3<Float>,
    pub up: Vector3<Float>,

    horizontal_angle: Float,
    vertical_angle: Float,

    forward_pressed: bool,
    backward_pressed: bool,
    left_pressed: bool,
    right_pressed: bool,

    res: (u32, u32),
}

impl CameraState {

    pub fn new(res: (u32, u32)) -> CameraState {
        CameraState {
            fov: 75.0,
            aspect_ratio: 4.0 / 3.0,
            position: Point3::new(0.0, 0.0, 0.0),
            direction: Vector3::new(0.0, 0.0, -1.0),
            up: Vector3::new(0.0, 1.0, 0.0),
            znear: 0.01,
            zfar: 1000.0,
            horizontal_angle: 0.0,
            vertical_angle: 0.0,
            forward_pressed: false,
            backward_pressed: false,
            left_pressed: false,
            right_pressed: false,
            res: res,
        }
    }

    pub fn perspective(&self) -> Matrix4<f32> {
        cgmath::perspective(Deg(self.fov), self.aspect_ratio, self.znear, self.zfar)
    }

    pub fn update(&mut self) {
        if self.forward_pressed {
            self.position += self.direction * MOVEMENT_SPEED;
        } 

        if self.backward_pressed {
            self.position += self.direction * -MOVEMENT_SPEED;
        } 

        if self.left_pressed {
            let mut right_vector = self.direction.cross(Vector3::unit_y());
            right_vector = right_vector * MOVEMENT_SPEED;
            self.position += -right_vector;
        }
        
        if self.right_pressed {
            let mut right_vector = self.direction.cross(Vector3::unit_y());
            right_vector = right_vector * MOVEMENT_SPEED;
            self.position += right_vector; 
        }
    }

    pub fn view(&self) -> Matrix4<Float> {
        Matrix4::look_at(
            Point3 { //position
                x: self.position.x,
                y: self.position.y,
                z: self.position.z,
            },
            self.position + self.direction,
            self.up,
        )
    }

    pub fn rotate(&mut self, angle_delta: (Float, Float)) {
        self.horizontal_angle += angle_delta.0;
        self.vertical_angle += angle_delta.1;

        self.vertical_angle = clamp(self.vertical_angle, MIN_ANGLE, MAX_ANGLE);

        self.direction = Vector3 {
            x: self.vertical_angle.cos() * self.horizontal_angle.sin(),
            y: self.vertical_angle.sin(),
            z: self.vertical_angle.cos() * self.horizontal_angle.cos(),
        };
    }

    pub fn calculate_mouse_delta(&self, x: i32, y: i32) -> (i32, i32) {
        let midpoint = (self.res.0 as i32 / 2, self.res.1 as i32 / 2); 
        if x < 0 {
            (0, 0)
        } else {
            let delta = (midpoint.0 - x, midpoint.1 - y);
            delta
        }
    }

    pub fn process_input(&mut self, event: &WindowEvent) {
        match *event {
            WindowEvent::Resized(w, h) => {
                self.res = (w, h);
                self.aspect_ratio = w as Float / h as Float;
            }
            WindowEvent::MouseMoved { position, .. } => {
                let (dx, dy) = self.calculate_mouse_delta(position.0 as i32, position.1 as i32);
                let rotate_by = (
                        (dx as Float / self.res.0 as Float) * MOUSE_SENSITIVITY, 
                        (dy as Float / self.res.1 as Float) * MOUSE_SENSITIVITY / 2.0);
                self.rotate(rotate_by);
            },
            WindowEvent::KeyboardInput { input, .. } => {
                match input.virtual_keycode {
                    Some(VirtualKeyCode::W) => {
                        self.forward_pressed = input.state == ElementState::Pressed;
                    },
                    Some(VirtualKeyCode::A) => {
                        self.left_pressed = input.state == ElementState::Pressed;
                    },
                    Some(VirtualKeyCode::S) => {
                        self.backward_pressed = input.state == ElementState::Pressed;
                    },
                    Some(VirtualKeyCode::D) => {
                        self.right_pressed = input.state == ElementState::Pressed;
                    },
                    _ => (), 
                };
            },
            _ => (),
        };
    }
}
