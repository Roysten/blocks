#![feature(slice_patterns)]
extern crate cgmath;
#[macro_use]
extern crate glium;
#[macro_use]
extern crate lazy_static;
extern crate png;

mod gl;
mod util;
mod game;
mod model;
mod world;
mod loader;
mod camera;

use glium::{glutin, Surface};
use cgmath::{Point3, Matrix4, SquareMatrix, EuclideanSpace};

use game::Game;
use gl::{build_vertex_buffer, build_index_buffer};
use util::types::Float;
use model::meshes::Meshes;

lazy_static! {
    static ref MESHES: Meshes = Meshes::load();
}

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new().with_dimensions(640, 480);
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();
    let _ = display.gl_window().set_cursor_state(glutin::CursorState::Grab);
    let _ = display.gl_window().set_cursor(glutin::MouseCursor::NoneCursor);

    let mut game = Game::new(&display);
    game.world_mut().add_block(&Point3::origin());

    // compiling shaders and linking them together
    let program = program!(&display,
        330 => {
            vertex: include_str!("shader/block.vertex"),
            fragment: include_str!("shader/block.fragment"),
        },
    ).unwrap();

    // draw parameters
    let params = glium::DrawParameters {
        depth: glium::Depth {
            test: glium::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        .. Default::default()
    };

    let model_matrix = Matrix4::identity();
    let mut stop = false;

    let decoder = png::Decoder::new(std::io::Cursor::new(&include_bytes!("../textures/minecraft.png")[..]));
    let (info, mut reader) = decoder.read_info().unwrap();
    let mut tex_buf = vec![0; info.buffer_size()];
    reader.next_frame(&mut tex_buf).unwrap();

    let raw_image = glium::texture::RawImage2d::from_raw_rgba_reversed(&tex_buf, (info.width, info.height));
    let texture = glium::texture::SrgbTexture2d::new(&display, raw_image).unwrap();

    while !stop {
        game.camera_mut().update();
        let view_matrix = game.camera().view();
        let projection_matrix = game.camera().perspective();
        let mvp_matrix: [[Float; 4]; 4] = std::convert::Into::into(projection_matrix * view_matrix * model_matrix);
        let uniforms = uniform! {
            mvp_matrix: mvp_matrix,
            tex: texture.sampled().magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest),
        };

        let mut target = display.draw();
        target.clear_color_srgb_and_depth((66.0/255.0, 196.0/255.0, 247.0/255.0, 1.0), 1.0);
        {
            let vbuf = build_vertex_buffer(&display, &MESHES.block);
            let ibuf = build_index_buffer(&display, &MESHES.block);
            if let Some(translations_buf) = game.world_mut().translations() {
                target.draw((&vbuf, translations_buf.per_instance().unwrap()),
                             &ibuf,
                             &program,
                             &uniforms,
                             &params).unwrap();
            }
            game.overlay().draw(&mut target); 
            target.finish().unwrap();
        }

        events_loop.poll_events(|event| {
            if let glutin::Event::WindowEvent { event, .. } = event { 
                match event {
                    // Break from the main loop when the window is closed.
                    glutin::WindowEvent::Closed => stop = true,
                    ev => { 
                        game.camera_mut().process_input(&ev);
                        game.process_input(&ev);
                    },
                }
            }
        });
        
        let (w, h) = display.get_framebuffer_dimensions();
        display.gl_window().set_cursor_position(w as i32 / 2, h as i32 / 2).unwrap();
    }
}
