mod lily;
use lily::image::*;
use core::num::Wrapping;

fn main() {
    let mut img = Img::new(256, 256, 3).unwrap();

    for i in 0..256usize{
        for j in 0..256usize{
            let __j = Wrapping(j); 
            let __i = Wrapping(i); 
            let mut pixel = img.get_pixel(i, j);

            *pixel[0] = ((__i + __j).0 % 255) as u8;
            *pixel[1] = ((__i - __j).0 % 255) as u8;
            *pixel[2] = 255u8;
        }
    }

    img.write(ImgFmt::PNG, "test.png");

}
