pub mod obj_loader;

use std::error::Error;

use model::mesh::Mesh;

pub trait MeshLoader {
    fn load_from_str(&mut self, mesh_str: &str) -> Result<Mesh, Box<Error>>;

    fn load_from_file(&mut self, filename: &str) -> Result<Mesh, Box<Error>>;
}
