use std::ops;

#[derive(Default, Copy, Clone)]
pub struct Complex<T: Default + Copy>{
    pub real: T,
    pub img:  T,
}

impl<T: Default + Copy> ops::Add<Complex<T>> for Complex<T>{
    type Output = Complex<T>;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        Self::Output::default()
    }
}
