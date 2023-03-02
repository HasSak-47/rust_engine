pub mod complex;
pub mod matrix;
pub mod quaternion;

pub trait Units {
    const UNIT: Self;
    const NEGU: Self;
    const ZERO: Self;
}

// triats are confusing
trait SquareRoot{
    type Output;
    fn _sqrt(&self) -> Self::Output;
}
trait Absolute{
    type Output;
    fn __abs(&self) -> Self::Output;
}

impl Absolute   for f32{ type Output = Self; fn __abs(&self) -> Self {self.abs()}}
impl SquareRoot for f32{ type Output = Self; fn _sqrt(&self) -> Self {self.sqrt()}}

fn abs<T>(a: T) -> <T as Absolute>::Output
where
    T: Absolute
{
    a.__abs()
}

fn sqrt<T>(a: T) -> <T as SquareRoot>::Output
where
    T: SquareRoot
{
    a._sqrt()
}
