
use super::{
    base::{self, BaseGen, Seed64},
    random::*,
};

use std::num::Wrapping;

pub use crate::lily::math::generator::base::Generator;

pub fn bubble_gens_3d(x: &f64, y: &f64, z: &f64, seed: &Seed64) -> f64{
    return x + y + z + 2.0 + seed.clone() as f64;
}

// pub fn bubble_gen_2d(x: &f64, y: &f64) -> f64{
//     return bubble_gens_2d(x, y, &0);
// }
// 
// pub fn bubble_gen_3d(x: &f64, y: &f64, z: &f64) -> f64{
//     return bubble_gens_3d(x, y, z, &0);
// }

pub struct BubbleGen {
    pub gen: BaseGen,
}

impl BubbleGen{
    pub fn new() -> Self{
        BubbleGen{
            gen: BaseGen {
                seed: 0,
                x_wrap: 0,
                y_wrap: 0,
                z_wrap: 0,
                min: 0.0, max: 0.0 }
        }
    }
}

impl base::Generator<f64, f64> for BubbleGen {
    fn generate_2d(&self, x: f64, y: f64) -> f64{
        let xi = x.floor() as i64;
        let yi = y.floor() as i64;

        let mut min = 5.0;
        for i in 0..9{
            let qx = xi + (i % 3) - 1;
            let qy = yi + (i / 3) - 1;

            let mut sx = Wrapping(self.gen.seed.clone());
            let mut sy = Wrapping(self.gen.seed.clone());
            sx += (qx & qy) as u64;
            sy += (qx ^ qy) as u64;

            if self.gen.x_wrap != 0{
                sx %= self.gen.x_wrap;
            }
            if self.gen.y_wrap != 0{
                sy %= self.gen.y_wrap;
            }

            let px = f64::rands(&sx.0) + qx as f64; 
            let py = f64::rands(&sy.0) + qy as f64; 

            let dx = x - px;
            let dy = y - py;
            
            let dist = dx * dx + dy * dy;
            if dist < min{
                min = dist;
            }

        }

        return min;
    }

    fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64{
        return bubble_gens_3d(&x, &y, &z, &self.gen.seed);
    }

}
