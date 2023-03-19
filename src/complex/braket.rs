use super::{Float, Complex};
use num::Zero;
use smallvec::{smallvec, SmallVec};

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
        Self {
            shape,
            inner: smallvec![Complex::<T>::zero(); shape.size() ],
        }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_braket() {
    }

    #[test]
    fn test_operator() {
        todo!();
    }

    #[test]
    fn test_arithmetic() {
        todo!();
    }
}
