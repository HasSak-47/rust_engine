use std::ops::Deref;

macro_rules! vec_maker{
    ($name: ident, $($comp: ident), *) => {
        #[repr(C)]
        #[derive(Clone, Copy, Eq, PartialEq)]
        pub struct $name<T>{
            $(pub $comp: T,)*
        }
    };
}

vec_maker!(Vec2, x, y);
vec_maker!(Vec3, x, y, z);
vec_maker!(Vec4, x, y, z, w);

vec_maker!(Cmpl, r, i);
vec_maker!(Quat, r, i, j, k);
