use winit::error::EventLoopError;
use winit::event::{Event, StartCause};
use winit::event_loop::{ControlFlow, EventLoop};

pub enum Action {
    Stop,
    Continue,
}

pub fn start_loop<F>(event_loop: EventLoop<()>, mut callback: F) -> Result<(), EventLoopError>
where
    F: 'static + FnMut(&Vec<Event<()>>) -> Action,
{
    let mut events_buffer = Vec::new();
    let next_frame_time =
        std::time::Instant::now() + std::time::Duration::from_nanos(16_666_667);
    event_loop.run(move |event, elwt| {
        let run_callback = match event {
            Event::NewEvents(cause) => matches!(cause, StartCause::ResumeTimeReached { .. } | StartCause::Init),
            event => {
                events_buffer.push(event);
                false
            }
        };

        let action = if run_callback {
            let action = callback(&events_buffer);

            events_buffer.clear();
            action
        } else {
            Action::Continue
        };

        match action {
            Action::Continue => {
                elwt.set_control_flow(ControlFlow::WaitUntil(next_frame_time));
            }
            Action::Stop => elwt.exit(),
        }
    })
}

use glium::glutin::surface::WindowSurface;

pub struct WindowedDisplay {
    window: winit::window::Window,
    display: glium::Display<WindowSurface>,
}

impl WindowedDisplay {
    pub fn new(
            window: winit::window::Window,
            display: glium::Display<WindowSurface>,
            ) -> Self {
        WindowedDisplay { window, display }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn display(&self) -> &glium::Display<WindowSurface> {
        &self.display
    }
}

pub mod input {
    use crate::engine::Action;
    use crate::CameraState;

    use winit::event::{ElementState, Event, KeyEvent, WindowEvent};
    use winit::keyboard::{KeyCode, PhysicalKey::Code};

    #[derive(Default)]
    pub struct KeyboardState {
        pub alt_pressed: bool,
        pub shift_pressed: bool,
        pub space_pressed: bool,
        pub d_pressed: bool,
        pub s_pressed: bool,
        pub t_pressed: bool,
        pub enter_pressed: [bool; 2],
    }

    impl KeyboardState {
        pub fn new() -> Self {
            KeyboardState {
                alt_pressed: false,
                shift_pressed: false,
                space_pressed: false,
                d_pressed: false,
                s_pressed: false,
                t_pressed: false,
                enter_pressed: [false, false],
            }
        }
    }

    pub fn process_input(
        display: &super::WindowedDisplay,
        camera: &mut CameraState,
        keyboard: &mut KeyboardState,
        cursor: &mut Option<(i32, i32)>,
        events: &Vec<Event<()>>,
    ) -> Action {
        let mut action = Action::Continue;

        for event in events {
            if let Event::WindowEvent { event, window_id } = event {
                let main_display = *window_id == display.window().id();

                match event {
                    WindowEvent::CloseRequested => {
                        if main_display {
                            action = Action::Stop
                        }
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        *cursor = Some(position.cast::<i32>().into());
                    }
                    ev @ WindowEvent::KeyboardInput { event, .. } => {
                        match event {
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::Enter),
                                ..
                            } => keyboard.enter_pressed[if main_display { 0 } else { 1 }] = true,
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::AltLeft),
                                ..
                            } => keyboard.alt_pressed = true,
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::AltRight),
                                ..
                            } => keyboard.alt_pressed = true,
                            KeyEvent {
                                state: ElementState::Released,
                                physical_key: Code(KeyCode::AltLeft),
                                ..
                            } => keyboard.alt_pressed = false,
                            KeyEvent {
                                state: ElementState::Released,
                                physical_key: Code(KeyCode::AltRight),
                                ..
                            } => keyboard.alt_pressed = false,
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::ShiftLeft),
                                ..
                            } => keyboard.shift_pressed = true,
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::ShiftRight),
                                ..
                            } => keyboard.shift_pressed = true,
                            KeyEvent {
                                state: ElementState::Released,
                                physical_key: Code(KeyCode::ShiftLeft),
                                ..
                            } => keyboard.shift_pressed = false,
                            KeyEvent {
                                state: ElementState::Released,
                                physical_key: Code(KeyCode::ShiftRight),
                                ..
                            } => keyboard.shift_pressed = false,
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::Space),
                                ..
                            } => keyboard.space_pressed = true,
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::KeyD),
                                ..
                            } => keyboard.d_pressed = true,
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::KeyS),
                                ..
                            } => keyboard.s_pressed = true,
                            KeyEvent {
                                state: ElementState::Pressed,
                                physical_key: Code(KeyCode::KeyT),
                                ..
                            } => keyboard.t_pressed = true,
                            _ => (),
                        }

                        if main_display {
                            camera.process_input(ev);
                        }
                    }
                    _ => (),
                }
            }
        }

        action
    }
}

