
use crate::renderer::{MandelbrotRenderer};
use std::error::Error;
use packed_simd::*;
use rayon::prelude::*;

pub struct SIMDMandelbrot{
    dims: (usize, usize)
}

#[derive(Copy, Clone)]
struct Complexx8 {
    real: f64x8,
    imag: f64x8
}
impl Complexx8 {
    #[inline]
    fn escapes(self, threshold: f64, limit: usize) -> u64x8 {
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
            z = z.next_point(self);
        }
        count
    }
    fn next_point(self, start: Complexx8) -> Complexx8 {
        let Complexx8 { real: c_x, imag: c_y } = start;
        let Complexx8 { real: x, imag: y } = self;

        let xx = x * x;
        let yy = y * y;
        let xy = x * y;

        let new_x = c_x + (xx - yy);
        let new_y = c_y + (xy + xy);

        Complexx8 { real: new_x, imag: new_y}
    }
}


impl MandelbrotRenderer for SIMDMandelbrot {
    fn new(dims: (usize, usize)) -> SIMDMandelbrot {
       SIMDMandelbrot{dims}
    }
    fn render(&self, xr: std::ops::Range<f64>, yr: std::ops::Range<f64>, limit: usize) ->Result<Vec<u64>, Box<dyn Error>> {
        let (width, height) = self.dims;

        let block_size = f64x8::lanes();

        assert_eq!(
            width % block_size,
            0,
            "image width = {} is not divisible by the number of vector lanes = {}",
            width,
            block_size,
        );

        let width_in_blocks = width / block_size;

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
                *count = z.escapes(4.0, limit);
            });
        });

        let result = unsafe {
            let mut out: Vec<u64> = std::mem::transmute(out);
            // This is safe, we're transmuting from a more-aligned type to a
            // less-aligned one.
            out.set_len(width * height);
            out
        };
        Ok(result)
    }
}





//#[cfg(test)]
//mod test {
//    use super::*;
//    #[test]
//    fn test_add() {
//        let renderer= SIMDMandelbrot::new((300, 300));
//        let dims = (2000, 2000);
//        let xr = std::ops::Range{start: -1., end: -2.};
//        let yr = std::ops::Range{start: -1., end: -2.};
//        assert_eq!(renderer.generate(dims, xr, yr, 100).expect("error"), vec![1,1]);
//    }
//}