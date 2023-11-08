#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glutin::context::NotCurrentGlContext;
use glutin::display::{GetGlDisplay, GlDisplay};
use glutin::surface::{SurfaceAttributesBuilder, WindowSurface};
use winit::dpi::PhysicalPosition;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::{Icon, WindowBuilder};
use raw_window_handle::HasRawWindowHandle;

#[allow(dead_code)]
mod simple_targa;
use simple_targa::read_targa;
mod text;
use text::Console;
mod font;
use font::Font;

fn main() {
    let icon = read_targa("res/icon.tga").unwrap();
    let event_loop = EventLoop::new().unwrap();
    let wb = WindowBuilder::new()
        .with_window_icon(Icon::from_rgba(icon.bytes, icon.width, icon.height).ok())
        .with_resizable(false)
        .with_title("text")
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
                std::num::NonZeroU32::new(800u32).unwrap(),
                std::num::NonZeroU32::new(600u32).unwrap()
            )
        ).unwrap()
    };
    let current_context = not_current_context.treat_as_possibly_current();
    let display = glium::Display::new(current_context, window_surface).expect("unable to create a new display");

    let mut console = Console::new(&display);
    let font = Font::new(&display, read_targa("res/font.tga").unwrap());

    let params = glium::DrawParameters {
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    event_loop.run(move |event, elwt| {
        match event {
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => elwt.exit(),
                    WindowEvent::RedrawRequested => {
                        let mut frame = display.draw();
                        frame.clear_color(0.0, 0.0, 0.0, 1.0);

                        frame.draw(
                            console.echo_line().vertices(),
                            console.echo_line().indices(),
                            &font.shader,
                            &uniform! {
                                translation: [-1.0f32, -1.0],
                                font: &font.texture,
                            },
                            &params,
                        )
                        .unwrap();

                        frame.finish().unwrap();
                        window.request_redraw();
                    },
                    WindowEvent::KeyboardInput { event, .. } => console.write(event),
                    WindowEvent::ModifiersChanged(mods) => console.set_modifiers(mods.state()),
                    _ => (),
                }
            }
            _ => (),
        }
    })
    .unwrap();
}
