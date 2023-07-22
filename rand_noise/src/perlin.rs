use std::{f32::consts::PI, num::Wrapping};

use super::{
    base::{Grid2d, Engine2d},
    random::rands_range,
};

pub struct PerlinWrapping{
   pub grid: Grid2d<(f32, f32)>, 
}

impl PerlinWrapping{
    fn fuck_number(a: u64, b: u64) -> u64{
        let a = Wrapping(a);
        let b = Wrapping(b);
        let mut r = (a * b) + a + b;

        r.0
    }

    pub const fn new(x: usize, y: usize, seed: u64) -> Self{
        PerlinWrapping {grid: Grid2d::new(x, y, seed)}
    }

    pub fn init(&mut self) {

        let x = self.grid.x.clone();

        let random = |xr: usize, yr: usize, seed: u64| -> (f32, f32) {
            let (xr, yr) = (xr as u64, yr as u64);
            let x = x as u64;
            let angle = rands_range(0., 2. * PI, (xr + yr * x) * (1 + seed));

            angle.sin_cos()
        };


        self.grid.init(random);
        //PerlinWrapping {grid: Grid2d::init(seed, random)}
    }
}

impl Engine2d<f32, f32> for PerlinWrapping{

    fn generate(&self, x: f32, y: f32) -> f32 {
        let interpolate = |a: f32, b: f32, w: f32| (b - a) * (3.0 - w * 2.0) * w * w + a;
        let gradient    = |coord_x : isize, coord_y: isize, x: f32, y: f32| {
            let delta_x = x - coord_x as f32;
            let delta_y = y - coord_y as f32;
            
            let arrow = self.grid.iget(coord_x, coord_y);
            delta_x * arrow.0 + delta_y * arrow.1
        };

        let cuad_x  = x.floor() as isize;
        let cuad_y  = y.floor() as isize;
        let delta_x = x - cuad_x as f32;
        let delta_y = y - cuad_y as f32;

        let ix0 = interpolate(
            gradient(cuad_x + 0, cuad_y + 0, x, y),
            gradient(cuad_x + 1, cuad_y + 0, x, y),
            delta_x,
        );

        let ix1 = interpolate(
            gradient(cuad_x + 0, cuad_y + 1, x, y),
            gradient(cuad_x + 1, cuad_y + 1, x, y),
            delta_x,
        );

        interpolate(ix0, ix1, delta_y)
    }
}

