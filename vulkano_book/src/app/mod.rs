use std::sync::Arc;

use vulkano::{
    buffer::Subbuffer,
    command_buffer::{PrimaryAutoCommandBuffer, allocator::StandardCommandBufferAllocator},
    device::{Device, Queue, physical::PhysicalDevice},
    instance::Instance,
    memory::allocator::StandardMemoryAllocator,
    pipeline::{GraphicsPipeline, graphics::viewport::Viewport},
    render_pass::{Framebuffer, RenderPass},
    shader::ShaderModule,
    swapchain::{Surface, Swapchain, SwapchainCreateInfo},
};
use winit::{application::ApplicationHandler, event::WindowEvent, window::Window};

use crate::{
    controls::vulkan_controls,
    device::logical::{get_command_buffers, get_framebuffers, get_pipeline},
    images::MyVertex,
};

#[derive(Default)]
pub struct App {
    window: Option<Arc<Window>>,
    instance: Option<Arc<Instance>>,
    surface: Option<Arc<Surface>>,

    vs: Option<Arc<ShaderModule>>,
    fs: Option<Arc<ShaderModule>>,

    vertex_buffer: Option<Subbuffer<[MyVertex]>>,
    queue: Option<Arc<Queue>>,

    swapchain: Option<Arc<Swapchain>>,
    framebuffers: Option<Vec<Arc<Framebuffer>>>,
    render_pass: Option<Arc<RenderPass>>,
    viewport: Option<Viewport>,
    command_buffers: Option<Vec<Arc<PrimaryAutoCommandBuffer>>>,
    pipeline: Option<Arc<GraphicsPipeline>>,

    command_buffer_allocator: Option<Arc<StandardCommandBufferAllocator>>,
    memory_allocator: Option<Arc<StandardMemoryAllocator>>,

    physical_device: Option<Arc<PhysicalDevice>>,
    device: Option<Arc<Device>>,

    recreate_swapchain: bool,
    window_resized: bool,
}

impl App {
    pub fn new(instance: Option<Arc<Instance>>) -> Self {
        App {
            instance,
            window: None,
            surface: None,

            vs: None,
            fs: None,

            vertex_buffer: None,
            queue: None,

            swapchain: None,
            framebuffers: None,
            render_pass: None,
            viewport: None,
            command_buffers: None,
            pipeline: None,

            memory_allocator: None,
            command_buffer_allocator: None,

            physical_device: None,
            device: None,

            recreate_swapchain: false,
            window_resized: false,
        }
    }

    pub fn get_window(&self) -> Arc<Window> {
        self.window.as_ref().unwrap().clone()
    }

    pub fn get_instance(&self) -> Arc<Instance> {
        self.instance.as_ref().unwrap().clone()
    }

    pub fn get_surface(&self) -> Arc<Surface> {
        self.surface.as_ref().unwrap().clone()
    }

    pub fn set_swapchain(&mut self, swapchain: Arc<Swapchain>) {
        self.swapchain = Some(swapchain);
    }

    pub fn set_framebuffers(&mut self, framebuffers: Vec<Arc<Framebuffer>>) {
        self.framebuffers = Some(framebuffers);
    }

    pub fn set_render_pass(&mut self, render_pass: Arc<RenderPass>) {
        self.render_pass = Some(render_pass);
    }

    pub fn set_viewport(&mut self, viewport: Viewport) {
        self.viewport = Some(viewport);
    }

    pub fn set_comand_buffers(&mut self, command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>) {
        self.command_buffers = Some(command_buffers);
    }

    pub fn set_comand_buffer_allocator(
        &mut self,
        command_buffer_allocator: Arc<StandardCommandBufferAllocator>,
    ) {
        self.command_buffer_allocator = Some(command_buffer_allocator);
    }

    pub fn set_memory_allocator(&mut self, memory_allocator: Arc<StandardMemoryAllocator>) {
        self.memory_allocator = Some(memory_allocator);
    }

    pub fn set_device(&mut self, device: Arc<Device>) {
        self.device = Some(device);
    }

    pub fn set_pipeline(&mut self, pipeline: Arc<GraphicsPipeline>) {
        self.pipeline = Some(pipeline);
    }

    pub fn set_vertex_buffer(&mut self, vertex_buffer: Subbuffer<[MyVertex]>) {
        self.vertex_buffer = Some(vertex_buffer);
    }

    pub fn set_queue(&mut self, queue: Arc<Queue>) {
        self.queue = Some(queue);
    }

    pub fn set_vs_and_fs(&mut self, vs: Arc<ShaderModule>, fs: Arc<ShaderModule>) {
        self.vs = Some(vs);
        self.fs = Some(fs);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // for games
        event_loop.set_control_flow(winit::event_loop::ControlFlow::Poll);

        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );

        let surface = Some(
            Surface::from_window(self.instance.as_ref().unwrap().clone(), window.clone()).unwrap(),
        )
        .expect("failed to create a surface");

        self.window = Some(window);
        self.surface = Some(surface);

        println!("Window and surface created!");

        vulkan_controls::setup(self);
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(window) = self.window.as_ref() {
            window.request_redraw();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::Resized(_) => {
                self.window_resized = true;
            }
            WindowEvent::RedrawRequested => {
                
                if self.recreate_swapchain {
                    self.recreate_swapchain = false;

                    let new_dimension = self.window.as_ref().unwrap().inner_size();

                    if let Some(x) = self.swapchain.as_ref() {
                        let (new_swapchain, new_images) = x
                            .recreate(SwapchainCreateInfo {
                                image_extent: new_dimension.into(),
                                ..Default::default()
                            })
                            .expect("failed to recreate swapchain: {e}");

                        self.set_swapchain(new_swapchain);

                        if let Some(rp) = self.render_pass.as_ref() {
                            self.set_framebuffers(get_framebuffers(&new_images, rp));

                            if self.window_resized {
                                self.window_resized = false;

                                self.viewport.as_mut().unwrap().extent = new_dimension.into();

                                self.set_pipeline(get_pipeline(
                                    self.device.as_ref().unwrap().clone(),
                                    self.vs.as_ref().unwrap().clone(),
                                    self.fs.as_ref().unwrap().clone(),
                                    self.render_pass.as_ref().unwrap().clone(),
                                    self.viewport.as_ref().unwrap().clone(),
                                ));

                                self.set_comand_buffers(get_command_buffers(
                                    &self.command_buffer_allocator.as_ref().unwrap().clone(),
                                    &self.queue.as_ref().unwrap().clone(),
                                    &self.pipeline.as_ref().unwrap().clone(),
                                    &self.framebuffers.as_ref().unwrap().clone(),
                                    &self.vertex_buffer.as_ref().unwrap().clone(),
                                ));
                            }
                        }
                    }
                }

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}
