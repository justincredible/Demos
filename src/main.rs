#[macro_use]
extern crate glium;

pub mod shapes;
use shapes::shapes::*;

use glam::{Mat4, Quat, Vec3};
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
            colour = vec4(1.0, gl_FrontFacing, 1, 1);
        }
        "#,
        None,
    ).unwrap();

    let shape = Dodecahedron::new(&display);

    let params = glium::DrawParameters {
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
        polygon_mode: glium::draw_parameters::PolygonMode::Line,
        ..Default::default()
    };

    let mut rotation = Quat::from_axis_angle(Vec3::ONE, 0.0);
    let rotation_delta = Quat::from_axis_angle(Vec3::ONE, 0.01);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawEventsCleared => display.gl_window().window().request_redraw(),
            Event::RedrawRequested(_) => {
                rotation *= rotation_delta;

                let mut frame = display.draw();

                frame.clear_color(0.0, 0.0, 0.0, 1.0);

                frame.draw(
                    &shape.vertices,
                    &shape.indices,
                    &program,
                    &uniform! { transform: Mat4::from_scale_rotation_translation(0.5*Vec3::ONE, rotation.normalize(), Vec3::ZERO).to_cols_array_2d() },
                    &params,
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
