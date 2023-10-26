use std::mem::transmute;

pub fn i8_to_str(slice: &[i8]) -> &str{
    unsafe{
        std::str::from_utf8(transmute(slice)).expect("passed utf8 i8 slice")
    }
}

macro_rules! is_x{
    ($condition: expr) => {
        if $condition {'x'} else {' '}
    }
}

pub(crate) use is_x;


