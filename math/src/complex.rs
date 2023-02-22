use std::ops::{
    Add, Sub, Mul, Div
};

#[derive(Debug, Default, Copy, Clone)]
pub struct Complex{
    pub r: f64,
    pub i: f64,
}

pub const COMMON : [Complex; 4] = [
    Complex::new( 1.0,  0.0),
    Complex::new( 0.0,  1.0),
    Complex::new(-1.0,  0.0),
    Complex::new( 0.0, -1.0),
];

impl Complex{
    pub const fn new(r: f64, i: f64) -> Complex{
        Complex{r, i}
    }

    pub fn abs(&self) -> f64{
        ((self.r * self.r) +
         (self.i * self.i)).abs()
    }

    pub fn inv(&mut self) -> &mut Self{
        let a = 1.0 / (self.r * self.r + self.i * self.i);

        *self = Complex::new(self.r * a, -self.i * a);
        self
    }
}

impl Add<Complex> for Complex{
    type Output = Complex;

    fn add(self, rhs: Complex) -> Self::Output {
        Complex{
            r: self.r + rhs.r,
            i: self.i + rhs.i,
        }
    }
}

impl Sub<Complex> for Complex{
    type Output = Complex;

    fn sub(self, rhs: Complex) -> Self::Output {
        Complex{
            r: self.r - rhs.r,
            i: self.i - rhs.i,
        }
    }
}

impl Mul<Complex> for Complex{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        let r = self.r * rhs.r - self.i * rhs.i ;
        let i = self.r * rhs.i + self.i * rhs.r ;
        Complex{ r, i }
    }
}

// I am not going to bother putting this behind a macro lmao
impl Mul<f64> for Complex{
    type Output = Complex;

    fn mul(self, rhs: f64) -> Self::Output {
        Complex{
            r: self.r * rhs,
            i: self.i * rhs,
        }
    }
}

impl Mul<Complex> for f64{
    type Output = Complex;

    fn mul(self, rhs: Complex) -> Self::Output {
        Complex{
            r: rhs.r * self,
            i: rhs.i * self,
        }
    }
}

impl Div<Complex> for Complex{
    type Output = Complex;

    fn div(self, mut rhs: Complex) -> Self::Output {
        self * *rhs.inv()
    }
}

impl Div<f64> for Complex{
    type Output = Complex;

    fn div(self, rhs: f64) -> Self::Output {
        Complex{
            r: self.r / rhs,
            i: self.i / rhs,
        }
    }
}

impl Div<Complex> for f64{
    type Output = Complex;

    fn div(self, mut rhs: Complex) -> Self::Output {
        self * *rhs.inv()
    }
}
