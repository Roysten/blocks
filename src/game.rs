use glium::{glutin, Display};
use glium::glutin::WindowEvent;

use util::types::Float;
use gl::overlay::Overlay;
use gl::world_proxy::WorldProxy;
use camera::free_cam::CameraState;

pub struct Game<'a> {
    world: WorldProxy,
    overlay: Overlay<'a>,
    camera: CameraState,
}

impl<'a> Game<'a> {
    pub fn new(display: &Display) -> Self {
        Self {
            world: WorldProxy::with_capacity(&display, 16),
            overlay: Overlay::new(&display),
            camera: CameraState::new(display.get_framebuffer_dimensions()),
        }
    }

    pub fn world(&self) -> &WorldProxy {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut WorldProxy {
        &mut self.world
    }

    pub fn overlay(&self) -> &Overlay {
        &self.overlay
    }

    pub fn camera(&self) -> &CameraState {
        &self.camera
    }

    pub fn camera_mut(&mut self) -> &mut CameraState {
        &mut self.camera
    }

    pub fn process_input(&mut self, event: &WindowEvent) {
        if let &WindowEvent::MouseInput { state, button, .. } = event {
            let (cam_pos, cam_dir) = (self.camera.position, self.camera.direction);
            match (button, state) {
                (glutin::MouseButton::Right, glutin::ElementState::Pressed) => {
                    let place_block_at = self.world.find_block_look_at(&cam_pos, &cam_dir).and_then(|(block, hit_pos)| {
                        let hit_pos = hit_pos - block.world_pos();
                        let arr: [Float; 3] = hit_pos.into();
                        
                        arr.iter().position(|&val| val.abs() >= 0.99).and_then(|face_axis| {
                            let val = arr[face_axis];
                            if (val < 0.0 && block.local_pos()[face_axis] > 0) || val > 0.0 {
                                let mut new_pos = *block.local_pos();
                                if val < 0.0 { 
                                    new_pos[face_axis] -= 1; 
                                } else { 
                                    new_pos[face_axis] += 1;
                                };
                                Some(new_pos)
                            } else {
                                None
                            }
                        })
                    });
                    
                    if let Some(p) = place_block_at {
                        self.world.add_block(&p);
                    }
                },
                (glutin::MouseButton::Left, glutin::ElementState::Pressed) => {
                    let pos_to_remove = self.world.find_block_look_at(&cam_pos, &cam_dir).and_then(|(block, _)| {
                        Some(*block.local_pos())
                    });
                    if let Some(p) = pos_to_remove {
                        self.world.remove_block(&p);
                    }
                },
                _ => (),
            }
        }
    }
}
