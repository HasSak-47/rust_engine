#[derive(Clone)]
#[repr(C)]
pub struct Img{
    pub width   : i32,
    pub height  : i32,
    pub channels: i32,

    data: *mut u8, 
}

#[repr(C)]
pub enum ImgFmt{
    PNG = 0,
    JPG = 1,
    KOI = 2,
    QOI = 3,
}

extern "C" {
    pub fn img_new(width: i32, height: i32, channels: i32) -> Img;
    pub fn img_write(img: Img, fmt: ImgFmt, path: *const i8) -> cty::c_int;
    pub fn img_clear(img: Img);
}

impl Img{
    pub const unsafe fn data(&self) -> *mut u8{
        self.data
    }
}


impl Drop for Img{
    fn drop(&mut self) {
        unsafe{
            img_clear(self.clone());        
        }
    }

}
