use std::default::Default;

#[repr(C)]
#[allow(dead_code)]
pub struct BaseMat<Type, const WIDTH: usize, const HEIGH: usize>{
    pub data: [[Type; HEIGH]; WIDTH],
}

impl<Type, const WIDTH: usize, const HEIGH: usize> BaseMat<Type, WIDTH, HEIGH>{
    const fn new(d: [[Type; WIDTH]; HEIGH]) -> Self{
        
    }
}


impl<Type, const WIDTH: usize, const HEIGH: usize> Default for BaseMat<Type, WIDTH, HEIGH>
where
    Type: Default + Copy
{
    fn default() -> Self {
        Self { data: [[Type::default(); HEIGH]; WIDTH]}
    }
}

impl<Type, const WIDTH: usize, const HEIGH: usize> std::fmt::Display for BaseMat<Type, WIDTH, HEIGH>
where
    Type: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for a in &self.data{
            for b in a{
                write!(f, "{b} ")?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
