
macro_rules! make_deref {
    ($name: tt, $($param: tt), *) => {
        #[repr(C)]
        pub struct $name<T>{
            $( pub $param : T,)*
        }
    }
}

make_deref!(X   , x);
make_deref!(XY  , x, y);
make_deref!(XYZ , x, y, z);
make_deref!(XYZW, x, y, z, w);

make_deref!(MAT2x2,
    m11, m12,
    m21, m22
);

make_deref!(MAT3x3,
    m11, m12, m13,
    m21, m22, m23,
    m31, m32, m33
);

make_deref!(MAT4x4,
    m11, m12, m13, m14,
    m21, m22, m23, m24,
    m31, m32, m33, m34,
    m41, m42, m43, m44
);


use std::ops::{Deref, DerefMut};
use super::container::Container;

macro_rules! impl_deref {
    ($src: ty, $tgt: ty) => {
        impl<T> Deref for $src{
            type Target = $tgt;
            fn deref(&self) -> & Self::Target{
                let ptr: *const Self = &*self;
                unsafe{
                    return &*(ptr as *const Self::Target)
                }
            }
        }

        impl<T> DerefMut for $src{
            fn deref_mut(&mut self) -> &mut Self::Target{
                let ptr: *mut Self = &mut *self;
                unsafe{
                    return &mut *(ptr as *mut Self::Target)
                }
            }
        }
    }
}

impl_deref!(Container<T, 1, 1>, X<T>);
impl_deref!(Container<T, 2, 1>, XY<T>);
impl_deref!(Container<T, 3, 1>, XYZ<T>);
impl_deref!(Container<T, 4, 1>, XYZW<T>);

impl_deref!(Container<T, 2, 2>, MAT2x2<T>);
impl_deref!(Container<T, 3, 3>, MAT3x3<T>);
impl_deref!(Container<T, 4, 4>, MAT4x4<T>);

use super::complex::Complex;
make_deref!(RI  , r, i);
impl_deref!(Complex<T>, RI<T>);

use super::quaternion::Quaternion;
make_deref!(RIJK, r, i, j, k);
impl_deref!(Quaternion<T>, RIJK<T>);
