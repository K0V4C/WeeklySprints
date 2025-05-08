use std::sync::Arc;

use image::{ImageBuffer, Rgba};
use initialization::init;
use shaders::{cs, fractal_cs, fragment_shader, vertex_shader};
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage}, command_buffer::{
        allocator::{StandardCommandBufferAllocator, StandardCommandBufferAllocatorCreateInfo}, AutoCommandBufferBuilder, ClearColorImageInfo, CommandBufferUsage, CopyBufferInfo, CopyImageToBufferInfo, RenderPassBeginInfo, SubpassBeginInfo, SubpassContents, SubpassEndInfo
    }, descriptor_set::{
        allocator::StandardDescriptorSetAllocator, DescriptorSet, WriteDescriptorSet
    }, format::{ClearColorValue, Format}, image::{view::ImageView, Image, ImageCreateInfo, ImageType, ImageUsage}, instance::{Instance, InstanceCreateFlags, InstanceCreateInfo}, memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator}, pipeline::{
        compute::ComputePipelineCreateInfo, graphics::{color_blend::{ColorBlendAttachmentState, ColorBlendState}, input_assembly::InputAssemblyState, multisample::MultisampleState, rasterization::RasterizationState, vertex_input::{Vertex, VertexDefinition}, viewport::{Viewport, ViewportState}, GraphicsPipelineCreateInfo}, layout::PipelineDescriptorSetLayoutCreateInfo, ComputePipeline, GraphicsPipeline, Pipeline, PipelineBindPoint, PipelineLayout, PipelineShaderStageCreateInfo
    }, render_pass::{Framebuffer, FramebufferCreateInfo, Subpass}, swapchain::Surface, sync::{self, GpuFuture}, VulkanLibrary
};
use winit::{event_loop::EventLoop, window::WindowButtons};

mod initialization;
mod shaders;

#[derive(BufferContents)]
#[repr(C)]
struct MyStruct {
    a: u32,
    b: u32,
}

#[derive(BufferContents, Vertex)]
#[repr(C)]
struct MyVertex {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2]
}

