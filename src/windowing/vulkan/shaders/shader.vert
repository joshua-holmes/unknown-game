#version 460

layout(location = 0) in vec2 position;
layout(location = 0) out vec2 vert_position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    vert_position = position;
}
