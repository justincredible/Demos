#version 150

in vec2 v_texture;

out vec4 colour;

uniform sampler2D image;

void main() {
    colour = texture(image, v_texture);
}
