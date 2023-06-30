#[macro_use]
extern crate glium;

use glium::{glutin, Surface};
use glium::texture::{RawImage2d, CompressedSrgbTexture2d};
use glutin::dpi::PhysicalPosition;
use glutin::event::{Event, WindowEvent};
use glutin::event_loop::{EventLoop, ControlFlow};
use glutin::window::{Icon, WindowBuilder};

pub mod text;
use text::text::{CharString, Console};

fn main() {
    let icon = simple_targa::read_targa("res/icon.tga").unwrap();
    let event_loop = EventLoop::new();
    let wb = WindowBuilder::new()
        .with_window_icon(Icon::from_rgba(icon.bytes, icon.width, icon.height).ok())
        .with_resizable(false)
        .with_title("text")
        .with_position(PhysicalPosition::<i32>::from((50, 50)));
    let cb = glutin::ContextBuilder::new()
        .with_multisampling(4)
        .with_vsync(true);
    let display = glium::Display::new(wb, cb, &event_loop).expect("unable to create a new display");

    let mut console = Console::new();
    let mut char_string = CharString::new(&display);

    let targa = simple_targa::read_targa("res/font.tga").unwrap();
    let font_img = RawImage2d::from_raw_rgba(targa.bytes, (targa.width, targa.height));
    let font_tex = CompressedSrgbTexture2d::new(&display, font_img).unwrap();

    let font_shader = glium::Program::from_source(&display, &FONT_VS, &FONT_FS, None).unwrap();

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

                char_string.update(console.read());

                frame.draw(
                    char_string.vertices(),
                    char_string.indices(),
                    &font_shader,
                    &uniform! {
                        translation: [-1.0f32, -1.0],
                        font: glium::uniforms::Sampler::new(&font_tex),
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

const FONT_VS: &'static str = r#"
    #version 150

    in vec2 pos;
    in vec2 tex;

    uniform vec2 translation;

    out vec2 coordinates;

    void main() {
        coordinates = tex;
        gl_Position = vec4(pos + translation, 0.5, 1.0);
    }
"#;

const FONT_FS: &'static str = r#"
    #version 150

    in vec2 coordinates;

    uniform sampler2D font;

    out vec4 colour;

    void main() {
        colour = texture(font, coordinates);

        if (colour.a == 0) discard;
    }
"#;

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
        // read read not read
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
