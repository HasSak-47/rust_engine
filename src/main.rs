use std::f32::{INFINITY, NEG_INFINITY};

struct FloatMap{
    data: Vec<f32>,
    width: usize,
    height: usize,
}

impl FloatMap {
    fn new(width: usize, height: usize) -> Self{ Self { data: Vec::with_capacity(width * height), width, height } }
    fn get(&self, x: usize, y: usize) -> &f32{ &self.data[x + y * self.width] }
    fn get_mut(&mut self, x: usize, y: usize) -> &mut f32{ &mut self.data[x + y * self.width] }

    fn normalize(&mut self){
        let mut min = INFINITY;
        let mut max = NEG_INFINITY;

        for i in 0..self.width * self.height{
            let d = self.data[i]; 

            if d < min { min = d; }
            if d > max { max = d; }
        }
        for i in 0..self.width * self.height{
            self.data[i] = (self.data[i] - min) / (max - min); 
        }
    }
}

const RESO: usize = 512;
const STEP: usize = 16;

fn main(){
    let mut height_map = FloatMap::new(RESO, RESO);
    // let perlin = Wra::new();
    for indx in 0..RESO*RESO{
        let i = indx % RESO;
        let j = indx / RESO;


    }
}
