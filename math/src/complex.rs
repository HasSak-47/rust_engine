use super::vertex::{Ver2d, Unit};
use std::ops::{
    Add, Sub, Mul, Div, Neg
};

#[derive(Debug, Default, Clone, Copy)]
pub struct Complex<T>
where
    T: Default + Unit + Clone + Copy + Eq + PartialEq + Add + Sub + Mul + Div + Neg,
{
    v: Ver2d<T>,
}

impl<T> Complex<T>
where
    T: Default + Unit + Clone + Copy + Eq + PartialEq + Add + Sub + Mul + Div + Neg,
{

    const COMMON : [Complex<T>; 4]= [
        Self::new(T::UNIT, T::ZERO),
        Self::new(T::ZERO, T::UNIT),
        Self::new(T::NEGU, T::ZERO),
        Self::new(T::ZERO, T::NEGU),
    ];

    pub const fn r(&self) -> T{self.v.data[0].clone()}
    pub const fn i(&self) -> T{self.v.data[1].clone()}

    pub const fn new(r: T, i: T) -> Complex<T>{
        Complex {v: Ver2d::<T>{data: [r, i]}}
    }

    pub fn abs(&self) -> T{
        ((self.r() * self.r()) +
         (self.i() * self.i())).abs()
    }

    //pub fn inv(&mut self) -> &mut Self{
    //    let a = T::UNIT / (*self.r() * *self.r() + *self.i() * *self.i());

    //    *self = Complex::new(*self.r() * a, -*self.i() * a);
    //    self
    //}
}

// impl Add<Complex> for Complex{
//     type Output = Complex;
// 
//     fn add(self, rhs: Complex) -> Self::Output {
//         Complex{
//             r: self.r + rhs.r,
//             i: self.i + rhs.i,
//         }
//     }
// }
// 
// impl Sub<Complex> for Complex{
//     type Output = Complex;
// 
//     fn sub(self, rhs: Complex) -> Self::Output {
//         Complex{
//             r: self.r - rhs.r,
//             i: self.i - rhs.i,
//         }
//     }
// }
// 
// impl Mul<Complex> for Complex{
//     type Output = Complex;
// 
//     fn mul(self, rhs: Complex) -> Self::Output {
//         let r = self.r * rhs.r - self.i * rhs.i ;
//         let i = self.r * rhs.i + self.i * rhs.r ;
//         Complex{ r, i }
//     }
// }
// 
// // I am not going to bother putting this behind a macro lmao
// impl Mul<f64> for Complex{
//     type Output = Complex;
// 
//     fn mul(self, rhs: f64) -> Self::Output {
//         Complex{
//             r: self.r * rhs,
//             i: self.i * rhs,
//         }
//     }
// }
// 
// impl Mul<Complex> for f64{
//     type Output = Complex;
// 
//     fn mul(self, rhs: Complex) -> Self::Output {
//         Complex{
//             r: rhs.r * self,
//             i: rhs.i * self,
//         }
//     }
// }
// 
// impl Div<Complex> for Complex{
//     type Output = Complex;
// 
//     fn div(self, mut rhs: Complex) -> Self::Output {
//         self * *rhs.inv()
//     }
// }
// 
// impl Div<f64> for Complex{
//     type Output = Complex;
// 
//     fn div(self, rhs: f64) -> Self::Output {
//         Complex{
//             r: self.r / rhs,
//             i: self.i / rhs,
//         }
//     }
// }
// 
// impl Div<Complex> for f64{
//     type Output = Complex;
// 
//     fn div(self, mut rhs: Complex) -> Self::Output {
//         self * *rhs.inv()
//     }
// }
