use app::App;
use vulkan::create_instance;
use winit::event_loop::EventLoop;

mod app;
mod controls;
pub mod device;
mod images;
pub mod shaders;
mod vulkan;

fn main() {
    // General structure vvvv

    // instance

    // surface

    // physical device
    // logical device
    // queue creation

    // swapchain

    // render pass
    // framebuffers
    // vertex buffer
    // shaders
    // viewport
    // pipeline
    // command buffers

    // event loop

    // ==================== Final chapter: Windowing ==========================
    // All of this can be put away in some functions
    let event_loop = EventLoop::new().unwrap();
    let instance = create_instance(&event_loop);
    let mut app = App::new(Some(instance.clone()));
    let _ = event_loop.run_app(&mut app);
    println!("DONE");
}
