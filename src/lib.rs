use num::Complex;
use packed_simd::*;
use std::{io, ops};
use rayon::prelude::*;

#[inline]
pub fn escapes(c: Complex<f64>, limit: u32) -> Option<usize> {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return Some(i as usize);
        }
    }
    None
}



pub fn compute_mandel(x: f64, y: f64, iterations: f64) -> Option<usize> {
    let (mut z, mut c) = (x, y);
    let mut fc;
    let mut pc;
    for i in 0..iterations as i32 {
        fc = z * z - c * c + x;
        pc = 2.0 * z * c + y;
        z = fc;
        c = pc;
        if z*z + c*c > 4. {
            return Some(i as usize);
        }
    }
    None
}
type Range = ops::Range<f64>;

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(f64x8::lanes(), 8);
    }
    #[test]
    fn test_mandel() {

        let vec = vec![100, 100, 100, 100];
        let count = compute_mandel(0.5, 0., 100.0);
        let z = Complexx8{
            real: f64x8::splat(1.1041),
            imag: f64x8::splat(1.983)
        };
        let count = z.escapes(4.0, 100);
        let arr = [0,0,0,0];
        let dims = (8, 8);
        let xr = Range{start: -0.2, end: -0.1};
        let yr = Range{start: -1.0, end: -0.9};
        assert_eq!(generate(dims, xr, yr, 100), vec![6, 7, 8, 9, 10, 15, 12, 9, 6, 7, 8, 8, 10, 14, 23, 14, 7, 7, 7, 8, 9, 13, 14, 16, 7, 7, 7, 8, 9, 10, 12, 17, 7, 7, 7, 8, 9, 10, 11, 13, 7, 7, 8, 9, 9, 10, 12, 15, 7, 7, 8, 9, 10, 11, 19, 24, 7, 8, 9, 11, 16, 13, 15, 34]);
    }
}
