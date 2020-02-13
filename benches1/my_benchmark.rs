#[path="../src/lib.rs"]
mod lib;
#[path="../src/constants.rs"]
mod constants;

use lib::{escapes};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use constants::*;
use rayon::prelude::*;
use num::Complex;


fn test_escape () {
    let ratio = 1.0;
    let zoom = 1.0;
    let center_x = -1.0;
    let center_y = 0.0;
    let min_x = center_x - (zoom / 2.0);
    let min_y = center_y - (zoom / 2.0 / ratio);
    let iterations = 500;
    let colors = (0..(2000 * 2000) as usize)
        .into_par_iter()
        .map(|idx| {
            let x = idx % (2000 as usize );
            let y = idx / (2000 as usize);
            let point = Complex{re: min_x + x as f64 / 2000 as f64 * zoom as f64, im: min_y + y as f64 / 2000 as f64 * zoom / ratio};
            escapes(
                point,
                iterations as u32,
            )
        })
        .collect::<Vec<Option<usize>>>();
}

pub fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("escapes", |b| b.iter(|| test_escape()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);