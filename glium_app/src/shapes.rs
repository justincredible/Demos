pub mod shapes {
    #[derive(Clone, Copy)]
    pub struct CubeVertex {
        position: [f32; 3],
        normal: [f32; 3],
        texture: [f32; 2],
    }

    implement_vertex!(CubeVertex, position, normal, texture);

    const CUBE_INDICES: [u8; 36] = [
        0u8, 1, 2, 2, 1, 3, // 1st
        4, 5, 6, 6, 5, 7, // 2nd
        8, 9, 10, 10, 9, 11, // 3rd
        12, 13, 14, 14, 13, 15, // 4th
        16, 17, 18, 18, 17, 19, // 5th
        20, 21, 22, 22, 21, 23, // 6th
    ];

    pub struct Cube {
        vertices: glium::vertex::VertexBuffer<CubeVertex>,
        indices: glium::IndexBuffer<u8>,
        tessellices: glium::IndexBuffer<u8>,
    }

    impl Cube {
        pub fn new(display: &glium::Display) -> Self {
            let vertices = glium::vertex::VertexBuffer::new(
                display,
                &[
                    CubeVertex {
                        position: [0.5, -0.5, -0.5],
                        normal: [0.0, 0.0, -1.0],
                        texture: [0.0, 0.0],
                    },
                    CubeVertex {
                        position: [-0.5, -0.5, -0.5],
                        normal: [0.0, 0.0, -1.0],
                        texture: [1.0, 0.0],
                    },
                    CubeVertex {
                        position: [0.5, 0.5, -0.5],
                        normal: [0.0, 0.0, -1.0],
                        texture: [0.0, 1.0],
                    },
                    CubeVertex {
                        position: [-0.5, 0.5, -0.5],
                        normal: [0.0, 0.0, -1.0],
                        texture: [1.0, 1.0],
                    },
                    CubeVertex {
                        position: [-0.5, -0.5, -0.5],
                        normal: [0.0, -1.0, 0.0],
                        texture: [0.0, 0.0],
                    },
                    CubeVertex {
                        position: [0.5, -0.5, -0.5],
                        normal: [0.0, -1.0, 0.0],
                        texture: [1.0, 0.0],
                    },
                    CubeVertex {
                        position: [-0.5, -0.5, 0.5],
                        normal: [0.0, -1.0, 0.0],
                        texture: [0.0, 1.0],
                    },
                    CubeVertex {
                        position: [0.5, -0.5, 0.5],
                        normal: [0.0, -1.0, 0.0],
                        texture: [1.0, 1.0],
                    },
                    CubeVertex {
                        position: [-0.5, -0.5, -0.5],
                        normal: [-1.0, 0.0, 0.0],
                        texture: [0.0, 0.0],
                    },
                    CubeVertex {
                        position: [-0.5, -0.5, 0.5],
                        normal: [-1.0, 0.0, 0.0],
                        texture: [1.0, 0.0],
                    },
                    CubeVertex {
                        position: [-0.5, 0.5, -0.5],
                        normal: [-1.0, 0.0, 0.0],
                        texture: [0.0, 1.0],
                    },
                    CubeVertex {
                        position: [-0.5, 0.5, 0.5],
                        normal: [-1.0, 0.0, 0.0],
                        texture: [1.0, 1.0],
                    },
                    CubeVertex {
                        position: [0.5, -0.5, 0.5],
                        normal: [1.0, 0.0, 0.0],
                        texture: [0.0, 0.0],
                    },
                    CubeVertex {
                        position: [0.5, -0.5, -0.5],
                        normal: [1.0, 0.0, 0.0],
                        texture: [1.0, 0.0],
                    },
                    CubeVertex {
                        position: [0.5, 0.5, 0.5],
                        normal: [1.0, 0.0, 0.0],
                        texture: [0.0, 1.0],
                    },
                    CubeVertex {
                        position: [0.5, 0.5, -0.5],
                        normal: [1.0, 0.0, 0.0],
                        texture: [1.0, 1.0],
                    },
                    CubeVertex {
                        position: [-0.5, 0.5, 0.5],
                        normal: [0.0, 1.0, 0.0],
                        texture: [0.0, 0.0],
                    },
                    CubeVertex {
                        position: [0.5, 0.5, 0.5],
                        normal: [0.0, 1.0, 0.0],
                        texture: [1.0, 0.0],
                    },
                    CubeVertex {
                        position: [-0.5, 0.5, -0.5],
                        normal: [0.0, 1.0, 0.0],
                        texture: [0.0, 1.0],
                    },
                    CubeVertex {
                        position: [0.5, 0.5, -0.5],
                        normal: [0.0, 1.0, 0.0],
                        texture: [1.0, 1.0],
                    },
                    CubeVertex {
                        position: [-0.5, -0.5, 0.5],
                        normal: [0.0, 0.0, 1.0],
                        texture: [0.0, 0.0],
                    },
                    CubeVertex {
                        position: [0.5, -0.5, 0.5],
                        normal: [0.0, 0.0, 1.0],
                        texture: [1.0, 0.0],
                    },
                    CubeVertex {
                        position: [-0.5, 0.5, 0.5],
                        normal: [0.0, 0.0, 1.0],
                        texture: [0.0, 1.0],
                    },
                    CubeVertex {
                        position: [0.5, 0.5, 0.5],
                        normal: [0.0, 0.0, 1.0],
                        texture: [1.0, 1.0],
                    },
                ],
            )
            .unwrap();

            let indices = glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::TrianglesList,
                &CUBE_INDICES,
            )
            .unwrap();

            let tessellices = glium::IndexBuffer::new(
                display,
                glium::index::PrimitiveType::Patches {
                    vertices_per_patch: 3,
                },
                &CUBE_INDICES,
            )
            .unwrap();

            Cube {
                vertices,
                indices,
                tessellices,
            }
        }

        pub fn vertices(&self) -> &glium::vertex::VertexBuffer<CubeVertex> {
            &self.vertices
        }

        pub fn indices(&self) -> &glium::IndexBuffer<u8> {
            &self.indices
        }

        pub fn tessellices(&self) -> &glium::IndexBuffer<u8> {
            &self.tessellices
        }
    }

    #[derive(Clone, Copy)]
    pub struct CubeInstanceAttr {
        pub id: u32,
        pub world_position: [f32; 3],
        pub colour: [f32; 3],
    }

    implement_vertex!(CubeInstanceAttr, id, world_position, colour);

    pub const CUBE_INSTANCES: [CubeInstanceAttr; 8] = [
        CubeInstanceAttr {
            id: 0,
            world_position: [0.0, 3.0, 0.0],
            colour: [0.05, 0.05, 0.05],
        },
        CubeInstanceAttr {
            id: 1,
            world_position: [2.1, 2.1, 0.0],
            colour: [0.0, 0.0, 1.0],
        },
        CubeInstanceAttr {
            id: 2,
            world_position: [3.0, 0.0, 0.0],
            colour: [0.0, 1.0, 0.0],
        },
        CubeInstanceAttr {
            id: 3,
            world_position: [2.1, -2.1, 0.0],
            colour: [1.0, 0.0, 0.0],
        },
        CubeInstanceAttr {
            id: 4,
            world_position: [0.0, -3.0, 0.0],
            colour: [0.0, 1.0, 1.0],
        },
        CubeInstanceAttr {
            id: 5,
            world_position: [-2.1, -2.1, 0.0],
            colour: [1.0, 0.0, 1.0],
        },
        CubeInstanceAttr {
            id: 6,
            world_position: [-3.0, 0.0, 0.0],
            colour: [1.0, 1.0, 0.0],
        },
        CubeInstanceAttr {
            id: 7,
            world_position: [-2.1, 2.1, 0.0],
            colour: [1.0, 1.0, 1.0],
        },
    ];

    pub struct CubeInstances {
        instances: glium::vertex::VertexBuffer<CubeInstanceAttr>,
        picked: glium::texture::pixel_buffer::PixelBuffer<u32>,
    }

    impl CubeInstances {
        pub fn new(display: &glium::Display) -> Self {
            let instances = glium::vertex::VertexBuffer::new(display, &CUBE_INSTANCES).unwrap();

            let picked = glium::texture::pixel_buffer::PixelBuffer::new_empty(display, 1);
            picked.write(&[8]);

            CubeInstances { instances, picked }
        }

        pub fn instances(&self) -> &glium::vertex::VertexBuffer<CubeInstanceAttr> {
            &self.instances
        }

        pub fn picked(&self) -> &glium::texture::pixel_buffer::PixelBuffer<u32> {
            &self.picked
        }
    }

    #[derive(Copy, Clone)]
    pub struct SpritesBatchVertex {
        pub i_position: [f32; 2],
        pub i_tex_id: u32,
    }

    implement_vertex!(SpritesBatchVertex, i_position, i_tex_id);

    pub const SPRITES_COUNT: usize = 1024;
    pub const SPRITE_RADIAL: f32 = 0.02;

    pub struct SpritesBatch {
        sprites: glium::texture::Texture2dArray,
        sprite_vectors: Vec<[f32; 2]>,
        vertex_buffer: glium::VertexBuffer<SpritesBatchVertex>,
        index_buffer: glium::index::IndexBuffer<u16>,
    }

    impl SpritesBatch {
        pub fn new(display: &glium::Display) -> Self {
            let sprites = {
                let images = (0..64)
                    .map(|_| {
                        let color1: (f32, f32, f32) =
                            (rand::random(), rand::random(), rand::random());
                        let color2: (f32, f32, f32) =
                            (rand::random(), rand::random(), rand::random());
                        vec![vec![color1], vec![color2]]
                    })
                    .collect::<Vec<_>>();

                glium::texture::Texture2dArray::new(display, images).unwrap()
            };

            let mut sprite_vectors = Vec::with_capacity(SPRITES_COUNT);
            for _ in 0..SPRITES_COUNT {
                sprite_vectors.push(Self::random_sprite());
            }

            let (vertex_buffer, index_buffer) = {
                let mut vb: glium::VertexBuffer<SpritesBatchVertex> =
                    glium::VertexBuffer::empty_dynamic(display, SPRITES_COUNT * 4).unwrap();
                let mut ib_data = Vec::with_capacity(SPRITES_COUNT * 6);

                for (num, sprite) in vb.map().chunks_mut(4).enumerate() {
                    let tex_id: u32 = rand::random();
                    let tex_id = tex_id % sprites.get_array_size().unwrap();

                    sprite[0].i_position[0] = -SPRITE_RADIAL;
                    sprite[0].i_position[1] = -SPRITE_RADIAL;
                    sprite[0].i_tex_id = tex_id;
                    sprite[1].i_position[0] = SPRITE_RADIAL;
                    sprite[1].i_position[1] = -SPRITE_RADIAL;
                    sprite[1].i_tex_id = tex_id;
                    sprite[2].i_position[0] = -SPRITE_RADIAL;
                    sprite[2].i_position[1] = SPRITE_RADIAL;
                    sprite[2].i_tex_id = tex_id;
                    sprite[3].i_position[0] = SPRITE_RADIAL;
                    sprite[3].i_position[1] = SPRITE_RADIAL;
                    sprite[3].i_tex_id = tex_id;

                    let num = num as u16;
                    ib_data.push(num * 4);
                    ib_data.push(num * 4 + 1);
                    ib_data.push(num * 4 + 2);
                    ib_data.push(num * 4 + 2);
                    ib_data.push(num * 4 + 1);
                    ib_data.push(num * 4 + 3);
                }

                (
                    vb,
                    glium::index::IndexBuffer::new(
                        display,
                        glium::index::PrimitiveType::TrianglesList,
                        &ib_data,
                    )
                    .unwrap(),
                )
            };

            SpritesBatch {
                sprites,
                sprite_vectors,
                vertex_buffer,
                index_buffer,
            }
        }

        pub fn process_sprites(&mut self) {
            for (sprite, direction) in self
                .vertex_buffer
                .map()
                .chunks_mut(4)
                .zip(&mut self.sprite_vectors)
            {
                let x = direction[0] / 60.0;
                let y = direction[1] / 60.0;
                direction[1] -= 9.832 / 60.0;

                sprite[0].i_position[0] += x;
                sprite[0].i_position[1] += y;
                sprite[1].i_position[0] += x;
                sprite[1].i_position[1] += y;
                sprite[2].i_position[0] += x;
                sprite[2].i_position[1] += y;
                sprite[3].i_position[0] += x;
                sprite[3].i_position[1] += y;

                if sprite[0].i_position[1] < -2.0 {
                    *direction = Self::random_sprite();

                    sprite[0].i_position[0] = -SPRITE_RADIAL;
                    sprite[0].i_position[1] = -SPRITE_RADIAL;
                    sprite[1].i_position[0] = SPRITE_RADIAL;
                    sprite[1].i_position[1] = -SPRITE_RADIAL;
                    sprite[2].i_position[0] = -SPRITE_RADIAL;
                    sprite[2].i_position[1] = SPRITE_RADIAL;
                    sprite[3].i_position[0] = SPRITE_RADIAL;
                    sprite[3].i_position[1] = SPRITE_RADIAL;

                    let new_tex = rand::random::<u32>() % self.sprites.get_array_size().unwrap();

                    sprite[0].i_tex_id = new_tex;
                    sprite[1].i_tex_id = new_tex;
                    sprite[2].i_tex_id = new_tex;
                    sprite[3].i_tex_id = new_tex;
                }
            }
        }

        pub fn texture(&self) -> &glium::texture::Texture2dArray {
            &self.sprites
        }

        pub fn vertex_buffer(&self) -> &glium::VertexBuffer<SpritesBatchVertex> {
            &self.vertex_buffer
        }

        pub fn vertex_buffer_mut(&mut self) -> &mut glium::VertexBuffer<SpritesBatchVertex> {
            &mut self.vertex_buffer
        }

        pub fn sprite_vectors_mut(&mut self) -> &mut Vec<[f32; 2]> {
            &mut self.sprite_vectors
        }

        pub fn index_buffer(&self) -> &glium::index::IndexBuffer<u16> {
            &self.index_buffer
        }

        pub fn random_sprite() -> [f32; 2] {
            let mut x = 2.0 * rand::random::<f32>() - 1.0;
            let mut y = 2.0 * rand::random::<f32>() - 1.0;
            let scale = 3.0 * rand::random::<f32>();

            if f32::abs(x) > f32::abs(y) {
                std::mem::swap(&mut x, &mut y);
            }

            if y < 0.0 {
                y = -y;
            }

            [x, scale * y]
        }
    }
}
