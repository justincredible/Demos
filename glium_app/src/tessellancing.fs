#version 400

in vec3 g_normal;
in vec3 g_colour;
in vec2 g_tex_coord;
flat in uint g_id;

layout(location = 0) out vec4 f_colour;
layout(location = 1) out uint f_id;

uniform sampler2D colour_texture;

const vec3 LIGHT = vec3(-0.2, 0.1, 0.8);

void main() {
    float lum = max(dot(normalize(g_normal), normalize(LIGHT)), 0.0);
    vec3 tex_colour = texture(colour_texture, g_tex_coord).rgb;
    vec3 colour = (0.6 + 0.4 * lum) * tex_colour;

    f_colour = vec4(g_colour * colour, 1.0);
    f_id = g_id;
}
