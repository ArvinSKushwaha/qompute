use crate::prelude::*;

use once_cell::sync::Lazy;

macro_rules! impl_operator {
    ($x:ident $y:tt) => {
        pub const $x: Lazy<Operator<f32>> = Lazy::new( || {
            Operator::<f32>::from($y)
        });
    };
}

impl_operator!(I [[1., 0.], [0., 1.]]);

impl_operator!(H [[0.5, 0.5], [0.5, -0.5]]);
impl_operator!(X [[0., 1.], [1., 0.]]);
impl_operator!(Y [[cmpx!(0.), cmpx!(-1. j)], [cmpx!(1. j), cmpx!(0.)]]);
impl_operator!(Z [[1., 0.], [0., -1.]]);

impl_operator!(CNOT [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 0., 1.], [0., 0., 1., 0.]]);

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_involutory() {
        use gates::*;
        assert_eq!(&(&*X * &*X), &*I);
        assert_eq!(&(&*Y * &*Y), &*I);
        assert_eq!(&(&*Z * &*Z), &*I);

        dbg!(&*X * &*Y * &*Z);
        assert_eq!(&(cmpx!(-1. j) * &*X * &*Y * &*Z), &*I);
    }
}
