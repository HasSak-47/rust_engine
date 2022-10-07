use crate::lily::generator::{
    base::{self, Seed64},
    random::*,
};

pub fn bubble_gens_2d(x: &f64, y: &f64, seed: &Seed64) -> f64{
    let xi = x.floor() as i64;
    let yi = y.floor() as i64;

    let mut min = 5.0;
    for i in 0..9{
        let qx = xi + (i % 3) - 1;
        let qy = yi + (i / 3) - 1;

        let px = f64::rands(&(seed + (qx & qy) as u64)) + qx as f64; 
        let py = f64::rands(&(seed + (qx ^ qy) as u64)) + qy as f64; 

        let dx = x - px;
        let dy = y - py;
        
        let dist = dx * dx + dy * dy;
        if dist < min{
            min = dist;
        }

    }

    return min;
}


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

#[derive(Default)]
pub struct BubbleGen2D{
    pub seed: Seed64,
}

impl base::Generator<f64, &f64> for BubbleGen2D{
    fn generate_2d(self, x: &f64, y: &f64) -> f64{
        return bubble_gens_2d(&x, &y, &self.seed);
    }

    fn generate_3d(self, x: &f64, y: &f64, z: &f64) -> f64{
        return bubble_gens_3d(&x, &y, &z, &self.seed);
    }

}
