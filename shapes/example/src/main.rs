use std::f32::consts;

use glam::{Mat4, Quat, Vec3};
use glow::*;
use sdl2::image::LoadSurface;
use sdl2::surface::Surface;

use shapes::kepler_poinsot::KpPolyhedron;
use shapes::polygon::Polygon;
use shapes::platonic_solid::PlatonicSolid;
use shapes::Shaper;
use shapes::shapes::ShapingError;

mod egui_sdl2;
use egui_sdl2::{process_event, run_ui};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;
const LEFT_PANEL: u16 = 200;
const TOP_PANEL: u16 = 100;

fn main() -> Result<(), ShapingError> {
    // Create a context from a sdl2 window
    let (window, mut event_loop, gl, context, ui_gl, ui_context) = create_sdl2_context();
    let ctx = egui::Context::default();
    let ui_gl = std::sync::Arc::new(ui_gl);
    let mut painter = egui_glow::Painter::new(ui_gl, "", None, false).unwrap();
    window.gl_make_current(&context).unwrap();

    // Create a shader program from source
    let program = create_program(
        &gl,
        VERTEX_SHADER_SOURCE,
        FRAGMENT_SHADER_SOURCE,
        GEOMETRY_SHADER_SOURCE,
    );
    unsafe { gl.use_program(Some(program)); }

    unsafe {
        gl.viewport(
            LEFT_PANEL.into(),
            0,
            (WINDOW_WIDTH - LEFT_PANEL as u32).try_into().unwrap(),
            (WINDOW_HEIGHT - TOP_PANEL as u32).try_into().unwrap(),
        );
        gl.clear_color(0., 0., 0., 1.);
        gl.enable(glow::DEPTH_TEST);
        gl.enable(glow::CULL_FACE);
        gl.cull_face(glow::BACK);
    }

    let mut state = ExampleState {
        sides: 3,
        reset_polygon: false,
        shape: 5,
        rotating: true,
        initial_rotation: Quat::from_axis_angle(Vec3::ONE, 0.0),
        rotation: Quat::from_axis_angle(Vec3::ONE, 0.0),
        rotation_delta: Quat::from_axis_angle(Vec3::ONE, 0.01),
        should_quit: false,
    };
    let config = Default::default();
    let mut shapes = [
        Shape::new(&gl, Polygon::new(state.sides).shape(config)?),
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

    'render: loop {
        let mut raw_input = egui::RawInput::default();
        {
            for event in event_loop.poll_iter() {
                process_event(event, &mut raw_input, &mut state)
            }
        }

        let full_output = run_ui(&ctx, raw_input, &mut state);
        let paint_jobs = ctx.tessellate(full_output.shapes, full_output.pixels_per_point);

        if state.should_quit {
            break 'render;
        }

        if state.reset_polygon {
            shapes[0] = Shape::new(&gl, Polygon::new(state.sides).shape(config)?);
            state.reset_polygon = false;
        }

        if state.rotating {
            state.rotation *= state.rotation_delta;
        }

        let scale = Vec3::ONE * if state.shape == 0 {
            let angle = consts::TAU / state.sides as f32;
            f32::sin(angle) / f32::cos(0.5 * angle)
        } else if state.shape == 4 || state.shape == 8 {
            // the dodecahedron is rather large
            0.7
        } else {
            1.0
        };

        let matrix = Mat4::from_scale_rotation_translation(scale, state.rotation.normalize(), Vec3::ZERO);
        set_uniform_matrix(
            &gl,
            program,
            "transform",
            &matrix.to_cols_array(),
        );

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT | glow::DEPTH_BUFFER_BIT);
            gl.bind_vertex_array(Some(shapes[state.shape].vertices.0));
            //gl.bind_buffer(glow::ARRAY_BUFFER, Some(shapes[shape].vertices.1));
            match &shapes[state.shape].indices.0 {
                Indices::None => gl.draw_arrays(glow::TRIANGLE_STRIP, 0, shapes[state.shape].vertices.2 as i32),
                Indices::One(buffer) => {
                    gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.0));
                    gl.draw_elements(
                        shapes[state.shape].indices.1,
                        buffer.1 as i32,
                        glow::UNSIGNED_BYTE,
                        0,
                    );
                },
                Indices::Some(vertex) => {
                    for buffer in vertex {
                        gl.bind_buffer(glow::ELEMENT_ARRAY_BUFFER, Some(buffer.0));
                        gl.draw_elements(
                            shapes[state.shape].indices.1,
                            buffer.1 as i32,
                            glow::UNSIGNED_BYTE,
                            0,
                        );
                    }
                },
            }
        }

        window.gl_make_current(&ui_context).unwrap();
        painter.paint_and_update_textures(
            [WINDOW_WIDTH, WINDOW_HEIGHT],
            full_output.pixels_per_point,
            &paint_jobs,
            &full_output.textures_delta,
        );

        window.gl_swap_window();
        window.gl_make_current(&context).unwrap();
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
    sdl2::video::Window,
    sdl2::EventPump,
    glow::Context,
    sdl2::video::GLContext,
    glow::Context,
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
    let gl_context1 = window.gl_create_context().unwrap();
    let gl1 = unsafe { glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _) };
    let gl_context2 = window.gl_create_context().unwrap();
    let gl2 = unsafe { glow::Context::from_loader_function(|s| video.gl_get_proc_address(s) as *const _) };
    let event_loop = sdl.event_pump().unwrap();

    (window, event_loop, gl1, gl_context1, gl2, gl_context2)
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

struct ExampleState {
    pub sides: u16,
    pub reset_polygon: bool,
    pub shape: usize,
    pub rotating: bool,
    pub initial_rotation: Quat,
    pub rotation: Quat,
    pub rotation_delta: Quat,
    pub should_quit: bool,
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
