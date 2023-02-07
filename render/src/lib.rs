// cats and soup
use std::f32::consts::PI;

use glium::{self, Surface, implement_vertex, uniform};

#[derive(Clone, Copy)]
struct Vertex{
    position: [f32; 4],
    color   : [f32, 4],
    uv      : [f32, 4],
}

impl Vertex{
    fn new(a: f32, b: f32) -> Self{
        Vertex { position: [a, b] }
    }
    fn newa(position: [f32; 2]) -> Self{
        Vertex { position }
    }
}

implement_vertex!(Vertex, position);

pub fn main_loop(){
    let mut events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(600.0, 600.0))
        .with_title("penis");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();

    let vertices = [
        Vertex::newa([ 0.5, 0.0]),
        Vertex::newa([-0.5, 0.5]),
        Vertex::newa([-0.5,-0.5]),
    ];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let indices       = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let (vertex_shader, fragment_shader) = {
        let vdata = std::fs::read("shaders/vertex.glsl").unwrap();
        let fdata = std::fs::read("shaders/fragment.glsl").unwrap(); 

        (
            String::from_utf8(vdata).unwrap(),
            String::from_utf8(fdata).unwrap(),
        )
    };

    let program = glium::Program::from_source(&display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
    let mut angle : f32 = 0.0;

    events_loop.run(move |ev, _, control_flow|{
        // this is for events
        match ev {
            glium::glutin::event::Event::WindowEvent { event, .. } => match event{
                glium::glutin::event::WindowEvent::CloseRequested =>{
                    *control_flow = glium::glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }

        // wait until next frame
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_millis(16);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        //uniform stuff
        angle += (PI / 60.0) / 1000.0;
        if angle >= 2.0 * PI{
            angle = 0.0;
        }

        // rendering
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 0.0, 1.0);
        target.draw(&vertex_buffer, &indices, &program, &uniform! {angle: angle}, &Default::default()).unwrap();
        target.finish().unwrap();
    })
}
