#version 460

// Dot with dot_value property which represents elemental type in the game
struct Dot {
    uint dot_value;
};

layout(location = 0) out vec4 f_color;

layout(binding = 0) buffer DotBuffer {
    Dot dots[];
} dot;


void main() {
    ivec2 canvas_res = ivec2(10, 4);
    ivec2 device_res = ivec2(1920, 1080);

    // aspect ratios
    float canvas_ar = float(canvas_res.x) / float(canvas_res.y);
    float device_ar = float(device_res.x) / float(device_res.y);

    ivec2 offset = ivec2(0, 0);
    vec2 multi = ivec2(1, 1);
    if (device_ar > canvas_ar) {
        float corrected_device_x = canvas_ar * float(device_res.y);
        offset.x = int(round(
            (float(device_res.x) - corrected_device_x) / 2.
        ));
        multi.x = device_ar / canvas_ar;
    } else if (device_ar < canvas_ar) {
        float corrected_device_y = float(device_res.x) / canvas_ar;
        offset.y = int(round(
            (float(device_res.y) - corrected_device_y) / 2.
        ));
        multi.y = canvas_ar / device_ar;
    }

    vec2 adjusted = floor(gl_FragCoord.xy) - vec2(offset);
    ivec2 canvas_coord = ivec2(floor(adjusted * multi * vec2(canvas_res) / vec2(device_res)));

    if (canvas_coord.x < 0 || canvas_coord.x >= canvas_res.x) {
        f_color = vec4(0);
        return;
    } else if (canvas_coord.y < 0 || canvas_coord.y >= canvas_res.y) {
        f_color = vec4(0);
        return;
    }

    int flat_coord = canvas_coord.x + (canvas_res.x * canvas_coord.y);
    uint dot_value = dot.dots[flat_coord].dot_value;
    // uint dot_value = dot.dots[2563].dot_value;
    vec3 rgb = vec3(0);
    if (dot_value == 9) {
        rgb = vec3(.9, .6, .1);
    }

    f_color = vec4(rgb, 1);
}
