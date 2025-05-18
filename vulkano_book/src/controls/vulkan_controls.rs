use std::sync::Arc;

use vulkano::{
    command_buffer::allocator::{
        StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo,
    },
    device::DeviceExtensions,
    memory::allocator::StandardMemoryAllocator,
    pipeline::graphics::viewport::Viewport,
    swapchain::Surface,
};

use crate::{
    app::App,
    device::{
        logical::{
            create_logical_device, create_swapchain, get_command_buffers, get_framebuffers,
            get_pipeline, get_renderer_pass,
        },
        physical::select_physical_device,
    },
    images::create_triangle_buffer,
    shaders,
};

pub fn setup(app: &mut App) {
    let instance = app.get_instance();
    let window = app.get_window();
    let surface = app.get_surface();

    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::default()
    };

    let (physical_device, queue_family_index) =
        select_physical_device(&instance, &surface, &device_extensions);

    let (device, mut queues) = create_logical_device(
        physical_device.clone(),
        queue_family_index,
        device_extensions,
    );

    let queue = queues.next().unwrap();

    let (swapchain, images) = create_swapchain(&physical_device, &device, &surface, &window);

    let render_pass = get_renderer_pass(&device, &swapchain);

    let framebuffers = get_framebuffers(&images, &render_pass);

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    let vertex_buffer = create_triangle_buffer(&memory_allocator);

    let vs = shaders::vertex_shader::load(device.clone()).expect("failed to create shader module");
    let fs =
        shaders::fragment_shader::load(device.clone()).expect("failed to create shader module");

    let viewport = Viewport {
        offset: [0.0, 0.0],
        extent: window.inner_size().into(),
        depth_range: 0.0..=1.0,
    };

    let pipeline = get_pipeline(
        device.clone(),
        vs.clone(),
        fs.clone(),
        render_pass.clone(),
        viewport.clone(),
    );

    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    ));

    let command_buffers = get_command_buffers(
        &command_buffer_allocator,
        &queue,
        &pipeline,
        &framebuffers,
        &vertex_buffer,
    );

    app.set_swapchain(swapchain);
    app.set_framebuffers(framebuffers);
    app.set_render_pass(render_pass);
    app.set_comand_buffers(command_buffers);
    app.set_viewport(viewport);
    app.set_memory_allocator(memory_allocator);
    app.set_comand_buffer_allocator(command_buffer_allocator);
    app.set_pipeline(pipeline);
    app.set_vs_and_fs(vs, fs);
    app.set_queue(queue);
    app.set_vertex_buffer(vertex_buffer);
}
