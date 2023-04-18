
pub trait Sqrt{
    type Output;
    fn sqrt(&self) -> Self::Output;
}
pub trait Abs{
    type Output;
    fn  abs(&self) -> Self::Output;
}

pub trait Unit{
    const UNIT: Self;
}

pub trait Zero{
    const ZERO: Self;
}

pub trait NegU{
    const NEGU: Self;
}

macro_rules! implement_opers {
    ($type: ty) => {
        impl Sqrt for $type {
            type Output = $type;
            fn sqrt(&self) -> $type {<$type>::sqrt(*self)}
        }
        impl  Abs for $type {
            type Output = $type;
            fn abs(&self)  -> $type {<$type>::abs(*self)}
        }
    };
}

implement_opers!(f32);
implement_opers!(f64);


macro_rules! implement_units{
    ($type: ty) => {
        impl Unit for $type{const UNIT: Self = 1 as $type;}
        impl Zero for $type{const ZERO: Self = 0 as $type;}
        impl NegU for $type{const NEGU: Self =-1 as $type;}
    };
}

implement_units!(f64);
implement_units!(f32);
implement_units!(i128);
implement_units!(i64);
implement_units!(i32);
implement_units!(i16);
implement_units!(i8);
