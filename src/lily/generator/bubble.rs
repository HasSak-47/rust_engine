use crate::lily::generator::{
    base::{self, Seed64},
    random::*,
};

pub fn bubble_gen_2d(x: &f64, y: &f64, seed: &Seed64) -> f64{
    let xi = x.floor() as i64;
    let yi = y.floor() as i64;

    let mut min = 2.0;
    for i in 0..9{
        let qx = xi + (i % 3) - 1;
        let qy = yi + (i / 3) - 1;

        let px = f64::rands(&(seed + (qx & qy) as u64)) + x; 
        let py = f64::rands(&(seed + (qx ^ qy) as u64)) + y; 

        let dx = x - px;
        let dy = y - py;
        
        let dist = ( dx * dx + dy * dy ).sqrt();
        if dist < min{
            min = dist;
        }

    }

    return min;
}

pub fn bubble_gen_3d(x: &f64, y: &f64, z: &f64, seed: &Seed64) -> f64{
    let xi = x.floor() as i64;
    let yi = y.floor() as i64;
    let zi = y.floor() as i64;

    let mut min = 2.0;
    for i in 0..27{
        let mut _i = i % 9;
        let qx = xi + (_i % 3) - 1;
        let qy = yi + (_i / 3) - 1;
        
        let mut _i = i / 9;
        let qz = zi + (_i % 3) - 1;


        let px = f64::rands(&(seed + (qx & qy) as u64)) + x; 
        let py = f64::rands(&(seed + (qx ^ qy) as u64)) + y; 
        let pz = f64::rands(&(seed + (qx ^ qz) as u64)) + z; 

        let dx = x - px;
        let dy = y - py;
        let dz = z - pz;
        
        let dist = ( dz * dz + dx * dx + dy * dy ).sqrt();
        if dist < min{
            min = dist;
        }

    }

    return min;
}

#[derive(Default)]
pub struct BubbleGen2D{
    pub seed: Seed64,
}

impl base::Generator<f64, f64> for BubbleGen2D{
    fn generate_2d(self, x: f64, y: f64) -> f64{
        return bubble_gen_2d(&x, &y, &self.seed);
    }

    fn generate_3d(self, x: f64, y: f64, z: f64) -> f64{
        return bubble_gen_3d(&x, &y, &z, &self.seed);
    }

}
