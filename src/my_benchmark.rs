
mod lib;
mod constants;


use criterion::{black_box, criterion_group, criterion_main, Criterion, Fun};
use constants::*;
use rayon::prelude::*;
use num::Complex;
use lib::{escapes, compute_mandel};


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

fn test_escape_no_complex () {
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
            compute_mandel(
                min_x + x as f64 / 2000 as f64 * zoom as f64,
                min_y + y as f64 / 2000 as f64 * zoom / ratio,
                iterations as f64,
            )
        })
        .collect::<Vec<Option<usize>>>();
}

fn test_escape_single () {
    let ratio = 1.0;
    let zoom = 1.0;
    let center_x = -1.0;
    let center_y = 0.0;
    let min_x = center_x - (zoom / 2.0);
    let min_y = center_y - (zoom / 2.0 / ratio);
    let iterations = 500;
    let colors = (0..(2000 * 2000) as usize)
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

fn compare_escapes(c: &mut Criterion) {
    let fib_slow = Fun::new("no_complex", |b,_i| b.iter(|| test_escape_no_complex()));
    let fib_fast = Fun::new("complex", |b, _i| b.iter(|| test_escape()));

    let functions = vec![fib_slow, fib_fast];

    c.bench_functions("Fibonacci", functions, 20);
}

pub fn criterion_benchmark(c: &mut Criterion) {

    c.bench_function("escapes", |b| b.iter(|| test_escape()));
}

fn setup() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = benches;
    config = setup();
    targets = compare_escapes
}
criterion_main!(benches);