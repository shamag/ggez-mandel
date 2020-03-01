use std::error::Error;

pub trait MandelbrotRenderer {
    fn new(dims: (usize, usize)) -> Self  where Self: Sized;
    fn render(&self, xr: std::ops::Range<f64>, yr: std::ops::Range<f64>, limit: usize) -> Result<Vec<u64>, Box<Error>>;
}