#[macro_use]
extern crate glium;

use glam::{Mat4, Quat, Vec3};
use glium::{glutin, Surface};
use glutin::dpi::PhysicalPosition;
use glutin::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

pub mod shapes;
use shapes::platonic_solids::{PlatonicSolid, PlatonicSolids};

use crate::simple_targa::read_targa;

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new()
        .with_window_icon(read_icon("res/icon.tga").ok())
        .with_resizable(false)
        .with_title("Shapes")
        .with_position(PhysicalPosition::<i32>::from((50, 50)));
    let cb = glutin::ContextBuilder::new()
        .with_multisampling(4)
        .with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).expect("unable to create a new display");

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
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawEventsCleared => display.gl_window().window().request_redraw(),
            Event::RedrawRequested(_) => {
                rotation *= rotation_delta;
                let scale = Vec3::ONE
                    * if shape != 3 {
                        1.0
                    // the dodecahedron is rather large
                    } else {
                        0.5
                    };
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
            }

            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => match input {
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Up),
                        ..
                    } => match shape {
                        0 | 2 | 4 => shape = 1,
                        1 => shape = 3,
                        _ => (),
                    },
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Down),
                        ..
                    } => match shape {
                        3 => shape = 1,
                        1 => shape = 0,
                        _ => (),
                    },
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        ..
                    } => match shape {
                        4 => shape = 2,
                        2 => shape = 0,
                        _ => (),
                    },
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Right),
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
    });
}

fn read_icon(path: &str) -> std::io::Result<glutin::window::Icon> {
    let image = read_targa(path)?;

    Ok(glutin::window::Icon::from_rgba(image.bytes, image.width, image.height).unwrap())
}

pub mod simple_targa {
    use std::fs::File;
    use std::io::{Read, Result, Write};

    pub struct TargaImage {
        pub bytes: Vec<u8>,
        pub width: u32,
        pub height: u32,
    }

    impl TargaImage {
        pub fn new(bytes: Vec<u8>, width: u16, height: u16) -> Self {
            let width = width as u32;
            let height = height as u32;

            TargaImage {
                bytes,
                width,
                height,
            }
        }
    }

    const TGA_HDR: usize = 18;
    const TGA_WIDTH: usize = 12;
    const TGA_HEIGHT: usize = 14;

    pub fn read_targa(path: &str) -> Result<TargaImage> {
        const COMPONENTS: usize = 16;

        let mut file = File::open(path)?;

        let mut data = Vec::new();
        let _read = file.read_to_end(&mut data)?;

        let components = data[COMPONENTS];
        if components != 32 {
            panic!("unexpected TGA format");
        }
        let width = data[TGA_WIDTH + 1] as u32 * 256 + data[TGA_WIDTH] as u32;
        let height = data[TGA_HEIGHT + 1] as u32 * 256 + data[TGA_HEIGHT] as u32;
        let mut bytes = Vec::new();
        for i in 0..(width * height) as usize {
            let index = TGA_HDR + 4 * i;

            bytes.push(data[index + 2]);
            bytes.push(data[index + 1]);
            bytes.push(data[index + 0]);
            bytes.push(data[index + 3]);
        }

        Ok(TargaImage {
            bytes,
            width,
            height,
        })
    }

    pub fn write_targa(path: &str, mut image: TargaImage) -> Result<()> {
        let mut header = [0u8, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 32, 0];
        header[TGA_WIDTH] = (image.width % 256) as u8;
        header[TGA_WIDTH + 1] = (image.width / 256) as u8;
        header[TGA_HEIGHT] = (image.height % 256) as u8;
        header[TGA_HEIGHT + 1] = (image.height / 256) as u8;

        let mut file = File::create(path)?;

        file.write_all(&header)?;

        for i in 0..(image.width * image.height) as usize {
            let index = 4 * i;

            let byte = image.bytes[index];
            image.bytes[index] = image.bytes[index + 2];
            image.bytes[index + 2] = byte;
        }
        file.write_all(&image.bytes)?;

        Ok(())
    }
}
