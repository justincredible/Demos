#version 330 core

uniform sampler2DShadow shadow_map;
uniform vec3 light_loc;
uniform vec4 model_color;

in vec4 shadow_coord;
in vec4 model_normal;

out vec4 f_colour;
out uint f_id;

void main() {
    vec3 light_color = vec3(1,1,1);
    float bias = 0.0; // Geometry does not require bias

    float lum = max(dot(normalize(model_normal.xyz), normalize(light_loc)), 0.0);

    float visibility = texture(shadow_map, vec3(shadow_coord.xy, (shadow_coord.z-bias)/shadow_coord.w));

    f_colour = vec4(max(lum * visibility, 0.05) * model_color.rgb * light_color, 1.0);
    f_id = uint(8);
}
