#version 430
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform uint width;
uniform uint height;
uniform layout(binding=3, r8ui) readonly uimage2D previous_generation;
uniform layout(binding=4, rgba8ui) writeonly uimage2D next_generation;

// The rules do not require us to inspect the cell we are updating.
void main() {
    uint x = gl_GlobalInvocationID.x;
    uint y = gl_GlobalInvocationID.y;

    uint neighbour_sum = imageLoad(previous_generation, ivec2(x, (y + 1) % height)).x
        + imageLoad(previous_generation, ivec2((x + 1) % width, (y + 1) % height)).x
        + imageLoad(previous_generation, ivec2((x + 1) % width, y)).x
        + imageLoad(previous_generation, ivec2((x + 1) % width, (y - 1 + height) % height)).x
        + imageLoad(previous_generation, ivec2(x, (y - 1 + height) % height)).x
        + imageLoad(previous_generation, ivec2((x - 1 + width) % width, (y - 1 + height) % height)).x
        + imageLoad(previous_generation, ivec2((x - 1 + width) % width, y)).x
        + imageLoad(previous_generation, ivec2((x - 1 + width) % width, (y + 1) % height)).x;

    uint alive = 0;
    if (neighbour_sum != 2) {
        if (neighbour_sum == 3) alive = 1;

        imageStore(next_generation, ivec2(x, y), uvec4(alive));
    }
}
