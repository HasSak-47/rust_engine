#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[repr(C)]
#[derive(Debug, Clone)]
pub struct img {
    pub width: usize,
    pub height: usize,
    pub channels: usize,
    pub data: *mut u8,
}

#[repr(C)]
pub enum img_fmt{
    PNG = 0,
    JPG = 1,
    KOI = 2,
    QOI = 3,
}

extern "C" {
    pub fn img_new(width: usize, height: usize, channels: usize) -> img;
    pub fn img_load(path: *const ::std::os::raw::c_char) -> img;
    pub fn img_write(
        image: img,
        format: img_fmt,
        path: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
#[allow(dead_code)]
    pub fn img_clear(image: img);
}

pub type ImgFmt = img_fmt;
pub type Img    = img;

