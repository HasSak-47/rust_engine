use std::f32::consts::PI;

use super::{
    base::{Grid2d, Engine2d},
    random::rands_range,
};

pub struct PerlinWrapping{
   pub grid: Grid2d<(f32, f32)>, 
}

impl PerlinWrapping{
    fn random(x: usize, y: usize, seed: u64) -> (f32, f32) {
        let sd = x * x * x + y * x + y;
        let sd = sd as u64;
        let f = rands_range::<f32>(0., 2. * PI, sd * (seed + 1));

        (f.cos(), f.sin())
    }
    pub fn init(x: usize, y: usize, seed: u64) -> Self{
        PerlinWrapping {grid: Grid2d::init(x, y, seed, Self::random)}
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

