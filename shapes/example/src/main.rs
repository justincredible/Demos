use std::f32::consts;

use glam::{Mat4, Quat, Vec3};
use glow::*;
use sdl2::event::Event;
use sdl2::image::LoadSurface;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::surface::Surface;

use shapes::kepler_poinsot::KpPolyhedron;
use shapes::polygon::Polygon;
use shapes::platonic_solid::PlatonicSolid;
use shapes::Shaper;
use shapes::shapes::ShapingError;

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

fn main() -> Result<(), ShapingError> {
    // Create a context from a sdl2 window
    let (gl, window, mut event_loop, _context) = create_sdl2_context();
    let ctx = egui::Context::default();
    let gl = std::sync::Arc::new(gl);
    let mut painter = egui_glow::Painter::new(gl.clone(), "", None, false).unwrap();

    // Create a shader program from source
    let program = create_program(
        &gl,
        VERTEX_SHADER_SOURCE,
        FRAGMENT_SHADER_SOURCE,
        GEOMETRY_SHADER_SOURCE,
    );
    unsafe { gl.use_program(Some(program)); }

    unsafe {
        gl.clear_color(0., 0., 0., 1.);
        gl.cull_face(glow::BACK);
    }
    let mut sides = 3;
    let config = Default::default();
    let mut shapes = [
        Shape::new(&gl, Polygon::new(sides).shape(config)?),
        Shape::new(&gl, PlatonicSolid::Tetrahedron.shape(config)?),
        Shape::new(&gl, PlatonicSolid::Hexahedron.shape(config)?),
        Shape::new(&gl, PlatonicSolid::Octahedron.shape(config)?),
        Shape::new(&gl, PlatonicSolid::Dodecahedron.shape(config)?),
        Shape::new(&gl, PlatonicSolid::Icosahedron.shape(config)?),
        Shape::new(&gl, KpPolyhedron::StellatedDodecahedron.shape(config)?),
        Shape::new(&gl, KpPolyhedron::GreatDodecahedron.shape(config)?),
        Shape::new(&gl, KpPolyhedron::GreatStellatedDodecahedron.shape(config)?),
        Shape::new(&gl, KpPolyhedron::GreatIcosahedron.shape(config)?),
    ];

    let initial_rotation = Quat::from_axis_angle(Vec3::ONE, 0.0);
    let mut rotation = initial_rotation;
    let mut rotation_delta = Quat::from_axis_angle(Vec3::ONE, 0.01);
    let mut rotating = true;

    let mut shape = 5;

    let rh_cross = config.orientation.is_ccw() && config.orientation.is_right()
        || config.orientation.is_cw() && config.orientation.is_left();
    unsafe {
        gl.uniform_1_f32(
            gl.get_uniform_location(program, "rh_cross").as_ref(),
            (2 * Into::<i8>::into(rh_cross) - 1).into(),
        );
        gl.uniform_1_f32(
            gl.get_uniform_location(program, "rh").as_ref(),
            (2 * Into::<i8>::into(config.orientation.is_right()) - 1).into(),
        );
        if config.orientation.is_right() {
            gl.depth_func(glow::GREATER);
            gl.clear_depth(0.);
        }
        if config.orientation.is_cw() {
            gl.front_face(glow::CW);
        }
    }

    println!(
        "Up and Down arrows modify vertices per face.\n\
        Left and Right arrows modify faces per vertex.\n\
        G switches between polyhedra and polygons.\n\
        R toggles rotation.\n\
        H returns object to initial orientation."
    );
    'render: loop {
        let mut raw_input = egui::RawInput::default();
        {
            for event in event_loop.poll_iter() {
                match event {
                    Event::Quit { .. } => break 'render,
                    Event::MouseButtonDown { mouse_btn, clicks, x, y, .. } => {
                        for _ in 0..clicks {
                            raw_input.events.push(
                                egui::Event::PointerButton {
                                    pos: [(x as u16).into(), (y as u16).into()].into(),
                                    button: sdl2_mouse_button_to_egui_pointer_button(mouse_btn),
                                    pressed: true,
                                    modifiers: egui::Modifiers::NONE,
                                }
                            );
                        }
                    },
                    Event::MouseButtonUp { mouse_btn, clicks, x, y, .. } => {
                        for _ in 0..clicks {
                            raw_input.events.push(
                                egui::Event::PointerButton {
                                    pos: [(x as u16).into(), (y as u16).into()].into(),
                                    button: sdl2_mouse_button_to_egui_pointer_button(mouse_btn),
                                    pressed: false,
                                    modifiers: egui::Modifiers::NONE,
                                }
                            );
                        }
                    },
                    Event::KeyUp { keycode: Some(keycode), keymod, repeat, .. } => {
                        raw_input.events.push(
                            egui::Event::Key {
                                key: sdl2_keycode_to_egui_key(keycode),
                                physical_key: None,
                                pressed: true,
                                repeat,
                                modifiers: sdl2_mod_to_egui_modifiers(keymod),
                            }
                        );
                    },
                    Event::KeyDown { keycode: Some(keycode), keymod, repeat, .. } => {
                        raw_input.events.push(
                            egui::Event::Key {
                                key: sdl2_keycode_to_egui_key(keycode),
                                physical_key: None,
                                pressed: false,
                                repeat,
                                modifiers: sdl2_mod_to_egui_modifiers(keymod),
                            }
                        );
                        match keycode {
                            Keycode::Up => match shape {
                                1 | 3 => shape = 2,
                                2 | 5 => shape = 4,
                                0 => {
                                    sides = (sides + 1).min(255);
                                    shapes[shape] = Shape::new(&gl, Polygon::new(sides).shape(config)?);
                                },
                                _ => (),
                            },
                            Keycode::Down => match shape {
                                4 => shape = 2,
                                2 => shape = 1,
                                0 => {
                                    if sides > 3 {
                                        sides -= 1;
                                    }
                                    shapes[shape] = Shape::new(&gl, Polygon::new(sides).shape(config)?);
                                },
                                _ => (),
                            },
                            Keycode::Left => match shape {
                                9 => shape = 8,
                                8 => shape = 7,
                                7 => shape = 6,
                                5 => shape = 3,
                                3 => shape = 1,
                                _ => (),
                            },
                            Keycode::Right => match shape {
                                1 | 2 => shape = 3,
                                3 | 4 => shape = 5,
                                6 => shape = 7,
                                7 => shape = 8,
                                8 => shape = 9,
                                _ => (),
                            },
                            Keycode::G => match shape {
                                0 => {
                                    sides = 3;
                                    shapes[shape] = Shape::new(&gl, Polygon::new(sides).shape(config)?);
                                    shape = 5;
                                },
                                1 ..= 5 => shape = 6,
                                _ => shape = 0,
                            },
                            Keycode::R => rotating = !rotating,
                            Keycode::Num0 => rotation_delta = Quat::from_axis_angle(Vec3::ZERO, 0.01),
                            Keycode::Num1 => rotation_delta = Quat::from_axis_angle(Vec3::new(0., 0., 1.), 0.01),
                            Keycode::Num2 => rotation_delta = Quat::from_axis_angle(Vec3::new(0., 1., 0.), 0.01),
                            Keycode::Num3 => rotation_delta = Quat::from_axis_angle(Vec3::new(0., 1., 1.), 0.01),
                            Keycode::Num4 => rotation_delta = Quat::from_axis_angle(Vec3::new(1., 0., 0.), 0.01),
                            Keycode::Num5 => rotation_delta = Quat::from_axis_angle(Vec3::new(1., 0., 1.), 0.01),
                            Keycode::Num6 => rotation_delta = Quat::from_axis_angle(Vec3::new(1., 1., 0.), 0.01),
                            Keycode::Num7 => rotation_delta = Quat::from_axis_angle(Vec3::ONE, 0.01),
                            Keycode::Minus => rotation_delta = rotation_delta.conjugate(),
                            Keycode::H => rotation = initial_rotation,
                            _ => (),
                        }
                    },
                    _ => (),
                }
            }
        }

        if rotating {
            rotation *= rotation_delta;
        }
        let scale = Vec3::ONE * if shape == 0 {
            let angle = consts::TAU / sides as f32;
            f32::sin(angle) / f32::cos(0.5 * angle)
        } else if shape == 4 || shape == 8 {
            // the dodecahedron is rather large
            0.7
        } else {
            1.0
        };
        let matrix = Mat4::from_scale_rotation_translation(scale, rotation.normalize(), Vec3::ZERO);
        set_uniform_matrix(
            &gl,
            program,
            "transform",
            &matrix.to_cols_array(),
        );

        const LEFT_PANEL: u16 = 200;
        const TOP_PANEL: u16 = 100;
        const SPACER: f32 = 10.;
        let full_output = ctx.run_ui(raw_input, |ui| {
            egui::Panel::left("left menu")
                .min_size(LEFT_PANEL.into())
                .show(ui, |ui| {
                    ui.label("Menu");
                    ui.add_space(SPACER);
                    if ui.button("Click me!").clicked() {
                        eprintln!("Clicked!");
                    }
                });
            egui::Panel::top("top menu")
                .min_size(TOP_PANEL.into())
                .show(ui, |ui| {
                    ui.label(
                        "Up and Down arrows modify vertices per face.\n\
                        Left and Right arrows modify faces per vertex.\n\
                        G switches between polyhedra and polygons.\n\
                        R toggles rotation.\n\
                        H returns object to initial orientation."
                    );
                });
        });
        let paint_jobs = ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);

            painter.paint_and_update_textures(
                [WINDOW_WIDTH, WINDOW_HEIGHT],
                full_output.pixels_per_point,
                &paint_jobs,
                &full_output.textures_delta,
            );

            gl.viewport(
                LEFT_PANEL.into(),
                0,
                (WINDOW_WIDTH - LEFT_PANEL as u32).try_into().unwrap(),
                (WINDOW_HEIGHT - TOP_PANEL as u32).try_into().unwrap(),
            );
            gl.enable(glow::DEPTH_TEST);
            gl.enable(glow::CULL_FACE);
            gl.use_program(Some(program));
            gl.bind_vertex_array(Some(shapes[shape].vertices.0));
            //gl.bind_buffer(glow::ARRAY_BUFFER, Some(shapes[shape].vertices.1));
            match &shapes[shape].indices.0 {
                Indices::None => gl.draw_arrays(glow::TRIANGLE_STRIP, 0, shapes[shape].vertices.2 as i32),
                Indices::One(buffer) => {
                    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.0));
                    gl.draw_elements(
                        shapes[shape].indices.1,
                        buffer.1 as i32,
                        glow::UNSIGNED_BYTE,
                        0,
                    );
                },
                Indices::Some(vertex) => {
                    for buffer in vertex {
                        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.0));
                        gl.draw_elements(
                            shapes[shape].indices.1,
                            buffer.1 as i32,
                            glow::UNSIGNED_BYTE,
                            0,
                        );
                    }
                },
            }
        }

        window.gl_swap_window();
    }

    painter.destroy();

    // Clean up
    unsafe {
        gl.delete_program(program);
        gl.bind_vertex_array(None);
        gl.bind_buffer(glow::ARRAY_BUFFER, None);
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
        for shape in shapes {
            match &shape.indices.0 {
                Indices::None => (),
                Indices::One(buffer) => gl.delete_buffer(buffer.0),
                Indices::Some(vertex) => for buffer in vertex {
                    gl.delete_buffer(buffer.0);
                },
            }
            gl.delete_buffer(shape.vertices.1);
            gl.delete_vertex_array(shape.vertices.0);
        }
    }

    Ok(())
}

