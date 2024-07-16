use std::ops::{Add, Mul, Sub, Div};
#[cfg(use_single_precision)]
type float = f32;
#[cfg(not(use_single_precision))]
type float = f64;
mod test;
//Measurement is defined in terms of the 7 base SI units
// Time, Length, Mass, Current, Temperature, Substance Amount, Luminosity
//Base Units are Second, Meter, Kilogram, Ampere, Degree Kelvin, Mole, and Candela
#[derive(Debug)]
pub struct Measurement<
    const T: u8,
    const L: u8,
    const M: u8,
    const I: u8,
    const O: u8,
    const N: u8,
    const J: u8,
> {
    raw_value: float,
}

impl<
        const T: u8,
        const L: u8,
        const M: u8,
        const I: u8,
        const O: u8,
        const N: u8,
        const J: u8,
    > Add for Measurement<T, L, M, I, O, N, J>
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output{raw_value : {self.raw_value + rhs.raw_value}}
    }
}

impl<
        const T: u8,
        const L: u8,
        const M: u8,
        const I: u8,
        const O: u8,
        const N: u8,
        const J: u8,
    > Sub for Measurement<T, L, M, I, O, N, J>
{
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output{raw_value : {self.raw_value - rhs.raw_value}}
    }
}


impl<
        const T: u8,
        const L: u8,
        const M: u8,
        const I: u8,
        const O: u8,
        const N: u8,
        const J: u8,
    > Mul for Measurement<T, L, M, I, O, N, J>
{
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let raw : float = self.raw_value - rhs.raw_value;

        Self::Output{raw_value : {0.0}}
    }
    
}

pub type Time = Measurement<1, 0, 0, 0, 0, 0, 0>;
pub type Length = Measurement<0, 1, 0, 0, 0, 0, 0>;
pub type Mass = Measurement<0, 0, 1, 0, 0, 0, 0>;
pub type Current = Measurement<0, 0, 0, 1, 0, 0, 0>;
pub type Temperature = Measurement<0, 0, 0, 0, 1, 0, 0>;
pub type Amount = Measurement<0, 0, 0, 0, 0, 1, 0>;
pub type Luminosity = Measurement<0, 0, 0, 0, 0, 0, 1>;

macro_rules! scaled_getter {
    ($type_name:ident, $func_name:ident, $scalar:literal) => {
        impl $type_name {
            pub fn $func_name(self) -> float {
                self.raw_value * $scalar
            }
        }
    };
}

scaled_getter!(Length, getMeters, 1.0);
scaled_getter!(Length, getFeet, 3.28084);
scaled_getter!(Length, getInches, 39.37008);



pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_lengths(left: Length, right: Length) -> Length {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_Length(){
        let a : Length = Length{raw_value : 2.0};
        let b : Length = Length{raw_value : 1.0};
        let c : Length = a + b;
        assert_eq!(c.getMeters(), 3.0);
    }

}
