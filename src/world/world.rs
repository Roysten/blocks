use std::cmp::min;

use cgmath::Point3;

use util::types::Float;
use model::block::{self, Block, BlockType};

#[derive(Debug, Clone, Copy)]
pub struct WorldBlock<'a> {
    pub block: &'a Block,
    pos: Point3<usize>,
}

impl<'a> WorldBlock<'a> {
    pub fn local_pos(&'a self) -> &'a Point3<usize> {
        &self.pos
    }

    pub fn world_pos(&self) -> Point3<Float> {
        self.pos.cast() * block::DIM
    }
}

pub struct World {
    blocks: Vec<Vec<Vec<Block>>>,
}

impl World {
    pub fn with_capacity(capacity: usize) -> Self {
        Self { blocks: vec![vec![vec![Block::default(); capacity]; capacity]; capacity] }
    }

    pub fn add_block(&mut self, block_type: BlockType, pos: &Point3<usize>) {
        if pos.x < self.blocks.len() && pos.y < self.blocks[0].len() && pos.z < self.blocks[0][0].len() {
            self.blocks[pos.x][pos.y][pos.z] = Block::new(block_type);
        }
    }

    pub fn remove_block(&mut self, pos: &Point3<usize>) {
        self.blocks[pos.x][pos.y][pos.z] = Block::new(BlockType::Void(0));
    }

    pub fn blocks<'a>(&self, center: &Point3<usize>, dim: usize) -> Vec<WorldBlock> {
        let mut blocks = Vec::new();
        let (x, y, z) = (center.x, center.y, center.z);
        let range_x = x.saturating_sub(dim) .. min(x + dim, self.blocks.len());
        for (i, c) in range_x.clone().zip(self.blocks[range_x].iter()) {
            let range_y = y.saturating_sub(dim) .. min(y + dim, c.len());
            for (j, c) in range_y.clone().zip(c[range_y].iter()) {
                let range_z = z.saturating_sub(dim) .. min(z + dim, c.len());
                for (k, block) in range_z.clone().zip(c[range_z].iter()) {
                    if let BlockType::Solid(_) = block.get_type() {
                        blocks.push(WorldBlock { block: block, pos: Point3::new(i, j, k) });
                    }
                }
            }
        }
        blocks
    }
}