fn create_sdl2_context() -> (
    glow::Context,
    sdl2::video::Window,
    sdl2::EventPump,
    sdl2::video::GLContext,
) {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();
    let gl_attr = video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);
    gl_attr.set_context_flags().forward_compatible().set();
    let mut window = video
        .window("Shapes", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .opengl()
        .build()
        .unwrap();
    window.set_icon(Surface::from_file("res/icon.tga").unwrap());
    let gl_context = window.gl_create_context().unwrap();
    let gl = unsafe { glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _) };
    let event_loop = sdl.event_pump().unwrap();

    (gl, window, event_loop, gl_context)
}

fn create_program(
    gl: &glow::Context,
    vertex_shader_source: &str,
    fragment_shader_source: &str,
    geometry_shader_source: &str,
) -> NativeProgram {
    let program = unsafe { gl.create_program().expect("Cannot create program") };

    let shader_sources = [
        (glow::VERTEX_SHADER, vertex_shader_source),
        (glow::FRAGMENT_SHADER, fragment_shader_source),
        (glow::GEOMETRY_SHADER, geometry_shader_source),
    ];

    let mut shaders = Vec::with_capacity(shader_sources.len());

    for (shader_type, shader_source) in shader_sources.iter() {
        let shader = unsafe { gl
            .create_shader(*shader_type)
            .expect("Cannot create shader") };
        unsafe {
            gl.shader_source(shader, shader_source);
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
        }
        shaders.push(shader);
    }

    unsafe {
        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }
    }

    for shader in shaders {
        unsafe {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }
    }

    program
}

