pub use num::{Float, complex::Complex};

pub mod braket;
pub mod ops;
pub mod gates;
mod iters;

pub use iters::{ToBra, ToKet, ToOperator};


#[macro_export]
macro_rules! cmpx {
    () => {
        $crate::complex::Complex::new(0., 0.)
    };
    ($a:literal) => { $crate::complex::Complex::new($a, 0.) };

    ($b: literal j) => {
        $crate::complex::Complex::new(0., $b)
    };

    ($a:literal, $b:literal) => {
        $crate::complex::Complex::new($a, $b)
    };

    ($a:literal + $b:literal j) => {
        $crate::complex::Complex::new($a, $b)
    };

    ($a:literal - $b:literal j) => {
        $crate::complex::Complex::new($a, -$b)
    };
}

#[allow(non_upper_case_globals)]
pub const If32: Complex<f32> = cmpx!(1. j);
#[allow(non_upper_case_globals)]
pub const If64: Complex<f64> = cmpx!(1. j);
