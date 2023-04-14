#version 430
layout(local_size_x = 1, local_size_y = 1, local_size_z = 1) in;

uniform layout(binding=3, r8ui) readonly uimage2D uGoLTexture;
uniform layout(binding=4, rgba8) writeonly image2D destTexture;

void main() {
    ivec2 i = ivec2(gl_GlobalInvocationID.x, gl_GlobalInvocationID.y);

    imageStore(destTexture, i, vec4(imageLoad(uGoLTexture, i).x));
}
