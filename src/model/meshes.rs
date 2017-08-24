use model::mesh::Mesh;
use loader::MeshLoader;
use loader::obj_loader::ObjLoader;

pub struct Meshes {
    loader: Box<MeshLoader>,
    pub block: Mesh,
}

impl Meshes {
    pub fn load() -> Meshes {
        let mut loader = Box::new(ObjLoader::new());
        let block = loader.load_from_str(include_str!("../../cube.obj")).unwrap();
        Meshes {
            loader: loader,
            block: block,
        }
    }
}

unsafe impl Sync for Meshes {
    //Implement Sync so the meshes can be shared between threads. This is safe,
    //since the meshes class is just a read-only static storage of meshes.
}
