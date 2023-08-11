#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glutin::dpi::PhysicalPosition;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::{Icon, WindowBuilder};

#[allow(dead_code)]
mod simple_targa;
use simple_targa::read_targa;
mod text;
use text::Console;
mod font;
use font::Font;

fn main() {
    let icon = read_targa("res/icon.tga").unwrap();
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_window_icon(Icon::from_rgba(icon.bytes, icon.width, icon.height).ok())
        .with_resizable(false)
        .with_title("text")
        .with_position(PhysicalPosition::<i32>::from((50, 50)));
    let cb = glutin::ContextBuilder::new()
        .with_multisampling(4)
        .with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let mut console = Console::new(&display);
    let font = Font::new(&display, read_targa("res/font.tga").unwrap());

    let params = glium::DrawParameters {
        backface_culling: glium::BackfaceCullingMode::CullClockwise,
        ..Default::default()
    };

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::RedrawEventsCleared => display.gl_window().window().request_redraw(),
            Event::RedrawRequested(_) => {
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
            }
            Event::WindowEvent { event, .. } => {
                match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::KeyboardInput { input, .. } => console.write(input),
                    WindowEvent::ModifiersChanged(mods) => console.set_modifiers(mods),
                    _ => (),
                }
            }
            _ => (),
        }
    });
}
