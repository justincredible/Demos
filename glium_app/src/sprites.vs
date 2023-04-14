#version 140

in vec2 i_position;
in uint i_tex_id;

out vec2 v_tex_coords;
flat out uint v_tex_id;

uniform mat4 offset;

void main() {
    gl_Position = offset * vec4(i_position, -0.25, 1);

    if (gl_VertexID % 4 == 0) {
        v_tex_coords = vec2(0.0, 1.0);
    } else if (gl_VertexID % 4 == 1) {
        v_tex_coords = vec2(1.0, 1.0);
    } else if (gl_VertexID % 4 == 2) {
        v_tex_coords = vec2(0.0, 0.0);
    } else {
        v_tex_coords = vec2(1.0, 0.0);
    }

    v_tex_id = i_tex_id;
}
