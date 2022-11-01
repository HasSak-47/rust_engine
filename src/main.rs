mod lily;
use lily::image;

fn main() {
    let mut img = image::Img::new(128, 128, 3).unwrap();
    for iter in 0..(128 * 128){
        let i = iter % 128;
        let j = iter / 128;

        *img.get_pixel(i, j)[0] = i as u8;
        *img.get_pixel(i, j)[1] = j as u8;
    }

    img.write(image::ImgFmt::PNG, "test.png");
}
