mod lily;

use lily::image::*;
use lily::math::generator::perlin::PerlinGen;
use lily::math::generator::{
    base::BaseGen,
    bubble::*
};

fn main(){
    let mut img = Img::new(512, 512, 1).unwrap();
    let bubble_gen = PerlinGen{ gen: BaseGen{
        seed   : 0,
        x_wrap : 0,
        y_wrap : 0,
        z_wrap : 0,
        max : 1.0,
        min : 0.0,
    }};
    for i in 0..512{
        for j in 0..512{
            let mut pixel = img.get_pixel(i, j);
            let x = i as f64 / 64.0;
            let y = j as f64 / 64.0;
            let mut re = 0.0;
            for k in 1..=4{
                let dk = k as f64;
                re += (1.0 / dk) * (&bubble_gen).generate_2d(x * dk, y * dk);
            }
            re = (re + 2.0) / 4.0;
            *pixel[0] = (re * 255.0) as u8;
        }
    }

    img.write(ImgFmt::PNG, "test.png");
}
