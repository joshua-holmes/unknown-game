use std::sync::Arc;

use crate::{
    game::Game,
    game::geometry::{Model, Triangle},
};
use vulkano::{
    buffer::subbuffer::Subbuffer,
    buffer::{Buffer, BufferCreateInfo, BufferUsage},
    command_buffer::{
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo},
        AutoCommandBufferBuilder, CommandBufferExecFuture, CommandBufferUsage,
        PrimaryAutoCommandBuffer, RenderPassBeginInfo, SubpassBeginInfo, SubpassEndInfo,
    },
    descriptor_set::{
        allocator::{StandardDescriptorSetAllocator, StandardDescriptorSetAllocatorCreateInfo},
        PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::PhysicalDeviceType, Device, DeviceCreateInfo, DeviceExtensions, Features, Queue,
        QueueCreateInfo, QueueFlags,
    },
    format::ClearValue,
    image::{view::ImageView, Image, ImageUsage},
    instance::{Instance, InstanceCreateInfo},
    memory::allocator::{
        AllocationCreateInfo, MemoryAllocatePreference, MemoryTypeFilter, StandardMemoryAllocator,
    },
    pipeline::{
        graphics::{
            color_blend::{ColorBlendAttachmentState, ColorBlendState},
            input_assembly::InputAssemblyState,
            multisample::MultisampleState,
            rasterization::RasterizationState,
            vertex_input::{Vertex as VertexMacro, VertexDefinition},
            viewport::{Viewport, ViewportState},
            GraphicsPipelineCreateInfo,
        },
        layout::PipelineDescriptorSetLayoutCreateInfo,
        GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout,
        PipelineShaderStageCreateInfo,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass, Subpass},
    shader::ShaderModule,
    swapchain::{
        self, FullScreenExclusive, PresentFuture, PresentMode, Surface, Swapchain,
        SwapchainAcquireFuture, SwapchainCreateInfo, SwapchainPresentInfo,
    },
    sync::{
        self,
        future::{FenceSignalFuture, JoinFuture},
        GpuFuture,
    },
    Validated, Version, VulkanError, VulkanLibrary,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoop, window::Window};

use super::glsl_types::{Resolution, Vertex};
use super::load_shaders;

// set number of the available descriptor sets
// these numbers must align with what is used in the shaders
const DS_PER_FRAME_STORAGE_SET_NUM: u32 = 0;
const DS_INFREQUENT_UNIFORM_SET_NUM: u32 = 1;

pub type Fence = FenceSignalFuture<
    PresentFuture<CommandBufferExecFuture<JoinFuture<Box<dyn GpuFuture>, SwapchainAcquireFuture>>>,
>;

struct AppliedDescriptorSet {
    set: Arc<PersistentDescriptorSet>,
    set_number: u32,
}

struct AppliedDescriptorSets {
    ds_per_frame_storage: AppliedDescriptorSet,
    ds_infrequent_uniform: AppliedDescriptorSet,
}
impl AppliedDescriptorSets {
    fn to_vec_of_sorted_sets(&self) -> Vec<Arc<PersistentDescriptorSet>> {
        let mut sets = [&self.ds_per_frame_storage, &self.ds_infrequent_uniform]
            .into_iter()
            .collect::<Vec<_>>();
        sets.sort_by_key(|ds| ds.set_number);
        sets.into_iter().map(|ds| ds.set.clone()).collect()
    }
}

pub struct RenderEngine {
    canvas_buffer: Subbuffer<[u8]>,
    swapchain: Arc<Swapchain>,
    fences: Vec<Option<Arc<Fence>>>,
    vertex_buffer: Subbuffer<[Vertex]>,
    queue: Arc<Queue>,
    device: Arc<Device>,
    viewport: Viewport,
    render_pass: Arc<RenderPass>,
    command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
    vertex_shader: Arc<ShaderModule>,
    fragment_shader: Arc<ShaderModule>,
    descriptor_sets: AppliedDescriptorSets,
    window_res_buffer: Subbuffer<[Resolution]>,
}
impl RenderEngine {
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
                present_mode: PresentMode::Fifo, // TODO: switch to `Mailbox` mode potentially (aka triple buffering)
                full_screen_exclusive: FullScreenExclusive::Default, // TODO: add full screen mode in the future, potentially
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

