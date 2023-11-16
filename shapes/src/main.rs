#[macro_use]
extern crate glium;

use glam::{Mat4, Quat, Vec3};
use glium::{glutin, Surface};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey::Code};
use glutin::context::NotCurrentGlContext;
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::HasRawWindowHandle;
pub mod shapes;
use shapes::platonic_solids::{PlatonicSolid, PlatonicSolids};

use std::num::NonZeroU32;

use simple_targa::read_targa;

fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let wb = winit::window::WindowBuilder::new()
        .with_window_icon(read_icon("res/icon.tga").ok())
        .with_resizable(false)
        .with_title("Shapes")
        .with_position(PhysicalPosition::<i32>::from((50, 50)));
    let (window, config) = glutin_winit::DisplayBuilder::new().with_window_builder(Some(wb)).build(
        &event_loop,
        glutin::config::ConfigTemplateBuilder::new(),
        | mut config | {
            config.next().unwrap()
        }
    ).unwrap();
    let window = window.unwrap();
    let cab = glutin::context::ContextAttributesBuilder::new();
    let not_current_context = unsafe {
        config.display().create_context(&config, &cab.build(Some(window.raw_window_handle()))).unwrap()
    };
    let sab: SurfaceAttributesBuilder<WindowSurface> = SurfaceAttributesBuilder::new();
    let window_surface = unsafe {
        config.display().create_window_surface(
            &config,
            &sab.build(
                window.raw_window_handle(),
                NonZeroU32::new(800u32).unwrap(),
                NonZeroU32::new(600u32).unwrap()
            )
        ).unwrap()
    };
    let current_context = not_current_context.treat_as_possibly_current();
    let display = glium::Display::new(current_context, window_surface).expect("unable to create a new display");

    let program = glium::Program::from_source(
        &display,
        /* Vertex shader */
        r#"
            #version 150

            in vec3 position;

            out vec3 v_position;

            void main() {
                v_position = position;
            }
        "#,
        /* Fragment shader */
        r#"
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
        /* Geometry shader */
        Some(
            r#"
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
        "#,
        ),
    )
    .unwrap();

    let shapes = vec![
        PlatonicSolid::new(&display, PlatonicSolids::Tetrahedron),
        PlatonicSolid::new(&display, PlatonicSolids::Hexahedron),
        PlatonicSolid::new(&display, PlatonicSolids::Octahedron),
        PlatonicSolid::new(&display, PlatonicSolids::Dodecahedron),
        PlatonicSolid::new(&display, PlatonicSolids::Icosahedron),
    ];

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

    let mut shape = 4;

    println!(
        "Up and Down arrows modify vertices per face.\n\
        Left and Right arrows modify faces per vertex."
    );
    event_loop.run(move |event, window_target| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    rotation *= rotation_delta;
                    // the dodecahedron is rather large
                    let scale = Vec3::ONE * if shape != 3 { 1.0 } else { 0.5 };
                    let matrix =
                        Mat4::from_scale_rotation_translation(scale, rotation.normalize(), Vec3::ZERO);

                    let mut frame = display.draw();

                    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), -1.0);

                    frame
                        .draw(
                            &shapes[shape].vertices,
                            &shapes[shape].indices,
                            &program,
                            &uniform! { transform: matrix.to_cols_array_2d() },
                            &params,
                        )
                        .unwrap();

                    frame.finish().unwrap();

                    window.request_redraw();
                },
                WindowEvent::KeyboardInput { event, .. } => match event {
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::ArrowUp),
                        ..
                    } => match shape {
                        0 | 2 | 4 => shape = 1,
                        1 => shape = 3,
                        _ => (),
                    },
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::ArrowDown),
                        ..
                    } => match shape {
                        3 => shape = 1,
                        1 => shape = 0,
                        _ => (),
                    },
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::ArrowLeft),
                        ..
                    } => match shape {
                        4 => shape = 2,
                        2 => shape = 0,
                        _ => (),
                    },
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::ArrowRight),
                        ..
                    } => match shape {
                        0 => shape = 2,
                        2 => shape = 4,
                        _ => (),
                    },
                    _ => (),
                },
                _ => (),
            },

            _ => (),
        }
    }).unwrap();
}

fn read_icon(path: &str) -> std::io::Result<winit::window::Icon> {
    let image = read_targa(path)?;

    Ok(winit::window::Icon::from_rgba(image.bytes, image.width, image.height).unwrap())
}
