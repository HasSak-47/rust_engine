#![allow(dead_code)]
#![allow(unused_imports)]

mod platform;
mod consts;
mod debug;

use std::os::raw::c_void;

use ash::vk::{self, API_VERSION_1_3};
use winit::{
    event_loop::{EventLoop, ControlFlow}, window::WindowBuilder,
    event::WindowEvent,
    dpi::{Size, PhysicalSize},
};

struct QueueFamilyIndices{
    graphics_family: Option<u32>,
}

impl QueueFamilyIndices{
    pub fn is_complete(&self) -> bool{
        self.graphics_family.is_some()
    }
}

struct VulkanApp{
    _entry: ash::Entry,
    instance: ash::Instance,
    debug_utils_loader: ash::extensions::ext::DebugUtils,
    debug_messanger: ash::vk::DebugUtilsMessengerEXT,
}

pub use consts::*;
pub use debug::*;

use platform::required_extension_names;

impl VulkanApp{
    pub fn new() -> Self{
        let _entry = unsafe{ ash::Entry::load().unwrap() };
        let instance = Self::create_instance(&_entry);

        let utils = debug::setup_debug_utils(&_entry, &instance);
        Self{
            _entry,
            instance,
            debug_utils_loader: utils.0,
            debug_messanger: utils.1,
        }
    } 

    pub fn create_instance(entry: &ash::Entry) -> ash::Instance{
        if ENABLE_VALIDATION && debug::check_validation_layer_support(entry) == false {
            panic!("Validation layers requested, but not available!");
        }
        let app_name = APP_NAME.as_ptr() as *const i8;
        let eng_name = ENGINE_NAME.as_ptr() as *const i8;

        let app_info = vk::ApplicationInfo{
            s_type: vk::StructureType::APPLICATION_INFO,
            p_next: std::ptr::null(),
            p_application_name: app_name,
            p_engine_name: eng_name,
            application_version: APPLICATION_VERSION,
            engine_version: ENGINE_VERSION,
            api_version: API_VERSION_1_3,
        };

        let debug_utils_create_info = populate_debug_messenger_create_info();
        let extension_names = platform::required_extension_names();
        
        let create_info = vk::InstanceCreateInfo{
            s_type: vk::StructureType::INSTANCE_CREATE_INFO,
            p_next:
                if ENABLE_VALIDATION
                    { &debug_utils_create_info as *const vk::DebugUtilsMessengerCreateInfoEXT as *const std::ffi::c_void }
                else
                    { std::ptr::null() },
            flags: vk::InstanceCreateFlags::empty(),
            p_application_info: &app_info,
            pp_enabled_layer_names:
                if ENABLE_VALIDATION
                    { VALIDATION_LAYERS.as_ptr() }
                else
                    { std::ptr::null() },
            enabled_layer_count: 
                if ENABLE_VALIDATION
                    { VALIDATION_LAYERS.len() as u32 }
                else
                    { 0 },
            pp_enabled_extension_names: extension_names.as_ptr(),
            enabled_extension_count: extension_names.len() as u32,
        };

        let instance: ash::Instance = unsafe{
            entry
                .create_instance(&create_info, None)
                .expect("Failed to create instance")
        };

        instance
    }

    fn is_physical_device_suitable(instance: &ash::Instance, physical_device: vk::PhysicalDevice) -> bool{
        let device_properties = unsafe {instance.get_physical_device_properties(physical_device)}
        let device_features   = unsafe {instance.get_physical_device_features(physical_device)}

        let device_queue_families = unsafe{
            instance.get_physical_device_queue_family_properties(physical_device)
        };

        let device_type = match device_properties.device_type {
            vk::PhysicalDeviceType::CPU            => "CPU",
            vk::PhysicalDeviceType::INTEGRATED_GPU => "Integrated GPU",
            vk::PhysicalDeviceType::DISCRETE_GPU   => "Discrete GPU",
            vk::PhysicalDeviceType::VIRTUAL_GPU    => "Virtual GPU",
            vk::PhysicalDeviceType::OTHER          => "Other",
        };

        let device_name = device_properties.device_name;
        let major_version = vk::api_version_major(device_properties.api_version);
        let minor_version = vk::api_version_minor(device_properties.api_version);
        let patch_version = vk::api_version_patch(device_properties.api_version);

        for queue_family in device_queue_families.iter(){
        }

    }

    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        WindowBuilder::new()
            .with_inner_size(Size::Physical( PhysicalSize::new(600, 600)))
            .with_title(WIN_TITLE)
            .build(&event_loop)
            .expect("Failed to create window!")
    }

    pub fn draw_frame(&mut self) {}
}

impl Drop for VulkanApp{
    fn drop(&mut self) { unsafe {
        if ENABLE_VALIDATION{
            self.debug_utils_loader.destroy_debug_utils_messenger(self.debug_messanger, None);
        }
        self.instance.destroy_instance(None);
    }}
}

pub fn main(){
    let event_loop = EventLoop::new();
    let _window    = VulkanApp::init_window(&event_loop);

    VulkanApp::init_window(&event_loop);
    let mut app = VulkanApp::new();

    use winit::event::{Event, VirtualKeyCode, ElementState, KeyboardInput, WindowEvent};
    use winit::event_loop::{EventLoop, ControlFlow};
    use winit::window::{Window, WindowBuilder};

    event_loop.run(move |event, _, control_flow| {
        match event{
            Event::WindowEvent { event, .. } => {
                match event {
                    | WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit
                    },
                    | WindowEvent::KeyboardInput { input, .. } => {
                        match input {
                            | KeyboardInput { virtual_keycode, state, .. } => {
                                match (virtual_keycode, state) {
                                    | (Some(VirtualKeyCode::Escape), ElementState::Pressed) => {
                                        *control_flow = ControlFlow::Exit
                                    },
                                    | _ => {},
                                }
                            },
                        }
                    },
                    | _ => {},
                }
            },
            Event::MainEventsCleared => { _window.request_redraw() },
            Event::RedrawRequested(_window_id) => { app.draw_frame(); },
            _ => (),
        }
    });
}

