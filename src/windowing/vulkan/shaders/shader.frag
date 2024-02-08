#version 460

struct Dot {
    uint dot_value;
};

layout(location = 0) out vec4 f_color;

layout(binding = 0) buffer DotBuffer {
    Dot dots[];
} dot;


void main() {
    int canvas_w = 8;
    int canvas_h = 6;
    int device_w = 1920;
    int device_h = 1080;

    // aspect ratios
    float canvas_ar = float(canvas_w) / float(canvas_h);
    float device_ar = float(device_w) / float(device_h);

    int offset_x = 0;
    int offset_y = 0;
    float mult_x = 1;
    float mult_y = 1;
    if (device_ar > canvas_ar) {
        float corrected_device_w = canvas_ar * float(device_h);
        offset_x = int(round(
            (float(device_w) - corrected_device_w) / 2.
        ));
        mult_x = device_ar / canvas_ar;
    } else if (device_ar < canvas_ar) {
    }

    float adjusted_x = floor(gl_FragCoord.x) - float(offset_x);
    float adjusted_y = floor(gl_FragCoord.y) - float(offset_y);
    int x = int(floor(adjusted_x * mult_x * float(canvas_w) / float(device_w)));
    int y = int(floor(adjusted_y * mult_y * float(canvas_h) / float(device_h)));

    if (x < 0 || x >= canvas_w) {
        f_color = vec4(0);
        return;
    }

    int flat_coord = x + (canvas_w * y);
    uint dot_value = dot.dots[flat_coord].dot_value;
    // uint dot_value = dot.dots[2563].dot_value;
    vec3 rgb = vec3(0);
    if (dot_value == 9) {
        rgb = vec3(.9, .6, .1);
    }

    f_color = vec4(rgb, 1);
}
