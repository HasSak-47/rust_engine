use std::f32::{INFINITY, NEG_INFINITY};

use image::GrayImage;
use math::rand_noise::{
    perlin::PerlinWrapping,
    bubble::WorleyWrapping,
    base::Engine2d,
};

struct FloatMap{
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl FloatMap {
    fn new(width: usize, height: usize) -> Self{ Self { data: Vec::with_capacity(width * height), width, height } }
    fn get(&self, x: usize, y: usize) -> &f32{ &self.data[x + y * self.width] }
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
const STEP: usize = 16;
const ZOOM: f32   = 16.;

fn main(){
    let mut height_map = FloatMap::new(RESO, RESO);
    let mut   step_map = FloatMap::new(RESO, RESO);
    let perlin = PerlinWrapping::init(2, 2, 0);
    // let worley = WorleyWrapping::init(1, 1, 0);
    for indx in 0..RESO*RESO{
        let (i, j) = (indx % RESO, indx / RESO);
        let (x, y) = ( i as f32 / ZOOM, j as f32 / ZOOM);

        *height_map.get_mut(i, j) = perlin.generate(x, y);
    }

    height_map.normalize();

    for indx in 0..RESO*RESO{
        let (i, j) = (indx % RESO, indx / RESO);
        *step_map.get_mut(i, j) = (height_map.get(i, j) * STEP as f32) % 1.;
    }

    height_map.into();
    // let height_image: GrayImage = height_map.into();
    // let   step_image: GrayImage =   step_map.into();




}
