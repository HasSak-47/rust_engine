use super::base::Engine2d;

pub struct BakedMap{
    map: Vec<f32>,
    width: usize,
    height: usize,
    zoom: u32,
}

impl BakedMap{
    pub fn new(width: usize, height: usize, zoom: u32) ->  BakedMap{
        BakedMap { map: Vec::with_capacity(height * width), width, height, zoom }
    }

    pub fn bake<E: Engine2d<f32, f32>>(&mut self, engine: E) -> &mut Self{

        for indx in 0..self.width * self.height{
            let i = indx % self.width;
            let j = indx / self.width;
            
            let x = i as f32 / self.zoom as f32;
            let y = j as f32 / self.zoom as f32;

            self.map[j * self.width + i] = engine.generate(x, y);
        }


        self
    }

    pub fn get(&self, width: usize, height: usize) -> &f32 {
        let width = width % self.width;
        let height = height % self.height;
        &self.map[width + height * self.width]
    }

    pub fn get_unchecked(&self, width: usize, height: usize) -> &f32 {
        &self.map[width + height * self.width]
    }

    pub fn iget(&self, width: isize, height: isize) -> &f32 {
        let width = width.rem_euclid(self.width as isize) as usize;
        let height = height.rem_euclid(self.height as isize) as usize;
        &self.map[width + height * self.width]
    }

    pub fn fget(&self, x: f32, y: f32) -> &f32{
        let i = x * self.zoom as f32;
        let j = y * self.zoom as f32;

       self.iget(i as isize, j as isize)
    } 
}
