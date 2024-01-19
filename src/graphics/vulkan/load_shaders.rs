vulkano_shaders::shader! {
    shaders: {
        vertex: {
            ty: "vertex",
            path: "src/graphics/vulkan/shaders/shader.vert"
        },
        fragment: {
            ty: "fragment",
            path: "src/graphics/vulkan/shaders/shader.frag"
        }
    }
}
