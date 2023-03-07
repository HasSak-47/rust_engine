use std::ops::{
    Add, Sub, Mul, Div, // regular arithmetic
    Neg, // times negative one
    AddAssign, SubAssign, MulAssign, DivAssign, // assign arithmetic
};

#[derive(Clone, Copy, Debug)]
pub struct Quaternion<T>{
    pub r: T,
    pub i: T,
    pub j: T,
    pub k: T,
}

impl<T> Quaternion<T>
where
    T: Mul<T, Output = T> +
       Div<T, Output = T> +
       Add<T, Output = T> +
       Neg<   Output = T> +
       Sub<T, Output = T> +
       Copy + Clone
{
    pub const fn new(r: T, i: T, j: T, k: T) -> Self{
        Quaternion::<T>{r, i, j ,k}
    }

    pub const fn abs(self) -> f64{0.}

    pub fn inv(self) -> Self{
        let numerator = 
            self.r * self.r +
            self.i * self.i +
            self.j * self.j +
            self.k * self.k;

        Quaternion::new(
             self.r / numerator,
            -self.i / numerator,
            -self.j / numerator,
            -self.k / numerator,
        )
    }
}

macro_rules! impl_op{
    ($op: tt, $op_n: tt, $type: tt, $oper: block, $self: tt, $rhs: tt) => {
        impl<T> $op<$type<T>> for $type<T>
        where
            T: Add<T, Output = T> +
               Sub<T, Output = T> +
               Mul<T, Output = T> +
               Div<T, Output = T> +
               Neg<   Output = T> +
               Copy + Clone
        {
            type Output = $type<T>;
            fn $op_n($self, $rhs: $type<T>) -> $type<T>{ $oper }
        }
    }
}

impl_op!(Add, add, Quaternion, {
    Quaternion::new(
        self.r + rhs.r,
        self.i + rhs.i,
        self.j + rhs.j,
        self.k + rhs.k,
    )
}, self, rhs);

impl_op!(Sub, sub, Quaternion, {
    Quaternion::new(
        self.r - rhs.r,
        self.i - rhs.i,
        self.j - rhs.j,
        self.k - rhs.k,
    )
}, self, rhs);

impl_op!(Mul, mul, Quaternion, {
    Quaternion::new(
        // real part
        self.r * rhs.r - 
        self.i * rhs.i - 
        self.j * rhs.j - 
        self.k * rhs.k,

        // i part
        self.r * rhs.i + 
        self.i * rhs.r + 
        self.j * rhs.k - 
        self.k * rhs.j,

        // j part
        self.r * rhs.j + 
        self.j * rhs.r + 
        self.k * rhs.i - 
        self.i * rhs.k,

        // k part
        self.r * rhs.k + 
        self.k * rhs.r + 
        self.i * rhs.j - 
        self.j * rhs.i,
    )
}, self, rhs);


// finish divition bc it is just multiplication
impl_op!(Div, div, Quaternion, {
    Quaternion::new(
        // real part
        self.r * rhs.r - 
        self.i * rhs.i - 
        self.j * rhs.j - 
        self.k * rhs.k,

        // i part
        self.r * rhs.i + 
        self.i * rhs.j + 
        self.j * rhs.k - 
        self.k * rhs.j,

        // j part
        self.r * rhs.j + 
        self.j * rhs.r + 
        self.k * rhs.i - 
        self.i * rhs.k,

        // k part
        self.r * rhs.k + 
        self.k * rhs.r + 
        self.i * rhs.j - 
        self.j * rhs.i,
    )
}, self, rhs);

impl<T> AddAssign<Quaternion<T>> for Quaternion<T>
where
    T: Add<T, Output = T> +
       Sub<T, Output = T> +
       Mul<T, Output = T> +
       Div<T, Output = T> +
       Neg<   Output = T> +
       Copy + Clone
{fn add_assign(&mut self, rhs: Quaternion<T>) {*self = *self + rhs;}}

impl<T> SubAssign<Quaternion<T>> for Quaternion<T>
where
    T: Add<T, Output = T> +
       Sub<T, Output = T> +
       Mul<T, Output = T> +
       Div<T, Output = T> +
       Neg<   Output = T> +
       Copy + Clone
{fn sub_assign(&mut self, rhs: Quaternion<T>) {*self = *self - rhs;}}

impl<T> MulAssign<Quaternion<T>> for Quaternion<T>
where
    T: Add<T, Output = T> +
       Sub<T, Output = T> +
       Mul<T, Output = T> +
       Div<T, Output = T> +
       Neg<   Output = T> +
       Copy + Clone
{fn mul_assign(&mut self, rhs: Quaternion<T>) {*self = *self * rhs;}}

impl<T> DivAssign<Quaternion<T>> for Quaternion<T>
where
    T: Add<T, Output = T> +
       Sub<T, Output = T> +
       Mul<T, Output = T> +
       Div<T, Output = T> +
       Neg<   Output = T> +
       Copy + Clone
{fn div_assign(&mut self, rhs: Quaternion<T>) {*self = *self / rhs;}}
