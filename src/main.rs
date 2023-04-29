#[macro_use]
extern crate glium;

pub mod shapes;
use shapes::shapes::Triangle;

use glam::Mat4;
use glium::{glutin, Surface};
use glutin::dpi::PhysicalPosition;
use glutin::{event::{Event, WindowEvent}, event_loop::ControlFlow};

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Shapes")
        .with_position(PhysicalPosition::<i32>::from((50, 50)));
    let cb = glutin::ContextBuilder::new()
        .with_multisampling(4)
        .with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).expect("unable to create a new display");

    let program = glium::Program::from_source(
        &display,
        /* Vertex shader */r#"
        #version 150

        in vec3 position;
        uniform mat4 transform;

        void main() {
            gl_Position = transform * vec4(position, 1.0);
        }
        "#,
        /* Fragment shader */r#"
        #version 140

        out vec4 colour;

        void main() {
            colour = vec4(1.0, 1, 1, 1);
        }
        "#,
        None,
    ).unwrap();

    let triangle = Triangle::new(&display);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawEventsCleared => display.gl_window().window().request_redraw(),
            Event::RedrawRequested(_) => {
                let mut frame = display.draw();

                frame.clear_color(0.0, 0.0, 0.0, 1.0); // as Surface

                frame.draw(
                    &triangle.vertices,
                    &triangle.indices,
                    &program,
                    &uniform! { transform: Mat4::IDENTITY.to_cols_array_2d() },
                    &Default::default(),
                ).unwrap();

                frame.finish().unwrap();
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => (),
                }
            },
            _ => (),
        }
    });
}
