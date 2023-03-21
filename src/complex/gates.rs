use crate::prelude::*;

use once_cell::sync::Lazy;

macro_rules! impl_operator {
    ($x:ident($($v:ident : $ty:ty),+) $($y:tt)+) => {
        pub fn $x($($v: $ty),+) -> Operator<f32> {
            Operator::<f32>::from($($y)+)
        }
    };
    ($x:ident $($y:tt)+) => {
        pub const $x: Lazy<Operator<f32>> = Lazy::new( || {
            Operator::<f32>::from($($y)+)
        });
    };
}

impl_operator!(I [[1., 0.], [0., 1.]]);
impl_operator!(ZERO [[0., 0.], [0., 0.]]);

impl_operator!(H Operator::<f32>::from([[1., 1.], [1., -1.]]) * Complex::from(0.5).sqrt());
impl_operator!(X [[0., 1.], [1., 0.]]);
impl_operator!(Y [[cmpx!(0.), cmpx!(-1. j)], [cmpx!(1. j), cmpx!(0.)]]);
impl_operator!(Z [[1., 0.], [0., -1.]]);

impl_operator!(CNOT [[1., 0., 0., 0.], [0., 1., 0., 0.], [0., 0., 0., 1.], [0., 0., 1., 0.]]);
impl_operator!(CZ Operator::from_diag([1.,1.,1.,-1.].into_iter().map(Complex::from)));

impl_operator!(SWAP [[1., 0., 0., 0.], [0., 0., 1., 0.], [0., 1., 0., 0.], [0., 0., 0., 1.]]);

impl_operator!(phase(theta: f32) {
    [[ cmpx!(1.), cmpx!(0.)], [cmpx!(0.), Complex::new(0., theta).exp()]]
});

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_involutory() {
        use gates::*;
        assert_eq!(&(&*X * &*X), &*I);
        assert_eq!(&(&*Y * &*Y), &*I);
        assert_eq!(&(&*Z * &*Z), &*I);

        assert_eq!(&(cmpx!(-1. j) * &*X * &*Y * &*Z), &*I);

        assert_eq!(
            &*I & &*I,
            Operator::from_diag([1., 1., 1., 1.].into_iter().map(Complex::from))
        );
        
        assert_eq!(
            &*H & &*I,
            Operator::from([[1., 0., 1., 0.], [0., 1., 0., 1.], [1., 0., -1., 0.], [0., 1., 0., -1.]]) * cmpx!(0.5).sqrt(),
        );
        
        assert_eq!(
            &*I & &*H,
            Operator::from([[1., 1., 0., 0.], [1., -1., 0., 0.], [0., 0., 1., 1.], [0., 0., 1., -1.]]) * cmpx!(0.5).sqrt(),
        );

        assert_eq!(
            &*H & &*I,
            Operator::try_from([[I.clone(), I.clone()],[I.clone(), -I.clone()]]).unwrap() * cmpx!(0.5).sqrt(),
        );
        
        assert_eq!(
            &*I & &*H,
            Operator::from([[1., 1., 0., 0.], [1., -1., 0., 0.], [0., 0., 1., 1.], [0., 0., 1., -1.]]) * cmpx!(0.5).sqrt(),
        );

        
        assert_eq!(
            &*I & &*H,
            Operator::try_from([[H.clone(), ZERO.clone()], [ZERO.clone(), H.clone()]]).unwrap(),
        );
    }
}
