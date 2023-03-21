use crate::prelude::*;

use num::{One, Zero};
use slice_of_array::prelude::*;
use smallvec::{smallvec, SmallVec, ToSmallVec};
use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq, Hash)]
pub enum QomputeTypeError {
    #[error("It seems your types don't match size.")]
    NonMatchingSizes,
    #[error("You may have passed in a value with zero size, when a non-zero size was expected")]
    EmptyInput,
}

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
    pub(crate) rows: usize,
    pub(crate) cols: usize,
}

impl Shape {
    pub fn transpose(self) -> Self {
        let Self { cols, rows } = self;
        Self {
            cols: rows,
            rows: cols,
        }
    }

    pub fn size(self) -> usize {
        self.cols * self.rows
    }
}

impl From<(usize, usize)> for Shape {
    fn from(value: (usize, usize)) -> Self {
        Shape {
            rows: value.0,
            cols: value.1,
        }
    }
}

impl From<Shape> for (usize, usize) {
    fn from(value: Shape) -> Self {
        let Shape { rows, cols } = value;
        (rows, cols)
    }
}

pub trait ComplexObject<T: Float>: Sized {
    type ConjugateTranspose: ComplexObject<T>;
    type InnerProduct: ComplexObject<T>;
    type OuterProduct: ComplexObject<T>;
    type IndexType: Copy;

    const ORIENTATION: Orientation;

    fn dagger(&self) -> Self::ConjugateTranspose;
    fn shape(&self) -> Shape;
    fn tensorprod(&self, rhs: &Self) -> Self;

    fn cols(&self) -> usize {
        self.shape().cols
    }

    fn rows(&self) -> usize {
        self.shape().rows
    }

    fn size(&self) -> usize {
        self.shape().size()
    }

    fn hermitian(&self) -> bool;

    fn index(&self, idx: Self::IndexType) -> &Complex<T>;
    fn index_mut(&mut self, idx: Self::IndexType) -> &mut Complex<T>;
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
    pub fn new_with_shape(shape: Shape) -> Self {
        let mut op = Self {
            shape,
            inner: smallvec![Complex::<T>::zero(); shape.size() ],
        };

        (0..shape.cols.min(shape.rows)).for_each(|i| op[(i, i)] = Complex::<T>::one());

        op
    }

    pub fn from_diag<I>(n: I) -> Self
    where
        I: IntoIterator<Item = Complex<T>>,
    {
        let n = n.into_iter().collect::<Vec<_>>();
        let shape = (n.len(), n.len()).into();
        let inner = (0..n.len())
            .map(|i| {
                let mut m = vec![Complex::<T>::zero(); n.len()];
                m[i] = n[i];
                m.into_iter()
            })
            .flatten()
            .collect();
        Self { shape, inner }
    }
}

impl<T: Float> ComplexObject<T> for Ket<T> {
    type ConjugateTranspose = Bra<T>;
    type InnerProduct = Complex<T>;
    type OuterProduct = Operator<T>;
    type IndexType = usize;

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
            cols: 1,
            rows: self.inner.len(),
        }
    }

    fn hermitian(&self) -> bool {
        false
    }

    fn index(&self, idx: Self::IndexType) -> &Complex<T> {
        &self[idx]
    }

    fn index_mut(&mut self, idx: Self::IndexType) -> &mut Complex<T> {
        &mut self[idx]
    }

    fn tensorprod(&self, rhs: &Self) -> Self {
        // TODO: Implement tensor product for objects.
        let (lhs_rows, lhs_cols) = self.shape().into();
        let (rhs_rows, rhs_cols) = rhs.shape().into();
        let out_shape = Shape {
            rows: lhs_rows * rhs_rows,
            cols: lhs_cols * rhs_cols,
        };

        let mut bra = Ket::new(out_shape.rows);

        for i0 in 0..lhs_cols {
            let i_off = i0 * rhs_cols;
            for i1 in 0..rhs_cols {
                bra[i_off + i1] = self[i0] * rhs[i1];
            }
        }

        bra
    }
}

impl<T: Float> ComplexObject<T> for Bra<T> {
    type ConjugateTranspose = Ket<T>;
    type InnerProduct = Complex<T>;
    type OuterProduct = Operator<T>;
    type IndexType = usize;

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
            cols: self.inner.len(),
            rows: 1,
        }
    }

    fn hermitian(&self) -> bool {
        false
    }

    fn index(&self, idx: Self::IndexType) -> &Complex<T> {
        &self[idx]
    }

    fn index_mut(&mut self, idx: Self::IndexType) -> &mut Complex<T> {
        &mut self[idx]
    }

    fn tensorprod(&self, rhs: &Self) -> Self {
        // TODO: Implement tensor product for objects.
        let (lhs_rows, lhs_cols) = self.shape().into();
        let (rhs_rows, rhs_cols) = rhs.shape().into();
        let out_shape = Shape {
            rows: lhs_rows * rhs_rows,
            cols: lhs_cols * rhs_cols,
        };

        let mut bra = Bra::new(out_shape.cols);

        for j0 in 0..lhs_cols {
            let j_off = j0 * rhs_cols;
            for j1 in 0..rhs_cols {
                bra[j_off + j1] = self[j0] * rhs[j1];
            }
        }

        bra
    }
}