    fn create_resolution_buffer(
        memory_allocator: Arc<StandardMemoryAllocator>,
        resolution: Resolution,
    ) -> Subbuffer<[Resolution]> {
        Buffer::from_iter(
            memory_allocator,
            BufferCreateInfo {
                usage: BufferUsage::UNIFORM_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                ..Default::default()
            },
            [resolution],
        )
        .unwrap()
    }

    fn create_vertex_buffer(
        memory_allocator: Arc<StandardMemoryAllocator>,
        data: Vec<Vertex>,
    ) -> Subbuffer<[Vertex]> {
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

    fn create_canvas_buffer(
        memory_allocator: Arc<StandardMemoryAllocator>,
        data: Vec<u8>,
    ) -> Subbuffer<[u8]> {
        Buffer::from_iter(
            memory_allocator.clone(),
            BufferCreateInfo {
                usage: BufferUsage::STORAGE_BUFFER,
                ..Default::default()
            },
            AllocationCreateInfo {
                memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                    | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
                allocate_preference: MemoryAllocatePreference::AlwaysAllocate,
                ..Default::default()
            },
            data.clone(),
        )
        .unwrap()
    }

    fn create_canvas_model() -> Model {
        // Creates a model the size of the screen
        // calculate 4 corners
        let norm_corner1 = [-1., -1.];
        let norm_corner2 = [-1., 1.];
        let norm_corner3 = [1., -1.];
        let norm_corner4 = [1., 1.];

        // generate triangles and model from corners
        let triangle1 = Triangle::new(norm_corner1, norm_corner2, norm_corner3);
        let triangle2 = Triangle::new(norm_corner2, norm_corner3, norm_corner4);
        Model::new([triangle1, triangle2].into_iter())
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

        let vertex_input_state = Vertex::per_vertex()
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
    fn create_ds_per_frame_storage(
        descriptor_set_allocator: &StandardDescriptorSetAllocator,
        pipeline: Arc<GraphicsPipeline>,
        set_number: u32,
        canvas_buffer: &Subbuffer<[u8]>,
    ) -> AppliedDescriptorSet {
        let layout = pipeline
            .layout()
            .set_layouts()
            .get(set_number as usize)
            .unwrap();
        AppliedDescriptorSet {
            set: PersistentDescriptorSet::new(
                descriptor_set_allocator,
                layout.clone(),
                [WriteDescriptorSet::buffer(0, canvas_buffer.clone())],
                [],
            )
            .unwrap(),
            set_number,
        }
    }

    // descriptor set where the buffers get updated infrequently
    fn create_ds_infrequent_uniform(
        descriptor_set_allocator: &StandardDescriptorSetAllocator,
        pipeline: Arc<GraphicsPipeline>,
        set_number: u32,
        window_res_buffer: &Subbuffer<[Resolution]>,
        canvas_res_buffer: &Subbuffer<[Resolution]>,
    ) -> AppliedDescriptorSet {
        let layout = pipeline
            .layout()
            .set_layouts()
            .get(set_number as usize)
            .unwrap();
        AppliedDescriptorSet {
            set: PersistentDescriptorSet::new(
                descriptor_set_allocator,
                layout.clone(),
                [window_res_buffer, canvas_res_buffer]
                    .into_iter()
                    .enumerate()
                    .map(|(i, buf)| WriteDescriptorSet::buffer(i as u32, buf.clone()))
                    .collect::<Vec<_>>(),
                [],
            )
            .unwrap(),
            set_number,
        }
    }

    fn create_command_buffers(
        device: Arc<Device>,
        queue: Arc<Queue>,
        pipeline: Arc<GraphicsPipeline>,
        framebuffers: &Vec<Arc<Framebuffer>>,
        vertex_buffer: &Subbuffer<[Vertex]>,
        descriptor_sets: &AppliedDescriptorSets,
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
                        descriptor_sets.to_vec_of_sorted_sets(),
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

    pub fn recreate_swapchain(&mut self, window_size: PhysicalSize<u32>) -> Vec<Arc<Image>> {
        let (new_swapchain, new_images) = self
            .swapchain
            .recreate(SwapchainCreateInfo {
                image_extent: window_size.into(),
                ..self.swapchain.create_info()
            })
            .unwrap();
        self.swapchain = new_swapchain;
        new_images
    }

    pub fn recreate_swapchain_and_resize_window(&mut self, window: Arc<Window>) {
        self.flush_swapchain();

        let new_images = self.recreate_swapchain(window.inner_size());
        let new_framebuffers = Self::create_framebuffers(&new_images, self.render_pass.clone());

        self.viewport.extent = window.inner_size().into();
        for res in self.window_res_buffer.write().unwrap().iter_mut() {
            res.update_from(window.inner_size());
        }

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
            &self.descriptor_sets,
        );

        self.command_buffers = new_command_buffers;
    }

    pub fn display_next_frame(&mut self, game: &Game, window: Arc<Window>) {
        // if set to true any time during this function call, swapchain will
        // be recreated and this function will be called again
        let mut recreate_swapchain_after_presentation = false;

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
            recreate_swapchain_after_presentation = true;
            println!("WARNING: swapchain function is suboptimal");
        }

        // wait for image in current position to finish displaying
        if let Some(image_fence) = self.fences[image_i as usize].clone() {
            image_fence.wait(None).unwrap();
        }

        // wait for buffer to be available
        // FIX: when Vulkano 0.35 drops, setup a multibuffer system using descriptor set binding updates to swap
        // between buffers. That way, we don't need to wait for all images in the swapchain to complete presentation
        // every frame, which defeats the purpose of the swapchain
        self.flush_swapchain();

        // write canvas data to buffer
        for (mat, new_mat) in self
            .canvas_buffer
            .write()
            .unwrap()
            .iter_mut()
            .zip(game.canvas.iter_materials_as_bytes())
        {
            *mat = new_mat;
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
                recreate_swapchain_after_presentation = true;
                None
            }
            Err(e) => {
                println!("failed to flush future from img '{}': {}", image_i, e);
                None
            }
        };

        if recreate_swapchain_after_presentation {
            self.recreate_swapchain(window.inner_size());
            self.display_next_frame(game, window);
        }
    }