fn fun_part_one() {    
    // ================== First chapter ===============================
    let (device, queue, queue_family_index) = init();

    // ================== Second chapter ==============================
    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));

    // let data = MyStruct{
    //     a: 5, b: 5
    // };

    // let buffer = Buffer::from_data(
    //     memory_allocator.clone(),
    //     BufferCreateInfo{
    //         usage: BufferUsage::UNIFORM_BUFFER,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo{
    //         memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    //         ..Default::default()
    //     },
    //     data)
    // .expect("failed to create a buffer");

    // let mut contents = buffer.write().unwrap();

    // contents.a *= 5;
    // contents.b = 9;

    // let data = (0..128).map(|_| 5u32);

    // let buffer = Buffer::from_iter(memory_allocator.clone(),
    //     BufferCreateInfo{
    //         usage: BufferUsage::UNIFORM_BUFFER,
    //         ..Default::default()
    //     },
    //     AllocationCreateInfo{
    //         memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
    //         ..Default::default()
    //     },
    //     data
    //     ).expect("failed to create a buffer");

    // let mut contents = buffer.write().unwrap();
    // contents[10] = 20;

    let source_content: Vec<i32> = (0..64).collect();
    let source = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        source_content,
    )
    .expect("failed to create source buffer");

    let destination_content: Vec<i32> = (0..64).map(|_| 0).collect();
    let destination = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        destination_content,
    )
    .expect("failed to create destination buffer");

    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    ));

    let mut builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .copy_buffer(CopyBufferInfo::buffers(source.clone(), destination.clone()))
        .unwrap();

    let command_buffer = builder.build().unwrap();

    // Execute it
    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    // Wait for gpu to finish its job with dst
    future.wait(None).unwrap();

    let src_content = source.read().unwrap();
    let destination_content = destination.read().unwrap();
    assert_eq!(&*src_content, &*destination_content);

    println!("Everything succeeded!");

    // =========================== Chater Three: Compute =============================

    let data_iter = 0..65536u32;
    let data_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::STORAGE_BUFFER,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        },
        data_iter,
    )
    .expect("failed to create a buffer");

    let shader = cs::load(device.clone()).expect("failed to create shader module");

    // Creating pipeline
    let cs = shader.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(cs);
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    )
    .expect("could not create compute pipeline");

    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));

    let pipeline_layout = compute_pipeline.layout();
    let descriptor_set_layouts = pipeline_layout.set_layouts();

    // We set this also in compute shader
    let descriptor_set_layout_index = 0;
    let descriptor_set_layout = descriptor_set_layouts
        .get(descriptor_set_layout_index)
        .unwrap();

    let descriptor_set = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        descriptor_set_layout.clone(),
        [WriteDescriptorSet::buffer(0, data_buffer.clone())], // notice how binding here is also 0
        [],
    )
    .unwrap();

    // Create allocator for command buffers
    let command_buffer_allocator = Arc::new(StandardCommandBufferAllocator::new(
        device.clone(),
        StandardCommandBufferAllocatorCreateInfo::default(),
    ));

    // Create builder for command buffer
    let mut command_buffer_builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue.queue_family_index(),
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    let work_group_counts = [1024, 1, 1];
    // dispatch in unsafe
    unsafe {
        command_buffer_builder
            .bind_pipeline_compute(compute_pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                compute_pipeline.layout().clone(),
                descriptor_set_layout_index as u32,
                descriptor_set,
            )
            .unwrap()
            .dispatch(work_group_counts)
            .unwrap();
    }

    let command_buffer = command_buffer_builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let content = data_buffer.read().unwrap();
    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32 * 12);
    }

    println!("2: Everything succeeded!");

    // ===================== Chapetr Four: Images ===========================

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1], // dimesnsion for the image
            usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();

    let image_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_HOST
                | MemoryTypeFilter::HOST_RANDOM_ACCESS,
            ..Default::default()
        },
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .expect("failed to create buffer for the image");

    let mut builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();

    builder
        .clear_color_image(ClearColorImageInfo {
            clear_value: ClearColorValue::Float([0.0, 0.0, 1.0, 1.0]),
            ..ClearColorImageInfo::image(image.clone())
        })
        .unwrap()
        .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
            image.clone(),
            image_buffer.clone(),
        ))
        .unwrap();

    let command_buffer = builder.build().unwrap();

    let future = sync::now(device.clone())
        .then_execute(queue.clone(), command_buffer)
        .unwrap()
        .then_signal_fence_and_flush()
        .unwrap();

    future.wait(None).unwrap();

    let buffer_content = image_buffer.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();

    image.save("image.png").unwrap();
    println!("3: Everything suceeded!");

    // Drawing a fractal with a shader

    let image = Image::new(
        memory_allocator.clone(),
        ImageCreateInfo {
            image_type: ImageType::Dim2d,
            format: Format::R8G8B8A8_UNORM,
            extent: [1024, 1024, 1],
            usage: ImageUsage::STORAGE | ImageUsage::TRANSFER_SRC,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
            ..Default::default()
        },
    )
    .unwrap();

    let view = ImageView::new_default(image.clone()).unwrap();

    // !!! We have to create proper pipeline
    
    let shader = fractal_cs::load(device.clone()).expect("failed to create shader module");
    // Creating pipeline
    let cs = shader.entry_point("main").unwrap();
    let stage = PipelineShaderStageCreateInfo::new(cs);
    let layout = PipelineLayout::new(
        device.clone(),
        PipelineDescriptorSetLayoutCreateInfo::from_stages([&stage])
            .into_pipeline_layout_create_info(device.clone())
            .unwrap(),
    )
    .unwrap();

    let compute_pipeline = ComputePipeline::new(
        device.clone(),
        None,
        ComputePipelineCreateInfo::stage_layout(stage, layout),
    )
    .expect("could not create compute pipeline");

    let descriptor_set_allocator = Arc::new(StandardDescriptorSetAllocator::new(
        device.clone(),
        Default::default(),
    ));

    // Create descriptor set
    let layout = compute_pipeline.layout().set_layouts().get(0).unwrap();
    let set = DescriptorSet::new(
        descriptor_set_allocator.clone(),
        layout.clone(),
        [WriteDescriptorSet::image_view(0, view.clone())], // 0 is the binding
        [],
    )
    .unwrap();

    let buf = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo {
            usage: BufferUsage::TRANSFER_DST,
            ..Default::default()
        },
        AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::HOST_RANDOM_ACCESS
                | MemoryTypeFilter::PREFER_HOST,
            ..Default::default()
        },
        (0..1024 * 1024 * 4).map(|_| 0u8),
    )
    .unwrap();

    let mut builder = AutoCommandBufferBuilder::primary(
        command_buffer_allocator.clone(),
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit,
    )
    .unwrap();
    
    unsafe {
        builder
            .bind_pipeline_compute(compute_pipeline.clone())
            .unwrap()
            .bind_descriptor_sets(
                PipelineBindPoint::Compute,
                compute_pipeline.layout().clone(),
                0,
                set,
            )
            .unwrap()
            .dispatch([1024 / 8, 1024 / 8, 1])
            .unwrap()
            .copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                image.clone(),
                buf.clone(),
            ))
            .unwrap();
    }
    
    let command_buffer = builder.build().unwrap();
    
    let future = sync::now(device.clone()).then_execute(queue.clone(), command_buffer).unwrap()
        .then_signal_fence_and_flush().unwrap();
    
    future.wait(None).unwrap();
    
    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
    image.save("mandelbroter_set.png").unwrap();
    
    println!("4: Everything succeeded!");
 
    let vertex1 = MyVertex { position: [-0.5, -0.5] };
    let vertex2 = MyVertex { position: [ 0.0,  0.5] };
    let vertex3 = MyVertex { position: [ 0.5, -0.25] };
    
    let vertex_buffer = Buffer::from_iter(
        memory_allocator.clone(),
        BufferCreateInfo{
            usage: BufferUsage::VERTEX_BUFFER,
            ..Default::default()
        }, 
        AllocationCreateInfo{
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        }, 
        vec![vertex1, vertex2, vertex3]).unwrap();
    
   let render_pass = vulkano::single_pass_renderpass!(
       device.clone(),
       attachments: {
              color: {
                  format: Format::R8G8B8A8_UNORM,
                  samples: 1,
                  load_op: Clear,
                  store_op: Store,
              },
          },
          pass: {
              color: [color],
              depth_stencil: {},
          },
   ).unwrap(); 
   
   let image = Image::new(
       memory_allocator.clone(),
       ImageCreateInfo {
           image_type: ImageType::Dim2d,
           format: Format::R8G8B8A8_UNORM,
           extent: [1024, 1024, 1], // dimesnsion for the image
           usage: ImageUsage::TRANSFER_DST | ImageUsage::TRANSFER_SRC | ImageUsage::SAMPLED | ImageUsage::COLOR_ATTACHMENT,
           ..Default::default()
       },
       AllocationCreateInfo {
           memory_type_filter: MemoryTypeFilter::PREFER_DEVICE,
           ..Default::default()
       },
   )
   .unwrap();
   
   let view = ImageView::new_default(image.clone()).unwrap();
   
   let framebuffer = Framebuffer::new(
       render_pass.clone(),
       FramebufferCreateInfo{
           attachments: vec![view],
           ..Default::default()
       }
   ).unwrap();
   
   let vs = vertex_shader::load(device.clone()).expect("failed to create shader module");
   let fs = fragment_shader::load(device.clone()).expect("failed to create shader module");
   
   // More on this latter.
   let viewport = Viewport {
       offset: [0.0, 0.0],
       extent: [1024.0, 1024.0],
       depth_range: 0.0..=1.0,
   };
   
   let pipeline = {
       
       let vs = vs.entry_point("main").unwrap();
       let fs = fs.entry_point("main").unwrap();
       
       let vertex_input_state = MyVertex::per_vertex().definition(&vs).unwrap();
       
       let stages = [
       PipelineShaderStageCreateInfo::new(vs),
       PipelineShaderStageCreateInfo::new(fs),
       ];
       
       let layout = PipelineLayout::new(
           device.clone(),
           PipelineDescriptorSetLayoutCreateInfo::from_stages(&stages).into_pipeline_layout_create_info(device.clone()).unwrap()
       ).unwrap();
       
       let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
       GraphicsPipeline::new(
              device.clone(),
              None,
              GraphicsPipelineCreateInfo {
                  // The stages of our pipeline, we have vertex and fragment stages.
                  stages: stages.into_iter().collect(),
                  // Describes the layout of the vertex input and how should it behave.
                  vertex_input_state: Some(vertex_input_state),
                  // Indicate the type of the primitives (the default is a list of triangles).
                  input_assembly_state: Some(InputAssemblyState::default()),
                  // Set the fixed viewport.
                  viewport_state: Some(ViewportState {
                      viewports: [viewport].into_iter().collect(),
                      ..Default::default()
                  }),
                  // Ignore these for now.
                  rasterization_state: Some(RasterizationState::default()),
                  multisample_state: Some(MultisampleState::default()),
                  color_blend_state: Some(ColorBlendState::with_attachment_states(
                      subpass.num_color_attachments(),
                      ColorBlendAttachmentState::default(),
                  )),
                  // This graphics pipeline object concerns the first pass of the render pass.
                  subpass: Some(subpass.into()),
                  ..GraphicsPipelineCreateInfo::layout(layout)
              },
          )
          .unwrap()  
   };
   
   let mut builder = AutoCommandBufferBuilder::primary(
       command_buffer_allocator.clone(),
       queue.queue_family_index(),
       CommandBufferUsage::OneTimeSubmit,
   )
   .unwrap();
      
   let buf = Buffer::from_iter(
       memory_allocator.clone(),
       BufferCreateInfo {
           usage: BufferUsage::TRANSFER_DST,
           ..Default::default()
       },
       AllocationCreateInfo {
           memory_type_filter: MemoryTypeFilter::PREFER_HOST
               | MemoryTypeFilter::HOST_RANDOM_ACCESS,
           ..Default::default()
       },
       (0..1024 * 1024 * 4).map(|_| 0u8),
   )
   .expect("failed to create buffer");
   
   unsafe {
        builder
            .begin_render_pass(
                RenderPassBeginInfo {
                    clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                    ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                },
                SubpassBeginInfo {
                    contents: SubpassContents::Inline,
                    ..Default::default()
                },
            )
            .unwrap()
        
            // new stuff
            .bind_pipeline_graphics(pipeline.clone())
            .unwrap()
            .bind_vertex_buffers(0, vertex_buffer.clone())
            .unwrap()
            .draw(
                3, 1, 0, 0, // 3 is the number of vertices, 1 is the number of instances
            )
            .unwrap()
        
            .end_render_pass(SubpassEndInfo::default())
            .unwrap().copy_image_to_buffer(CopyImageToBufferInfo::image_buffer(
                image, buf.clone()
            )).unwrap();
   }
   
   let command_buffer = builder.build().unwrap();
   
   let future = sync::now(device.clone())
       .then_execute(queue.clone(), command_buffer)
       .unwrap()
       .then_signal_fence_and_flush()
       .unwrap();
   future.wait(None).unwrap();
   
   let buffer_content = buf.read().unwrap();
   let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();
   image.save("triangle.png").unwrap();
   
   println!("5: Everything succeeded!");

}

fn main() {
   // ==================== Final chapter: Windowing ==========================
   
   let event_loop = EventLoop::new().unwrap();
   
   let library = VulkanLibrary::new().unwrap();
   let required_extensions = Surface::required_extensions(&event_loop).unwrap();
   
   let instance = Instance::new(
       library, 
       InstanceCreateInfo {
           flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
           enabled_extensions: required_extensions,
           ..Default::default()
       }
   ).unwrap();
   
   
   
   // let window = Arc::new(
   //     WindowBui::new().build(&event_loop).unwrap()
   // );
   
   
    
}
