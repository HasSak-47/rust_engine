use super::{
    base::{Grid2d, Engine2d},
    random::rands,
};

pub struct WorleyWrapping {
    pub grid: Grid2d<(f32, f32)>,
}

impl WorleyWrapping{
    pub const fn new(x: usize, y: usize, seed: u64) -> Self{
        WorleyWrapping {grid: Grid2d::new(x, y, seed)}
    }

    pub fn init(&mut self) {
        let x = self.grid.x as u64;
        let random = |xr: usize, yr: usize, seed: u64| -> (f32, f32) {
            let (xr, yr) = (xr as u64, yr as u64);
            let x = x as u64;
            let seeda = (xr + seed) + yr * x;
            let seedb = (yr + seed) + xr * x;

            (rands::<f32>(seeda), rands::<f32>(seedb))
        };
        self.grid.init(random);
        //WorleyWrapping { grid: Grid2d::init(x, y, seed, random)}
    }
}

impl Engine2d<f32, f32> for WorleyWrapping{
    fn generate(&self, x: f32, y: f32) -> f32 {
        let x = x % self.grid.x as f32;
        let y = y % self.grid.y as f32;

        let cuad_x = x.floor() as isize; 
        let cuad_y = y.floor() as isize; 

        let points = [
            (-1, -1), (-1,  0), (-1,  1),
            ( 0, -1), ( 0,  0), ( 0,  1),
            ( 1, -1), ( 1,  0), ( 1,  1),
        ];

        let mut min_d = f32::INFINITY;
        for point in points{
            let cuad = (
                (cuad_x + point.0) as f32,
                (cuad_y + point.1) as f32
            );

            let point = self.grid.iget(cuad_x + point.0, cuad_y + point.1);

            let dx = (point.0 + cuad.0) - x;
            let dy = (point.1 + cuad.1) - y;

            let d = (dx * dx + dy * dy).sqrt();
            if d < min_d{
                min_d = d;
            }
        }
        min_d
    }

}
