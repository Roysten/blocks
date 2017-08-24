extern crate glium;

use cgmath::{Vector3, Point3};

use glium::{Display, VertexBuffer};
use glium::vertex::VertexBufferSlice;

use gl::vertex::Translation;
use world::world::WorldBlock;
use model::block::{self, BlockType};
use model::aabb::ray_intersect;
use world::world::World;
use util::constants::REACH_DISTANCE;
use util::types::Float;
use util::math::safe_cast;

fn local_pos_to_translation(p: &Point3<usize>) -> Translation {
    Translation {
        translation: [
            p.x as f32 * block::DIM,
            p.y as f32 * block::DIM,
            p.z as f32 * block::DIM,
        ]
    }
}

pub struct WorldProxy {
    world: World,
    translations: VertexBuffer<Translation>,
    buf_tmp: Vec<Translation>,
    update_required: bool,
}

impl WorldProxy {
    pub fn with_capacity(display: &Display, capacity: usize) -> Self {
        WorldProxy {
            world: World::with_capacity(capacity), 
            translations: VertexBuffer::empty_dynamic(&display.clone(), capacity.pow(3)).unwrap(),
            buf_tmp: Vec::new(),
            update_required: true,
        }
    }

    pub fn translations(&mut self) -> Option<VertexBufferSlice<Translation>> {
        if let Some(slice) = self.translations.slice(..self.buf_tmp.len()) {
            if self.update_required && self.buf_tmp.len() > 0 {
                self.update_required = false;
                slice.write(&self.buf_tmp);
            }
            Some(slice)
        } else {
            None
        }
    }

    pub fn add_block(&mut self, pos: &Point3<usize>) {
        self.update_required = true;
        let t = local_pos_to_translation(pos);
        let search_result = self.buf_tmp.binary_search(&t);
        match search_result {
            Ok(i) => self.buf_tmp[i] = t,
            Err(i) => self.buf_tmp.insert(i, t),
        }
        self.world.add_block(BlockType::Solid(0), pos);
    }

    pub fn remove_block(&mut self, pos: &Point3<usize>) {
        self.update_required = true;
        let t = local_pos_to_translation(pos);
        
        if let Ok(i) = self.buf_tmp.binary_search(&t) {
            self.buf_tmp.remove(i); 
            self.world.remove_block(pos);
        }
    }

    pub fn find_block_look_at<'a>(&'a self, pos: &Point3<Float>, dir: &Vector3<Float>) -> Option<(WorldBlock, Point3<Float>)> {
        let grid_pos = safe_cast(pos) / block::DIM as usize;
        let blocks_to_search = self.world.blocks(&grid_pos, REACH_DISTANCE);
        let closest_block_info = blocks_to_search.iter()
            .fold(None, |closest_block, block_info| {
                let ray_len = (REACH_DISTANCE + 1) as f32 * block::DIM;
                match ray_intersect(&pos, &dir, ray_len, block_info.block, &block_info.world_pos()) {
                    Some((fraction, hit_pos)) => {
                        match closest_block {
                            Some((smallest_frac, _, _)) => {
                                if fraction < smallest_frac {
                                    Some((fraction, hit_pos, block_info))
                                } else {
                                    closest_block
                                }
                            },
                            None => Some((fraction, hit_pos, block_info)),
                        }
                    },
                    None => closest_block,
                }
            }
        );

        match closest_block_info {
            Some((_, hit_pos, block_info)) => Some((*block_info, hit_pos)),
            None => None,
        }
    }
}
