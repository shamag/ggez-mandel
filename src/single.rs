
use crate::renderer::{MandelbrotRenderer};
use num::Complex;
use std::error::Error;

pub struct SingleMandelbrot{
    dims: (usize, usize)
}

impl MandelbrotRenderer for SingleMandelbrot {
    fn new(dims: (usize, usize)) -> SingleMandelbrot {
        SingleMandelbrot { dims }
    }
    fn render(&self, xr: std::ops::Range<f64>, yr: std::ops::Range<f64>, limit: usize) -> Result<Vec<u64>, Box<dyn Error>> {
        let (width, height) = self.dims;
        let colors = (0..(width * height) as usize)
            .map(|idx| {
                let x = idx % (width as usize) ;
                let y = idx / (width as usize);
                let dx = (xr.end - xr.start) / (width as f64);
                let dy = (yr.end - yr.start) / (height as f64);
                let point = Complex { re: xr.start + x as f64 * dx, im: yr.start + y as f64 * dy };
                escapes(
                    point,
                    limit as u64,
                )
            })
            .collect::<Vec<u64>>();
        Ok(colors)
    }
}

#[inline]
fn escapes(c: Complex<f64>, limit: u64) -> u64 {
    let mut z = Complex { re: 0.0, im: 0.0 };
    for i in 0..limit {
        z = z*z + c;
        if z.norm_sqr() > 4.0 {
            return i;
        }
    }
    limit
}

