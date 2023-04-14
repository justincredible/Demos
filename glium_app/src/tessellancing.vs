#version 400

// vertex
in vec3 position;
in vec3 normal;
in vec2 texture;
// instance
in uint id;
in vec3 world_position;
in vec3 colour;

out vec3 v_position;
out vec3 v_worldpos;
out vec3 v_normal;
out vec3 v_colour;
out vec2 v_tex_coord;
flat out uint v_id;

void main() {
    v_position = position;
    v_worldpos = world_position;
    v_normal = normal;
    v_colour = colour;
    v_tex_coord = texture;
    v_id = id;
}
