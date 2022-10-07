use cc;

fn main(){
    cc::Build::new()
        .file("src/image.c")
        .compile("stb_img_bindings.a");
}
