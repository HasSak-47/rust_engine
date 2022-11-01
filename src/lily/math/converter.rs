
pub trait Converter{
    fn convert(input: u64) -> Self;
    fn convert_range(min: Self, max: Self, input: u64) -> Self;
}

impl Converter for u64{
    fn convert(input: u64) -> u64{
        return input.clone();
    }

    fn convert_range(min: u64, max: u64, input: u64) -> u64{
        return min + (input % (max - min));
    }
}

macro_rules! ConverterFloatImpl {
    ($Type: ty, $Power: literal) => {

        impl Converter for $Type{
            fn convert(input: u64) -> $Type{
                let max: u64 = 1 << $Power;
                return (input % (max + 1)) as $Type / (max as $Type);
            }
        
            fn convert_range(min: $Type, max: $Type, input: u64) -> $Type {
                return min + <$Type>::convert(input) * (max - min);
            }
        }
    };
}

macro_rules! ConverterIntImpl {
    ($Type: ty) => {
        impl Converter for $Type{
            fn convert(input: u64) -> $Type{
                return (input % (<$Type>::MAX as u64)) as $Type;
            }

            fn convert_range(min: $Type, max: $Type, input: u64) -> $Type{
                let val = (input % (<$Type>::MAX as u64)) as $Type; 
                if max - min == 0{
                    return 0;
                }
                return min + (val % (max - min));
            }
        }
    }
}

ConverterFloatImpl!(f32, 10);
ConverterFloatImpl!(f64, 14);

ConverterIntImpl!(usize);
ConverterIntImpl!(u32);
ConverterIntImpl!(u16);
ConverterIntImpl!(u8 );
