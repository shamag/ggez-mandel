use num::Complex;
use packed_simd::*;
use std::{io, ops};
use rayon::prelude::*;



#[derive(Copy, Clone)]
struct Complexx8 {
    real: f64x8,
    imag: f64x8
}

impl Complexx8 {
    #[inline]
    fn escapes(self, threshold: f64, limit: usize) -> u64x8 {
        //let Self { real: x, imag: y, threshold , limit} = *self;
        let mut count = u64x8::splat(0);
        let mut z = self;
        for _ in 0..limit {
            let x = z.real;
            let y = z.imag;

            let xx = x * x;
            let yy = y * y;
            let sum = xx + yy;

            let escapes = sum.le(f64x8::splat(threshold));
            if escapes.none() {
                break
            }
            count += escapes.select(u64x8::splat(1), u64x8::splat(0));
            z = z.nextPoint(self);
        }
        count
    }
    fn nextPoint(self, start: Complexx8) -> Complexx8 {
        let Complexx8 { real: c_x, imag: c_y } = start;
        let Complexx8 { real: x, imag: y } = self;

        let xx = x * x;
        let yy = y * y;
        let xy = x * y;

        let new_x = c_x + (xx - yy);
        let new_y = c_y + (xy + xy);

        return Complexx8 { real: new_x, imag: new_y};

        //Sself.current)
    }
}

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
#[allow(dead_code)]
pub type Dimensions = (usize, usize);
#[allow(dead_code)]
pub fn generate(dims: Dimensions, xr: Range, yr: Range, iterations: usize) ->Vec<u64> {
    let (width, height) = dims;

    let block_size = f64x8::lanes();

    assert_eq!(
        width % block_size,
        0,
        "image width = {} is not divisible by the number of vector lanes = {}",
        width,
        block_size,
    );

    let width_in_blocks = width / block_size;

    // The initial X values are the same for every row.
    let xs = unsafe {
        let dx = (xr.end - xr.start) / (width as f64);
        let mut buf: Vec<f64x8> = vec![f64x8::splat(0.); width_in_blocks];

        std::slice::from_raw_parts_mut(buf.as_mut_ptr() as *mut f64, width)
            .iter_mut()
            .enumerate()
            .for_each(|(j, x)| {
                *x = xr.start + dx * (j as f64);
            });

        buf
    };

    let dy = (yr.end - yr.start) / (height as f64);

    let len = width_in_blocks * height;
    let mut out = Vec::with_capacity(len);
    unsafe {
        out.set_len(len);
    }

    out.par_chunks_mut(width_in_blocks).enumerate().for_each(|(i, row)| {
        let y = f64x8::splat(yr.start + dy * (i as f64));
        row.iter_mut().enumerate().for_each(|(j, count)| {
            let x = xs[j];
            let z = Complexx8 { real: x, imag: y };
            *count = z.escapes(4.0, iterations);
        });
    });

    unsafe {
        let mut out: Vec<u64> = std::mem::transmute(out);
        // This is safe, we're transmuting from a more-aligned type to a
        // less-aligned one.
        out.set_len(width * height);
        out
    }
}

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

//pub struct MandelbrotIterator{
//    start: Complex,
//    current: Complex
//}