    pub fn new(event_loop: &EventLoop<()>, window: Arc<Window>, game: &Game) -> Self {
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
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: window.inner_size().into(),
            depth_range: 0.0..=1.0,
        };

        // vertex setup
        let canvas_model = Self::create_canvas_model();
        let vertex_buffer = Self::create_vertex_buffer(
            memory_allocator.clone(),
            canvas_model.into_vec_of_verticies(),
        );

        // canvas setup
        let canvas_buffer =
            Self::create_canvas_buffer(memory_allocator.clone(), game.canvas.iter_materials_as_bytes().collect());

        // resolutions_setup
        let window_res_buffer = Self::create_resolution_buffer(
            memory_allocator.clone(),
            Resolution::from(window.inner_size()),
        );
        let canvas_res_buffer =
            Self::create_resolution_buffer(memory_allocator.clone(), game.resolution);

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

        // create descriptor set allocator
        let descriptor_set_allocator = StandardDescriptorSetAllocator::new(
            device.clone(),
            StandardDescriptorSetAllocatorCreateInfo {
                update_after_bind: true,
                ..Default::default()
            },
        );

        // create descriptor sets
        let ds_per_frame_storage = Self::create_ds_per_frame_storage(
            &descriptor_set_allocator,
            pipeline.clone(),
            DS_PER_FRAME_STORAGE_SET_NUM,
            &canvas_buffer,
        );
        let ds_infrequent_uniform = Self::create_ds_infrequent_uniform(
            &descriptor_set_allocator,
            pipeline.clone(),
            DS_INFREQUENT_UNIFORM_SET_NUM,
            &window_res_buffer,
            &canvas_res_buffer,
        );
        let descriptor_sets = AppliedDescriptorSets {
            ds_per_frame_storage,
            ds_infrequent_uniform,
        };

        // create command buffers
        let command_buffers = Self::create_command_buffers(
            device.clone(),
            queue.clone(),
            pipeline.clone(),
            &framebuffers,
            &vertex_buffer,
            &descriptor_sets,
        );

        // setup fences vector so CPU doesn't have to wait for GPU
        let fences: Vec<Option<Arc<FenceSignalFuture<_>>>> = vec![None; images.len()];

        Self {
            device,
            fences,
            queue,
            swapchain,
            viewport,
            render_pass,
            command_buffers,
            vertex_shader,
            vertex_buffer,
            canvas_buffer,
            fragment_shader,
            descriptor_sets,
            window_res_buffer,
        }
    }
}
