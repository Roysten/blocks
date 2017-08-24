use gl::vertex::Vertex;

#[derive(Debug)]
pub struct Mesh {
    pub vertex_info: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn new() -> Mesh {
        Mesh { 
            vertex_info: Vec::new(), 
            indices: Vec::new()
        }
    }
}
