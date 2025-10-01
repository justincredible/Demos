#[macro_use]
extern crate glium;
extern crate glam;
extern crate winit;
extern crate raw_window_handle;
extern crate simple_targa;
extern crate shapes;

use std::num::NonZeroU32;
use std::f32::consts;

use glam::{Mat4, Quat, Vec3};
use glium::{backend::Facade, glutin, index::{IndicesSource, NoIndices, PrimitiveType}, IndexBuffer, Surface, VertexBuffer};
use winit::dpi::PhysicalPosition;
use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
use winit::keyboard::{KeyCode, PhysicalKey::Code};
use glutin::context::NotCurrentGlContext;
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use raw_window_handle::HasRawWindowHandle;

use simple_targa::read_targa;

use shapes::polygons::Polygon;
use shapes::platonic_solids::PlatonicSolids;
use shapes::Shaper;

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

    let mut sides = 3;
    let config = Default::default();
    let mut shapes = vec![
        Shape::new(&display, Polygon::new(sides).make(config)),
        Shape::new(&display, PlatonicSolids::Tetrahedron.make(config)),
        Shape::new(&display, PlatonicSolids::Hexahedron.make(config)),
        Shape::new(&display, PlatonicSolids::Octahedron.make(config)),
        Shape::new(&display, PlatonicSolids::Dodecahedron.make(config)),
        Shape::new(&display, PlatonicSolids::Icosahedron.make(config)),
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

    let initial_rotation = Quat::from_axis_angle(Vec3::ONE, 0.0);
    let mut rotation = initial_rotation;
    let rotation_delta = Quat::from_axis_angle(Vec3::ONE, 0.01);
    let mut rotating = true;

    let mut shape = 5;

    println!(
        "Up and Down arrows modify vertices per face.\n\
        Left and Right arrows modify faces per vertex.\n\
        G switches to and from polygons.\n\
        R toggles rotation.\n\
        H returns object to initial orientation."
    );
    event_loop.run(move |event, window_target| {
        if let Event::WindowEvent { event, .. } = event {
            match event {
                WindowEvent::CloseRequested => window_target.exit(),
                WindowEvent::RedrawRequested => {
                    if rotating {
                        rotation *= rotation_delta;
                    }
                    let scale = Vec3::ONE * if shape == 0 {
                        let angle = consts::TAU / sides as f32;
                        f32::sin(angle) / f32::cos(0.5 * angle)
                    } else if shape == 4 {
                        // the dodecahedron is rather large
                        0.5
                    } else {
                        1.0
                    };
                    let matrix =
                        Mat4::from_scale_rotation_translation(scale, rotation.normalize(), Vec3::ZERO);

                    let mut frame = display.draw();

                    frame.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), -1.0);

                    frame
                        .draw(
                            &shapes[shape].vertices,
                            shapes[shape].indices.source(),
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
                        1 | 3 | 5 => shape = 2,
                        2 => shape = 4,
                        0 => {
                            sides = sides.saturating_add(1);
                            shapes[shape] = Shape::new(&display, Polygon::new(sides).make(config));
                        },
                        _ => (),
                    },
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::ArrowDown),
                        ..
                    } => match shape {
                        4 => shape = 2,
                        2 => shape = 1,
                        0 => {
                            if sides > 3 {
                                sides -= 1;
                            }
                            shapes[shape] = Shape::new(&display, Polygon::new(sides).make(config));
                        },
                        _ => (),
                    },
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::ArrowLeft),
                        ..
                    } => match shape {
                        5 => shape = 3,
                        3 => shape = 1,
                        _ => (),
                    },
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::ArrowRight),
                        ..
                    } => match shape {
                        1 => shape = 3,
                        3 => shape = 5,
                        _ => (),
                    },
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::KeyG),
                        ..
                    } => match shape {
                        0 => {
                            sides = 3;
                            shapes[shape] = Shape::new(&display, Polygon::new(sides).make(config));
                            shape = 5;
                        },
                        _ => shape = 0,
                    },
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::KeyR),
                        ..
                    } => rotating = !rotating,
                    KeyEvent {
                        state: ElementState::Pressed,
                        physical_key: Code(KeyCode::KeyH),
                        ..
                    } => rotation = initial_rotation,
                    _ => (),
                },
                _ => (),
            }
        }
    }).unwrap();
}

#[derive(Clone, Copy)]
struct PosVertex {
    position: [f32; 3],
}

impl From<[f32; 3]> for PosVertex {
    fn from(position: [f32; 3]) -> Self {
       Self { position }
    }
}

implement_vertex!(PosVertex, position);

enum Indices {
    None(NoIndices),
    One(Box<IndexBuffer<u8>>),
}

impl Indices {
    pub fn source<'a>(&'a self) -> IndicesSource<'a> {
        match self {
            Indices::None(i) => i.into(),
            Indices::One(i) => (&(**i)).into(),
        }
    }
}

struct Shape {
    vertices: VertexBuffer<PosVertex>,
    indices: Indices,
}

impl Shape {
    pub fn new(display: &impl Facade, shape: shapes::Shape<f32, u8>) -> Self {
        if let shapes::Shape::Triangles { vertices, indices } = shape {
            let vertices = VertexBuffer::new(
                display,
                &vertices
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<_>>()).unwrap();
            let indices = Indices::One(Box::new(IndexBuffer::new(
                display,
                PrimitiveType::TrianglesList,
                &indices).unwrap()));

            Self { vertices, indices }
        } else if let shapes::Shape::Strips { vertices, strips } = shape {
            let vertices = VertexBuffer::new(
                display,
                &vertices
                    .iter()
                    .map(|&p| p.into())
                    .collect::<Vec<_>>()).unwrap();
            let indices = if strips.is_empty() {
                Indices::None(NoIndices(PrimitiveType::TriangleStrip))
            } else {
                Indices::One(Box::new(IndexBuffer::new(
                display,
                PrimitiveType::TriangleStrip,
                &strips[0]).unwrap()))
            };

            Self { vertices, indices }
        } else { panic!() }
    }
}

fn read_icon(path: &str) -> std::io::Result<winit::window::Icon> {
    let image = read_targa(path)?;

    Ok(winit::window::Icon::from_rgba(image.bytes, image.width, image.height).unwrap())
}
