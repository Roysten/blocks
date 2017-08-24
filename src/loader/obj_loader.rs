use std::io::{Cursor, BufReader, Read};
use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::collections::HashMap;

use util::types::{Float, GLIndex};
use model::mesh::Mesh;
use loader::MeshLoader;
use gl::vertex::Vertex;

pub struct ObjLoader {
    index_map: HashMap<[Option<GLIndex>; 3], GLIndex>,
    verts: Vec<[Float; 3]>,
    vert_normals: Vec<[Float; 3]>,
    tex_coords: Vec<[Float; 2]>,
}

impl ObjLoader {
    pub fn new() -> ObjLoader {
        ObjLoader { 
            index_map: HashMap::new(), 
            verts: Vec::new(), 
            vert_normals: Vec::new(),
            tex_coords: Vec::new(),
        }
    }

    fn handle_line(&mut self, line: &str, mesh: &mut Mesh) -> Result<(), Box<Error>> {
        let mut words = line.split_whitespace();
        match words.next() {
            Some(vertex_type @ "v") | Some(vertex_type @ "vn") => {
                if let (Some(a), Some(b), Some(c)) = (words.next(), words.next(), words.next()) {
                    let (x, y, z) = (a.parse()?, b.parse()?, c.parse()?);
                    Ok(match vertex_type {
                        "v" => self.verts.push([x, y, z]),
                        "vn" => self.vert_normals.push([x, y, z]),
                        _ => (),
                    })
                } else {
                    Err(Box::<Error>::from(format!("Invalid vertex: ``{}``", line)))
                }
            },
            Some("vt") => {
                if let(Some(a), Some(b)) = (words.next(), words.next()) {
                    let (x, y) = (a.parse()?, b.parse()?);
                    Ok(self.tex_coords.push([x, y]))
                } else {
                    Err(Box::<Error>::from(format!("Only 2d texture coordinates are supported: ``{}``", line)))
                }
            },
            Some("f") => {
                for indices_str in words {
                    let parsed_indices = self.parse_face_vertex(indices_str)?;
                    if self.index_map.contains_key(&parsed_indices) {
                        let cached_vertex_pos = *self.index_map.get(&parsed_indices).unwrap();
                        mesh.indices.push(cached_vertex_pos);
                    } else if let [Some(index_v), index_vt, index_vn] = parsed_indices {
                        let vertex_info_len = mesh.vertex_info.len() as GLIndex;
                        let vn = match index_vn {
                            Some(i) => self.vert_normals[i as usize],
                            None => [0.0; 3],
                        };
                        let vt = match index_vt {
                            Some(i) => self.tex_coords[i as usize],
                            None => [0.0; 2],
                        };
                        mesh.vertex_info.push(Vertex::new(self.verts[index_v as usize], vt, vn));
                        mesh.indices.push(vertex_info_len);
                        self.index_map.insert(parsed_indices, vertex_info_len);
                    }
                }
                Ok(())
            }
            //Ignore object markers, object groups, materials, and smoothing groups for now
            Some("o") | Some("s") | Some("g") | Some("#") | Some("mtllib") | Some("usemtl") => Ok(()),
            Some(_) | None => Err(Box::<Error>::from(format!("Unrecognized line in .obj file: ``{}``", line))),
        }
    }

    fn parse_face_vertex(&self, face_str: &str) -> Result<[Option<GLIndex>; 3], Box<Error>> {
        let indices = face_str.split('/').take(3).collect::<Vec<&str>>();
        let index_v = indices[0].parse::<GLIndex>()? - 1;
        let index_vt = indices[1].parse::<GLIndex>().map(|i| { Some(i - 1) }).unwrap_or(None);
        let index_vn = indices[2].parse::<GLIndex>().map(|i| { Some(i - 1) }).unwrap_or(None);
        Ok([Some(index_v), index_vt, index_vn])
    }

    fn clear(&mut self) {
        self.verts.clear();
        self.vert_normals.clear();
        self.tex_coords.clear();
        self.index_map.clear();
    }

    fn load_mesh<T>(&mut self, read_from: T) -> Result<Mesh, Box<Error>> 
        where T: Read
    {
        let reader = BufReader::new(read_from);
        let mut mesh = Mesh::new();
        for line in reader.lines().map(|line| line.unwrap()) {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                match self.handle_line(trimmed, &mut mesh) {
                    Ok(_) => (),
                    Err(e) => return Err(Box::<Error>::from(format!("Failed to parse .obj file: {}", e)))
                }
            }
        }
        self.clear();
        Ok(mesh)
    }
}

impl MeshLoader for ObjLoader {
    fn load_from_str(&mut self, mesh_str: &str) -> Result<Mesh, Box<Error>> {
        self.load_mesh(Cursor::new(mesh_str))
    }

    fn load_from_file(&mut self, filename: &str) -> Result<Mesh, Box<Error>> {
        let f = File::open(filename)?;
        self.load_mesh(f)
    }
}
