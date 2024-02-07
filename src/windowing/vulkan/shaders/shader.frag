#version 460

struct Dot {
    uint dot_value;
};

layout(location = 0) out vec4 f_color;

layout(binding = 0) buffer DotBuffer {
    Dot dots[];
} dot;


void main() {
    uint dot_value = dot.dots[0].dot_value;
    vec3 rgb = vec3(1);
    if (dot_value == 9) {
        rgb = vec3(.1, .6, .9);
    }

    f_color = vec4(rgb, 1);
}
