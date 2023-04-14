#version 150
#extension GL_ARB_shader_subroutine : require

in vec4 v_colour;

out vec4 f_colour;
out uint f_id;
subroutine vec4 colour_t();

subroutine uniform colour_t colour;
uniform float percentage;

subroutine(colour_t)
vec4 ColourBlack() {
    return mix(v_colour, vec4(0.0, 0, 0, 1), percentage);
}

subroutine(colour_t)
vec4 ColourBlue() {
    return mix(v_colour, vec4(0.0, 0, 1, 1), percentage);
}

subroutine(colour_t)
vec4 ColourGreen() {
    return mix(v_colour, vec4(0.0, 1, 0, 1), percentage);
}

subroutine(colour_t)
vec4 ColourRed() {
    return mix(v_colour, vec4(1.0, 0, 0, 1), percentage);
}

subroutine(colour_t)
vec4 ColourCyan() {
    return mix(v_colour, vec4(0.0, 1, 1, 1), percentage);
}

subroutine(colour_t)
vec4 ColourMagenta() {
    return mix(v_colour, vec4(1.0, 0, 1, 1), percentage);
}

subroutine(colour_t)
vec4 ColourYellow() {
    return mix(v_colour, vec4(1.0, 1, 0, 1), percentage);
}

subroutine(colour_t)
vec4 ColourWhite() {
    return mix(v_colour, vec4(1.0, 1, 1, 1), percentage);
}

subroutine(colour_t)
vec4 ColourNone() {
    return v_colour;
}

void main() {
    f_colour = colour();
    f_id = uint(8);
}
