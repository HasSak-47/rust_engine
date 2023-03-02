use std::ops::{Add, Sub, Mul, Div};
use super::{Units, SquareRoot, Absolute, sqrt, abs};

#[derive(Clone, Copy)]
pub struct Complex<T>
where
    T: Copy + Clone
{
    pub d: [T; 2]
}

impl<T> Complex<T>
where 
    T: Units + Copy + Clone {
    const fn new(a: T, b: T) -> Self{
        Complex::<T> {d:[a, b]}
    }
    
    const COMMON : [Complex<T>; 4] = [
        Complex::new(T::UNIT, T::ZERO),
        Complex::new(T::ZERO, T::UNIT),
        Complex::new(T::NEGU, T::ZERO),
        Complex::new(T::ZERO, T::NEGU),
    ];
}

impl<T> From<T> for Complex<T>
where
    T: Units + Clone + Copy
{
    fn from(value: T) -> Self {
        Self::new(value, T::ZERO)
    }
}

impl<T> Mul<Complex<T>> for Complex<T>
where
    T: Add<Output = T>
     + Mul<Output = T>
     + Sub<Output = T>
     + Units + Copy + Clone
{
    type Output = Complex<T>;

    fn mul(self, rhs: Complex<T>) -> Self::Output {
        Complex::<T>::new(
            self.d[0] * rhs.d[0] - self.d[1] * rhs.d[1],
            self.d[0] * rhs.d[1] + self.d[1] * rhs.d[0],
        )
    }
}

impl<T> Add<Complex<T>> for Complex<T>
where
    T: Add<Output = T>
     + Units + Copy + Clone
{
    type Output = Complex<T>;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        Complex::<T>::new(
            self.d[0] + rhs.d[0],
            self.d[1] + rhs.d[1],
        )
    }
}


impl<T> Sub<Complex<T>> for Complex<T>
where
    T: Sub<Output = T>
     + Units + Copy + Clone
{
    type Output = Complex<T>;

    fn sub(self, rhs: Complex<T>) -> Self::Output {
        Complex::<T>::new(
            self.d[0] - rhs.d[0],
            self.d[1] - rhs.d[1],
        )
    }
}

impl<T> Absolute for Complex<T>
where
    T: SquareRoot<Output = T>
     + Mul<Output = T>
     + Units + Copy + Clone,
{
    type Output = T;
    fn __abs(&self) -> T{
        sqrt(self.d[0] * self.d[0])
    }

}

fn unit<T>(c: Complex<T>) -> Complex<T>
where
    T: Add<Output = T>
     + Sub<Output = T>
     + Mul<Output = T>
     + Div<Output = T>
     + Absolute<Output = T>
     + SquareRoot<Output = T>
     + Units + Copy + Clone,
{
    c
} 
