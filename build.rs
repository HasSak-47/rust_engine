use bindgen;
use cc;

use std::env;
use std::path::PathBuf;


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
    /*
    let stb_bindings = bindgen::builder()
        .header("include/lilith/image.h")
        .header("include/lilith/perlin.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("unable to generate binds");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    stb_bindings.write_to_file(out_path.join("stb_bindings.rs"))
        .expect("Couldn't generate stb_bindings");
    */

    let mut cc_cm = cc::Build::new();
    cc_cm.include("include");
    cc_cm
        .file("src/stb_impl/image.c")
        .file("src/stb_impl/perlin.c");
    cc_cm.compile("stb_bings.a");



    /*
    // I really really want to use cpp but its a bad idea
    let mut cppbuild = cc::Build::new();
    cppbuild.include("include");
    cppbuild
        .file("src/vulkan.cpp")
        .file("src/debug.cpp")
    ;
    cppbuild.compiler("gcc")
        .compile("cppbinds")
    ;
    */
    
    println!("cargo:rustc-link-lib=dylib=glfw");
    println!("cargo:rustc-link-lib=dylib=vulkan");
    println!("cargo:rustc-link-lib=static=dl");
    println!("cargo:rustc-link-lib=static=pthread");
    println!("cargo:rustc-link-lib=dylib=X11");
    println!("cargo:rustc-link-lib=dylib=Xi");
    println!("cargo:rustc-link-lib=dylib=Xrandr");
}
