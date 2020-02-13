use num::Complex;

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
        if z*z * c*c > 4. {
            return Some(i as usize);
        }
    }
    None
}