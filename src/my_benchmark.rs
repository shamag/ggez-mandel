
mod lib;
mod constants;
mod opencl;


use criterion::{black_box, criterion_group, criterion_main, Criterion, Fun};
use constants::*;
use rayon::prelude::*;
use std::{ops};
use num::Complex;
use lib::{escapes, compute_mandel, generate};


fn test_escape () {
    let ratio = 1.0;
    let zoom = 1.0;
    let center_x = -1.0;
    let center_y = 0.0;
    let min_x = center_x - (zoom / 2.0);
    let min_y = center_y - (zoom / 2.0 / ratio);
    let iterations = 100;
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

fn test_escape_simd () {
    let dims = (2000, 2000);
    let xr = std::ops::Range{start: -1.0, end: 0.5};
    let yr = std::ops::Range{start: -1.0, end: 1.0};
    let _result = generate(dims, xr, yr, 1500);
}

fn test_escape_opencl () {
    let dims = (2000, 2000);
    let xr = std::ops::Range{start: -1.0, end: 0.5};
    let yr = std::ops::Range{start: -1.0, end: 1.0};
    let _result = opencl::generate(dims.0, dims.1, 1500);
}

//fn test_escape_simd_iter () {
//    let dims = (2000, 2000);
//    let xr = std::ops::Range{start: -2.0, end: 1.25};
//    let yr = std::ops::Range{start: -1.25, end: 1.25};
//    let _result = simd_par::generate(dims, xr, yr);
//}

fn test_escape_no_complex () {
    let ratio = 1.0;
    let zoom = 1.0;
    let center_x = -1.0;
    let center_y = 0.0;
    let min_x = center_x - (zoom / 2.0);
    let min_y = center_y - (zoom / 2.0 / ratio);
    let iterations = 100;
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
    let iterations = 100;
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
    //let mand_slow = Fun::new("no_complex", |b,_i| b.iter(|| test_escape_no_complex()));
    let mand_par = Fun::new("par", |b, _i| b.iter(|| test_escape()));
    let mand_simd = Fun::new("simd", |b, _i| b.iter(|| test_escape_simd()));
    let mand_opencl = Fun::new("opencl", |b, _i| b.iter(|| test_escape_opencl()));

    let functions = vec![mand_par, mand_simd, mand_opencl];

    c.bench_functions("Mandelbrot", functions, 20);
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