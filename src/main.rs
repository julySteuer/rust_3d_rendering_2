extern crate glium;
use glium::implement_vertex;
use lib3d::structure::{Mats, Vertex::{Vertex2d, self, GLVertex3d}, Polygon};
use glium::{glutin, Surface};

fn main() {
    let vertex1 = Vertex::GLVertex2d { position: [-0.5, -0.5] };
    let vertex2 = Vertex::GLVertex2d { position: [ 0.0,  0.5] };
    let vertex3 = Vertex::GLVertex2d { position: [ 0.5, -0.25] };
    let shape = vec![vertex3, vertex1, vertex2];
    let polys = Vertex::load_from_obj(String::from("exported3.obj"));//CHANGE HERE TO OTHER OBJ FILES
    let mut new_vert: Vec<GLVertex3d> = Vec::new(); 
    for i in polys{
        Polygon::polygon_to_renderable(i, &mut new_vert)
    }
    //new_vert = new_vert.into_iter().rev().collect();
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, &new_vert).unwrap(); // multible objects in one VBO with index buffer 
    let index_buffer = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
    let vertex_shader_src = r#"
    #version 140

    in vec3 position;

    void main() {
        gl_Position = vec4(position, 1.0);
    }
    "#;

    let fragment_shader_src = r#"
    #version 140

    out vec4 color;

    void main() {
        color = vec4(1.0, 0.0, 0.0, 1.0);
    }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src, None).unwrap();

    event_loop.run(move |ev, _, control_flow| {

        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        target.draw(&vertex_buffer, &index_buffer, &program, &glium::uniforms::EmptyUniforms,
            &Default::default()).unwrap();
        target.finish().unwrap();

        let next_frame_time = std::time::Instant::now() +
            std::time::Duration::from_nanos(16_666_667);

        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        match ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                    return;
                },
                _ => return,
            },
            _ => (),
        }
    });
}