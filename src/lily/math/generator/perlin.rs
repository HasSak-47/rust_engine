use crate::lily::{
    math::generator::base,
    bindings::c_perlin,
};

pub use crate::lily::math::generator::base::*;

pub struct BasePerlinGen(pub BaseGen);

impl base::Generator<f64, f64> for BasePerlinGen {
    fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        unsafe{ c_perlin::stb_perlin_noise3_seed(
                x as f32,
                y as f32,
                z as f32,
                self.0.x_wrap as i32,
                self.0.y_wrap as i32,
                self.0.z_wrap as i32,
                self.0.seed as i32
            ) as f64
        }
    }

    fn generate_2d(&self, x: f64, y: f64) -> f64 {
        self.generate_3d(x, y, 0.0)
    }
}

pub struct PerlinGen{
    pub gen: BasePerlinGen,
    pub iter: usize,
    pub scale: f64,
}

impl PerlinGen{
    pub fn new() -> Self{
        PerlinGen{
            gen: BasePerlinGen{
                0 : BaseGen{
                    seed: 0,
                    x_wrap: 0,
                    y_wrap: 0,
                    z_wrap: 0,

                    min: 0.0,
                    max: 0.0,
                }
            },
            iter: 4,
            scale: 2.0,
        }
    }
}

impl base::Generator<f64, f64> for PerlinGen{
    fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut r = 0.0;
        for i in 0..self.iter{
            let d = self.scale.powi(i as i32);
            r += (1.0 / d) * self.gen.generate_3d(x * d, y * d, z * d)
        }
        r / (1.0 + 1.0 / ((self.iter - 1) as f64))
    }

    fn generate_2d(&self, x: f64, y: f64) -> f64 {
        self.generate_3d(x, y, 0.0)
    }

}
