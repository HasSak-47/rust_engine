pub type Seed64 = u64;

pub struct BaseGen{
    pub seed: Seed64,
    pub x_wrap: u64,
    pub y_wrap: u64,
    pub z_wrap: u64,

    pub min: f64,
    pub max: f64,
}

pub trait Generator<ReturnT, InT> {
    fn generate_3d(&self, x: InT, y: InT, z: InT) -> ReturnT;
    fn generate_2d(&self, x: InT, y: InT) -> ReturnT;
}
