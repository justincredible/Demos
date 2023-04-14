#version 140

uniform sampler2DArray tex;

in vec2 v_tex_coords;
flat in uint v_tex_id;

out vec4 f_colour;
out uint f_id;

void main() {
    f_colour = texture(tex, vec3(v_tex_coords, float(v_tex_id)));
    f_id = uint(8);
}
