pub use super::base::*;

pub struct BasePerlinGen{
    pub gen: BaseGen,
}

impl Generator<f64, f64> for BasePerlinGen {
    fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        0.
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
    #[deprecated]
    pub fn new() -> Self{
        PerlinGen{
            gen: BasePerlinGen{
                gen : BaseGen{
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

impl Generator<f64, f64> for PerlinGen{
    fn generate_3d(&self, x: f64, y: f64, z: f64) -> f64 {
        let mut r = 0.0;
        for i in 0..self.iter{
            let d = self.scale.powi(i as i32);
            r += (1.0 / d) * self.gen.generate_3d(x * d, y * d, z * d)
        }
        r
    }

    fn generate_2d(&self, x: f64, y: f64) -> f64 {
        self.generate_3d(x, y, 0.0)
    }

}
