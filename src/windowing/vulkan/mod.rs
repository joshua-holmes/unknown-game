use std::{error::Error, sync::Arc};

use vulkano::{
    buffer::subbuffer::Subbuffer,
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    device::{
        physical::PhysicalDeviceType,
        Device, DeviceCreateInfo, DeviceExtensions, Queue, QueueCreateInfo, QueueFlags,
    },
    image::{view::ImageView, Image},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, MemoryTypeFilter,
        StandardMemoryAllocator,
    },
    pipeline::{
        graphics::{
            color_blend::ColorBlendState,
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::ShaderModule,
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
    Version, VulkanError, VulkanLibrary, command_buffer::{PrimaryAutoCommandBuffer, allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}, AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo, SubpassEndInfo}, format::ClearValue,
};
use winit::{event_loop::EventLoop, window::Window};

use crate::geometry;

mod load_shaders;

type VulkanApiError = Box<dyn Error>;

fn init_vulkan_and_window(
    event_loop: &EventLoop<()>,
    window: Arc<Window>,
) -> Result<(Arc<Instance>, Arc<Surface>), VulkanApiError> {
    let library = VulkanLibrary::new()?;
    let required_extensions = Surface::required_extensions(event_loop);
    let vk_instance = Instance::new(
        library,
        InstanceCreateInfo {
            application_name: Some("Unknown Game".to_string()),
            application_version: Version::major_minor(0, 0),
            enabled_extensions: required_extensions,
            ..Default::default()
        },
    )?;
    let vk_surface = Surface::from_window(vk_instance.clone(), window)?;

    Ok((vk_instance, vk_surface))
}

fn get_graphics_device(
    vk_instance: Arc<Instance>,
    vk_surface: Arc<Surface>,
) -> Result<(Arc<Device>, Arc<Queue>), VulkanApiError> {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..Default::default()
    };

    let (device, mut queues) = vk_instance
        .enumerate_physical_devices()?
        .filter(|pysical_device| {
            pysical_device
                .supported_extensions()
                .contains(&device_extensions)
        })
        .filter_map(|pysical_device| {
            pysical_device
                .queue_family_properties()
                .iter()
                .enumerate()
                .position(|(i, queue_family_properties)| {
                    queue_family_properties
                        .queue_flags
                        .contains(QueueFlags::GRAPHICS)
                        && pysical_device
                            .surface_support(i as u32, &vk_surface)
                            .unwrap_or(false)
                })
                .map(|q| (pysical_device, q as u32))
        })
        .min_by_key(
            |(pysical_device, _)| match pysical_device.properties().device_type {
                PhysicalDeviceType::DiscreteGpu => 0,
                PhysicalDeviceType::IntegratedGpu => 1,
                PhysicalDeviceType::VirtualGpu => 2,
                PhysicalDeviceType::Cpu => 3,
                _ => 4,
            },
        )
        .ok_or(VulkanError::DeviceLost) // FIX: Use my own error because this is wrong
        .map(|(physical_device, queue_family_index)| {
            Device::new(
                physical_device.clone(),
                DeviceCreateInfo {
                    queue_create_infos: vec![QueueCreateInfo {
                        queue_family_index,
                        ..Default::default()
                    }],
                    enabled_extensions: device_extensions,
                    ..Default::default()
                },
            )
        })??;

    Ok((device, queues.next().unwrap()))
}

fn create_swapchain(
    device: Arc<Device>,
    vk_surface: Arc<Surface>,
    window: Arc<Window>,
) -> Result<(Arc<Swapchain>, Vec<Arc<Image>>), VulkanApiError> {
    let capabilities = device
        .physical_device()
        .surface_capabilities(&vk_surface, Default::default())?;
    let (image_format, image_color_space) = device
        .physical_device()
        .surface_formats(&vk_surface, Default::default())?[0];
    Ok(Swapchain::new(
        device.clone(),
        vk_surface.clone(),
        SwapchainCreateInfo {
            min_image_count: capabilities.min_image_count + 1,
            image_format,
            image_color_space,
            image_extent: window.inner_size().into(),
            composite_alpha: capabilities
                .supported_composite_alpha
                .into_iter()
                .next()
                .unwrap(), // TODO: setup error handling
            ..Default::default()
        },
    )?)
}

