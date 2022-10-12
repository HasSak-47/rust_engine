use crate::lily::{
    math::generator::base::{self, BaseGen},
    bindings::c_perlin,
};

pub struct PerlinGen{
    pub gen: BaseGen,
}

impl base::Generator<f64, f64> for PerlinGen {
    fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        unsafe{ c_perlin::stb_perlin_noise3_seed(
                x as f32,
                y as f32,
                z as f32,
                self.gen.x_wrap as i32,
                self.gen.y_wrap as i32,
                self.gen.z_wrap as i32,
                self.gen.seed as i32
            ) as f64
        }
    }

    fn generate_2d(&self, x: f64, y: f64) -> f64 {
        self.generate_3d(x, y, 0.0)
    }
}
