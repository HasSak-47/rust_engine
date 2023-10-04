#![allow(dead_code)]
#![allow(unused_imports)]

mod platform;
mod consts;
mod debug;

use std::os::raw::c_void;

use winit::{
    event_loop::{EventLoop, ControlFlow}, window::WindowBuilder,
    event::WindowEvent,
    dpi::{Size, PhysicalSize},
};

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
    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        WindowBuilder::new()
            .with_inner_size(Size::Physical( PhysicalSize::new(600, 600)))
            .with_title(WIN_TITLE)
            .build(&event_loop)
            .expect("Failed to create window!")
    }
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
}

