#[macro_use]
pub mod complex;

#[cfg(test)]
mod tests {
    use num::Complex;

    #[test]
    fn test_cmpx_macro() {
        let x: Complex<f32> = cmpx!();
        let y: Complex<f64> = cmpx!();

        assert_eq!((x.re, x.im), (0., 0.));
        assert_eq!((y.re, y.im), (0., 0.));

        let z = cmpx!(1. + 2. j);
        assert_eq!((z.re, z.im), (1., 2.));

        let w = (x + z) / z;
        assert_eq!((w.re, w.im), (1., 0.));
    }
}