fn create_vertex_buffer(gl: &glow::Context, vertices: &[[f32; 3]]) -> (NativeVertexArray, NativeBuffer, usize) {
    let vao = unsafe { gl.create_vertex_array().unwrap() };
    unsafe { gl.bind_vertex_array(Some(vao)); }
    let vbo = unsafe { gl.create_buffer().unwrap() };
    unsafe { gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo)); }
    let vertices_flat = vertices.iter().flatten().map(Clone::clone).collect::<Vec<_>>();
    let vertices_u8: &[u8] = unsafe {
        core::slice::from_raw_parts(
            vertices_flat.as_ptr() as *const u8,
            vertices_flat.len() * core::mem::size_of::<f32>(),
        )
    };
    unsafe { gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, vertices_u8, glow::STATIC_DRAW); }
    unsafe {
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 3 * core::mem::size_of::<f32>() as i32, 0);
        gl.bind_vertex_array(None);
        gl.bind_buffer(glow::ARRAY_BUFFER, None);
    }

    (vao, vbo, vertices_flat.len())
}

fn create_index_buffer(gl: &glow::Context, indices: &[u8]) -> (NativeBuffer, usize) {
    let ebo = unsafe { gl.create_buffer().unwrap() };
    unsafe {
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(ebo));
        gl.buffer_data_u8_slice(glow::ELEMENT_ARRAY_BUFFER, indices, glow::STATIC_DRAW);
        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, None);
    }

    (ebo, indices.len())
}

