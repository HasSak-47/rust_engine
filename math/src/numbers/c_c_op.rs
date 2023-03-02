use std::ops::{Add, Sub, Mul, Div};
use super::{Units, SquareRoot, Absolute, sqrt, abs};
use super::complex::Complex;

impl<T> Add<Complex<T>> for Complex<T>
where
    T: Add<Output = T>
     + Units + Copy + Clone
{
    type Output = Complex<T>;

    fn add(self, rhs: Complex<T>) -> Self::Output {
        // fuck off rust
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

impl<T> Div<T> for Complex<T>
where
    T: Add<Output = T>
     + Sub<Output = T>
     + Mul<Output = T>
     + Div<Output = T>
     + Units + Copy + Clone
{
}
