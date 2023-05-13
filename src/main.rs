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

        out vec3 v_position;

        void main() {
            v_position = position;
        }
        "#,
        /* Fragment shader */r#"
        #version 140

        const vec3 LIGHT_DIR = vec3(-1.0, 1, 1);
        const vec4 AMBIENT = vec4(0.01, 0.01, 0.01, 1);

        in vec3 g_normal;

        out vec4 f_colour;

        void main() {
            float saturation = clamp(dot(normalize(LIGHT_DIR), g_normal), 0, 1);
            vec4 colour = vec4(1.0, gl_FrontFacing, 1, 1);

            f_colour = saturation * colour + AMBIENT;
        }
        "#,
        /* Geometry shader */Some(r#"
            #version 150

            layout(triangles) in;
            layout(triangle_strip, max_vertices = 3) out;

            in vec3 v_position[];

            out vec3 g_normal;

            uniform mat4 transform;

            void main() {
                vec3 a = normalize(v_position[1] - v_position[0]);
                vec3 b = normalize(v_position[2] - v_position[0]);
                vec3 normal = normalize(mat3x3(transform) * cross(a, b));

                gl_Position = transform * vec4(v_position[0], 1);
                g_normal = normal;
                EmitVertex();

                gl_Position = transform * vec4(v_position[1], 1);
                g_normal = normal;
                EmitVertex();

                gl_Position = transform * vec4(v_position[2], 1);
                g_normal = normal;
                EmitVertex();

                EndPrimitive();
            }
            "#
        ),
    ).unwrap();

    let shape = Dodecahedron::new(&display);

    let params = glium::DrawParameters {
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
        //polygon_mode: glium::draw_parameters::PolygonMode::Line,
        depth: glium::draw_parameters::Depth {
            write: true,
            test: glium::draw_parameters::DepthTest::IfMore,
            ..Default::default()
        },
        ..Default::default()
    };

    let mut rotation = Quat::from_axis_angle(Vec3::ONE, 0.0);
    let rotation_delta = Quat::from_axis_angle(Vec3::ONE, 0.01);

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawEventsCleared => display.gl_window().window().request_redraw(),
            Event::RedrawRequested(_) => {
                rotation *= rotation_delta;
                let matrix = Mat4::from_scale_rotation_translation(0.5*Vec3::ONE, rotation.normalize(), Vec3::ZERO);

                let mut frame = display.draw();

                frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), -1.0);

                frame.draw(
                    &shape.vertices,
                    &shape.indices,
                    &program,
                    &uniform! { transform: matrix.to_cols_array_2d() },
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
