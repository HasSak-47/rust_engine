use cc;

fn main(){
    cc::Build::new()
        .file("src/stb_impl/image.c")
        .file("src/stb_impl/perlin.c")
        .compile("c_bindings.a");
}
