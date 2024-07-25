use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use typenum::consts::*;
use typenum::{Diff, Sum};

#[cfg(use_single_precision)]
type Float = f32;
#[cfg(not(use_single_precision))]
type Float = f64;

//const PI : Float = 3.1415926535897932384626433832795028841971693993751058209;

pub struct UnitType<
    T, //Time
    L, //Length
    M, //Mass
    I, //Current
    O, //Amount
    N, //Temperature
    J, //Luminosity
> {
    time: PhantomData<T>,
    length: PhantomData<L>,
    mass: PhantomData<M>,
    current: PhantomData<I>,
    amount: PhantomData<O>,
    temperature: PhantomData<N>,
    luminosity: PhantomData<J>,
}

impl<T, L, M, I, O, N, J, _T, _L, _M, _I, _O, _N, _J> Sub<UnitType<_T, _L, _M, _I, _O, _N, _J>>
    for UnitType<T, L, M, I, O, N, J>
where
    T: typenum::Integer + std::ops::Sub<_T>,
    L: typenum::Integer + std::ops::Sub<_L>,
    M: typenum::Integer + std::ops::Sub<_M>,
    I: typenum::Integer + std::ops::Sub<_I>,
    O: typenum::Integer + std::ops::Sub<_O>,
    N: typenum::Integer + std::ops::Sub<_N>,
    J: typenum::Integer + std::ops::Sub<_J>,
{
    type Output = UnitType<
        Diff<T, _T>,
        Diff<L, _L>,
        Diff<M, _M>,
        Diff<I, _I>,
        Diff<O, _O>,
        Diff<N, _N>,
        Diff<J, _J>,
    >;
    fn sub(self, _rhs: UnitType<_T, _L, _M, _I, _O, _N, _J>) -> Self::Output {
        Self::Output {
            time: PhantomData,
            length: PhantomData,
            mass: PhantomData,
            current: PhantomData,
            amount: PhantomData,
            temperature: PhantomData,
            luminosity: PhantomData,
        }
    }
}

impl<T, L, M, I, O, N, J, _T, _L, _M, _I, _O, _N, _J> Add<UnitType<_T, _L, _M, _I, _O, _N, _J>>
    for UnitType<T, L, M, I, O, N, J>
where
    T: typenum::Integer + std::ops::Add<_T>,
    L: typenum::Integer + std::ops::Add<_L>,
    M: typenum::Integer + std::ops::Add<_M>,
    I: typenum::Integer + std::ops::Add<_I>,
    O: typenum::Integer + std::ops::Add<_O>,
    N: typenum::Integer + std::ops::Add<_N>,
    J: typenum::Integer + std::ops::Add<_J>,
{
    type Output = UnitType<
        Sum<T, _T>,
        Sum<L, _L>,
        Sum<M, _M>,
        Sum<I, _I>,
        Sum<O, _O>,
        Sum<N, _N>,
        Sum<J, _J>,
    >;
    fn add(self, _rhs: UnitType<_T, _L, _M, _I, _O, _N, _J>) -> Self::Output {
        Self::Output {
            time: PhantomData,
            length: PhantomData,
            mass: PhantomData,
            current: PhantomData,
            amount: PhantomData,
            temperature: PhantomData,
            luminosity: PhantomData,
        }
    }
}

//Measurement is defined in terms of the 7 base SI units
// Time, Length, Mass, Current, Temperature, Substance Amount, Luminosity
//Base Units are Second, Meter, Kilogram, Ampere, Degree Kelvin, Mole, and Candela
#[derive(Debug)]
pub struct Measurement<U> {
    _phantom_unit: PhantomData<U>, //Unit Type
    raw_value: Float,
}

impl<U> Add for Measurement<U> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            raw_value: { self.raw_value + rhs.raw_value },
            _phantom_unit: PhantomData,
        }
    }
}

impl<U> Sub for Measurement<U> {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self::Output {
            raw_value: { self.raw_value - rhs.raw_value },
            _phantom_unit: PhantomData,
        }
    }
}

impl<U, _U> Mul<Measurement<_U>> for Measurement<U>
where
    U: std::ops::Add<_U>,
{
    type Output = Measurement<Sum<U, _U>>;

    fn mul(self, rhs: Measurement<_U>) -> Self::Output {
        Self::Output {
            raw_value: { self.raw_value * rhs.raw_value },
            _phantom_unit: PhantomData,
        }
    }
}

