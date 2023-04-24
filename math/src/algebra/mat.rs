use super::essentials::{Unit, Zero, NegU};
use std::ops::{
	Add, Sub, Mul, Div,
	AddAssign, SubAssign, MulAssign, DivAssign,
	Neg,
};

#[repr(C)]
#[allow(dead_code)]
#[derive(Debug)]
pub struct Mat<T, const WIDTH: usize, const HEIGHT: usize> {
	data: [[T;WIDTH];HEIGHT],
}

macro_rules! impl_identity {
	($ident: tt, $ident_f: tt, $func: block) => {
		impl<T, const WIDTH: usize, const HEIGHT: usize> Mat<T, WIDTH, HEIGHT>
		where
			T:  Zero + $ident + Copy
		{
			pub const fn $ident_f() -> Self{
				$func
			}
		}
	};
}

impl_identity!(Zero, zero, {Self{data:[[T::ZERO ;WIDTH];HEIGHT]}});
impl_identity!(Unit, unit, {
	let mut a = Self::zero();
	let mut i = 0; let mut j = 0;
	while i < WIDTH{
		while j < HEIGHT{
			a.data[i][j] = T::UNIT;
			j += 1;
		}
		i += 1;
	}
	a
});

impl_identity!(NegU, negu, {
	let mut a = Self::zero();
	let mut i = 0; let mut j = 0;
	while i < WIDTH{
		while j < HEIGHT{
			a.data[i][j] = T::NEGU;
			j += 1;
		}
		i += 1;
	}
	a
});

impl<T, const WIDTH: usize, const HEIGHT: usize> Zero for Mat<T, WIDTH, HEIGHT>
where T: Zero + Copy{ const ZERO: Self = Self::zero();}

impl<T, const WIDTH: usize, const HEIGHT: usize> Unit for Mat<T, WIDTH, HEIGHT>
where T: Unit + Zero + Copy{ const UNIT: Self = Self::unit();}

impl<T, const WIDTH: usize, const HEIGHT: usize> NegU for Mat<T, WIDTH, HEIGHT>
where T: NegU + Zero + Copy{ const NEGU: Self = Self::negu();}

macro_rules! impl_mat_op {
	($op: tt, $op_n: tt, $for_type: ty, $o_type: ty, $out_type: ty, $oper: block, $self: tt, $rhs: tt, $i: tt, $j: tt) => {
		impl<T, const N: usize> $op<$o_type> for $for_type
		where
			T:  Add<T, Output = T> +
				Sub<T, Output = T> +
				Mul<T, Output = T> +
				Div<T, Output = T> +
				Neg<   Output = T> +
				Zero +
				Copy + Clone
		{
			type Output = $out_type;
			fn $op_n($self, $rhs: $o_type) -> $out_type {
				let mut $i = 0; let mut $j = 0;
				let mut z = Self::zero();
				while $i < N{
					while $j < N{
						z.data[$i][$j] = $oper;
						$j += 1;
					}
					$i += 1;
				}
				$self
			}
		}
	}
}

impl_mat_op!(Add, add, Mat<T, N, N>, Mat<T, N, N>, Mat<T, N, N>, {
	self.data[i][j] + rhs.data[i][j]
}, self, rhs, i, j);

impl_mat_op!(Sub, sub, Mat<T, N, N>, Mat<T, N, N>, Mat<T, N, N>, {
	self.data[i][j] - rhs.data[i][j]
}, self, rhs, i, j);

impl_mat_op!(Mul, mul, Mat<T, N, N>, Mat<T, N, N>, Mat<T, N, N>, {
	let mut a = T::ZERO;
	let mut n = 0;
	while n < N{
		a = a + self.data[i][n] * rhs.data[n][j];
		n += 1;
	}
	a
}, self, rhs, i, j);

