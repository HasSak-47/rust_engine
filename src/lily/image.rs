use crate::lily::bindings::c_img::*;
use std::ffi::CString;

pub use crate::lily::bindings::c_img::{
    Img, ImgFmt
};

impl Img{
    pub fn new(width: i32, height: i32, channels: i32) -> Option<Img>{
        unsafe{
            let new_self = img_new(width, height, channels);

            if new_self.width == 0{
                None
            }
            else{
                Some(new_self)
            }
        }
    }

    pub fn write(&self, fmt: ImgFmt, path: &str) -> i32{
        unsafe{
            let cpath = match CString::new(path){
                Ok(p) => p,
                Err(_) => return -1,
            };
            img_write(self.clone(), fmt, cpath.as_ptr())
        }
    }

    pub fn get_pixel(&mut self, x: usize, y: usize) -> Vec<&mut u8>{
        let mut vec = Vec::<&mut u8>::new();
        unsafe{
            let ptr = self.data() as usize + self.channels as usize * (x + self.height as usize * y);
            for i in 0..self.channels{
                vec.push(&mut *((ptr + i as usize) as *mut u8));
            }
        }

        return vec;

    }
}

