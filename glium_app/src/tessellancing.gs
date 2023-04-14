#version 400

layout(triangles) in;
layout(triangle_strip, max_vertices = 3) out;

uniform mat4 view_matrix;

in vec4 te_position[];
in vec3 te_normal[];
in vec2 te_tex_coord[];
in vec3 te_colour[];
flat in uint te_id[];

out vec3 g_colour;
out vec3 g_normal;
out vec2 g_tex_coord;
flat out uint g_id;

void main() {
    g_id = te_id[0];
    g_colour = te_colour[0];
    g_normal = te_normal[0];
    g_tex_coord = te_tex_coord[0];
    gl_Position = te_position[0];
    EmitVertex();

    g_id = te_id[1];
    g_colour = te_colour[1];
    g_normal = te_normal[1];
    g_tex_coord = te_tex_coord[1];
    gl_Position = te_position[1];
    EmitVertex();

    g_id = te_id[2];
    g_colour = te_colour[2];
    g_normal = te_normal[2];
    g_tex_coord = te_tex_coord[2];
    gl_Position = te_position[2];
    EmitVertex();

    EndPrimitive();
}
