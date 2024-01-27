#version 460

// layout(location = 0) in vec4 f_coord;
layout(location = 0) in vec3 color;
layout(location = 0) out vec4 f_color;

void main() {
    // // Normalized pixel coordinates (from 0 to 1)
    // vec2 uv = fragCoord/iResolution.xy;
    //
    // // Time varying pixel color
    // vec3 col = 0.5 + 0.5*cos(iTime+uv.xyx+vec3(0,2,4));
    //
    // // Output to screen
    // fragColor = vec4(col,1.0);
    // vec4 color;
    // if (mod(f_coord.x, 2) == 0 && mod(f_coord.y, 2) == 0) {
    //     color = vec4(1.0, 0.0, 0.0, 1.0);
    // } else {
    //     color = vec4(0.0, 0.0, 0.0, 1.0);
    // }
    f_color = vec4(color, 1.0);
}
