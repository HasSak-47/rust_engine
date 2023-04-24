use super::brute_impl::{impl_op, impl_ass};
use std::ops::{
	Add, Sub, Mul, Div,
	Neg,

	AddAssign, SubAssign, MulAssign, DivAssign,
	Deref
};

use super::vector::Vec2;


#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Complex<T> {
	t : [T; 2],
}

impl<T> Deref for Complex<T>{
	type Target = Vec2<T>;
	fn deref(&self) -> &Self::Target {
		let this_ptr : *const Self = &*self;
		unsafe{ &*(this_ptr as *const Vec2<T>) }
	}

}

#[allow(dead_code)]
impl<T> Complex<T> {
	pub const fn new(x: T, y: T) -> Self{
		Complex { t: [x, y] }
	}
}

#[allow(dead_code)]
impl<T> Complex<T> where
	T:  Copy + Clone +
		Add<Output = T> +
		Neg<Output = T> +
		Mul<Output = T> +
		Div<Output = T>
{
	pub fn inv(&self) -> Self{
		let div = self.t[0] * self.t[0] + self.t[1] * self.t[1] ;

		Self::new(self.t[0] / div, -self.t[1] / div,)
	}


}


// fucking hate this
impl_op!(Add, add, Complex<T>, Complex<T>, Complex<T>, { Complex::new(self.t[0] + rhs.t[0], self.t[1] + rhs.t[1]) }, self, rhs);
impl_op!(Sub, sub, Complex<T>, Complex<T>, Complex<T>, { Complex::new(self.t[0] - rhs.t[0], self.t[1] - rhs.t[1]) }, self, rhs);
impl_op!(Mul, mul, Complex<T>, Complex<T>, Complex<T>, { Complex::new(self.t[0] * rhs.t[0] - self.t[1] * rhs.t[1], self.t[1] * rhs.t[0] + self.t[0] * rhs.t[1],) }, self, rhs);
impl_op!(Div, div, Complex<T>, Complex<T>, Complex<T>, { self * rhs.inv() }, self, rhs);

impl_op!(Add, add, &mut Complex<T>, Complex<T>, Complex<T>, { *self + rhs }, self, rhs);
impl_op!(Sub, sub, &mut Complex<T>, Complex<T>, Complex<T>, { *self - rhs }, self, rhs);
impl_op!(Mul, mul, &mut Complex<T>, Complex<T>, Complex<T>, { *self * rhs }, self, rhs);
impl_op!(Div, div, &mut Complex<T>, Complex<T>, Complex<T>, { *self / rhs }, self, rhs);
 
impl_op!(Add, add, &Complex<T>, Complex<T>, Complex<T>, { *self + rhs }, self, rhs);
impl_op!(Sub, sub, &Complex<T>, Complex<T>, Complex<T>, { *self - rhs }, self, rhs);
impl_op!(Mul, mul, &Complex<T>, Complex<T>, Complex<T>, { *self * rhs }, self, rhs);
impl_op!(Div, div, &Complex<T>, Complex<T>, Complex<T>, { *self / rhs }, self, rhs);

impl_ass!(AddAssign, add_assign, Complex<T>, Complex<T>, { *self = *self + rhs; }, self , rhs);
impl_ass!(SubAssign, sub_assign, Complex<T>, Complex<T>, { *self = *self - rhs; }, self , rhs);
impl_ass!(MulAssign, mul_assign, Complex<T>, Complex<T>, { *self = *self * rhs; }, self , rhs);
impl_ass!(DivAssign, div_assign, Complex<T>, Complex<T>, { *self = *self / rhs; }, self , rhs);

#[cfg(test)]
mod test{
	use super::Complex;

	#[test]
	fn test_mul() {
		let mut a : Complex<f64> = Complex::new(1., 2.);
		let b : Complex<f64> = Complex::new(2., 1.);
		let r : Complex<f64> = Complex::new(0., 5.);

		assert_eq!(a * b, r);
		assert_eq!(&a * b, r);
		assert_eq!(&mut a * b, r);
	}

	#[test]
	fn deref_test(){
		let a : Complex<f64> = Complex::new(1., 2.);
		assert_eq!(a.t[0], a.x);
		assert_eq!(a.t[1], a.y);
	}
}
