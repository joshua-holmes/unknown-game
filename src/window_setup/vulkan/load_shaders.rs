vulkano_shaders::shader! {
    shaders: {
        vertex: {
            ty: "vertex",
            path: "src/vulkan/shaders/shader.vert"
        },
        fragment: {
            ty: "fragment",
            path: "src/vulkan/shaders/shader.frag"
        }
    }
}
