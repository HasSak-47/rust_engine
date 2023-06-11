use std::f32::{INFINITY, NEG_INFINITY};


use image::GrayImage;

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

const RESO: usize = 512;
const STEP: usize = 16;

fn main(){
    let mut height_map = FloatMap::new(RESO, RESO);
    let perlin = Wra::new();
    for indx in 0..RESO*RESO{
        let i = indx % RESO;
        let j = indx / RESO;


    }
}

/*
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
*/
