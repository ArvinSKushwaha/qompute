pub use num::{Float, complex::Complex};

pub mod braket;
pub mod ops;


#[macro_export]
macro_rules! cmpx {
    () => {
        $crate::complex::Complex::new(0., 0.)
    };
    ($a:literal + $b:literal j) => {
        $crate::complex::Complex::new($a, $b)
    };
}