impl<T: Float> ComplexObject<T> for Operator<T> {
    type ConjugateTranspose = Operator<T>;
    type InnerProduct = Operator<T>;
    type OuterProduct = Operator<T>;
    type IndexType = (usize, usize);

    const ORIENTATION: Orientation = Orientation::NonOriented;

    fn dagger(&self) -> Self::ConjugateTranspose {
        let Shape { cols, rows } = self.shape;
        let inner = (0..rows) // For each row...
            .zip(std::iter::repeat(0..cols).flatten()) // Traverse width
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
            && (0..self.rows())
                .zip(std::iter::repeat(0..self.cols()).flatten())
                .all(|(i, j)| self[(i, j)].conj() == self[(j, i)])
    }

    fn index(&self, idx: Self::IndexType) -> &Complex<T> {
        &self[idx]
    }

    fn index_mut(&mut self, idx: Self::IndexType) -> &mut Complex<T> {
        &mut self[idx]
    }

    fn tensorprod(&self, rhs: &Self) -> Self {
        // TODO: Implement tensor product for objects.
        let (lhs_rows, lhs_cols) = self.shape().into();
        let (rhs_rows, rhs_cols) = rhs.shape().into();
        let out_shape = Shape {
            rows: lhs_rows * rhs_rows,
            cols: lhs_cols * rhs_cols,
        };

        let mut op = Operator::new_with_shape(out_shape);

        for i0 in 0..lhs_rows {
            let i_off = i0 * rhs_rows;
            for j0 in 0..lhs_cols {
                let j_off = j0 * rhs_cols;
                for i1 in 0..rhs_rows {
                    for j1 in 0..rhs_cols {
                        op[(i_off + i1, j_off + j1)] = self[(i0, j0)] * rhs[(i1, j1)];
                    }
                }
            }
        }

        op
    }
}

impl<T: Float> ComplexObject<T> for Complex<T> {
    type ConjugateTranspose = Complex<T>;
    type InnerProduct = Complex<T>;
    type OuterProduct = Complex<T>;
    type IndexType = ();

    const ORIENTATION: Orientation = Orientation::NonOriented;

    fn dagger(&self) -> Self::ConjugateTranspose {
        self.conj()
    }

    fn shape(&self) -> Shape {
        Shape { cols: 1, rows: 1 }
    }

    fn hermitian(&self) -> bool {
        true
    }

    fn index(&self, _idx: Self::IndexType) -> &Complex<T> {
        self
    }

    fn index_mut(&mut self, _idx: Self::IndexType) -> &mut Complex<T> {
        self
    }

    fn tensorprod(&self, rhs: &Self) -> Self {
        self * rhs
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
            shape: Shape { cols, .. },
            ..
        } = self;

        self.inner.index(index.0 * cols + index.1)
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
            shape: Shape { cols, .. },
            ..
        } = *self;

        self.inner.index_mut(index.0 * cols + index.1)
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
            inner: value.iter().map(Complex::<T>::from).collect(),
        }
    }
}

impl<T: Float> From<&[T]> for Bra<T> {
    fn from(value: &[T]) -> Self {
        Bra {
            inner: value.iter().map(Complex::<T>::from).collect(),
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

impl<T: Float, const M: usize, const N: usize> TryFrom<[[Operator<T>; N]; M]> for Operator<T> {
    type Error = QomputeTypeError;

    fn try_from(value: [[Operator<T>; N]; M]) -> Result<Self, Self::Error> {
        if M == 0 || N == 0 {
            return Err(QomputeTypeError::EmptyInput);
        }

        let matching_shapes = value
            .as_slice()
            .flat()
            .windows(2)
            .all(|s| s[0].shape() == s[1].shape());

        if !matching_shapes {
            return Err(QomputeTypeError::NonMatchingSizes);
        }


        let inner_shape = value[0][0].shape();
        let op_shape = Shape {
            rows: inner_shape.rows * M,
            cols: inner_shape.cols * N,
        };
        let mut op = Operator::new_with_shape(op_shape);

        for i0 in 0..M {
            let i_off = i0 * inner_shape.rows;
            for j0 in 0..N {
                let j_off = j0 * inner_shape.cols;
                for i1 in 0..(inner_shape.rows) {
                    for j1 in 0..(inner_shape.cols) {
                        op[(i_off + i1, j_off + j1)] = value[i0][j0][(i1, j1)];
                    }
                }
            }
        }

        Ok(op)
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;

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

        let mut x = Ket::new(5);
        let mut y = Ket::new(5);

        (0..5)
            .map(|i| cmpx!(0.5) * i as f32)
            .enumerate()
            .for_each(|(i, f)| x[i] = f.into());

        (0..5)
            .map(|i| cmpx!(-0.5 j) * i as f32)
            .enumerate()
            .for_each(|(i, f)| y[i] = f.into());

        assert_eq!(
            y.dagger() * x,
            (0..5)
                .map(|i| 0.25 * (i * i) as f32 * If32)
                .sum::<Complex<f32>>()
        );
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

        let op2 = Operator::from([[0., 2.], [1., 0.]]);
        let op3 = &op1 * &op2;

        assert_eq!(
            op3,
            Operator::from([[cmpx!(1. j), cmpx!(2.)], [cmpx!(0.), cmpx!(-2. j)]])
        );
    }
}
