use std::num::Wrapping;
use super::super::converter::Converter;

static mut RANDOM_SEED : u64 = 0;

#[allow(dead_code)]
pub fn set_global_seed(seed: u64){
    unsafe{
        RANDOM_SEED = seed;
    }
}

pub fn rands_noise(seed: u64) -> u64{
    let mut return_val = Wrapping(seed.clone());
    return_val ^= 0xb00bad1c;
    return_val *= 0xd1c510be;
    return_val ^= return_val >> 13;
    return_val *= 0xd1c510be;
    return_val ^= return_val>> 17;
    return_val *= 0x94a0fbb7; //0xd1c510be;

    return return_val.0;
}

pub fn rand_noise() -> u64{
    let prev_val : u64;
    unsafe{
        prev_val = RANDOM_SEED.clone();
        RANDOM_SEED += 1;
    }

    return rands_noise(prev_val);
}

pub trait Random<T: Converter = Self>{
    fn rands(seed: u64) -> T {
        T::convert(rands_noise(seed))
    }

    fn random() -> T{
        T::convert(rand_noise())
    }

    fn rands_range(min: T, max: T, seed: u64) -> T{
        T::convert_range(min, max, rands_noise(seed))
    }

    fn rand_range(min: T, max: T) -> T{
        T::convert_range(min, max, rand_noise())
    }
   
    //takes ownership
    fn orands(seed: u64) -> T {
        T::convert(rands_noise(seed))
    }

    fn orands_range(min: T, max: T, seed: u64) -> T{
        T::convert_range(min, max, rands_noise(seed))
    }

    fn orand_range(min: T, max: T) -> T{
        T::convert_range(min, max, rand_noise())
    }
}

impl Random for usize{}
impl Random for u64{}
impl Random for f64{}