impl<U> Mul<Float> for Measurement<U> {
    type Output = Measurement<U>;

    fn mul(self, rhs: Float) -> Self::Output {
        Self::Output {
            raw_value: { self.raw_value * rhs },
            _phantom_unit: PhantomData,
        }
    }
}

impl<U, _U> Div<Measurement<_U>> for Measurement<U>
where
    U: std::ops::Sub<_U>,
{
    type Output = Measurement<Diff<U, _U>>;

    fn div(self, rhs: Measurement<_U>) -> Self::Output {
        Self::Output {
            raw_value: { self.raw_value / rhs.raw_value },
            _phantom_unit: PhantomData,
        }
    }
}

impl<U> Div<Float> for Measurement<U> {
    type Output = Measurement<U>;

    fn div(self, rhs: Float) -> Self::Output {
        Self::Output {
            raw_value: { self.raw_value / rhs },
            _phantom_unit: PhantomData,
        }
    }
}

pub type Time = Measurement<UnitType<P1, Z0, Z0, Z0, Z0, Z0, Z0>>;
pub type Length = Measurement<UnitType<Z0, P1, Z0, Z0, Z0, Z0, Z0>>;
pub type Area = Measurement<UnitType<Z0, P2, Z0, Z0, Z0, Z0, Z0>>;
pub type Speed = Measurement<UnitType<N1, P1, Z0, Z0, Z0, Z0, Z0>>;
pub type Mass = Measurement<UnitType<Z0, Z0, P1, Z0, Z0, Z0, Z0>>;
pub type Current = Measurement<UnitType<Z0, Z0, Z0, P1, Z0, Z0, Z0>>;
pub type Temperature = Measurement<UnitType<Z0, Z0, Z0, Z0, P1, Z0, Z0>>;
pub type Amount = Measurement<UnitType<Z0, Z0, Z0, Z0, Z0, P1, Z0>>;
pub type Luminosity = Measurement<UnitType<Z0, Z0, Z0, Z0, Z0, Z0, P1>>;

macro_rules! scaled_getter {
    ($func_name:ident, $scalar:expr) => {
        pub fn $func_name(&self) -> Float {
            self.raw_value * $scalar
        }
    };
}

#[allow(unused_macros)]
macro_rules! measurement {
    ($type:ident, $scalar:expr) => {
        $type {
            raw_value: $scalar,
            _phantom_unit: PhantomData,
        } as $type
    };
}


impl Length 
{
    pub const METERS: Float = 1.0;
    pub const KILOMETERS: Float = 1.0 / 1000.0;
    pub const FEET: Float = 3.28084;
    pub const INCHES: Float = 39.37008;
    
    scaled_getter!(get_meters, Length::METERS);
    scaled_getter!(get_kilometers, Length::KILOMETERS);
    scaled_getter!(get_feet, Length::FEET);
    scaled_getter!(get_inches, Length::INCHES);   
}

impl Time
{
    pub const SECONDS: Float = 1.0;
    pub const MINUTES: Float = 1.0 / 60.0;
    pub const HOURS: Float = 1.0 / 3600.0;   

    scaled_getter!(get_seconds, Time::SECONDS);
    scaled_getter!(get_minutes,  Time::MINUTES );
    scaled_getter!(get_hours,  Time::HOURS );
}


impl Area
{

}

impl Speed
{
    
}

impl Mass
{

}
impl Mass 
{

}
impl Current 
{

}
impl Temperature 
{

}
impl Amount 
{

}
impl Luminosity 
{

}





pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn add_lengths(left: Length, right: Length) -> Length {
    left + right
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! assert_floats_close {
        //When doing math is easier than representing the literal
        ($lhs:expr, $rhs:expr) => {
            assert!(($lhs - $rhs).abs() < f64::EPSILON);
        };
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn testing() {
        let a = measurement!(Length, 32.0);

        let b = measurement!(Time, 2.0);

        let c: Speed = a / b;

        assert_eq!(c.raw_value, 16.0);

        let d: Time = Time {
            raw_value: 16.2,
            _phantom_unit: PhantomData,
        };

        assert_eq!(d.get_seconds(), 16.2);
        assert_floats_close!(d.get_minutes(), 0.27);
    }
}
