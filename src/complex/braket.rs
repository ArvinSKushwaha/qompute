use super::{Complex, Float};
use num::{One, Zero};
use slice_of_array::prelude::*;
use smallvec::{smallvec, SmallVec, ToSmallVec};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Orientation {
    NonOriented,
    Row,
    Column,
}

impl std::ops::Neg for Orientation {
    type Output = Orientation;

    fn neg(self) -> Self::Output {
        use Orientation::*;

        match self {
            NonOriented => NonOriented,
            Row => Column,
            Column => Row,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Shape {
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl Shape {
    pub fn transpose(self) -> Self {
        let Self { width, height } = self;
        Self {
            width: height,
            height: width,
        }
    }

    pub fn size(self) -> usize {
        self.width * self.height
    }
}

impl From<(usize, usize)> for Shape {
    fn from(value: (usize, usize)) -> Self {
        Shape {
            width: value.0,
            height: value.1,
        }
    }
}

pub trait ComplexObject<T: Float> {
    type ConjugateTranspose: ComplexObject<T>;
    type InnerProduct: ComplexObject<T>;
    type OuterProduct: ComplexObject<T>;

    const ORIENTATION: Orientation;

    fn dagger(&self) -> Self::ConjugateTranspose;
    fn shape(&self) -> Shape;

    fn width(&self) -> usize {
        self.shape().width
    }

    fn height(&self) -> usize {
        self.shape().height
    }

    fn size(&self) -> usize {
        self.shape().size()
    }

    fn hermitian(&self) -> bool;
}

#[derive(Clone, Debug, PartialEq)]
pub struct Ket<T: Float> {
    pub inner: SmallVec<[Complex<T>; 2]>,
}

impl<T: Float> Ket<T> {
    fn new(size: usize) -> Self {
        Self {
            inner: smallvec![Complex::<T>::zero(); size],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Bra<T: Float> {
    pub inner: SmallVec<[Complex<T>; 2]>,
}

impl<T: Float> Bra<T> {
    fn new(size: usize) -> Self {
        Self {
            inner: smallvec![Complex::<T>::zero(); size],
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Operator<T: Float> {
    pub(crate) shape: Shape,
    pub inner: SmallVec<[Complex<T>; 4]>,
}

impl<T: Float> Operator<T> {
    pub fn new(shape: Shape) -> Self {
        let mut op = Self {
            shape,
            inner: smallvec![Complex::<T>::zero(); shape.size() ],
        };

        (0..shape.width.min(shape.height))
            .into_iter()
            .for_each(|i| op[(i, i)] = Complex::<T>::one());

        op
    }
}

impl<T: Float> ComplexObject<T> for Ket<T> {
    type ConjugateTranspose = Bra<T>;
    type InnerProduct = Complex<T>;
    type OuterProduct = Operator<T>;

    const ORIENTATION: Orientation = Orientation::Column;

    fn dagger(&self) -> Self::ConjugateTranspose {
        Self::ConjugateTranspose {
            inner: self
                .inner
                .iter()
                .copied()
                .map(|a| a.conj())
                .collect::<SmallVec<_>>(),
        }
    }

    fn shape(&self) -> Shape {
        Shape {
            width: 1,
            height: self.inner.len(),
        }
    }

    fn hermitian(&self) -> bool {
        false
    }
}

impl<T: Float> ComplexObject<T> for Bra<T> {
    type ConjugateTranspose = Ket<T>;
    type InnerProduct = Complex<T>;
    type OuterProduct = Operator<T>;

    const ORIENTATION: Orientation = Orientation::Row;

    fn dagger(&self) -> Self::ConjugateTranspose {
        Self::ConjugateTranspose {
            inner: self
                .inner
                .iter()
                .copied()
                .map(|a| a.conj())
                .collect::<SmallVec<_>>(),
        }
    }

    fn shape(&self) -> Shape {
        Shape {
            width: self.inner.len(),
            height: 1,
        }
    }

    fn hermitian(&self) -> bool {
        false
    }
}

impl<T: Float> ComplexObject<T> for Operator<T> {
    type ConjugateTranspose = Operator<T>;
    type InnerProduct = Operator<T>;
    type OuterProduct = Operator<T>;

    const ORIENTATION: Orientation = Orientation::NonOriented;

    fn dagger(&self) -> Self::ConjugateTranspose {
        let Shape { width, height } = self.shape;
        let inner = (0..height) // For each row...
            .into_iter()
            .zip(std::iter::repeat(0..width).flatten()) // Traverse width
            .map(|(i, j)| self[(j, i)].conj()) // And grab the opposing conjugate
            .collect::<SmallVec<_>>();

        Self::ConjugateTranspose {
            shape: self.shape.transpose(),
            inner,
        }
    }

    fn shape(&self) -> Shape {
        self.shape
    }

    fn hermitian(&self) -> bool {
        self.shape == self.shape.transpose()
            && (0..self.height())
                .into_iter()
                .zip(std::iter::repeat(0..self.width()).flatten())
                .all(|(i, j)| self[(i, j)].conj() == self[(j, i)])
    }
}

impl<T: Float> ComplexObject<T> for Complex<T> {
    type ConjugateTranspose = Complex<T>;
    type InnerProduct = Complex<T>;
    type OuterProduct = Complex<T>;

    const ORIENTATION: Orientation = Orientation::NonOriented;

    fn dagger(&self) -> Self::ConjugateTranspose {
        self.conj()
    }

    fn shape(&self) -> Shape {
        Shape {
            width: 1,
            height: 1,
        }
    }

    fn hermitian(&self) -> bool {
        true
    }
}

impl<T: Float> std::ops::Index<usize> for Ket<T> {
    type Output = Complex<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(index)
    }
}

impl<T: Float> std::ops::Index<usize> for Bra<T> {
    type Output = Complex<T>;

    fn index(&self, index: usize) -> &Self::Output {
        self.inner.index(index)
    }
}

impl<T: Float> std::ops::Index<(usize, usize)> for Operator<T> {
    type Output = Complex<T>;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        let Self {
            shape: Shape { width, .. },
            ..
        } = self;

        self.inner.index(index.0 * width + index.1)
    }
}

impl<T: Float> std::ops::IndexMut<usize> for Ket<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner.index_mut(index)
    }
}

impl<T: Float> std::ops::IndexMut<usize> for Bra<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.inner.index_mut(index)
    }
}

impl<T: Float> std::ops::IndexMut<(usize, usize)> for Operator<T> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        let Self {
            shape: Shape { width, .. },
            ..
        } = *self;

        self.inner.index_mut(index.0 * width + index.1)
    }
}

impl<T: Float> From<&[Complex<T>]> for Ket<T> {
    fn from(value: &[Complex<T>]) -> Self {
        Ket {
            inner: value.to_smallvec(),
        }
    }
}

impl<T: Float> From<&[Complex<T>]> for Bra<T> {
    fn from(value: &[Complex<T>]) -> Self {
        Bra {
            inner: value.to_smallvec(),
        }
    }
}

impl<T: Float, const N: usize> From<[Complex<T>; N]> for Ket<T> {
    fn from(value: [Complex<T>; N]) -> Self {
        Ket {
            inner: value.to_smallvec(),
        }
    }
}

impl<T: Float, const N: usize> From<[Complex<T>; N]> for Bra<T> {
    fn from(value: [Complex<T>; N]) -> Self {
        Bra {
            inner: value.to_smallvec(),
        }
    }
}

impl<T: Float> From<&[T]> for Ket<T> {
    fn from(value: &[T]) -> Self {
        Ket {
            inner: value.into_iter().map(Complex::<T>::from).collect(),
        }
    }
}

impl<T: Float> From<&[T]> for Bra<T> {
    fn from(value: &[T]) -> Self {
        Bra {
            inner: value.into_iter().map(Complex::<T>::from).collect(),
        }
    }
}

impl<T: Float, const M: usize, const N: usize> From<[[Complex<T>; N]; M]> for Operator<T> {
    fn from(value: [[Complex<T>; N]; M]) -> Self {
        Self {
            shape: (M, N).into(),
            inner: value.as_slice().flat().to_smallvec(),
        }
    }
}

impl<T: Float, const M: usize, const N: usize> From<[[T; N]; M]> for Operator<T> {
    fn from(value: [[T; N]; M]) -> Self {
        Self {
            shape: (M, N).into(),
            inner: value
                .as_slice()
                .flat()
                .iter()
                .map(Complex::<T>::from)
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        cmpx,
        complex::braket::{Bra, ComplexObject, Operator},
    };

    use super::Ket;

    #[test]
    fn test_braket() {
        let x = Ket::from([cmpx!(1. + 2. j)]);
        let y = Ket::from([cmpx!(2. + 1.0 j)]);

        let z = Ket::from([cmpx!(3., 3.)]);

        assert_eq!(z, &x + &y);
        assert_eq!(z.dagger() * &z, cmpx!(18.));
        assert_eq!(z.dagger() * &y, cmpx!(9. - 3. j));
        assert_eq!(z.dagger() * &x, cmpx!(9. + 3. j));

        let w = Bra::from([cmpx!(3., -3.)]);
        assert_eq!(z.dagger(), w);

        let z_proj = &z * &z.dagger();
        let z_proj_op = Operator::from([[cmpx!(18.)]]);

        assert_eq!(z_proj, z_proj_op);
        assert_eq!(&x * cmpx!(1. - 2. j), Ket::from([cmpx!(5.0)]));
    }

    #[test]
    fn test_operator() {
        // TODO: Add a more robust test suite.
        let op1 = Operator::from([[cmpx!(1.), cmpx!(1. j)], [cmpx!(-1. j), cmpx!(0.)]]);

        assert!(op1.hermitian());
        assert_eq!(op1[(0, 0)], cmpx!(1.));
        assert_eq!(op1[(0, 1)], cmpx!(1. j));
        assert_eq!(op1[(1, 0)], cmpx!(-1. j));
        assert_eq!(op1[(1, 1)], cmpx!(0.));
    }
}
