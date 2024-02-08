use std::{sync::Arc, collections::HashMap};

use crate::geometry::{self, Canvas};
use vulkano::{
    buffer::{subbuffer::Subbuffer, BufferContents},
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
        AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage, CopyBufferInfo,
        CopyBufferToImageInfo, PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo,
        SubpassEndInfo,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Queue, Features,
        QueueCreateInfo, QueueFlags,
    },
    format::{ClearValue, Format},
    image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, FreeListAllocator, GenericMemoryAllocator, MemoryAllocatePreference,
        MemoryTypeFilter, StandardMemoryAllocator,
    },
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, PipelineLayout, PipelineShaderStageCreateInfo, PipelineBindPoint, Pipeline,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::ShaderModule,
    swapchain::{
        self, PresentFuture, Surface, Swapchain, SwapchainAcquireFuture, SwapchainCreateInfo,
        SwapchainPresentInfo,
    },
    sync::{
        self,
        future::{FenceSignalFuture, JoinFuture},
        GpuFuture,
    },
    Validated, Version, VulkanError, VulkanLibrary, descriptor_set::{DescriptorSet, PersistentDescriptorSet, allocator::{DescriptorSetAlloc, DescriptorSetAllocator, StandardDescriptorSetAllocator, StandardDescriptorSetAllocatorCreateInfo}, layout::{DescriptorSetLayout, DescriptorSetLayoutCreateInfo, DescriptorSetLayoutCreateFlags}, WriteDescriptorSet, DescriptorBufferInfo, CopyDescriptorSet, pool::{DescriptorPool, DescriptorPoolCreateInfo}},
};
use winit::{dpi::{PhysicalSize, PhysicalPosition}, event_loop::EventLoop, window::Window};

mod load_shaders;

pub type Fence = FenceSignalFuture<
    PresentFuture<CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>>,
>;

pub struct VulkanGraphicsPipeline {
    pub canvas: Canvas,
    swapchain: Arc<Swapchain>,
    fences: Vec<Option<Arc<Fence>>>,
    vertex_buffer: Subbuffer<[geometry::Vertex]>,
    queue: Arc<Queue>,
    device: Arc<Device>,
    window: Arc<Window>,
    viewport: Viewport,
    render_pass: Arc<RenderPass>,
    command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,
    descriptor_set: Arc<PersistentDescriptorSet>,
}
impl VulkanGraphicsPipeline {
    fn init_vulkan_and_window(
        event_loop: &EventLoop<()>,
        window: Arc<Window>,
    ) -> (Arc<Instance>, Arc<Surface>) {
        let library =
            VulkanLibrary::new().expect("Cannot find Vulkan. Vulkan is likely not installed"); // TODO: better error hanlding
        let required_extensions = Surface::required_extensions(event_loop);
        let vk_instance = Instance::new(
            library,
            InstanceCreateInfo {
                application_name: Some("Unknown Game".to_string()),
                application_version: Version::major_minor(0, 0),
                enabled_extensions: required_extensions,
                ..Default::default()
            },
        )
        .unwrap();
        let vk_surface = Surface::from_window(vk_instance.clone(), window).unwrap();

        (vk_instance, vk_surface)
    }

    fn get_graphics_device(
        vk_instance: Arc<Instance>,
        vk_surface: Arc<Surface>,
    ) -> (Arc<Device>, Arc<Queue>) {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };

