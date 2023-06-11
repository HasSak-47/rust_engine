
fn main(){}

/*
use std::f32::consts::PI;

use math::rand_noise::{bubble::WorleyWrapping, perlin::PerlinWrapping, base::Engine2d, base::inter_gen};
use image::{GrayImage, RgbImage, Rgb};

const RESO : u32 = 512;
const ZOOM : u32 = 256;
const STEP : u32 = 32;

fn main(){
    let perlin = PerlinWrapping::init(7, 7, 0);
    let worley = WorleyWrapping::init(3, 3, 0);

    let mut height_map = GrayImage::new(RESO, RESO);
    let mut step_map   = GrayImage::new(RESO, RESO);
    let mut max = 0.;
    let mut min = 1.;
    for indx in 0..RESO * RESO{
        let i = indx % RESO;
        let j = indx / RESO;

        let x = i as f32 / (ZOOM as f32);
        let y = j as f32 / (ZOOM as f32);

        let a = worley.generate(x, y) * PI;
        let (x, y)= (
            x, // + 0.1 * a.cos(),
            y // + 0.1 * a.sin()
        );
        let mut height = inter_gen(&perlin, 4, x, y);
        height += 1.;
        height /= 2.;

        // corretion 
        height -= 0.257;
        height /= 0.838 - 0.258;
        let step = (height * STEP as f32) % 1.;

        if height > max{ max = height; }
        if height < min{ min = height; }
        
        height_map.get_pixel_mut(i, j).0[0] = (STEP as f32 * height) as u8;
        step_map.get_pixel_mut(i, j).0[0]   = (255. * step) as u8;
    }

    println!("{min:3.3} {max:3.3}");


    use math::image::filter3x3;

    let vertical_border_kernel = [
        0.5, 0.0, -0.5,
        1.0, 0.0, -1.0,
        0.5, 0.0, -0.5,
    ];

    let horizon_border_kernel = [
         0.5, 1.0,  0.5,
         0.0, 0.0,  0.0,
        -0.5,-1.0, -0.5,
    ];

    let vertical = filter3x3(&mut step_map, &vertical_border_kernel, |r, g, b, a| (r.abs() as u8, g.abs() as u8, b.abs() as u8, a.abs() as u8) );
    let horizon  = filter3x3(&mut step_map, &horizon_border_kernel , |r, g, b, a| (r.abs() as u8, g.abs() as u8, b.abs() as u8, a.abs() as u8) );

    let mut border_map = GrayImage::new(RESO as u32, RESO as u32);
    for indx in 0..(RESO * RESO) as u32{
        let i = indx % RESO;
        let j = indx / RESO;
        let vp = vertical.get_pixel(i, j).0[0] as u32;
        let hp =  horizon.get_pixel(i, j).0[0] as u32;

        let r = if vp + hp > 200 { 0 } else { 255 };

        border_map.get_pixel_mut(i, j).0[0] = r as u8;
    }


    let mut map = RgbImage::new(RESO, RESO);
    let mut color_opts : [Rgb<u8>; (STEP + 1) as usize] = unsafe { std::mem::zeroed() };
    let color_border = Rgb([0, 0, 0u8]);

    for i in 0..STEP{
        color_opts[i as usize].0[0] = i as u8;
        color_opts[i as usize].0[1] = i as u8;
        color_opts[i as usize].0[2] = i as u8;
    }


    for indx in 0..(RESO * RESO) as u32{
        let i = indx % RESO;
        let j = indx / RESO;
    
        let border = border_map.get_pixel(i, j).0[0];
        let height = height_map.get_pixel(i, j).0[0] as u8;


        let color = if border == 0 { color_border } else{ color_opts[height as usize]};
        *map.get_pixel_mut(i, j) = color;
    }
    
      step_map.save("test/step.png").unwrap();
    height_map.save("test/height.png").unwrap();
    border_map.save("test/border.png").unwrap();
    map.save("test/map.png").unwrap();

}
*/
