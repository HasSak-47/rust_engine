use bindgen;
use cc;

use std::env;
use std::path::PathBuf;

#[allow(dead_code)]
fn cbind(path : &str, out_name: &str){
    println!("cargo:rerun-if-changed={}", path);
    let binder = bindgen::builder()
        .size_t_is_usize(true)
        .header(path)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect(path);

    let out = PathBuf::from(env::var("OUT_DIR").unwrap());

    binder.write_to_file(out.join(out_name))
        .expect("Couldn't generate some script");
}


fn main(){
    let mut cc_cm = cc::Build::new();
    cc_cm.include("include");
    cc_cm
        .file("src/stb_impl/image.c")
        .file("src/stb_impl/perlin.c");
    cc_cm.compile("stb_bings.a");

    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=vulkan");
    println!("cargo:rustc-link-lib=static=dl");
    println!("cargo:rustc-link-lib=static=pthread");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=Xi");
    println!("cargo:rustc-link-lib=dylib=Xrandr");
}
