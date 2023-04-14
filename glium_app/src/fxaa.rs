pub mod fxaa {
    use crate::read_shader;

    use glium::{self, Surface};
    use glium::backend::Facade;
    use glium::backend::Context;
    use glium::framebuffer::{MultiOutputFrameBuffer, ToColorAttachment};

    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct FxaaSystem {
        context: Rc<Context>,
        vertex_buffer: glium::VertexBuffer<SpriteVertex>,
        index_buffer: glium::IndexBuffer<u16>,
        program: glium::Program,
        target_color: RefCell<Option<glium::Texture2d>>,
        target_pickr: RefCell<Option<glium::texture::UnsignedTexture2d>>,
        target_depth: RefCell<Option<glium::framebuffer::DepthRenderBuffer>>,
    }

    #[derive(Copy, Clone)]
    struct SpriteVertex {
        position: [f32; 2],
        i_tex_coords: [f32; 2],
    }

    implement_vertex!(SpriteVertex, position, i_tex_coords);

    impl FxaaSystem {
        pub fn new<F: ?Sized>(facade: &F) -> FxaaSystem where F: Facade + Clone {
            FxaaSystem {
                context: facade.get_context().clone(),

                vertex_buffer: glium::VertexBuffer::new(facade,
                    &[
                        SpriteVertex { position: [-1.0, -1.0], i_tex_coords: [0.0, 0.0] },
                        SpriteVertex { position: [ 1.0, -1.0], i_tex_coords: [1.0, 0.0] },
                        SpriteVertex { position: [-1.0,  1.0], i_tex_coords: [0.0, 1.0] },
                        SpriteVertex { position: [ 1.0,  1.0], i_tex_coords: [1.0, 1.0] },
                    ]
                ).unwrap(),

                index_buffer: glium::index::IndexBuffer::new(facade,
                    glium::index::PrimitiveType::TriangleStrip, &[0u16, 1, 2, 3]).unwrap(),

                program: program!(facade,
                    100 => {
                        vertex: &read_shader("src/fxaa.vs").unwrap(),
                        fragment: &read_shader("src/fxaa.fs").unwrap(),
                    }
                ).unwrap(),

                target_color: RefCell::new(None),
                target_pickr: RefCell::new(None),
                target_depth: RefCell::new(None),
            }
        }

        pub fn picking_texture(&self) -> std::cell::Ref<'_, Option<glium::texture::UnsignedTexture2d>> {
            self.target_pickr.borrow()
        }
    }

    pub fn draw<T, F, R>(system: &FxaaSystem, target: &mut T, enabled: bool, mut draw: F) -> R
    where T: Surface, F: FnMut(&mut MultiOutputFrameBuffer<'_>) -> R
    {
        let target_dimensions = target.get_dimensions();

        let mut target_color = system.target_color.borrow_mut();
        let mut target_pickr = system.target_pickr.borrow_mut();
        let mut target_depth = system.target_depth.borrow_mut();

        if let &Some(ref tex) = &*target_color {
            if tex.get_width() != target_dimensions.0
            || tex.get_height().unwrap() != target_dimensions.1 {
                *target_color = None;
            }
        }

        if let &Some(ref tex) = &*target_pickr {
            if tex.get_width() != target_dimensions.0
            || tex.get_height().unwrap() != target_dimensions.1 {
                *target_pickr = None;
            }
        }

        if let &Some(ref tex) = &*target_depth {
            if tex.get_dimensions() != target_dimensions {
                *target_depth = None;
            }
        }

        if target_color.is_none() {
            let texture = glium::Texture2d::empty(
                &system.context,
                target_dimensions.0,
                target_dimensions.1,
            ).unwrap();
            *target_color = Some(texture);
        }
        let target_color = target_color.as_ref().unwrap();

        if target_pickr.is_none() {
            let texture = glium::texture::UnsignedTexture2d::empty_with_format(
                &system.context,
                glium::texture::UncompressedUintFormat::U32,
                glium::texture::MipmapsOption::NoMipmap,
                target_dimensions.0,
                target_dimensions.1,
            ).unwrap();
            texture.main_level().first_layer().into_image(None).unwrap().raw_clear_buffer([8u32; 4]);
            *target_pickr = Some(texture);
        }
        let target_pickr = target_pickr.as_ref().unwrap();

        if target_depth.is_none() {
            let texture = glium::framebuffer::DepthRenderBuffer::new(
                &system.context,
                glium::texture::DepthFormat::I24,
                u32::max(1, target_dimensions.0),
                u32::max(1, target_dimensions.1),
            ).unwrap();
            *target_depth = Some(texture);
        }
        let target_depth = target_depth.as_ref().unwrap();

        let output = draw(&mut MultiOutputFrameBuffer::with_depth_buffer(
            &system.context,
            [("f_colour", target_color.to_color_attachment()), ("f_id", target_pickr.to_color_attachment())],
            target_depth
        ).unwrap());

        let uniforms = uniform! {
            tex: &*target_color,
            enabled: enabled as i32,
            resolution: (target_dimensions.0 as f32, target_dimensions.1 as f32)
        };

        target.draw(&system.vertex_buffer, &system.index_buffer, &system.program, &uniforms, &Default::default()).unwrap();

        output
    }
}
