use cgmath::Point3;

use model::Model;
use model::mesh::Mesh;
use model::aabb::AABB;
use util::types::Float;

#[derive(Debug, Copy, Clone)]
pub enum BlockType {
    Void(usize),
    Solid(usize),
}

#[derive(Debug, Clone)]
pub struct Block {
    block_type: BlockType,
}

pub const DIM: Float = 2.0;

impl Block {
    pub fn new(block_type: BlockType) -> Block {
        Block { block_type: block_type }
    }

    pub fn get_type(&self) -> BlockType {
        self.block_type
    }
}

impl Model for Block {
    fn get_mesh(&self) -> &Mesh {
        &::MESHES.block
    }
}

impl AABB for Block {
    fn get_min(&self) -> Point3<Float> {
        Point3::from([-DIM / 2.0; 3])
    }

    fn get_max(&self) -> Point3<Float> {
        Point3::from([DIM / 2.0; 3])
    }
}

impl Default for Block {
    fn default() -> Self {
        Self { block_type: BlockType::Void(0) }
    }
}
