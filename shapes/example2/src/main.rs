use glow::*;

fn main() {
    // Create a context from a sdl2 window
    let (gl, window, mut event_loop, _context) = create_sdl2_context();

    // Create a shader program from source
    let program = create_program(
        &gl,
        VERTEX_SHADER_SOURCE,
        FRAGMENT_SHADER_SOURCE,
        GEOMETRY_SHADER_SOURCE,
    );
    unsafe { gl.use_program(Some(program)); }

    // Create a vertex buffer and vertex array object
    let (vbo, vao) = create_vertex_buffer(&gl);

    // Upload some uniforms
    set_uniform_matrix(
        &gl,
        program,
        "transform",
        &[1.0f32, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.],
    );

    unsafe { gl.clear_color(0.0, 0.0, 0.0, 1.0); }

    'render: loop {
        {
            for event in event_loop.poll_iter() {
                if let sdl2::event::Event::Quit { .. } = event {
                    break 'render;
                }
            }
        }

        unsafe {
            gl.clear(glow::COLOR_BUFFER_BIT);
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
            window.gl_swap_window();
        }
    }

    // Clean up
    unsafe {
        gl.delete_program(program);
        gl.delete_vertex_array(vao);
        gl.delete_buffer(vbo)
    }
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
    let window = video
        .window("Shapes", 800, 600)
        .opengl()
        .resizable()
        .build()
        .unwrap();
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

fn create_vertex_buffer(gl: &glow::Context) -> (NativeBuffer, NativeVertexArray) {
    // This is a flat array of f32s that are to be interpreted as vec3s.
    let triangle_vertices = [0.0f32, 0.5, 0., -0.5, -0.5, 0., 0.5, -0.5, 0.];
    let triangle_vertices_u8: &[u8] = unsafe { core::slice::from_raw_parts(
        triangle_vertices.as_ptr() as *const u8,
        triangle_vertices.len() * core::mem::size_of::<f32>(),
    ) };

    // We construct a buffer and upload the data
    let vbo = unsafe { gl.create_buffer().unwrap() };
    unsafe {
        gl.bind_buffer(glow::ARRAY_BUFFER, Some(vbo));
        gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, triangle_vertices_u8, glow::STATIC_DRAW);
    }

    // We now construct a vertex array to describe the format of the input buffer
    let vao = unsafe { gl.create_vertex_array().unwrap() };
    unsafe {
        gl.bind_vertex_array(Some(vao));
        gl.enable_vertex_attrib_array(0);
        gl.vertex_attrib_pointer_f32(0, 3, glow::FLOAT, false, 12, 0);
    }

    (vbo, vao)
}

fn set_uniform_matrix(gl: &glow::Context, program: NativeProgram, name: &str, matrix: &[f32]) {
    let uniform_location = unsafe { gl.get_uniform_location(program, name) };
    unsafe { gl.uniform_matrix_4_f32_slice(
        uniform_location.as_ref(),
        false,
        matrix,
    ); }
}

const VERTEX_SHADER_SOURCE: &str = r#"#version 150

    in vec3 position;

    out vec3 v_position;

    void main() {
        v_position = position;
    }
"#;
const FRAGMENT_SHADER_SOURCE: &str = r#"#version 140

    const vec3 LIGHT_DIR = vec3(-1.0, 1, 1);
    const vec4 AMBIENT = vec4(0.01, 0.01, 0.01, 1);

    in vec3 g_normal;

    out vec4 f_colour;

    void main() {
        float saturation = clamp(dot(normalize(LIGHT_DIR), g_normal), 0, 1);
        vec4 colour = vec4(1.0, gl_FrontFacing, 1, 1);

        f_colour = saturation * colour + AMBIENT;
    }
"#;
const GEOMETRY_SHADER_SOURCE: &str = r#"#version 150

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
"#;
