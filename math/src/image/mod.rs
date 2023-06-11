use super::algebra::number::*;
mod loader;

pub trait Pixel{
    type SubPixel;
}

pub trait PixelCast<T>{
    fn p_into(&self) -> T;
}

pub trait ImageCast<SrcPixel, OutPixel>{
    type Output;
    fn copy_to(&self) -> Self::Output;
    fn cast_to(self) -> Self::Output where Self: Sized { self.copy_to() }
}

#[allow(dead_code)]
pub struct Image<Pixel>{
    data: Vec<Pixel>,
    width : usize,
    height: usize,
}

/*
#[allow(dead_code)]
impl<Pixel: Default + Copy> Image<Pixel>{
    fn new(width: usize, height: usize) -> Self{
        Image { data: Vec::with_capacity(width * height), width, height }
    }

    const fn size(&self) -> usize { self.width * self.height }
}

#[allow(dead_code)]
impl<Pixel> Image<Pixel>{
    fn get_pixel(&self, x: usize, y: usize) -> &Pixel{ &self.data[x + y * self.width] }
    fn get_pixel_mut(&mut self, x: usize, y: usize) -> &mut Pixel{ &mut self.data[x + y * self.width] }
}

impl<SrcPixel, DstPixel> ImageCast<SrcPixel, DstPixel> for Image<SrcPixel>
where
    SrcPixel: PixelCast<DstPixel>,
    DstPixel: Default + Copy
{
    type Output = Image<DstPixel>;
    fn copy_to(&self) -> Self::Output{
        let mut output = Image::<DstPixel>::new(self.width, self.height);

        for i in 0..self.height{
            for j in 0..self.width{
                *output.get_pixel_mut(i, j) = self.get_pixel(i, j).p_into();
            }
        }

        output
    }
}

trait PrimitiveCast<T>{ fn cast(self) -> T; }

//Pixels
#[allow(dead_code)]
struct Rgb <T>([T; 3]);
#[allow(dead_code)]
struct Rgba<T>([T; 4]);
#[allow(dead_code)]
struct Grey<T>([T; 1]);
#[allow(dead_code)]
struct GrAl<T>([T; 2]);
*/
