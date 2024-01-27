#version 460

layout(location = 0) out vec4 f_color;
layout(location = 0) in vec2 vert_position;

void main() {
    vec3 color = vec3(vert_position + 0.5, 0.0);
    f_color = vec4(color, 1.0);
}
