vulkano_shaders::shader! {
    shaders: {
        vertex: {
            ty: "vertex",
            path: "src/windowing/vulkan/shaders/shader.vert"
        },
        fragment: {
            ty: "fragment",
            path: "src/windowing/vulkan/shaders/shader.frag"
        }
    }
}
