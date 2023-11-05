use crate::read_shader;
use glium::glutin::*;
use glium::*;
use display::{GetGlDisplay, GlDisplay};
use context::NotCurrentGlContext;
use raw_window_handle::HasRawWindowHandle;

pub const HALF_DEBUG: u32 = 512;

#[derive(Clone, Copy)]
pub struct QuadVertex {
    position: [f32; 2],
    texture: [f32; 2],
}

implement_vertex!(QuadVertex, position, texture);

pub struct DebugWindow {
    vertices: glium::vertex::VertexBuffer<QuadVertex>,
    indices: glium::index::NoIndices,
    shader: glium::Program,
    pub enabled: bool,
    display: glium::Display<surface::WindowSurface>,
    window: winit::window::Window,
    image: glium::texture::Texture2d,
}

impl DebugWindow {
    pub fn new(config: &config::Config, cab: context::ContextAttributesBuilder, event_loop: &winit::event_loop::EventLoop<()>) -> Self {
        let wb = winit::window::WindowBuilder::new()
            .with_position(winit::dpi::PhysicalPosition::<i32>::from((900, 50)))
            .with_decorations(false)
            .with_visible(false);

        let window = wb.build(&event_loop).unwrap();

        let sab: surface::SurfaceAttributesBuilder<surface::WindowSurface> = surface::SurfaceAttributesBuilder::new();
        // SAFETY: main window is kept alive indefinitely and window creation errors panic
        let surface = unsafe {
            config.display().create_window_surface(
                    &config,
                    &sab.build(
                        window.raw_window_handle(),
                        std::num::NonZeroU32::new(800).unwrap(),
                        std::num::NonZeroU32::new(600).unwrap() ) )
                .unwrap()
        };
        let ca = cab.build(Some(window.raw_window_handle()));
        // SAFETY: main window is kept alive indefinitely and window creation errors panic
        let ncc = unsafe {
            config.display().create_context(&config, &ca).unwrap()
        };
        let context = ncc.treat_as_possibly_current();

        let display = glium::Display::new(context, surface).expect("unable to create glium display");

        let vertices = vertex::VertexBuffer::new(
            &display,
            &[
                QuadVertex {
                    position: [-1.0, -1.0],
                    texture: [0.0, 0.0],
                },
                QuadVertex {
                    position: [1.0, -1.0],
                    texture: [1.0, 0.0],
                },
                QuadVertex {
                    position: [-1.0, 1.0],
                    texture: [0.0, 1.0],
                },
                QuadVertex {
                    position: [1.0, 1.0],
                    texture: [1.0, 1.0],
                },
            ],
        )
        .unwrap();

        let indices = index::NoIndices(index::PrimitiveType::TriangleStrip);

        let shader = Program::from_source(
            &display,
            &read_shader("src/debug.vs").unwrap(),
            &read_shader("src/debug.fs").unwrap(),
            None,
        )
        .unwrap();

        let image = Texture2d::empty_with_format(
            &display,
            texture::UncompressedFloatFormat::U8U8U8U8,
            texture::MipmapsOption::NoMipmap,
            2 * HALF_DEBUG,
            2 * HALF_DEBUG,
        )
        .unwrap();
        image
            .as_surface()
            .clear(None, Some((0.0, 0.0, 0.0, 1.0)), true, None, None);

        DebugWindow {
            vertices,
            indices,
            shader,
            enabled: false,
            display,
            window,
            image,
        }
    }

    pub fn vertices(&self) -> &vertex::VertexBuffer<QuadVertex> {
        &self.vertices
    }

    pub fn indices(&self) -> &index::NoIndices {
        &self.indices
    }

    pub fn program(&self) -> &Program {
        &self.shader
    }

    pub fn display(&mut self) -> &mut Display<surface::WindowSurface> {
        &mut self.display
    }

    pub fn window(&mut self) -> &mut winit::window::Window {
        &mut self.window
    }

    pub fn image(&mut self) -> &mut Texture2d {
        &mut self.image
    }

    pub fn draw(&self) -> Frame {
        self.display.draw()
    }

    pub fn set_image(&mut self, image: texture::RawImage2d<u8>) {
        self.image = Texture2d::new(&self.display, image).unwrap();
    }
}
