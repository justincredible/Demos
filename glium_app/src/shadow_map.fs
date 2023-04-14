#version 330 core

layout(location = 0) out vec4 depth_colour;

void main(){
    float fd = gl_FragCoord.z;

    depth_colour = vec4(fd, fd, fd, 1);
}
