use slice_of_array::SliceFlatExt;
use smallvec::ToSmallVec;

use crate::prelude::*;

pub trait ToKet<T: Float>: Sized {
    fn to_ket(self) -> Ket<T>;
}

pub trait ToBra<T: Float>: Sized {
    fn to_bra(self) -> Bra<T>;
}

pub trait ToOperator<T: Float>: Sized {
    fn to_operator(self) -> Operator<T>;
}

impl<T: Float, I: Iterator<Item = Complex<T>>> ToKet<T> for I {
    fn to_ket(self) -> Ket<T> {
        Ket { inner: self.collect() }
    }
}

impl<T: Float, I: Iterator<Item = Complex<T>>> ToBra<T> for I {
    fn to_bra(self) -> Bra<T> {
        Bra { inner: self.collect() }
    }
}

impl<T: Float, const N: usize> ToOperator<T> for &[[T; N]] {
    fn to_operator(self) -> Operator<T> {
        let rows = self.len();
        Operator {
            shape: Shape { rows, cols: N },
            inner: self.flat().into_iter().map(Complex::<T>::from).collect(),
        }
    }
}

impl<T: Float, const N: usize> ToOperator<T> for &[[Complex<T>; N]] {
    fn to_operator(self) -> Operator<T> {
        let rows = self.len();
        Operator {
            shape: Shape { rows, cols: N },
            inner: self.flat().to_smallvec(),
        }
    }
}
