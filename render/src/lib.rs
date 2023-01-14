#[allow(unused_imports)]
use vulkano::{
    VulkanLibrary,
    instance::{Instance, InstanceCreateInfo},
    device::{Device, DeviceCreateInfo, Features, QueueCreateInfo},
    buffer::{BufferUsage, CpuAccessibleBuffer},
    command_buffer::{AutoCommandBufferBuilder, CommandBufferUsage, CopyBufferInfo},
    sync::{self, GpuFuture},
};
use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Default, Copy, Clone, Zeroable, Pod)]
struct CuBufferData{
    pub a: u32,
    pub b: u32,
}


pub fn main_loop() {
    let library  = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = Instance::new(library, InstanceCreateInfo::default()).expect("failed to create instance");
    let physical = instance
        .enumerate_physical_devices()
        .expect("could not enumarate devices")
        .next()
        .expect("no devices availble");

    for family in physical.queue_family_properties(){
        println!("found a queue family with {:?} queue(s)", family.queue_count);
    }

    let queue_family_index = physical
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(_, q)| q.queue_flags.graphics)
        .expect("couldn't find a graphical queue family") as u32;

    let (device, mut queues) = Device::new(
        physical,
        DeviceCreateInfo { 
            queue_create_infos : vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            ..Default::default()
        },
    ).expect("failed to create device");
    let queue = queues.next().unwrap();

    //example buffer
    //--------------------------------------------------//
    //let data = CuBufferData{a: 5, b: 10};
    //let buffer = CpuAccessibleBuffer::from_data(
    //    device.clone(), 
    //    BufferUsage {
    //        uniform_buffer: true,
    //        ..Default::default()
    //    },
    //    false,
    //    data
    //).expect("failed to create buffer");
    //--------------------------------------------------//

    //source buffer
    let source_content : Vec<i32> = (0..64).collect();
    let source = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage {
            transfer_src: true,
            ..Default::default()
        },
        false,
        source_content 
    ).expect("failed to create source buffer");

    //destination buffer
    let destination_content : Vec<i32> = (0..64).map(|_| 0).collect();
    let destination = CpuAccessibleBuffer::from_iter(
        device.clone(),
        BufferUsage {
            transfer_dst: true,
            ..Default::default()
        },
        false,
        destination_content
    ).expect("failed to create source buffer");


    let mut builder = AutoCommandBufferBuilder::primary(
        device.clone(),
        queue_family_index,
        CommandBufferUsage::OneTimeSubmit
    )
    .unwrap();

    builder
        .copy_buffer(CopyBufferInfo::buffers(source.clone(), destination.clone()))
        .unwrap();
    let command_buffer = builder.build().unwrap();

    sync::now(device.clone())
    .then_execute(queue.clone(), command_buffer)
    .unwrap()
    .flush()
    .unwrap();

    let dest_result = destination.read().unwrap();
    let src_result = source.read().unwrap();

    assert_eq!(&*dest_result, &*src_result);
}
