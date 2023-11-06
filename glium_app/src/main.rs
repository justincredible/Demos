#[macro_use]
extern crate glium;

pub mod camera;
use camera::CameraState;
pub mod debug;
use debug::{DebugWindow, HALF_DEBUG};
pub mod engine;
use engine::{engine::start_loop, WindowedDisplay};
use engine::input::{process_input, KeyboardState};
use engine::screenshot::AsyncScreenshotTaker;
use engine::simple_targa::{read_targa, write_targa, TargaImage};
pub mod fxaa;
pub mod shapes;
use shapes::shapes::{Cube, CubeInstances, SpritesBatch, CUBE_INSTANCES, SPRITES_COUNT};

use glam::{Mat4, Quat, Vec3};
use glium::glutin;
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use glutin::context::{ContextAttributesBuilder, NotCurrentGlContext};
use glutin::display::{GetGlDisplay, GlDisplay};
use winit::window::Fullscreen;
use raw_window_handle::HasRawWindowHandle;
use glium::{Api, Profile, Surface, Version};
use std::f32::consts::TAU;
use std::fs::File;
use std::io::Read;

fn main() {
    use fxaa::fxaa;

    let event_loop = winit::event_loop::EventLoop::new();
    let wb = winit::window::WindowBuilder::new()
        .with_resizable(false)
        .with_title("Glium-based application")
        .with_window_icon(read_icon("resource/glium.tga").ok())
        .with_position(winit::dpi::PhysicalPosition::<i32>::from((50, 50)));
    let ctb = glutin::config::ConfigTemplateBuilder::new();
    // window and display backed config
    let (window, dbc) = glutin_winit::DisplayBuilder::new()
        .with_window_builder(Some(wb))
        .build(
            &event_loop,
            ctb,
            | mut config | config.next().unwrap(),
        )
        .unwrap(); // safety panic
    let window = window.unwrap(); // safety panic

    let glutin_display = &dbc.display();
    let sab: SurfaceAttributesBuilder<WindowSurface> = SurfaceAttributesBuilder::new();
    // SAFETY: main window is kept alive indefinitely and window creation errors panic
    let surface = unsafe {
        glutin_display
            .create_window_surface(
                &dbc,
                &sab.build(
                    window.raw_window_handle(),
                    std::num::NonZeroU32::new(800).unwrap(),
                    std::num::NonZeroU32::new(600).unwrap() ) )
            .unwrap()
    };
    let ca = ContextAttributesBuilder::new().build(Some(window.raw_window_handle()));
    // SAFETY: main window is kept alive indefinitely and window creation errors panic
    let ncc = unsafe {
        glutin_display.create_context(&dbc, &ca).unwrap()
    };
    let context = ncc.treat_as_possibly_current();

    let debug_cab = ContextAttributesBuilder::new().with_sharing(&context);

    let display = glium::Display::new(context, surface).expect("unable to create glium display");
    let windowed_display = WindowedDisplay::new(window, display);

    display_info(windowed_display.display());

    let mut debug = DebugWindow::new(&dbc, debug_cab, &event_loop);

    let cube = Cube::new(windowed_display.display());
    let cubes = CubeInstances::new(windowed_display.display());

    let subroutine_shader = glium::Program::from_source(
            windowed_display.display(),
            &read_shader("src/subroutine.vs").unwrap(),
            &read_shader("src/subroutine.fs").unwrap(),
            None )
        .unwrap();
    let tessellancing_shader = glium::Program::new(
        windowed_display.display(),
        glium::program::SourceCode {
            vertex_shader: &read_shader("src/tessellancing.vs").unwrap(),
            tessellation_control_shader: Some(&read_shader("src/tessellancing.tcs").unwrap()),
            tessellation_evaluation_shader: Some(&read_shader("src/tessellancing.tes").unwrap()),
            geometry_shader: Some(&read_shader("src/tessellancing.gs").unwrap()),
            fragment_shader: &read_shader("src/tessellancing.fs").unwrap(),
        },
    )
    .unwrap();
    let sprites_shader = glium::Program::from_source(
        windowed_display.display(),
        &read_shader("src/sprites.vs").unwrap(),
        &read_shader("src/sprites.fs").unwrap(),
        None,
    )
    .unwrap();
    let shadow_map_shader = glium::Program::from_source(
        windowed_display.display(),
        &read_shader("src/shadow_map.vs").unwrap(),
        &read_shader("src/shadow_map.fs").unwrap(),
        None,
    )
    .unwrap();
    let shadows_shader = glium::Program::from_source(
        windowed_display.display(),
        &read_shader("src/shadows.vs").unwrap(),
        &read_shader("src/shadows.fs").unwrap(),
        None,
    )
    .unwrap();

    let targa = read_targa("resource/opengl.tga").unwrap();
    let tessell_img =
        glium::texture::RawImage2d::from_raw_rgba(targa.bytes, (targa.width, targa.height));
    let dbg_img = glium::texture::RawImage2d::from_raw_rgba(
        (&tessell_img.data).to_vec(),
        (tessell_img.width, tessell_img.height),
    );
    let opengl_texture =
        glium::texture::CompressedSrgbTexture2d::new(windowed_display.display(), tessell_img).unwrap();
    let debug_texture = glium::Texture2d::new(windowed_display.display(), dbg_img).unwrap();
    debug_texture.as_surface().blit_whole_color_to(
        &debug.image().as_surface(),
        &glium::BlitTarget {
            left: HALF_DEBUG,
            bottom: HALF_DEBUG,
            width: 400,
            height: 300,
        },
        glium::uniforms::MagnifySamplerFilter::Linear,
    );

    const SHADOW_MAP_SIZE: u32 = 1024;

    let shadow_texture =
        glium::Texture2d::empty(windowed_display.display(), SHADOW_MAP_SIZE, SHADOW_MAP_SIZE).unwrap();
    let shadow_rect = glium::BlitTarget {
        left: 0,
        bottom: 0,
        width: SHADOW_MAP_SIZE as i32 / 2,
        height: SHADOW_MAP_SIZE as i32 / 2,
    };
    let depth_texture =
        glium::texture::DepthTexture2d::empty(windowed_display.display(), SHADOW_MAP_SIZE, SHADOW_MAP_SIZE).unwrap();

    let config_texture_0 = glium::texture::UnsignedTexture2d::empty_with_format(
        windowed_display.display(),
        glium::texture::UncompressedUintFormat::U8,
        glium::texture::MipmapsOption::NoMipmap,
        HALF_DEBUG,
        HALF_DEBUG,
    )
    .unwrap();
    let config_texture_1 = glium::texture::UnsignedTexture2d::empty_with_format(
        windowed_display.display(),
        glium::texture::UncompressedUintFormat::U8,
        glium::texture::MipmapsOption::NoMipmap,
        HALF_DEBUG,
        HALF_DEBUG,
    )
    .unwrap();
    let final_texture = glium::texture::Texture2d::empty_with_format(
        windowed_display.display(),
        glium::texture::UncompressedFloatFormat::U8U8U8U8,
        glium::texture::MipmapsOption::NoMipmap,
        HALF_DEBUG,
        HALF_DEBUG,
    )
    .unwrap();
    let gol_init_program = glium::program::ComputeShader::from_source(
        windowed_display.display(),
        &read_shader("src/gol_init.cs").unwrap(),
    )
    .unwrap();
    let gol_exec_program = glium::program::ComputeShader::from_source(
        windowed_display.display(),
        &read_shader("src/gol_exec.cs").unwrap(),
    )
    .unwrap();
    let gol_copy_program = glium::program::ComputeShader::from_source(
        windowed_display.display(),
        &read_shader("src/gol_copy.cs").unwrap(),
    )
    .unwrap();
    let image_unit = config_texture_0
        .image_unit(glium::uniforms::ImageUnitFormat::R8UI)
        .unwrap()
        .set_access(glium::uniforms::ImageUnitAccess::Write);
    gol_init_program.execute(
        uniform! {
            width: config_texture_0.width(),
            height: config_texture_0.height(),
            target_texture: image_unit,
        },
        config_texture_0.width(),
        config_texture_0.height(),
        1,
    );

    let mut sprites_batch = SpritesBatch::new(windowed_display.display());

    let mut camera = CameraState::new();
    camera.set_position(10.0 * Vec3::Z);

    let mut fullscreen = [false, false];
    let mut keyboard = KeyboardState::new();
    let mut screenshot_taker = AsyncScreenshotTaker::new(5);
    let mut i = 0;
    let mut j = 0;

    const SUBR_DUR: usize = 100;
    let mut tess_level = 64;

    let mut per_instance = CUBE_INSTANCES.clone();

    let mut cursor_position: Option<(i32, i32)> = None;

    let fxaa = fxaa::FxaaSystem::new(windowed_display.display());
    let mut fxaa_enabled = false;

    let mut switch = false;
    let mut counter = 0;

    let start = std::time::Instant::now();
    start_loop(event_loop, move |events| {
        let mut take_screenshot = false;

        if counter < 10 {
            counter += 1;
        } else {
            let previous_texture = if switch {
                &config_texture_1
            } else {
                &config_texture_0
            };
            let next_texture = if switch {
                &config_texture_0
            } else {
                &config_texture_1
            };

            let prev_unit = previous_texture
                .image_unit(glium::uniforms::ImageUnitFormat::R8UI)
                .unwrap()
                .set_access(glium::uniforms::ImageUnitAccess::Read);
            let next_unit = next_texture
                .image_unit(glium::uniforms::ImageUnitFormat::R8UI)
                .unwrap()
                .set_access(glium::uniforms::ImageUnitAccess::Write);
            gol_exec_program.execute(
                uniform! {
                    width: config_texture_0.width(),
                    height: config_texture_0.height(),
                    previous_generation: prev_unit,
                    next_generation: next_unit,
                },
                config_texture_0.width(),
                config_texture_0.height(),
                1,
            );
            let gol_unit = next_texture
                .image_unit(glium::uniforms::ImageUnitFormat::R8UI)
                .unwrap()
                .set_access(glium::uniforms::ImageUnitAccess::Read);
            let final_unit = final_texture
                .image_unit(glium::uniforms::ImageUnitFormat::RGBA8)
                .unwrap()
                .set_access(glium::uniforms::ImageUnitAccess::Write);
            gol_copy_program.execute(
                uniform! {
                    uGoLTexture: gol_unit,
                    destTexture: final_unit,
                },
                final_texture.width(),
                final_texture.height(),
                1,
            );

            final_texture.as_surface().blit_whole_color_to(
                &debug.image().as_surface(),
                &glium::BlitTarget {
                    left: 0,
                    bottom: HALF_DEBUG,
                    width: HALF_DEBUG as i32,
                    height: HALF_DEBUG as i32,
                },
                glium::uniforms::MagnifySamplerFilter::Nearest,
            );

            switch = !switch;
            counter = 0;
        }

        camera.update();

        let picked_object = {
            let data = cubes.picked().read().map(|d| d[0]).unwrap_or(8);
            if data < 8 {
                per_instance.binary_search_by(|x| x.id.cmp(&data)).ok()
            } else {
                None
            }
        };

        per_instance = CUBE_INSTANCES.clone();

        let subroutine = match picked_object {
            Some(0) => "ColourBlack",
            Some(1) => "ColourBlue",
            Some(2) => "ColourGreen",
            Some(3) => "ColourRed",
            Some(4) => "ColourCyan",
            Some(5) => "ColourMagenta",
            Some(6) => "ColourYellow",
            Some(7) => "ColourWhite",
            _ => {
                j = 0;
                "ColourNone"
            }
        };

        let window_size = windowed_display.display().get_framebuffer_dimensions();
        let aspect_ratio = window_size.0 as f32 / window_size.1 as f32;
        camera.set_aspect_ratio(aspect_ratio);

        let angle = (std::time::Instant::now() - start).as_secs_f32();
        let projection = camera.get_perspective();
        let view = camera.get_view();
        let project_view = projection * view;
        let model = Mat4::from_axis_angle(Vec3::ONE.normalize(), angle);
        let floor = Mat4::from_scale_rotation_translation(
            Vec3::new(10.0, 10.0, 0.01),
            Quat::IDENTITY,
            -3.0 * Vec3::Z,
        );
        let ring = Mat4::from_rotation_z(i as f32 / 8.0 / SUBR_DUR as f32 * TAU);
        const LIGHT_LOC: [f32; 3] = [-2.24593993, 5.0, 7.98890769];
        let depth_projection = Mat4::orthographic_rh(-4.0, 4.0, -4.0, 4.0, -10.0, 20.0);
        let depth_view = Mat4::look_at_rh(LIGHT_LOC.into(), Vec3::ZERO, Vec3::Y);
        let project_depth = depth_projection * depth_view;

        sprites_batch.process_sprites();
        let ib_slice = sprites_batch
            .index_buffer()
            .slice(0..SPRITES_COUNT * 6)
            .unwrap();

        let per_instance_buffer =
            glium::vertex::VertexBuffer::new(windowed_display.display(), &per_instance).unwrap();

        let mut params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLessOrEqual,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::BackfaceCullingMode::CullCounterClockwise,
            ..Default::default()
        };

        let mut depth_target = glium::framebuffer::SimpleFrameBuffer::with_depth_buffer(
            windowed_display.display(),
            &shadow_texture,
            &depth_texture,
        )
        .unwrap();
        depth_target.clear_color(1.0, 1.0, 1.0, 1.0);
        depth_target.clear_depth(1.0);

        depth_target
            .draw(
                cube.vertices(),
                cube.indices(),
                &shadow_map_shader,
                &uniform! {
                    depth_mvp: (project_depth * floor).to_cols_array_2d(),
                },
                &params,
            )
            .unwrap();
        depth_target
            .draw(
                cube.vertices(),
                cube.indices(),
                &shadow_map_shader,
                &uniform! {
                    depth_mvp : (project_depth * model).to_cols_array_2d(),
                },
                &params,
            )
            .unwrap();

        for instance in &CUBE_INSTANCES {
            depth_target.draw(
                cube.vertices(),
                cube.indices(),
                &shadow_map_shader,
                &uniform! {
                    depth_mvp : (project_depth * ring * Mat4::from_translation(instance.world_position.into())).to_cols_array_2d(),
                },
                &params,
            ).unwrap();
        }

        params.backface_culling = glium::BackfaceCullingMode::CullClockwise;

        let mut target = windowed_display.display().draw();
        fxaa::draw(&fxaa, &mut target, fxaa_enabled, |target| {
            target.clear_color_and_depth((1.0, 0.0, 1.0, 1.0), 1.0);

            target.draw(
                sprites_batch.vertex_buffer(),
                &ib_slice,
                &sprites_shader,
                &uniform! {
                    offset: (projection * view * Mat4::from_scale(Vec3::splat(4.0))).to_cols_array_2d(),
                    tex: sprites_batch.texture(),
                },
                &params,
            ).unwrap();

            target
                .draw(
                    cube.vertices(),
                    cube.indices(),
                    &subroutine_shader,
                    &uniform! {
                        matrix: (project_view * model).to_cols_array_2d(),
                        colour: (subroutine, glium::program::ShaderStage::Fragment),
                        percentage: j as f32 / SUBR_DUR as f32,
                    },
                    &params,
                )
                .unwrap();

            const BIAS_MATRIX: [[f32; 4]; 4] = [
                [0.5, 0.0, 0.0, 0.0],
                [0.0, 0.5, 0.0, 0.0],
                [0.0, 0.0, 0.5, 0.0],
                [0.5, 0.5, 0.5, 1.0],
            ];

            target.draw(
                cube.vertices(),
                cube.indices(),
                &shadows_shader,
                &uniform! {
                    light_loc: LIGHT_LOC,
                    perspective_matrix: projection.to_cols_array_2d(),
                    view_matrix: view.to_cols_array_2d(),
                    model_matrix: floor.to_cols_array_2d(),
                    model_color: [0.73f32, 0.31, 0.17, 1.0],
                    mvp: (projection * view * floor).to_cols_array_2d(),
                    depth_bias_mvp: (Mat4::from_cols_array_2d(&BIAS_MATRIX) * project_depth * floor).to_cols_array_2d(),
                    shadow_map: depth_texture.sampled() //glium::uniforms::Sampler::new(&depth_texture)
                        .wrap_function(glium::uniforms::SamplerWrapFunction::Clamp)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
                        .depth_texture_comparison(Some(glium::uniforms::DepthTextureComparison::LessOrEqual)),
                },
                &params,
            ).unwrap();

            target
                .draw(
                    (cube.vertices(), per_instance_buffer.per_instance().unwrap()),
                    cube.tessellices(),
                    &tessellancing_shader,
                    &uniform! {
                        inner_level: tess_level as f32,
                        outer_level: tess_level as f32,
                        projection_matrix: projection.to_cols_array_2d(),
                        view_matrix: (view * ring).to_cols_array_2d(),
                        height_texture: &opengl_texture,
                        elevation: 0.1f32,
                        colour_texture: &opengl_texture,
                    },
                    &params,
                )
                .unwrap();
        });

        if let (Some(cursor), Some(ref picking_texture)) =
            (cursor_position, &*fxaa.picking_texture())
        {
            let read_target = glium::Rect {
                left: (cursor.0 - 1) as u32,
                bottom: picking_texture
                    .get_height()
                    .unwrap()
                    .saturating_sub(std::cmp::max(cursor.1 - 1, 0) as u32),
                width: 1,
                height: 1,
            };

            if read_target.left < picking_texture.get_width()
                && read_target.bottom < picking_texture.get_height().unwrap()
            {
                picking_texture
                    .main_level()
                    .first_layer()
                    .into_image(None)
                    .unwrap()
                    .raw_read_to_pixel_buffer(&read_target, cubes.picked());
            } else {
                cubes.picked().write(&[8]);
            }
        } else {
            cubes.picked().write(&[8]);
        }

        target.finish().unwrap();

        shadow_texture.as_surface().blit_whole_color_to(
            &debug.image().as_surface(),
            &shadow_rect,
            glium::uniforms::MagnifySamplerFilter::Linear,
        );
        let target = debug.display().draw();
        debug
            .image()
            .as_surface()
            .fill(&target, glium::uniforms::MagnifySamplerFilter::Linear);
        target.finish().unwrap();

        keyboard.enter_pressed = [false, false];
        keyboard.space_pressed = false;
        keyboard.d_pressed = false;
        keyboard.s_pressed = false;
        keyboard.t_pressed = false;
        let action = process_input(
            &windowed_display,
            &mut camera,
            &mut keyboard,
            &mut cursor_position,
            events,
        );

        if keyboard.alt_pressed && keyboard.enter_pressed[0] {
            if fullscreen[0] {
                windowed_display.window().set_fullscreen(None);
                fullscreen[0] = false;
            } else {
                let monitor_handle = windowed_display
                    .window()
                    .available_monitors()
                    .next()
                    .unwrap();
                let fs = Fullscreen::Borderless(Some(monitor_handle));
                windowed_display.window().set_fullscreen(Some(fs));
                fullscreen[0] = true;
            }
        } else if keyboard.alt_pressed && keyboard.enter_pressed[1] {
            if fullscreen[1] {
                debug.window().set_fullscreen(None);
                fullscreen[1] = false;
            } else {
                let monitor_handle = debug
                    .window()
                    .available_monitors()
                    .next()
                    .unwrap();
                let fs = Fullscreen::Borderless(Some(monitor_handle));
                debug
                    .window()
                    .set_fullscreen(Some(fs));
                fullscreen[1] = true;
            }
        }

        if keyboard.space_pressed {
            fxaa_enabled = !fxaa_enabled;
        }

        if keyboard.alt_pressed && keyboard.t_pressed {
            if keyboard.shift_pressed {
                if tess_level > 1 {
                    tess_level -= 1;
                }
            } else {
                if tess_level < 64 {
                    tess_level += 1;
                }
            }
        }

        if keyboard.alt_pressed && keyboard.d_pressed {
            debug.enabled = !debug.enabled;
            let copy = debug.enabled;
            debug.window().set_visible(copy);
            windowed_display
                .window()
                .request_user_attention(Some(winit::window::UserAttentionType::Informational));
        }

        if keyboard.alt_pressed && keyboard.s_pressed {
            take_screenshot = true;
        }

        screenshot_taker.next_frame();

        if take_screenshot {
            screenshot_taker.take_screenshot(windowed_display.display());
        }

        screenshot_taker.process_screenshots();

        i += 1;
        if i == 8 * SUBR_DUR {
            i = 0;
        }
        j = std::cmp::min(j + 1, SUBR_DUR);

        action
    });
}

