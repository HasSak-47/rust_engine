mod platform;
mod consts;
mod debug;

use winit::{
    event_loop::{EventLoop, ControlFlow}, window::WindowBuilder,
    event::WindowEvent,
    dpi::{Size, PhysicalSize}, };

struct VulkanApp{
    _entry: ash::Entry,
    instance: ash::Instance,
}

pub use consts::*;
pub use debug::*;

impl VulkanApp{

    fn create_instance(entry: &ash::Entry) -> ash::Instance {
        use ash::{
            Instance,
            vk::{
                ApplicationInfo,
                StructureType,
                API_VERSION_1_0,
                make_api_version,
                InstanceCreateInfo,
                InstanceCreateFlags,
            },
        };

        if ENABLE_VALIDATION && !check_validation_layer_support(&entry){
            panic!("could not find validation layer");
        }
        let (layers_ptr, layer_count) = get_validation_layers();


        let app_name = std::ffi::CString::new(APP_NAME).unwrap();
        let engine_name = std::ffi::CString::new(ENGINE_NAME).unwrap();

        let app_info = ApplicationInfo {
            s_type: StructureType::APPLICATION_INFO,
            p_engine_name: engine_name.as_ptr(),
            p_application_name: app_name.as_ptr(),
            engine_version: make_api_version(1, 0, 0, 0),
            application_version: make_api_version(1, 0, 0, 0),
            api_version: API_VERSION_1_0,
            p_next: std::ptr::null(),
        };


        let create_info = InstanceCreateInfo {
            s_type: StructureType::INSTANCE_CREATE_INFO,
            p_application_info: &app_info,
            enabled_extension_count: 0,
            flags: InstanceCreateFlags::empty(),
            pp_enabled_extension_names: std::ptr::null(),

            // validation layers
            enabled_layer_count: layer_count,
            pp_enabled_layer_names: layers_ptr,
            p_next: std::ptr::null(),
        };

        let instance: Instance = unsafe{
            entry.create_instance(&create_info, None)
            .expect("Failed to create instance!")
        };

        instance
    }

    fn init_window(event_loop: &EventLoop<()>) -> winit::window::Window {
        WindowBuilder::new()
            .with_inner_size(Size::Physical( PhysicalSize::new(600, 600)))
            .with_title(WIN_TITLE)
            .build(&event_loop)
            .expect("Failed to create window!")
    }

    pub fn draw_frame(&self) {

    }

    pub fn new() -> Self{
        use ash::Entry;
        let entry = unsafe{ Entry::load().expect("failed to load entry!") };
        let instance = Self::create_instance(&entry);

        Self { _entry: entry, instance }
    }

}

impl Drop for VulkanApp{
    fn drop(&mut self) {
        unsafe{ self.instance.destroy_instance(None); }
    }
}

pub fn _main(){
    let event_loop = EventLoop::new();
    let _window = VulkanApp::init_window(&event_loop);
    let vulkan_app = VulkanApp::new();

    use winit::event::Event::*;
    event_loop.run(move |event, _, control_flow| {
        let now = std::time::Instant::now();
        let dur = std::time::Duration::from_millis(100);
        control_flow.set_wait_until(now + dur);

        match event{
            WindowEvent { event , ..} => {
                use winit::event::WindowEvent::*;
                match event{
                    CloseRequested => *control_flow = ControlFlow::Exit,
                    _ => {}
                }
            }

            //MainEventsCleared => window.request_redraw(),
            RedrawRequested(_) => vulkan_app.draw_frame(),

            _ => (),
        }
    })
}

pub fn main(){
    _main();
}
