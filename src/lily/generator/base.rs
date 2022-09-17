
pub type Seed64 = u64;

pub trait Generator<ReturnT, InT> {
    fn generate_3d(self, x: InT, y: InT, z: InT) -> ReturnT;
    fn generate_2d(self, x: InT, y: InT) -> ReturnT;
}
