use super::brute_impl::{impl_op, impl_ass};
use std::ops::{
    Add, Sub, Mul, Div,
    Neg,

    AddAssign, SubAssign, MulAssign, DivAssign,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Quaternion<T>{
    pub t: [T; 4],
}

//generic stuff
impl<T> Quaternion<T> {
    pub const fn new(r: T, i: T, j: T, k: T) -> Self{
        Quaternion::<T>{t:[ r, i, j ,k]}
    }
}

// math stuff
impl<T> Quaternion<T>
where
    T: Mul<T, Output = T> +
       Div<T, Output = T> +
       Add<T, Output = T> +
       Neg<   Output = T> +
       Copy + Clone
{

    pub fn inv(self) -> Self{
        let numerator = 
            self.t[0] * self.t[0] +
            self.t[1] * self.t[1] +
            self.t[2] * self.t[2] +
            self.t[3] * self.t[3];

        Quaternion::new(
             self.t[0] / numerator,
            -self.t[1] / numerator,
            -self.t[2] / numerator,
            -self.t[3] / numerator,
        )
    }

    pub fn to_mat4(&self) -> [[T; 4]; 4]{
        [
            [ self.t[0],  self.t[3], -self.t[1], -self.t[2]],
            [-self.t[3],  self.t[0],  self.t[2], -self.t[1]],
            [ self.t[1], -self.t[2],  self.t[0], -self.t[3]],
            [ self.t[2],  self.t[1],  self.t[3],  self.t[0]],
        ]
    }
}

impl_op!(Add, add, Quaternion<T>, Quaternion<T>, Quaternion<T>, {Quaternion::<T>::new(
    self.t[0] + rhs.t[0], self.t[1] + rhs.t[1], self.t[2] + rhs.t[2], self.t[3] + rhs.t[3],
)}, self, rhs);

impl_op!(Sub, sub, Quaternion<T>, Quaternion<T>, Quaternion<T>, {Quaternion::<T>::new(
    self.t[0] - rhs.t[0], self.t[1] - rhs.t[1], self.t[2] - rhs.t[2], self.t[3] - rhs.t[3],
)}, self, rhs);

impl_op!(Mul, mul, Quaternion<T>, Quaternion<T>, Quaternion<T>, {
    Quaternion::new(
        self.t[0] * rhs.t[0] - self.t[1] * rhs.t[1] - self.t[2] * rhs.t[2] - self.t[3] * rhs.t[3],
        self.t[0] * rhs.t[1] + self.t[1] * rhs.t[0] + self.t[2] * rhs.t[3] - self.t[3] * rhs.t[2],
        self.t[0] * rhs.t[2] + self.t[2] * rhs.t[0] + self.t[3] * rhs.t[1] - self.t[1] * rhs.t[3],
        self.t[0] * rhs.t[3] + self.t[3] * rhs.t[0] + self.t[1] * rhs.t[2] - self.t[2] * rhs.t[1],
    )
}, self, rhs);

impl_op!(Div, div, Quaternion<T>, Quaternion<T>, Quaternion<T>, {self * rhs.inv()}, self, rhs);

impl_op!(Add, add, &mut Quaternion<T>, Quaternion<T>, Quaternion<T>, { *self + rhs }, self, rhs);
impl_op!(Sub, sub, &mut Quaternion<T>, Quaternion<T>, Quaternion<T>, { *self - rhs }, self, rhs);
impl_op!(Mul, mul, &mut Quaternion<T>, Quaternion<T>, Quaternion<T>, { *self * rhs }, self, rhs);
impl_op!(Div, div, &mut Quaternion<T>, Quaternion<T>, Quaternion<T>, { *self / rhs }, self, rhs);
 
impl_op!(Add, add, &Quaternion<T>, Quaternion<T>, Quaternion<T>, { *self + rhs }, self, rhs);
impl_op!(Sub, sub, &Quaternion<T>, Quaternion<T>, Quaternion<T>, { *self - rhs }, self, rhs);
impl_op!(Mul, mul, &Quaternion<T>, Quaternion<T>, Quaternion<T>, { *self * rhs }, self, rhs);
impl_op!(Div, div, &Quaternion<T>, Quaternion<T>, Quaternion<T>, { *self / rhs }, self, rhs);

impl_ass!(AddAssign, add_assign, Quaternion<T>, Quaternion<T>, { *self = *self + rhs; }, self , rhs);
impl_ass!(SubAssign, sub_assign, Quaternion<T>, Quaternion<T>, { *self = *self - rhs; }, self , rhs);
impl_ass!(MulAssign, mul_assign, Quaternion<T>, Quaternion<T>, { *self = *self * rhs; }, self , rhs);
impl_ass!(DivAssign, div_assign, Quaternion<T>, Quaternion<T>, { *self = *self / rhs; }, self , rhs);

#[cfg(test)]
mod test{
    use super::Quaternion;

    #[test]
    fn test_mul() {
        let mut a : Quaternion<f64> = Quaternion::new(1., 2., 3., 4.);
        let     b : Quaternion<f64> = Quaternion::new(4., 3., 2., 1.);
        let r : Quaternion<f64> = Quaternion::new(-12., 6., 24., 12.);

        assert_eq!(a * b, r);
        assert_eq!(&a * b, r);
        assert_eq!(&mut a * b, r);
    }
}
