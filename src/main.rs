use std::f32::consts::PI;

use image::GrayImage;
use math::rand_noise::random::rand_range;

fn main(){
    let mut image = GrayImage::new(256, 256);

    const step : usize = 256 / 16;
    let mut pointers = [[(0., 0.); step]; step];

    for indx in 0..step * step{
        let i = indx % step;
        let j = indx / step;

        let angle : f32 = rand_range(-PI, PI);

        pointers[i][j].0 = angle.cos();
        pointers[i][j].1 = angle.sin();
    }


    let interpolate = |a: f32, b: f32, w: f32| (b - a) * (3.0 - w * 2.0) * w * w + a;
    let gradient = |coord_x : u32, coord_y: u32, x: f32, y: f32| {
        let delta_x = x - coord_x as f32;
        let delta_y = y - coord_y as f32;
        
        let arrow = pointers[coord_x as usize % step][coord_y as usize % step];

        delta_x * arrow.0 + delta_y * arrow.1
    };

    
    for indx in 0..256 * 256u32{
        let i = indx % 256;
        let j = indx / 256;

        let coord_x = i as f32 / step as f32;
        let coord_y = j as f32 / step as f32;

        let cuad_x = coord_x.floor() as u32;
        let cuad_y = coord_y.floor() as u32;
        let delta_x = coord_x - cuad_x as f32;
        let delta_y = coord_y - cuad_y as f32;

        let ix0 = interpolate(
            gradient(cuad_x + 0, cuad_y + 0, coord_x, coord_y),
            gradient(cuad_x + 1, cuad_y + 0, coord_x, coord_y),
            delta_x,
        );

        let ix1 = interpolate(
            gradient(cuad_x + 0, cuad_y + 1, coord_x, coord_y),
            gradient(cuad_x + 1, cuad_y + 1, coord_x, coord_y),
            delta_x,
        );

        let res = interpolate(ix0, ix1, delta_y);
        let res = (res + 1.) / 2.0;
        image.get_pixel_mut(i, j).0[0] = (res * 255.) as u8;
    }

    image.save("angle_test.png").unwrap();

}