fn set_uniform_matrix(gl: &glow::Context, program: NativeProgram, name: &str, matrix: &[f32]) {
    let uniform_location = unsafe { gl.get_uniform_location(program, name) };
    unsafe {
        gl.uniform_matrix_4_f32_slice(
            uniform_location.as_ref(),
            false,
            matrix,
        );
    }
}

fn sdl2_keycode_to_egui_key(keycode: Keycode) -> egui::Key {
    match keycode {
        Keycode::KP_TAB | Keycode::TAB => egui::Key::Tab,
        Keycode::KP_SPACE | Keycode::SPACE => egui::Key::Space,
        Keycode::KP_ENTER | Keycode::RETURN | Keycode::RETURN2 => egui::Key::Enter,
        _ => egui::Key::Escape,
    }
}

fn sdl2_mouse_button_to_egui_pointer_button(mouse_button: MouseButton) -> egui::PointerButton {
    match mouse_button {
        MouseButton::Left => egui::PointerButton::Primary,
        MouseButton::Middle => egui::PointerButton::Middle,
        MouseButton::Right => egui::PointerButton::Secondary,
        _ => egui::PointerButton::Extra2,
    }
}

fn sdl2_mod_to_egui_modifiers(modifiers: sdl2::keyboard::Mod) -> egui::Modifiers {
    match modifiers {
        _ => Default::default(),
    }
}

enum Indices {
    None,
    One((NativeBuffer, usize)),
    Some(Vec<(NativeBuffer, usize)>),
}

struct Shape {
    vertices: (NativeVertexArray, NativeBuffer, usize),
    indices: (Indices, u32),
}

impl Shape {
    pub fn new(gl: &glow::Context, shape: shapes::Shape<f32, u8>) -> Self {
       if shape.is_strips() {
            let vertices = create_vertex_buffer(gl, shape.vertices());
            let indices = if shape.indices().is_empty() {
                (Indices::None, glow::TRIANGLE_STRIP)
            } else {
                let buffers = shape.indices().iter().map(|strip|
                    create_index_buffer(gl, strip)
                ).collect();
                (Indices::Some(buffers), glow::TRIANGLE_STRIP)
            };

            Self { vertices, indices }
        } else {
            let vertices = create_vertex_buffer(gl, shape.vertices());
            let indices = (Indices::One(create_index_buffer(gl, shape.indices()[0])), glow::TRIANGLES);

            Self { vertices, indices }
        }
    }
}

const VERTEX_SHADER_SOURCE: &str = r#"#version 150

    in vec3 position;

    out vec3 v_position;

    void main() {
        v_position = position;
    }
"#;
const FRAGMENT_SHADER_SOURCE: &str = r#"#version 140

    const vec4 AMBIENT = vec4(0.01, 0.01, 0.01, 1);

    in vec3 g_normal;

    out vec4 f_colour;

    uniform float rh;

    void main() {
        vec3 light_dir = vec3(-1.0, 1, rh * 1);
        float saturation = clamp(dot(normalize(light_dir), g_normal), 0, 1);
        vec4 colour = vec4(1.0, gl_FrontFacing, 1, 1);

        f_colour = saturation * colour + AMBIENT;
    }
"#;
const GEOMETRY_SHADER_SOURCE: &str = r#"#version 150

    layout(triangles) in;
    layout(triangle_strip, max_vertices = 3) out;

    in vec3 v_position[];

    out vec3 g_normal;

    uniform float rh_cross;
    uniform mat4 transform;

    void main() {
        vec3 a = normalize(v_position[1] - v_position[0]);
        vec3 b = normalize(v_position[2] - v_position[0]);
        vec3 normal = normalize(mat3x3(transform) * rh_cross * cross(a, b));

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
"#;
