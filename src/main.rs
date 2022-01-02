extern crate glium;
use lib3d::structure::{Mats, Vertex::{Vertex2d, self, GLVertex3d}, Polygon};
use glium::{glutin, Surface};

fn main() {
    let (vertices, normals, polygons) = Vertex::load_from_obj(String::from("exported2.obj"));//CHANGE HERE TO OTHER OBJ FILES
    let mut real_vertices:Vec<GLVertex3d> = Vec::new();
    let mut indecies:Vec<u16> = Vec::new();
    let mut real_normals: Vec<GLVertex3d> = Vec::new();
    real_normals.push(Vertex::Vertex::new(0.0, 0.0,0.0).to());
    real_vertices.push(Vertex::Vertex::new(0.0,0.0,0.0).to());
    for i in vertices{
        real_vertices.push(i.to())
    }
    for i in polygons {
        for s in Polygon::polygon_to_renderable(i){
            indecies.push(s);
        }
    }
    for i in normals {//shift normals one to right i think
        real_normals.push(i.to());
    }
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();
    let vertex_buffer = glium::VertexBuffer::new(&display, &real_vertices).unwrap(); // multible objects in one VBO with index buffer 
    let index_buffer = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList, &indecies).unwrap();
    let normals = glium::VertexBuffer::new(&display, &real_normals).unwrap();
    let vertex_shader_src = r#"
    #version 140

    in vec3 position;
    in vec3 normals;

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
        target.draw((&vertex_buffer, &normals), &index_buffer, &program, &glium::uniforms::EmptyUniforms,
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