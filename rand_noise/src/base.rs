use std::fmt::Debug;

pub type Seed64 = u64;

#[derive(Default, Clone, Copy)]
pub struct Parameters{
    pub seed: Seed64,
    pub min: f64,
    pub max: f64,
}


pub struct Grid2d<T>{
    pub data: Vec<T>,

    pub x: usize,
    pub y: usize,
    pub seed: u64,
}

impl<T: Debug> Grid2d<T> {
    pub const fn new(x: usize, y: usize, seed: u64) -> Self{
        Grid2d { data: Vec::new(), x, y, seed }
    }


    pub fn init(&mut self, gen: impl Fn(usize, usize, u64) -> T){
        let x = self.x;
        let y = self.y;
        let _seed = self.seed;

        let total_len = x * y;
        for i in 0..total_len{
            let xi = i % x;
            let yi = i / x;
            let g = gen(xi, yi, self.seed);

            self.data.push(g);
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &T {
        let x = x % self.x;
        let y = y % self.y;
        &self.data[x + y * self.x]
    }

    pub fn get_unchecked(&self, x: usize, y: usize) -> &T {
        &self.data[x + y * self.x]
    }

    pub fn iget(&self, x: isize, y: isize) -> &T {
        let x = x.rem_euclid(self.x as isize) as usize;
        let y = y.rem_euclid(self.y as isize) as usize;
        &self.data[x + y * self.x]
    }

}

pub trait Engine2d<ReturnT, InT> {
    fn generate(&self, x: InT, y: InT) -> ReturnT;
}

pub fn iter_gen<T: Engine2d<f32, f32>>(gen: &T, iters: usize, x: f32, y: f32) -> f32{
    let mut r = 0.;
    let div = (1 << iters) as f32;
    for i in 0..=iters{
        let p = 2f32.powi(i as i32);
        r += gen.generate(x * p , y * p) / 2f32.powi(i as i32);
    }

    r * (2. - 1. / div)
}
