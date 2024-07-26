use std::{
    marker::PhantomData,
    ops::{Add, Div, Mul, Sub},
};
use typenum::consts::{N1 ,Z0, P1, P2};
use typenum::{Diff, Sum};

#[cfg(use_single_precision)]
type Float = f32;
#[cfg(not(use_single_precision))]
type Float = f64;

//const PI : Float = 3.1415926535897932384626433832795028841971693993751058209;
#[derive(Debug, Copy, Clone, PartialEq)]
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Measurement<U> {
    _phantom_unit: PhantomData<U>, //Unit Type
    raw_value: Float,
}

impl<U> Add for Measurement<U> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.raw_value += rhs.raw_value;
        self
    }
}

impl<U> Sub for Measurement<U> {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self::Output {
        self.raw_value -= rhs.raw_value;
        self
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

#[allow(unused_macros)]
macro_rules! measurement {
    ($type:ident, $scalar:expr) => {
        $type {
            raw_value: $scalar,
            _phantom_unit: PhantomData,
        } as $type
    };
}

#[allow(unused_macros)]
macro_rules! add_scalar {
    ($func_name:ident, $scalar_name:ident, $scalar_value:expr) => {

        pub const $scalar_name: Float = $scalar_value;

        pub fn $func_name(&self) -> Float {
            self.raw_value * $scalar_value
        }
    };
}

macro_rules! add_scalar_and_constructor {
    ($func_name:ident, $scalar_name:ident, $scalar_value:expr, $constructor_name:ident) => {
        add_scalar!($func_name, $scalar_name, $scalar_value);

        pub fn $constructor_name(raw : Float) -> Self
        {
            Self {
                raw_value: raw / $scalar_value,
                _phantom_unit: PhantomData,
            } as Self
        }
    };
}

impl Length 
{
    add_scalar_and_constructor!(get_inches, INCHES, 39.37008, inches);
    add_scalar_and_constructor!(get_feet, FEET, 3.28084, feet);
    add_scalar_and_constructor!(get_centimeters, CENTIMETERS, 1000.0, centimeters);
    add_scalar_and_constructor!(get_meters, METERS, 1.0, meters);
    add_scalar_and_constructor!(get_kilometers, KILOMETERS, 1.0 / 1000.0, kilometers);
    add_scalar_and_constructor!(get_miles, MILES, 1.0 / 1609.344, miles);
}

impl Time
{
    add_scalar_and_constructor!(get_nanoseconds ,NANOSECONDS, 1000000000.0, nanoseconds);
    add_scalar_and_constructor!(get_microseconds ,MICROISECONDS, 1000000.0, microseconds);
    add_scalar_and_constructor!(get_milliseconds ,MILLISECONDS, 1000.0, milliseconds);
    add_scalar_and_constructor!(get_seconds ,SECONDS, 1.0, seconds);
    add_scalar_and_constructor!(get_minutes ,MINUTES, 1.0 / 60.0, minutes);
    add_scalar_and_constructor!(get_hours ,HOURS, 1.0 / 3600.0, hours);
}


impl Area
{
    add_scalar_and_constructor!(get_square_meters, SQUARE_METERS, 1.0, square_meters);
}

impl Speed
{
    add_scalar_and_constructor!(get_meters_per_second, METERS_PER_SECOND, 1.0, meter_per_second);
    add_scalar_and_constructor!(get_kilometers_per_hour, KILOMETERS_PER_HOUR, 3.6, kilometers_per_hour);
    add_scalar_and_constructor!(get_miles_per_hour, MILES_PER_HOUR, 2.237136, miles_per_hour);
    add_scalar_and_constructor!(get_mach, MACH, 0.002939, mach);
}

impl Mass
{
    add_scalar_and_constructor!(get_kilograms, KILOGRAMS, 1.0, kilograms);
    add_scalar_and_constructor!(get_grams, GRAMS, 1.0/1000.0, grams);
    add_scalar_and_constructor!(get_pounds, POUNDS, 2.204623, pounds);
    
}

impl Current 
{
    add_scalar_and_constructor!(get_ampere, AMPERE, 1.0, ampere);
    //TODO
}

impl Temperature 
{
    add_scalar_and_constructor!(get_kelvin, KELVIN, 1.0, kelvin);
    //TODO
}

impl Amount 
{
    add_scalar_and_constructor!(get_moles, MOLE, 1.0, moles);
    //TODO
}

impl Luminosity 
{
    add_scalar_and_constructor!(get_candela, CANDELA, 1.0, candela);
    //TODO
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
    //Todo - write real tests
    #[test]
    fn testing() {
        let a = Length::kilometers(0.032);

        let b = Time::seconds(2.0);

        let c = a / b;

        assert_eq!(c.get_meters_per_second(), 16.0);
        
        let d = Time::milliseconds(16200.0);
        
        assert_floats_close!(d.get_seconds(), 16.2);
        assert_floats_close!(d.get_minutes(), 0.27);
        
        
        let e = Length::meters(2.0);
        let e = e * e;
        
        assert_eq!(e.get_square_meters(), 4.0);

    }
}
