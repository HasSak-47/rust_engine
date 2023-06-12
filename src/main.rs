use std::{f32::{INFINITY, NEG_INFINITY}, time::UNIX_EPOCH};

use image::GrayImage;
use math::{rand_noise::{
    perlin::PerlinWrapping,
    bubble::WorleyWrapping,
    base::Engine2d, base::inter_gen,
}, image::filter3x3};

struct FloatMap{
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl FloatMap {
    fn new(width: usize, height: usize) -> Self{ Self { data: vec![0.; width * height], width, height } } fn get(&self, x: usize, y: usize) -> &f32{ &self.data[x + y * self.width] }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut f32{ &mut self.data[x + y * self.width] }

    fn normalize(&mut self){
        let mut min = INFINITY;
        let mut max = NEG_INFINITY;

        for i in 0..self.width * self.height{
            let d = self.data[i]; 

            if d < min { min = d; }
            if d > max { max = d; }
        }
        for i in 0..self.width * self.height{
            self.data[i] = (self.data[i] - min) / (max - min); 
        }
    }
}

impl Into<GrayImage> for FloatMap{
    fn into(self) -> GrayImage {
        let mut out = GrayImage::new(self.width as u32, self.height as u32);
        for indx in 0..RESO*RESO{
            let (i, j) = (indx % self.width, indx / self.width);
            out.get_pixel_mut(i as u32, j as u32).0[0] = (self.get(i, j) * 255.) as u8;
        }
        out
    }

}

const RESO: usize = 512;
const STEP: usize =  16;
const ZOOM: f32   =  64.;

#[allow(unused_variables)]
fn main(){
    let mut height_map = FloatMap::new(RESO, RESO);
    let mut   step_map = FloatMap::new(RESO, RESO);
    let seed = 0; //std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
    let perlin = PerlinWrapping::init(4, 4, seed);
    let worley = WorleyWrapping::init(8, 8, seed);
    for indx in 0..RESO*RESO{
        let (i, j) = (indx % RESO, indx / RESO);
        let (x, y) = ( i as f32 / ZOOM, j as f32 / ZOOM);

        let h = perlin.generate(x, y);

        *height_map.get_mut(i, j) = h;
    }

    height_map.normalize();

    /*
    let heighter = |x: f32| {
        if x < 0.5
            {2. * x * x}
        else
            {-2. * (x - 1.) * (x - 1.) + 1.}
    };
    */

    let heighter = |x| x;

    for indx in 0..RESO*RESO{
        let (i, j) = (indx % RESO, indx / RESO);
        let step = (height_map.get(i, j) * STEP as f32) % 1.;
        *step_map.get_mut(i, j) = heighter(step);
    }

    let height_image: GrayImage = height_map.into();
    let   step_image: GrayImage =   step_map.into();

    height_image.save("test/height.png").unwrap();
      step_image.save("test/step.png").unwrap();

    let clamper = |a: f32, b : f32, c: f32, d: f32| (
        a.abs() as u8, b.abs() as u8,
        c.abs() as u8, d.abs() as u8,
    );

    let horizontal_border_image = filter3x3(&height_image, &[
        -1.0, -2.0, -1.0,
         0.0,  0.0,  0.0,
         1.0,  2.0,  1.0,
    ], clamper);
    
    let   vertical_border_image = filter3x3(&height_image, &[
        -1.0,  0.0,  1.0,
        -2.0,  0.0,  2.0,
        -1.0,  0.0,  1.0,
    ], clamper );

    let mut border_image = GrayImage::new(RESO as u32, RESO as u32);
        
    for indx in 0..RESO*RESO{
        let (i, j) = ((indx % RESO) as u32, (indx / RESO) as u32);

        border_image.get_pixel_mut(i, j).0[0] = if step_image.get_pixel(i, j).0[0] >= 200 { 255 } else { 0 }
    }

    border_image.save("test/border.png").unwrap();
}
