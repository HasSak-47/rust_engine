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

fn cpp_binds(){
    println!("cargo:rerun-if-changed={}", "src/logger.cpp");
    println!("cargo:rerun-if-changed={}", "src/main_app.cpp");
    let mut ccpp_cm = cc::Build::new();
    ccpp_cm.cpp(true);
    ccpp_cm.include("include");
    ccpp_cm
        .file("src/logger.cpp")
        .file("src/main_app.cpp")
    ;
    ccpp_cm.compile("vulkan_bings.a");
}

fn main(){
    cbind("include/lilith/debug/logger.hpp", "logger.rs");
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

    cpp_binds();


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
