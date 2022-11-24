use std::ffi::CString;
pub use super::c_img::{
    Img,
    ImgFmt,
};

use super::c_img::*;

#[allow(dead_code)]
impl Img{
    pub fn new(width: usize, height: usize, channels: usize) -> Option<Img>{

        if height == 0 || width == 0 || channels == 0{
            return None;
        }

        unsafe{
            let new_self = img_new(width as usize, height as usize, channels as usize);

            if new_self.width == 0{
                None
            }
            else{
                Some(new_self)
            }
        }
    }

    pub fn from_arr<const WIDTH: usize, const HEIGHT: usize, const CHANNELS: usize>
    (arr: [[[u8; CHANNELS]; WIDTH]; HEIGHT]) -> Option<Img>{
        let mut new_self = match Img::new(WIDTH, HEIGHT, CHANNELS){
            Some(img) => img,
            None => return None,
        };
        for i in 0..WIDTH{
        for j in 0..HEIGHT{
        for k in 0..CHANNELS{
            *(new_self.get_channel(i, j, k)) = arr[i][j][k];
        }}}


        Some(new_self)
    }

    pub fn from_arr1<const WIDTH: usize, const HEIGHT: usize>
    (arr: [[u8; WIDTH]; HEIGHT]) -> Option<Img>{
        let mut new_self = match Img::new(WIDTH, HEIGHT, 1){
            Some(img) => img,
            None => return None,
        };

        for i in 0..WIDTH{
        for j in 0..HEIGHT{
            *new_self.get_channel(i, j, 0) = arr[i][j];
        }
        }


        Some(new_self)
    }
    pub fn from_vec1(arr: Vec<Vec<u8>>) -> Option<Img>{
        let width  = arr.len();
        let height = arr[0].len();
        let mut new_self = match Img::new(width, height, 1){
            Some(img) => img,
            None => return None,
        };

        for i in 0..width{
        for j in 0..height{
            *new_self.get_pixel_mut(i, j)[0] = arr[i][j];
        }
        }


        Some(new_self)
    }

    pub fn write(&self, fmt: ImgFmt, path: &str) -> Result<(), i32>{
        unsafe{
            let cpath = match CString::new(path){
                Ok(p) => p,
                Err(_) => return Err(-1),
            };

            let result = img_write(self.clone(), fmt, cpath.as_ptr());

            if result == 0{
                return Err(result);
            }
        }

        Ok(())
    }

    pub fn get_pixel_mut(&mut self, x: usize, y: usize) -> Vec<&mut u8>{
        let mut vec = Vec::<&mut u8>::new();
        unsafe{
            let ptr = self.data as usize + self.channels as usize * (x + self.width as usize * y);
            for i in 0..self.channels{
                vec.push(&mut *((ptr + i as usize) as *mut u8));
            }
        }

        return vec;

    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Vec<&u8>{
        let mut vec = Vec::<&u8>::new();
        unsafe{
            let mut ptr = self.channels * (x + self.width as usize * y);
            if ptr > self.channels * self.width * self.height{
                panic!("overflow positions ({},{}) bigger than size: ({},{})", x ,y, self.width, self.height);
            }
            ptr += self.data as usize;
            for i in 0..self.channels{
                vec.push(&mut *((ptr + i as usize) as *mut u8));
            }
        }

        return vec;

    }

    pub fn get_channel(&mut self, x: usize, y: usize, z: usize) -> &mut u8{
        unsafe{
            let ptr = self.data as usize + z + self.channels * (x + self.height * y);
            &mut *(ptr as *mut u8)
        }
    }

    pub fn load(path: &str) -> Option<Self>{
        unsafe{
            let cpath = match CString::new(path){
                Ok(p) => p,
                Err(_) => return None,
            };
            let new = img_load(cpath.as_ptr());
            if new.width == 0{
                None
            }
            else {
                Some(new)
            }
        }
    }
}
