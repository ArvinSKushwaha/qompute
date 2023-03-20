use num::Zero;

use crate::prelude::*;

use super::braket::{Bra, Ket, Operator};

auto_ops::impl_op_ex!(+ <T: Float> |lhs: &Ket<T>, rhs: &Ket<T>| -> Ket<T> {
    assert_eq!(lhs.shape(), rhs.shape());
    Ket {
        inner: lhs
            .inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(a, b)| *a + *b)
            .collect(),
    }
});

auto_ops::impl_op_ex!(+ <T: Float> |lhs: &Bra<T>, rhs: &Bra<T>| -> Bra<T> {
    assert_eq!(lhs.shape(), rhs.shape());
    Bra {
        inner: lhs
            .inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(a, b)| *a + *b)
            .collect(),
    }
});

auto_ops::impl_op_ex!(+ <T: Float> |lhs: &Operator<T>, rhs: &Operator<T>| -> Operator<T> {
    assert_eq!(lhs.shape(), rhs.shape());
    Operator {
        shape: lhs.shape,
        inner: lhs
            .inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(a, b)| *a + *b)
            .collect(),
    }
});

auto_ops::impl_op_ex!(- <T: Float> |lhs: &Ket<T>, rhs: &Ket<T>| -> Ket<T> {
    assert_eq!(lhs.shape(), rhs.shape());
    Ket {
        inner: lhs
            .inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(a, b)| *a - *b)
            .collect(),
    }
});

auto_ops::impl_op_ex!(- <T: Float> |lhs: &Bra<T>, rhs: &Bra<T>| -> Bra<T> {
    assert_eq!(lhs.shape(), rhs.shape());
    Bra {
        inner: lhs
            .inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(a, b)| *a - *b)
            .collect(),
    }
});

auto_ops::impl_op_ex!(- <T: Float> |lhs: &Operator<T>, rhs: &Operator<T>| -> Operator<T> {
    assert_eq!(lhs.shape(), rhs.shape());
    Operator {
        shape: lhs.shape,
        inner: lhs
            .inner
            .iter()
            .zip(rhs.inner.iter())
            .map(|(a, b)| *a - *b)
            .collect(),
    }
});

auto_ops::impl_op_ex!(* <T: Float> |lhs: &Bra<T>, rhs: &Ket<T>| -> Complex<T> {
    assert_eq!(lhs.shape().cols, rhs.shape().rows);
    lhs.inner.iter().zip(rhs.inner.iter()).map(|(a, b)| *a * *b).fold(Complex::<T>::zero(), |a, b| { a + b })
});

auto_ops::impl_op_ex!(* <T: Float> |lhs: &Ket<T>, rhs: &Bra<T>| -> Operator<T> {
    let (_, lhs_rows) = lhs.shape().into();
    let (rhs_cols, _) = rhs.shape().into();

    let mut op = Operator::new_with_shape((rhs.rows(), lhs.cols()).into());

    for row in 0..lhs_rows {
        for col in 0..rhs_cols {
            op[(row, col)] = lhs[row] * rhs[col];
        }
    }

    op
});

auto_ops::impl_op_ex!(* <T: Float> |lhs: &Operator<T>, rhs: &Operator<T>| -> Operator<T> {
    let (lhs_rows, lhs_cols) = lhs.shape().into();
    let (rhs_rows, rhs_cols) = rhs.shape().into();

    assert_eq!(lhs_cols, rhs_rows);
    let ks = lhs_cols;

    let mut op = Operator::new_with_shape((lhs_cols, rhs_rows).into());

    for row in 0..lhs_rows {
        for col in 0..rhs_cols {
                op[(row, col)] = (0..ks).map(|k| lhs[(row, k)] * rhs[(k, col)]).sum();
        }
    }

    op
});

auto_ops::impl_op_ex!(* <T: Float> |lhs: &Ket<T>, rhs: Complex<T>| -> Ket<T> {
    Ket { inner: lhs.inner.iter().map(|a| *a * rhs).collect() }
});

auto_ops::impl_op_ex!(* <T: Float> |lhs: &Bra<T>, rhs: Complex<T>| -> Bra<T> {
    Bra { inner: lhs.inner.iter().map(|a| *a * rhs).collect() }
});

auto_ops::impl_op_ex!(* <T: Float> |lhs: &Operator<T>, rhs: Complex<T>| -> Operator<T> {
    Operator { shape: lhs.shape, inner: lhs.inner.iter().map(|a| *a * rhs).collect() }
});

auto_ops::impl_op_ex!(* <T: Float> |rhs: Complex<T>, lhs: &Ket<T>| -> Ket<T> {
    Ket { inner: lhs.inner.iter().map(|a| *a * rhs).collect() }
});

auto_ops::impl_op_ex!(* <T: Float> |rhs: Complex<T>, lhs: &Bra<T>| -> Bra<T> {
    Bra { inner: lhs.inner.iter().map(|a| *a * rhs).collect() }
});

auto_ops::impl_op_ex!(* <T: Float> |rhs: Complex<T>, lhs: &Operator<T>| -> Operator<T> {
    Operator { shape: lhs.shape, inner: lhs.inner.iter().map(|a| *a * rhs).collect() }
});
