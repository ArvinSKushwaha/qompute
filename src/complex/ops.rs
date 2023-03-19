use num::Zero;

use crate::complex::{braket::ComplexObject, Complex, Float};

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
    assert_eq!(lhs.shape().width, rhs.shape().height);
    lhs.inner.iter().zip(rhs.inner.iter()).map(|(a, b)| *a * *b).fold(Complex::<T>::zero(), |a, b| { a + b })
});

auto_ops::impl_op_ex!(* <T: Float> |lhs: &Ket<T>, rhs: &Bra<T>| -> Operator<T> {
    debug_assert_eq!(lhs.shape().width, rhs.shape().height);
    let mut op = Operator::new((rhs.height(), lhs.width()).into());

    (0..op.height()) // For each row...
        .into_iter()
        .zip(std::iter::repeat(0..op.width()).flatten()) // Traverse width
        .zip(op.inner.iter_mut())
        .for_each(|((i, j), v)| *v = lhs[i] * rhs[j]); // And compute

    op
});

auto_ops::impl_op_ex!(* <T: Float> |lhs: &Operator<T>, rhs: &Operator<T>| -> Operator<T> {
    debug_assert_eq!(lhs.width(), rhs.height());
    let mut op = Operator::new((rhs.height(), lhs.width()).into());

    (0..op.height()) // For each row...
        .into_iter()
        .zip(std::iter::repeat(0..op.width()).flatten()) // Traverse width
        .zip(op.inner.iter_mut())
        .for_each(|((i, j), v)| {
            *v = (0..lhs.height()).map(|k| lhs[(i, k)] * rhs[(k, j)]).fold(Complex::<T>::zero(), |a, b| a + b);
        }); // And compute

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