        let (device, mut queues) = vk_instance
            .enumerate_physical_devices()
            .unwrap()
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
                            .contains(QueueFlags::GRAPHICS | QueueFlags::TRANSFER)
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
                        enabled_features: Features {
                            runtime_descriptor_array: true,
                            ..Features::empty()
                        },
                        ..Default::default()
                    },
                )
            })
            .unwrap()
            .unwrap();

        (device, queues.next().unwrap())
    }

    fn create_swapchain(
        device: Arc<Device>,
        vk_surface: Arc<Surface>,
        window: Arc<Window>,
    ) -> (Arc<Swapchain>, Vec<Arc<Image>>) {
        let capabilities = device
            .physical_device()
            .surface_capabilities(&vk_surface, Default::default())
            .unwrap();
        let (image_format, image_color_space) = device
            .physical_device()
            .surface_formats(&vk_surface, Default::default())
            .unwrap()[0];
        Swapchain::new(
            device.clone(),
            vk_surface.clone(),
            SwapchainCreateInfo {
                min_image_count: capabilities.min_image_count + 1,
                image_format,
                image_color_space,
                image_extent: window.inner_size().into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha: capabilities
                    .supported_composite_alpha
                    .into_iter()
                    .next()
                    .unwrap(), // TODO: setup error handling
                ..Default::default()
            },
        )
        .unwrap()
    }

    fn create_vertex_buffer(
        memory_allocator: Arc<StandardMemoryAllocator>,
        data: Vec<geometry::Vertex>,
    ) -> Subbuffer<[geometry::Vertex]> {
        Buffer::from_iter(
            memory_allocator,
            BufferCreateInfo {
                usage: BufferUsage::VERTEX_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_HOST
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            data,
        )
        .unwrap()
    }

    fn create_canvas_buffers(
        memory_allocator: Arc<StandardMemoryAllocator>,
        data: Vec<geometry::Dot>,
        num_of_buffers: u32,
    ) -> Vec<Subbuffer<[geometry::Dot]>> {
        (0..num_of_buffers)
            .map(|_| {
                Buffer::from_iter(
                    memory_allocator.clone(),
                    BufferCreateInfo {
                        usage: BufferUsage::STORAGE_BUFFER,
                        ..Default::default()
                    },
                    AllocationCreateInfo {
                        memory_type_filter: MemoryTypeFilter::PREFER_HOST
                            | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                        ..Default::default()
                    },
                    data.clone(),
                )
                .unwrap()
            })
            .collect()
    }

    fn create_canvas_model() -> geometry::Model {
        // Creates a model the size of the screen
        // calculate 4 corners
        let norm_corner1 = [-1., -1.];
        let norm_corner2 = [-1., 1.];
        let norm_corner3 = [1., -1.];
        let norm_corner4 = [1., 1.];

        // generate triangles and model from corners
        let triangle1 = geometry::Triangle::new(norm_corner1, norm_corner2, norm_corner3);
        let triangle2 = geometry::Triangle::new(norm_corner2, norm_corner3, norm_corner4);
        geometry::Model::new([triangle1, triangle2].into_iter())
    }

    fn create_render_pass(device: Arc<Device>, swapchain: Arc<Swapchain>) -> Arc<RenderPass> {
        vulkano::single_pass_renderpass!(
            device,
            attachments: {
                clear_color: {
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [clear_color],
                depth_stencil: {}
            },
        )
        .unwrap()
    }

    fn create_framebuffers(
        images: &Vec<Arc<Image>>,
        render_pass: Arc<RenderPass>,
    ) -> Vec<Arc<Framebuffer>> {
        images
            .iter()
            .map(|i| {
                let view = ImageView::new_default(i.clone()).unwrap(); // TODO: setup with error handling
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap() // TODO: setup with error handling
            })
            .collect()
    }

    fn create_graphics_pipeline(
        device: Arc<Device>,
        vertex_shader: Arc<ShaderModule>,
        fragment_shader: Arc<ShaderModule>,
        render_pass: Arc<RenderPass>,
        viewport: Viewport,
    ) -> Arc<GraphicsPipeline> {
        let vs_entry_point = vertex_shader.entry_point("main").unwrap();
        let fs_entry_point = fragment_shader.entry_point("main").unwrap();

        let vertex_input_state = geometry::Vertex::per_vertex()
            .definition(&vs_entry_point.info().input_interface)
            .unwrap();

        let stages = [
            PipelineShaderStageCreateInfo::new(vs_entry_point),
            PipelineShaderStageCreateInfo::new(fs_entry_point),
        ];

        let layout = PipelineLayout::new(
            device.clone(),
            PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages)
                .into_pipeline_layout_create_info(device.clone())
                .unwrap(),
        )
        .unwrap();

        let subpass = Subpass::from(render_pass, 0).unwrap(); // TODO: setup with error handling

        GraphicsPipeline::new(
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
                color_blend_state: Some(ColorBlendState::with_attachment_states(
                    subpass.num_color_attachments(),
                    ColorBlendAttachmentState::default(),
                )),
                subpass: Some(subpass.into()),
                ..GraphicsPipelineCreateInfo::layout(layout)
            },
        )
        .unwrap()
    }

    // descriptor set, which so far only holds the canvas buffers
    // the buffers here get updated every frame
    fn create_descriptor_set(
        descriptor_set_allocator: &StandardDescriptorSetAllocator,
        descriptor_set_layout: Arc<DescriptorSetLayout>,
        canvas_buffers: Vec<Subbuffer<[geometry::Dot]>>,
    ) -> Arc<PersistentDescriptorSet> {
        PersistentDescriptorSet::new(
            descriptor_set_allocator,
            descriptor_set_layout.clone(),
            canvas_buffers.into_iter().map(|buf| {
                WriteDescriptorSet::buffer(
                    0,
                    buf.clone()
                )
            }).collect::<Vec<_>>(),
            []
        )
        .unwrap()
    }

    fn create_command_buffers(
        device: Arc<Device>,
        queue: Arc<Queue>,
        pipeline: Arc<GraphicsPipeline>,
        framebuffers: &Vec<Arc<Framebuffer>>,
        vertex_buffer: &Subbuffer<[geometry::Vertex]>,
        descriptor_set: Arc<PersistentDescriptorSet>,
    ) -> Vec<Arc<PrimaryAutoCommandBuffer>> {
        let command_buffer_allocator = StandardCommandBufferAllocator::new(
            device,
            StandardCommandBufferAllocatorCreateInfo::default(),
        );

        framebuffers
            .iter()
            .map(|framebuffer| {
                let mut builder = AutoCommandBufferBuilder::primary(
                    &command_buffer_allocator,
                    queue.queue_family_index(),
                    CommandBufferUsage::MultipleSubmit,
                )
                .unwrap(); // TODO: setup proper error handling

                builder
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            clear_values: vec![Some(ClearValue::Float([1., 1., 1., 1.]))],
                            ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                        },
                        SubpassBeginInfo::default(),
                    )
                    .unwrap()
                    .bind_pipeline_graphics(pipeline.clone())
                    .unwrap()
                    .bind_vertex_buffers(0, vertex_buffer.clone())
                    .unwrap()
                    .bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        pipeline.layout().clone(),
                        0,
                        [descriptor_set.clone()].to_vec()
                    )
                    .unwrap()
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    .end_render_pass(SubpassEndInfo::default())
                    .unwrap();

                builder.build().unwrap()
            })
            .collect()
    }

    fn flush_swapchain(&mut self) {
        for fence in self.fences.iter_mut() {
            if let Some(f) = fence {
                f.wait(None).unwrap();
            }
            *fence = None;
        }
    }

    pub fn recreate_swapchain(&mut self) -> Vec<Arc<Image>> {
        let (new_swapchain, new_images) = self
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: self.window.inner_size().into(),
                ..self.swapchain.create_info()
            })
            .unwrap();
        self.swapchain = new_swapchain;
        new_images
    }

    pub fn recreate_swapchain_and_resize_window(&mut self) {
        let new_images = self.recreate_swapchain();
        let new_framebuffers = Self::create_framebuffers(&new_images, self.render_pass.clone());

        self.viewport.extent = self.window.inner_size().into();

        let new_pipeline = Self::create_graphics_pipeline(
            self.device.clone(),
            self.vertex_shader.clone(),
            self.fragment_shader.clone(),
            self.render_pass.clone(),
            self.viewport.clone(),
        );

        let new_command_buffers = Self::create_command_buffers(
            self.device.clone(),
            self.queue.clone(),
            new_pipeline,
            &new_framebuffers,
            &self.vertex_buffer,
            self.descriptor_set.clone()
        );

        self.command_buffers = new_command_buffers;
    }

    pub fn display_next_frame(&mut self) {
        // aquire current image index and time that the image finishes being created
        let (image_i, suboptimal, acquire_image_future) =
            match swapchain::acquire_next_image(self.swapchain.clone(), None) {
                Ok(r) => r,
                Err(e) => panic!("failed to acquire next image: {}", e), // TODO: setup with proper error handling
            };
        let previous_image_i = if image_i == 0 {
            self.command_buffers.len() as u32 - 1
        } else {
            image_i - 1
        };

        // suboptimal if properties of swapchain and image differ, image will still display
        if suboptimal {
            self.recreate_swapchain();
            println!("WARNING: swapchain function is suboptimal");
        }

        // wait for image in current position to finish displaying
        if let Some(image_fence) = self.fences[image_i as usize].clone() {
            image_fence.wait(None).unwrap();
        }

        // get time that previous image finishes displaying (or now if there is no previous image)
        let previous_display_future = match self.fences[previous_image_i as usize].clone() {
            None => {
                let mut now = sync::now(self.device.clone());
                now.cleanup_finished();
                now.boxed()
            }
            Some(fence) => fence.boxed(),
        };

        // execute displaying image
        // also get time that this image finishes displaying merged with time that previous image finished displaying
        let current_display_future = previous_display_future
            .join(acquire_image_future)
            .then_execute(
                self.queue.clone(),
                self.command_buffers[image_i as usize].clone(),
            )
            .expect("failed to execute command buffer")
            .then_swapchain_present(
                self.queue.clone(),
                SwapchainPresentInfo::swapchain_image_index(self.swapchain.clone(), image_i),
            )
            .then_signal_fence_and_flush();

        self.fences[image_i as usize] = match current_display_future.map_err(Validated::unwrap) {
            Ok(value) => Some(Arc::new(value)),
            Err(VulkanError::OutOfDate) => {
                self.recreate_swapchain();
                None
            }
            Err(e) => {
                println!("failed to flush future from img '{}': {}", image_i, e);
                None
            }
        };
    }

    pub fn new(event_loop: &EventLoop<()>, window: Arc<Window>) -> Self {
        // init vulkan and window
        let (vk_instance, vk_surface) = Self::init_vulkan_and_window(event_loop, window.clone());

        // get graphics device
        let (device, queue) = Self::get_graphics_device(vk_instance.clone(), vk_surface.clone());

        // create memory allocator
        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

        // create swapchain
        let (swapchain, images) =
            Self::create_swapchain(device.clone(), vk_surface.clone(), window.clone());

        // setup viewport
        let resolution = PhysicalSize::new(1024, 1024);
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [resolution.width as f32, resolution.height as f32],
            depth_range: 0.0..=1.0,
        };

        // setup vertex data
        let canvas_model = Self::create_canvas_model();

        // setup vertex buffers
        let vertex_buffer =
            Self::create_vertex_buffer(memory_allocator.clone(), canvas_model.into_vec_of_verticies());

        // canvas setup
        let canvas_resolution = PhysicalSize::new(8, 6);
        let canvas = Canvas::new(&canvas_resolution);
        let canvas_buffers = Self::create_canvas_buffers(
            memory_allocator.clone(),
            canvas.to_vec_of_dots(),
            images.len() as u32,
        );

        // setup render pass
        let render_pass = Self::create_render_pass(device.clone(), swapchain.clone());

        // create image view
        let framebuffers = Self::create_framebuffers(&images, render_pass.clone());

        // load shaders
        let vertex_shader = load_shaders::load_vertex(device.clone()).unwrap();
        let fragment_shader = load_shaders::load_fragment(device.clone()).unwrap();

        // graphics pipeline
        let pipeline = Self::create_graphics_pipeline(
            device.clone(),
            vertex_shader.clone(),
            fragment_shader.clone(),
            render_pass.clone(),
            viewport.clone(),
        );

        // get descriptor set layout
        let descriptor_set_layout = pipeline.layout().set_layouts().first().unwrap().clone();

        // create descriptor set allocator
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(
            device.clone(),
            StandardDescriptorSetAllocatorCreateInfo {
                update_after_bind: true,
                ..Default::default()
            }
        );

        // create descriptor set
        let descriptor_set = Self::create_descriptor_set(
            &descriptor_set_allocator,
            descriptor_set_layout.clone(),
            canvas_buffers.clone(),
        );

        // create command buffers
        let command_buffers = Self::create_command_buffers(
            device.clone(),
            queue.clone(),
            pipeline.clone(),
            &framebuffers,
            &vertex_buffer,
            descriptor_set.clone(),
        );

        // setup fences vector so CPU doesn't have to wait for GPU
        let fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = vec![None; images.len()];

        Self {
            device,
            fences,
            queue,
            swapchain,
            window,
            viewport,
            render_pass,
            command_buffers,
            vertex_shader,
            vertex_buffer,
            canvas,
            fragment_shader,
            descriptor_set,
        }
    }
}
