use cc;

fn main(){
    cc::Build::new()
        .include("../include")
        .file("src/stb/image.c")
        .file("src/stb/perlin.c")
        .compile("c_bindings.a");
}
