use glium::texture::{RawImage2d, CompressedSrgbTexture2d};

pub struct Font {
    pub texture: CompressedSrgbTexture2d,
    pub shader: glium::Program,
}

impl Font {
    pub fn new(display: &glium::Display) -> Self {
        let targa = crate::read_targa("res/font.tga").unwrap();
        let image = RawImage2d::from_raw_rgba(targa.bytes, (targa.width, targa.height));
        let texture = CompressedSrgbTexture2d::new(display, image).unwrap();

        let shader = glium::Program::from_source(display, &FONT_VS, &FONT_FS, None).unwrap();

        Font { texture, shader }
    }
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
