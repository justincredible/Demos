#version 150

in vec2 position;
in vec2 texture;

out vec2 v_texture;

uniform mat4 matrix;

void main() {
    v_texture = texture;
    gl_Position = matrix * vec4(position, 0.0, 1.0);
}
