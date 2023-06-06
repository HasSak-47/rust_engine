#[repr(C)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub struct Container<Type, const M: usize, const N: usize>(pub [[Type;N]; M]);



#[allow(dead_code)]
impl<Type: Clone , const M: usize, const N: usize> Container<Type, M, N> {
    pub const fn new(d: [[Type; N]; M]) -> Self{
        Container(d)
    }
}

use std::default::Default; #[allow(dead_code)]
impl<Type, const M: usize, const N: usize> Default for Container<Type, M, N>
where
    Type: Default + Clone + Copy
{
    fn default() -> Self {
        Container([[Type::default(); N]; M])
    }
}

impl<Type, const M: usize, const N: usize> std::fmt::Display for Container<Type, M, N>
where
    Type: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for a in &self.0{
            for b in a{
                write!(f, "{b} ")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

use std::ops::{Add, Sub, Mul};

macro_rules! impl_add{
	($oper: tt, $oper_func: tt, $body: block, $SELF: ty, $RHS: ty, $OUT: ty, $self: tt, $rhs: tt) => {
		impl<T, const M: usize, const N: usize> $oper<$RHS> for $SELF where
			T: $oper<T, Output = T> + Copy,
		{
			type Output = $OUT;
			fn $oper_func($self, $rhs: $RHS) -> Self::Output { $body }
		}
	}
}

impl_add!( Add, add, {
    let mut result : Self::Output = unsafe {std::mem::zeroed()};
    for i in 0..M{
        for j in 0..N{
            result.0[i][j] = self.0[i][j] + rhs.0[i][j];
        }
    }
    result
    },
&Container<T, M, N>, &Container<T, M, N>, Container<T, M, N>, self, rhs);

impl_add!( Sub, sub, {
    let mut result : Self::Output = unsafe {std::mem::zeroed()};
    for i in 0..M{
        for j in 0..N{
            result.0[i][j] = self.0[i][j] - rhs.0[i][j];
        }
    }
    result
    },
&Container<T, M, N> , &Container<T, M, N>, Container<T, M, N>, self, rhs);

impl_add!(Add, add, {let a = &*self + &*rhs; a}, &mut Container<T, M, N>,  &mut Container<T, M, N>,  Container<T, M, N> ,self, rhs);
impl_add!(Add, add, {let a = &*self + &rhs; a} , &mut Container<T, M, N>,  Container<T, M, N>,  Container<T, M, N> ,self, rhs);
impl_add!(Add, add, {let a = &self  + &*rhs; a}, Container<T, M, N>,  &mut Container<T, M, N>,  Container<T, M, N> ,self, rhs);
impl_add!(Add, add, {let a = &self  + &rhs; a} , Container<T, M, N>,  Container<T, M, N>,  Container<T, M, N> ,self, rhs);

impl_add!(Sub, sub, {let a = &*self - &*rhs; a}, &mut Container<T, M, N>,  &mut Container<T, M, N>,  Container<T, M, N> ,self, rhs);
impl_add!(Sub, sub, {let a = &*self - &rhs; a} , &mut Container<T, M, N>,  Container<T, M, N>,  Container<T, M, N> ,self, rhs);
impl_add!(Sub, sub, {let a = &self  - &*rhs; a}, Container<T, M, N>,  &mut Container<T, M, N>,  Container<T, M, N> ,self, rhs);
impl_add!(Sub, sub, {let a = &self  - &rhs; a} , Container<T, M, N>,  Container<T, M, N>,  Container<T, M, N> ,self, rhs);


macro_rules! impl_mul{
	($body: block, $SELF: ty, $RHS: ty, $OUT: ty, $self: tt, $rhs: tt) => {
		impl<T, const M: usize, const N: usize, const P: usize> Mul<$RHS> for $SELF where
			T: Add<T, Output = T> + Mul<T, Output = T> + Copy,
		{
			type Output = $OUT;
			fn mul($self, $rhs: $RHS) -> Self::Output { $body }
		}
	}
}

impl_mul!({
       let mut out : Self::Output = unsafe {std::mem::zeroed()};

       // O(n^3) goes brrrrrr
       for i in 0..M{
           for j in 0..P{
               out.0[i][j] = unsafe {std::mem::zeroed()};
               for k in 0..N{
                   out.0[i][j] = out.0[i][j] + self.0[i][k] * rhs.0[k][j];
               }
           }
       }

       out
}, &Container<T, M, N>, &Container<T, N, P>, Container<T, M, P>, self, rhs);

impl_mul!({&*self * &*rhs}, &mut Container<T, M, N>, &mut Container<T, N, P>, Container<T, M, P>, self, rhs);
impl_mul!({&*self * & rhs}, &mut Container<T, M, N>,      Container<T, N, P>, Container<T, M, P>, self, rhs);
impl_mul!({& self * &*rhs},      Container<T, M, N>, &mut Container<T, N, P>, Container<T, M, P>, self, rhs);
impl_mul!({& self * & rhs},      Container<T, M, N>,      Container<T, N, P>, Container<T, M, P>, self, rhs);

macro_rules! impl_ass_add{
	($oper: tt, $oper_func: tt, $body: block, $RHS: ty, $self: tt, $rhs: tt, $($has: tt),*) => {
		impl<T, const M: usize, const N: usize> $oper<$RHS> for Container<T, M, N>
		where
			T: $($has <T,Output = T> + )* Copy + Clone
		{
			fn $oper_func(&mut $self, $rhs: $RHS) { $body }
		}
	}
}

use std::ops::{AddAssign, SubAssign, MulAssign};

impl_ass_add!(AddAssign, add_assign, {*self = &*self +   rhs;} , &   Container<T, M, N>, self, rhs, Add);
impl_ass_add!(AddAssign, add_assign, {*self = &*self + &*rhs;}, &mut Container<T, M, N>, self, rhs, Add);
impl_ass_add!(AddAssign, add_assign, {*self = &*self + & rhs;},      Container<T, M, N>, self, rhs, Add);

impl_ass_add!(SubAssign, sub_assign, {*self = &*self -   rhs;} , &   Container<T, M, N>, self, rhs, Sub);
impl_ass_add!(SubAssign, sub_assign, {*self = &*self - &*rhs;}, &mut Container<T, M, N>, self, rhs, Sub);
impl_ass_add!(SubAssign, sub_assign, {*self = &*self - & rhs;},      Container<T, M, N>, self, rhs, Sub);


macro_rules! impl_ass_mul{
	($body: block, $RHS: ty, $self: tt, $rhs: tt) => {
		impl<T, const M: usize> MulAssign<$RHS> for Container<T, M, M>
		where
			T: Add<T,Output = T> + Mul<T, Output = T> + Copy + Clone
		{
			fn mul_assign(&mut $self, $rhs: $RHS) { $body }
		}
	}
}

impl_ass_mul!({*self = &*self *   rhs;}, &    Container<T, M, M>, self, rhs);
impl_ass_mul!({*self = &*self * &*rhs;}, &mut Container<T, M, M>, self, rhs);
impl_ass_mul!({*self = &*self * & rhs;},      Container<T, M, M>, self, rhs);
