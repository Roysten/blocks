use std::cmp::Ordering;

use util::types::Float;

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    pub pos: [Float; 3],
    pub tex_coords: [Float; 2],
    pub norm: [Float; 3],
}

impl Vertex {
    pub fn new(pos: [Float; 3], tex_coords: [Float; 2], norm: [Float; 3]) -> Vertex {
        Vertex { pos: pos, tex_coords: tex_coords, norm: norm }
    }
}

#[derive(Copy, Clone)]
pub struct Translation {
    pub translation: [Float; 3],
}

impl Ord for Translation {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.translation[0] < other.translation[0] {
            Ordering::Less
        } else if self.translation[0] > other.translation[0] {
            Ordering::Greater
        } else {
            if self.translation[1] < other.translation[1] {
                Ordering::Less
            } else if self.translation[1] > other.translation[1] {
                Ordering::Greater
            } else {
                if self.translation[2] < other.translation[2] {
                    Ordering::Less
                } else if self.translation[2] > other.translation[2] {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            }
        }
    }
}

impl PartialOrd for Translation {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for Translation { }

impl PartialEq for Translation {
    fn eq(&self, other: &Self) -> bool {
        self.translation[0] == other.translation[0] && 
            self.translation[1] == other.translation[1] && 
            self.translation[2] == other.translation[2]
    }
}


implement_vertex!(Vertex, pos, tex_coords, norm);
implement_vertex!(Translation, translation);
