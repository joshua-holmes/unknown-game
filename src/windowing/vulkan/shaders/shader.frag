#version 460

layout(location = 0) out vec4 f_color;
layout(binding = 0) buffer my_block {
    uint dot_value;
};

void main() {
    f_color = vec4(0., dot_value, 0., 1.);
}
