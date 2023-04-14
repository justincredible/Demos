#version 150

in vec3 position;

out vec4 v_colour;

uniform mat4 matrix;

void main() {
    v_colour = vec4(position + vec3(0.5), 1.0);
    gl_Position = matrix * vec4(position, 1.0);
}