pub mod screenshot {
    use crate::{write_targa, TargaImage};

    use glium::texture::{Texture2d, Texture2dDataSink};
    use glium::Surface;
    use std::borrow::Cow;
    use std::collections::VecDeque;
    use std::vec::Vec;

    pub struct RGBAImageData {
        pub data: Vec<(u8, u8, u8, u8)>,
        pub width: u32,
        pub height: u32,
    }

    impl Texture2dDataSink<(u8, u8, u8, u8)> for RGBAImageData {
        fn from_raw(data: Cow<'_, [(u8, u8, u8, u8)]>, width: u32, height: u32) -> Self {
            RGBAImageData {
                data: data.into_owned(),
                width,
                height,
            }
        }
    }

    struct AsyncScreenshotTask {
        pub target_frame: u64,
        pub image: RGBAImageData,
    }

    impl AsyncScreenshotTask {
        fn new(facade: &dyn glium::backend::Facade, target_frame: u64) -> Self {
            let dimensions = facade.get_context().get_framebuffer_dimensions();
            let rect = glium::Rect {
                left: 0,
                bottom: 0,
                width: dimensions.0,
                height: dimensions.1,
            };
            let blit_target = glium::BlitTarget {
                left: 0,
                bottom: 0,
                width: dimensions.0 as i32,
                height: dimensions.1 as i32,
            };

            let texture = Texture2d::empty(facade, dimensions.0, dimensions.1).unwrap();
            let framebuffer = glium::framebuffer::SimpleFrameBuffer::new(facade, &texture).unwrap();
            framebuffer.blit_from_frame(
                &rect,
                &blit_target,
                glium::uniforms::MagnifySamplerFilter::Nearest,
            );

            let image = texture.read_to_pixel_buffer().read_as_texture_2d().unwrap();

            AsyncScreenshotTask {
                target_frame,
                image,
            }
        }
    }

    pub struct ScreenshotIterator<'a>(&'a mut AsyncScreenshotTaker);

    impl<'a> Iterator for ScreenshotIterator<'a> {
        type Item = RGBAImageData;

        fn next(&mut self) -> Option<RGBAImageData> {
            if self
                .0
                .screenshot_tasks
                .front()
                .map(|task| task.target_frame)
                == Some(self.0.frame)
            {
                let task = self.0.screenshot_tasks.pop_front().unwrap();
                Some(task.image)
            } else {
                None
            }
        }
    }

    pub struct AsyncScreenshotTaker {
        screenshot_delay: u64,
        frame: u64,
        screenshot_tasks: VecDeque<AsyncScreenshotTask>,
    }

    impl AsyncScreenshotTaker {
        pub fn new(screenshot_delay: u64) -> Self {
            AsyncScreenshotTaker {
                screenshot_delay,
                frame: 0,
                screenshot_tasks: VecDeque::new(),
            }
        }

        pub fn next_frame(&mut self) {
            self.frame += 1;
        }

        pub fn pickup_screenshots(&mut self) -> ScreenshotIterator<'_> {
            ScreenshotIterator(self)
        }

        pub fn take_screenshot(&mut self, facade: &dyn glium::backend::Facade) {
            self.screenshot_tasks.push_back(AsyncScreenshotTask::new(
                facade,
                self.frame + self.screenshot_delay,
            ));
        }

        pub fn process_screenshots(&mut self) {
            for image_data in self.pickup_screenshots() {
                std::thread::spawn(move || {
                    let pixels = {
                        let mut v = Vec::with_capacity(image_data.data.len() * 4);
                        for (a, b, c, d) in image_data.data {
                            v.push(a);
                            v.push(b);
                            v.push(c);
                            v.push(d);
                        }

                        v
                    };

                    write_targa(
                        "screenshot.tga",
                        TargaImage::new(pixels, image_data.width as u16, image_data.height as u16),
                    )
                    .unwrap();
                });
            }
        }
    }
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
            #[allow(clippy::identity_op)]
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

            image.bytes.swap(index, index + 2);
        }
        file.write_all(&image.bytes)?;

        Ok(())
    }
}