fn create_vertex_buffer(
    memory_allocator: Arc<StandardMemoryAllocator>,
    triangle: geometry::Triangle,
) -> Result<Subbuffer<[geometry::Vertex]>, VulkanApiError> {
    Ok(Buffer::from_iter(
        memory_allocator,
        BufferCreateInfo {
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        triangle.move_verticies_to_vec(),
    )?)
}

fn create_render_pass(
    device: Arc<Device>,
    vk_swapchain: Arc<Swapchain>,
) -> Result<Arc<RenderPass>, VulkanApiError> {
    Ok(vulkano::single_pass_renderpass!(
        device,
        attachments: {
            clear_color: {
                format: vk_swapchain.image_format(),
                samples: 1,
                load_op: Clear,
                store_op: Store,
            },
        },
        pass: {
            color: [clear_color],
            depth_stencil: {}
        },
    )?)
}

fn get_framebuffers(
    images: &Vec<Arc<Image>>,
    render_pass: Arc<RenderPass>,
) -> Result<Vec<Arc<Framebuffer>>, VulkanApiError> {
    Ok(images
        .iter()
        .map(|i| {
            let view = ImageView::new_default(i.clone()).unwrap(); // TODO: setup with error handling
            Framebuffer::new(
                render_pass,
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap() // TODO: setup with error handling
        })
        .collect())
}

fn get_graphics_pipeline(
    device: Arc<Device>,
    vs: Arc<ShaderModule>,
    fs: Arc<ShaderModule>,
    render_pass: Arc<RenderPass>,
    viewport: Viewport,
) -> Result<Arc<GraphicsPipeline>, VulkanApiError> {
    let vs_entry_point = vs.entry_point("main").unwrap();
    let fs_entry_point = fs.entry_point("main").unwrap();

    let vertex_input_state =
        geometry::Vertex::per_vertex().definition(&vs_entry_point.info().input_interface)?;

    let stages = [
        PipelineShaderStageCreateInfo::new(vs_entry_point),
        PipelineShaderStageCreateInfo::new(fs_entry_point),
    ];

    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
            .into_pipeline_layout_create_info(device.clone())?,
    )?;

    let subpass = Subpass::from(render_pass, 0).unwrap(); // TODO: setup with error handling

    Ok(GraphicsPipeline::new(
        device,
        None,
        GraphicsPipelineCreateInfo {
            stages: stages.into_iter().collect(),
            vertex_input_state: Some(vertex_input_state),
            input_assembly_state: Some(InputAssemblyState::default()),
            viewport_state: Some(ViewportState {
                viewports: [viewport].into_iter().collect(),
                ..Default::default()
            }),
            rasterization_state: Some(RasterizationState::default()),
            multisample_state: Some(MultisampleState::default()),
            color_blend_state: Some(ColorBlendState::default()),
            subpass: Some(subpass.into()),
            ..GraphicsPipelineCreateInfo::layout(layout)
        },
    )?)
}

fn create_command_buffers(
    device: Arc<Device>,
    queue: Arc<Queue>,
    pipeline: Arc<GraphicsPipeline>,
    framebuffers: &Vec<Arc<Framebuffer>>,
    vertex_buffer: &Subbuffer<[geometry::Vertex]>
) -> Result<Vec<Arc<PrimaryAutoCommandBuffer>>, VulkanApiError> {
    let command_buffer_allocator = StandardCommandBufferAllocator::new(
        device,
        StandardCommandBufferAllocatorCreateInfo::default(),
    );

    Ok(framebuffers.iter().map(|framebuffer| {
        let mut builder = AutoCommandBufferBuilder::primary(
            &command_buffer_allocator,
            queue.queue_family_index(),
            CommandBufferUsage::MultipleSubmit
        )
        .unwrap(); // TODO: setup proper error handling

        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some(ClearValue::Float([0., 1., 0., 1.]))],
                    ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                SubpassBeginInfo::default(),
            ).unwrap()
            .bind_pipeline_graphics(pipeline).unwrap()
            .bind_vertex_buffers(0, vertex_buffer.clone()).unwrap()
            .draw(vertex_buffer.len() as u32, 1, 0, 0).unwrap()
            .end_render_pass(SubpassEndInfo::default()).unwrap();

        builder.build().unwrap()
    })
    .collect())
}

pub fn init(event_loop: &EventLoop<()>, window: Arc<Window>) -> Result<(), VulkanApiError> {
    // init vulkan and window
    let (vk_instance, vk_surface) = init_vulkan_and_window(event_loop, window.clone())?;

    // get graphics device
    let (device, queue) = get_graphics_device(vk_instance.clone(), vk_surface.clone())?;

    // create memory allocator
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    // create swapchain
    let (mut vk_swapchain, images) =
        create_swapchain(device.clone(), vk_surface.clone(), window.clone())?;

    // setup basic triangle
    let mut my_triangle = geometry::Triangle::new([0.5, 0.5], [-0.3, 0.], [0., -0.7]);

    // setup vertex buffer
    let vertex_buffer = create_vertex_buffer(memory_allocator.clone(), my_triangle)?;

    // setup render pass
    let render_pass = create_render_pass(device.clone(), vk_swapchain.clone())?;

    // create image view
    let framebuffers = get_framebuffers(&images, render_pass.clone())?;

    // load shaders
    let vs = load_shaders::load_vertex(device.clone())?;
    let fs = load_shaders::load_fragment(device.clone())?;

    // setup viewport
    let mut viewport = Viewport {
        offset: [0.0, 0.0],
        extent: [1024.0, 1024.0],
        depth_range: 0.0..=1.0,
    };

    // graphics pipeline
    let pipeline = get_graphics_pipeline(
        device.clone(),
        vs.clone(),
        fs.clone(),
        render_pass.clone(),
        viewport.clone(),
    )?;

    // create command buffers
    let command_buffers = create_command_buffers(
        device.clone(), 
        queue.clone(), 
        pipeline.clone(), 
        &framebuffers,
        &vertex_buffer
    );

    // setup fences vector so CPU doesn't have to wait for GPU

    Ok(())
}
