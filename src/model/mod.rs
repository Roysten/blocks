pub mod mesh;
pub mod meshes;
pub mod block;
pub mod aabb;

use cgmath::{Point3};

use model::mesh::Mesh;
use util::types::Float;

pub trait Model {
    fn get_mesh(&self) -> &Mesh;
}

pub trait ModelExt : Model {
    fn get_pos(&self) -> Point3<Float>;
}
