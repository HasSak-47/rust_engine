mod lily;
use lily::image;
use lily::bindings::cpp_vulkan;

fn main() {
    unsafe{
        cpp_vulkan::main_loop();
    }
}
