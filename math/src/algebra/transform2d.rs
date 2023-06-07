use super::complex::Complex as Cmplx;

type Complex = Cmplx<f32>;

pub trait Opposite{
    fn opposite(&self) -> Self;
}

pub trait Transform2d{
    fn xmirror(&self) -> Self;
    fn ymirror(&self) -> Self;
    fn rotate(&self, degree: usize) -> Self;
}

const COMMON: [Complex; 4] = [
    Complex::new( 1.,  0.),
    Complex::new( 0.,  1.),
    Complex::new(-1.,  0.),
    Complex::new( 0., -1.),
];

impl<T: Default + Copy, const SIZE: usize> Transform2d for [[T; SIZE]; SIZE]{
    fn xmirror(&self) -> Self {
        [[T::default(); SIZE]; SIZE]        
    }
    fn ymirror(&self) -> Self {
        [[T::default(); SIZE]; SIZE]        
    }

    /*
    * this has floating point errors :)
    */
    fn rotate(&self, mut degree: usize) -> Self {
        degree %= 4;
        let offset = (SIZE as f32 - 1.0) / 2.0;
        let mut offset_matrix = [[Complex::new(-offset, -offset); SIZE]; SIZE];
        let mut ret_matrix = [[T::default(); SIZE]; SIZE];

        for val in 0..SIZE*SIZE{
            let i = val % SIZE;
            let j = val / SIZE;
            let x = i as f32;
            let y = j as f32;

            offset_matrix[i][j] = Complex::new(x, y) + offset_matrix[i][j];
            offset_matrix[i][j] = offset_matrix[i][j] * COMMON[degree];
            offset_matrix[i][j] = offset_matrix[i][j] + Complex::new(offset, offset);
            let off = offset_matrix[i][j];
            ret_matrix[i][j] = self[off.r as usize][off.i as usize];
        }


        ret_matrix
    }
}
