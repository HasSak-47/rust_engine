// cats and soup
// use std::f32::consts::PI;
use glium::{self, Surface, uniform};
use math::Quaternion;
use loader;

type Quat = Quaternion<f32>;

#[derive(Clone, Copy)]
pub struct Vertex{
    pub pos: [f32; 3],
    pub nor: [f32; 3],
    pub tex: [f32; 2],
}

glium::implement_vertex!(Vertex, pos, nor, tex);

fn cast_vertices(v: Vec<loader::Vertex>) -> Vec<Vertex>{
    let mut vertices = Vec::new();
    for vertex in v{
        vertices.push(Vertex {pos: vertex.pos, nor: vertex.nor, tex: vertex.tex} )
    }


    vertices
} 

pub fn main_thread(){
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_inner_size(glium::glutin::dpi::LogicalSize::new(600., 600.))
        .with_title("penis");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();


    // let (vertices, indices) = loader::load("test_assets/teaset/teapot.obj");
    // let input = std::io::BufRead::new(std::fs::File::open("").unwrap()).unwrap();
    let vertices = cast_vertices(loader::load_vertices("test_assets/untitled.obj"));

    //let vertices = [
    //    Vertex{pos: [0.0, 1.0, 0.0], nor: [0.0, 0.0, 0.0], tex: [0.0, 0.0]},
    //    Vertex{pos: [0.0, 0.0, 1.0], nor: [0.0, 0.0, 0.0], tex: [0.0, 0.0]},
    //    Vertex{pos: [1.0, 0.0, 0.0], nor: [0.0, 0.0, 0.0], tex: [0.0, 0.0]},
    //];

    let vertex_buffer = glium::VertexBuffer::new(&display, &vertices).unwrap();
    let index_buffer  = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    //let index_buffer  = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &o.indices).unwrap();

    let (vertex_shader, fragment_shader) = {
        let vdata = std::fs::read("shaders/vertex.glsl").unwrap();
        let fdata = std::fs::read("shaders/fragment.glsl").unwrap(); 

        (
            String::from_utf8(vdata).unwrap(),
            String::from_utf8(fdata).unwrap(),
        )
    };

    let program = glium::Program::from_source(&display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();
    // let scale_matrix = [
    //     [0.001, 0.000, 0.000, 0.000,],
    //     [0.000, 0.001, 0.000, 0.000,],
    //     [0.000, 0.000, 0.001, 0.000,],
    //     [0.000, 0.000, 0.000, 0.001f32,],
    // ];

    let start = std::time::Instant::now();
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

        let angle : f32 = std::time::Instant::now().duration_since(start).as_millis() as f32 / 1000.0;
        let mut scale_matrix : math::nalgebra::Matrix4<f32> =
            math::nalgebra::Matrix4::<f32>::identity() *
            math::nalgebra::Matrix4::<f32>::from_euler_angles(angle, angle, 0.)
        ;
        scale_matrix *= 0.001;

        // god has forsaken us
        let scale_matrix : [[f32; 4]; 4] = [
            [*scale_matrix.get(0).unwrap(),
             *scale_matrix.get(1).unwrap(),
             *scale_matrix.get(2).unwrap(),
             *scale_matrix.get(3).unwrap(),],
            [*scale_matrix.get(4).unwrap(),
             *scale_matrix.get(5).unwrap(),
             *scale_matrix.get(6).unwrap(),
             *scale_matrix.get(7).unwrap(),],
            [*scale_matrix.get(8).unwrap(),
             *scale_matrix.get(9).unwrap(),
             *scale_matrix.get(10).unwrap(),
             *scale_matrix.get(11).unwrap(),],
            [*scale_matrix.get(12).unwrap(),
             *scale_matrix.get(13).unwrap(),
             *scale_matrix.get(14).unwrap(),
             *scale_matrix.get(15).unwrap(),],
        ];
        // rendering
        let mut target = display.draw();
        target.clear_color(0., 0., 0., 1.);
        target.draw(&vertex_buffer, &index_buffer, &program, &uniform! {matrix: scale_matrix}, &Default::default()).unwrap();
        target.finish().unwrap();
    })
}
