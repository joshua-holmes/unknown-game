#version 460

layout(location = 0) in uint dot_value;
layout(location = 0) out vec3 v_color;

void main() {

    gl_Position = vec4(norm_position, 0.0, 1.0);

    if (dot_value == 1) {
        v_color = vec3(1.0, 0.0, 0.0);
    } else if (dot_value == 2) {
        v_color = vec3(0.0, 1.0, 0.0);
    } else if (dot_value == 3) {
        v_color = vec3(0.0, 0.0, 1.0);
    } else {
        v_color = vec3(1.0, 1.0, 1.0);
    }
}
