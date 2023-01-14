use cc;

fn main(){
    cc::Build::new()
        .include("../include")
        .file("src/stb/perlin.c")
        .compile("c_bindings.a");
}