fn read_icon(path: &str) -> std::io::Result<winit::window::Icon> {
    let image = read_targa(path).unwrap();

    let icon = winit::window::Icon::from_rgba(image.bytes, image.width, image.height).unwrap();
    Ok(icon)
}

fn read_shader(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;

    let mut string = "".to_string();
    let _read = file.read_to_string(&mut string)?;

    Ok(string)
}

fn display_info(display: &glium::Display<WindowSurface>) {
    let version = *display.get_opengl_version();
    let api = match version {
        Version(Api::Gl, _, _) => "OpenGL",
        Version(Api::GlEs, _, _) => "OpenGL ES",
    };
    println!(
        "{} context version: {}",
        api,
        display.get_opengl_version_string()
    );
    print!("{} context flags:", api);
    if display.is_forward_compatible() {
        print!(" forward-compatible");
    }
    if display.is_debug() {
        print!(" debug");
    }
    if display.is_robust() {
        print!(" robustness");
    }
    print!("\n");
    if version >= Version(Api::Gl, 3, 2) {
        println!(
            "{} profile mask: {}",
            api,
            match display.get_opengl_profile() {
                Some(Profile::Core) => "core",
                Some(Profile::Compatibility) => "compatibility",
                None => "unknown",
            }
        );
    }
    println!(
        "{} robustness strategy: {}",
        api,
        if display.is_context_loss_possible() {
            "lose"
        } else {
            "none"
        }
    );
    println!(
        "{} context renderer: {}",
        api,
        display.get_opengl_renderer_string()
    );
    println!(
        "{} context vendor: {}",
        api,
        display.get_opengl_vendor_string()
    );
}
