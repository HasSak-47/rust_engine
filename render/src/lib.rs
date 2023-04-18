/*
#[macro_use]
extern crate glium;

use std::fs::File;
use std::io::BufReader;
use obj::*;

mod teapot;

#[allow(unused_imports)]
use glium::{glutin, Surface};

pub fn main_thread() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::LogicalSize::new(600., 600.));

    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();


    let input     = BufReader::new(File::open("assets/test.obj").unwrap());
    let obj: Obj  = load_obj(input).unwrap();

    let positions = obj.vertex_buffer(&display).unwrap();
    let indices   = obj.index_buffer(&display).unwrap();

    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    // let normals   = glium::VertexBuffer::new(&display, &teapot::NORMALS ).unwrap();
    let indices   = glium:: IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let (vertex_shader, fragment_shader) = {
        let vdata = std::fs::read("shaders/vertex.shader").unwrap();
        let fdata = std::fs::read("shaders/fragment.shader").unwrap(); 

        (String::from_utf8(vdata).unwrap(),
        String::from_utf8(fdata).unwrap(),)
    };


    let vertex_shader_src = vertex_shader.as_str();
    let fragment_shader_src = fragment_shader.as_str();

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,None).unwrap();

    event_loop.run(move |event, _, control_flow| {
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        let mut target = display.draw();
        target.clear_color(0.2, 0.3, 0.3, 1.);

        let matrix = [
            [0.01, 0.0, 0.0, 0.0],
            [0.0, 0.01, 0.0, 0.0],
            [0.0, 0.0, 0.01, 0.0],
            [0.0, 0.0, 0.0,  1.0f32]
        ];

        target.draw(&positions, &indices, &program, &uniform! { matrix: matrix }, &Default::default()).unwrap();
        target.finish().unwrap();
    });
}
// cats and soup
// use std::f32::consts::PI;
/*
use glium::{self, Surface, uniform};

mod teapot;

pub fn main_thread(){
    let events_loop = glium::glutin::event_loop::EventLoop::new();
    let wb = glium::glutin::window::WindowBuilder::new()
        .with_title("penis");
    let cb = glium::glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &events_loop).unwrap();


    let positions = glium::VertexBuffer::new(&display, &teapot::VERTICES).unwrap();
    let normal    = glium::VertexBuffer::new(&display, &teapot::NORMALS).unwrap();
    let ind = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &teapot::INDICES).unwrap();

    let (vertex_shader, fragment_shader) = {
        let vdata = std::fs::read("shaders/vertex.glsl").unwrap();
        let fdata = std::fs::read("shaders/fragment.glsl").unwrap(); 

        (
            String::from_utf8(vdata).unwrap(),
            String::from_utf8(fdata).unwrap(),
        )
    };

    let program = glium::Program::from_source(&display, vertex_shader.as_str(), fragment_shader.as_str(), None).unwrap();

    events_loop.run(move |ev, _, control_flow|{
        // wait until next frame
        let next_frame_time = std::time::Instant::now() + std::time::Duration::from_millis(16);
        *control_flow = glium::glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);

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


        let mut target = display.draw();
        target.clear_color(0., 0., 0., 1.);

        let scale_matrix = [
            [1., 0., 0., 0.],
            [0., 1., 0., 0.],
            [0., 0., 1., 0.],
            [0., 0., 0., 1.0f32],
        ];
        target.draw((&positions, &normal), &ind, &program, &uniform! {matrix: scale_matrix}, &Default::default()).unwrap();
        target.finish().unwrap();
    })
}
*/
*/
