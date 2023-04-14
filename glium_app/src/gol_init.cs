#version 430
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform uint width;
uniform uint height;
uniform layout(binding=3, r8ui) writeonly uimage2D target_texture;

#define SIDE 100

void main() {
    uint x = gl_GlobalInvocationID.x;
    uint y = gl_GlobalInvocationID.y;

    uint alive = 0;
    if (x >= width/2 - SIDE && x <= width/2 + SIDE && y >= height/2 - SIDE && y <= height/2 + SIDE) {
        alive = 1;
    }

    imageStore(target_texture, ivec2(x, y), uvec4(alive));
}
