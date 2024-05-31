#version 460

struct Resolution {
    int width;
    int height;
};

layout(location = 0) out vec4 f_color;

layout(set = 0, binding = 0) buffer MaterialBuffer {
    highp uint materials[];
};

layout(std140, set = 1, binding = 0) uniform WindowRes {
    Resolution res;
} window;
layout(std140, set = 1, binding = 1) uniform CanvasRes {
    Resolution res;
} canvas;


vec3 hex_to_vec3(uint hex) {
    float r = float((hex & 0xff0000) >> 16) / 255.;
    float g = float((hex & 0x00ff00) >> 8) / 255.;
    float b = float(hex & 0x0000ff) / 255.;
    return vec3(r, g, b);
}


vec3 get_color(uint material) {
    switch (material) {
    case 1: // sand
        return hex_to_vec3(0xd7c9aa);
    case 2: // dirt
        return hex_to_vec3(0x564138);
    case 3: // blue
        return hex_to_vec3(0x36C9C6);
    case 4: // blue
        return hex_to_vec3(0xC03221);
    default:
        return hex_to_vec3(0x000000);
    }
}


uint get_material(uint flat_coord) {
    uint material_bytes = materials[flat_coord / 4];
    uint byte_index = uint(mod(flat_coord, 4));
    uint small_num = byte_index * 8;
    uint large_num = small_num + 8;
    uint byte_we_care_about = uint(pow(2, large_num) - pow(2, small_num));
    return (material_bytes & byte_we_care_about) >> small_num;
}


void main() {
    ivec2 window_res = ivec2(window.res.width, window.res.height);
    ivec2 canvas_res = ivec2(canvas.res.width, canvas.res.height);

    // aspect ratios
    float window_ar = float(window_res.x) / float(window_res.y);
    float canvas_ar = float(canvas_res.x) / float(canvas_res.y);

    ivec2 offset = ivec2(0, 0);
    vec2 multi = vec2(1., 1.);
    if (window_ar > canvas_ar) {
        float corrected_window_x = canvas_ar * float(window_res.y);
        offset.x = int(round(
            (float(window_res.x) - corrected_window_x) / 2.
        ));
        multi.x = window_ar / canvas_ar;
    } else if (window_ar < canvas_ar) {
        float corrected_window_y = float(window_res.x) / canvas_ar;
        offset.y = int(round(
            (float(window_res.y) - corrected_window_y) / 2.
        ));
        multi.y = canvas_ar / window_ar;
    }

    vec2 adjusted = floor(gl_FragCoord.xy) - vec2(offset);
    ivec2 canvas_coord = ivec2(floor(adjusted * multi * vec2(canvas_res) / vec2(window_res)));

    if (canvas_coord.x < 0 || canvas_coord.x >= canvas_res.x) {
        f_color = vec4(0.05);
        return;
    } else if (canvas_coord.y < 0 || canvas_coord.y >= canvas_res.y) {
        f_color = vec4(0.05);
        return;
    }

    int flat_coord = canvas_coord.x + (canvas_res.x * canvas_coord.y);
    uint material = get_material(flat_coord);

    vec3 rgb = get_color(material);

    f_color = vec4(rgb, 1);
}
