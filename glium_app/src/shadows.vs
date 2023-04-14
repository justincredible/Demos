#version 330 core

uniform mat4 mvp;
uniform mat4 depth_bias_mvp;
uniform mat4 model_matrix;
uniform vec4 model_color;

in vec3 position;
in vec3 normal;

out vec4 shadow_coord;
out vec4 model_normal;

void main() {
    vec4 pos_pnt = vec4(position, 1.0);
    gl_Position =  mvp * pos_pnt;
    model_normal = model_matrix * vec4(normal, 0.0);
    shadow_coord = depth_bias_mvp * pos_pnt;
}
