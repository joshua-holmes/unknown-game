#version 460

layout(location = 0) out vec4 f_color;
// layout(set = 0, binding = 0, r8ui) uniform uimage2D storage_image;

void main() {
    // imageLoad(storage_image, gl_FragCoord.xy);
    f_color = vec4(1., 1., 1., 1.);
}
