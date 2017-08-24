pub mod vertex;
pub mod overlay;
pub mod world_proxy;

use glium::{Display, IndexBuffer, VertexBuffer};
use glium::index::PrimitiveType;

use gl::vertex::Vertex;
use model::mesh::Mesh;

pub fn build_vertex_buffer(display: &Display, mesh: &Mesh) -> VertexBuffer<Vertex> {
    VertexBuffer::immutable(&display.clone(), &mesh.vertex_info).unwrap()
}

pub fn build_index_buffer(display: &Display, mesh: &Mesh) -> IndexBuffer<u32> {
    IndexBuffer::immutable(&display.clone(), PrimitiveType::TrianglesList, &mesh.indices).unwrap()
}
