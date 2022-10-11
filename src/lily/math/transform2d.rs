
pub trait Opposite{
    fn opposite(&self) -> Self;
}

pub trait Transform2d{
    fn xmirror(&self) -> Self;
    fn ymirror(&self) -> Self;
    fn rotate(&self, degree: usize) -> Self;
}

#[derive(Clone, Copy)]
struct Vec2d{
    pub x: f64,
    pub y: f64,
}

impl<T: Default + Copy, const SIZE: usize> Transform2d for [[T; SIZE]; SIZE]{
    fn xmirror(&self) -> Self {
        [[T::default(); SIZE]; SIZE]        
    }
    fn ymirror(&self) -> Self {
        [[T::default(); SIZE]; SIZE]        
    }
    fn rotate(&self, degree: usize) -> Self {
        //let offset_matrix = [[Vec2d{x: 0.0, y: 0.0}; SIZE]; SIZE];
        //let offset = (SIZE as f64 - 1.0) / 2.0;

        //for val in 0..SIZE*SIZE{
        //    let i = (val % SIZE);
        //    let x = i as f64 - offset;
        //    let j = (val / SIZE);
        //    let y = j as f64 - offset;

        //    offset_matrix[i][j] = Vec2d{x, y};
        //}
        [[T::default(); SIZE]; SIZE]        
    }
}
