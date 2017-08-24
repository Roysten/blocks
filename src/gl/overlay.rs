use glium::{self, Display, Frame, Surface};
use glium::vertex::VertexBuffer;

#[derive(Copy, Clone)]
struct OverlayVertex {
    pos: [f32; 2],
}

implement_vertex!(OverlayVertex, pos);

const VERTICES: [OverlayVertex; 4] = [
    OverlayVertex { pos: [1.0, -1.0] }, 
    OverlayVertex { pos: [-1.0, -1.0] },
    OverlayVertex { pos: [1.0, 1.0] },
    OverlayVertex { pos: [-1.0, 1.0] },
];

pub struct Overlay<'a> {
    display: glium::Display,
    program: glium::program::Program, 
    ibuf: glium::index::NoIndices,
    vbuf: glium::vertex::VertexBuffer<OverlayVertex>,
    params: glium::DrawParameters<'a>,
}

impl<'a> Overlay<'a> {
    pub fn new(display: &Display) -> Self {
        let my_display = display.clone();
        Self {
            program: program!(&my_display, 
                330 => {
                    vertex: include_str!("../shader/overlay.vertex"),
                    fragment: include_str!("../shader/overlay.fragment"),
                }
            ).unwrap(),
            vbuf: VertexBuffer::new(&my_display, &VERTICES).unwrap(),
            ibuf: glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip),
            params: glium::DrawParameters {
                blend: glium::Blend::alpha_blending(),
                .. Default::default()
            },

            display: my_display,
        }
    }

    pub fn draw(&self, frame: &mut Frame) {
        let dim = self.display.get_framebuffer_dimensions();
        let uniforms = uniform! {
            res: [dim.0 as f32, dim.1 as f32]
        };

        frame.draw(&self.vbuf, self.ibuf, &self.program, &uniforms, &self.params).unwrap();
    }
}
