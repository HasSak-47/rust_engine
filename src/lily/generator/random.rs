use std::num::Wrapping;

static mut RANDOM_SEED : u32 = 0;

pub fn rands_noise(seed: &u32) -> u32{
    let mut return_val = Wrapping(seed.clone());
    return_val ^= 0xb00bad1c;
    return_val *= 0xd1cab00b;
    return_val ^= return_val >> 13;
    return_val *= 0xd1c510be;
    return_val ^= return_val>> 17;
    return_val *= 0xb00bad1c;

    return return_val.0;
}

pub fn rand_noise() -> u32{
    let prev_val : u32;
    unsafe{
        prev_val = RANDOM_SEED.clone();
        RANDOM_SEED += 1;
    }

    return rands_noise(&prev_val);
}

pub trait Converter<T>{
    fn convert(input: &u32) -> T;
    fn convert_range(min: &T, max: &T, input: &u32) -> T;
}

impl Converter<u32> for u32{
    fn convert(input: &u32) -> u32{
        return input.clone();
    }

    fn convert_range(min: &u32, max: &u32, input: &u32) -> u32{
        return min + (input % (max - min));
    }
}

impl Converter<f32> for f32{
    fn convert(input: &u32) -> f32{
        let max: u32 = 4096;
        return (input % (max + 1)) as f32 / (max as f32)
    }

    fn convert_range(min: &f32, max: &f32, input: &u32) -> f32{
        return min +f32::convert(input) * (max - min);
    }
}

pub trait Random<T: Converter<T>>{
    fn rands(seed: &u32) -> T;
    fn random() -> T;
    fn rands_range(min: &T, max: &T, seed: &u32) -> T;
    fn rand_range(min: &T, max: &T) -> T;
}

impl<T: Converter<T>> Random<T> for T {
    fn rands(seed: &u32) -> T {
        return T::convert(&rands_noise(&seed));
    }

    fn random() -> T{
        return T::convert(&rand_noise());
    }

    fn rands_range(min: &T, max: &T, seed: &u32) -> T{
        return T::convert_range(min, max, &rands_noise(seed));
    }

    fn rand_range(min: &T, max: &T) -> T{
        return T::convert_range(min, max, &rand_noise());
    }
}
