vulkano_shaders::shader! {
    shaders: {
        vertex: {
            ty: "vertex",
            path: "src/rendering/shaders/shader.vert"
        },
        fragment: {
            ty: "fragment",
            path: "src/rendering/shaders/shader.frag"
        }
    }
}
