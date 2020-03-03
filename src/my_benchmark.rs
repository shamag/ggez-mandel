
mod lib;
mod constants;
mod opencl;
mod single;
mod renderer;
mod multi;
mod simd;


use single::SingleMandelbrot;
use multi::MultiMandelbrot;
use simd::SIMDMandelbrot;
use opencl::OCLMandelbrot;
use renderer::MandelbrotRenderer;
use criterion::{criterion_group, criterion_main, Criterion, Fun};

fn compare_escapes(c: &mut Criterion) {
    let dims = (1000, 1000);
    let limit = 200;
    let limit2 = limit.clone();
    let limit3 = limit.clone();
    let limit4 = limit.clone();
    let renderer_single= SingleMandelbrot::new(dims);
    let renderer_opencl= OCLMandelbrot::new(dims);
    let renderer_multi= MultiMandelbrot::new(dims);
    let renderer_simd= SIMDMandelbrot::new(dims);
    let mand_single = Fun::new("single", move |b, _i| b.iter(|| renderer_single.render(std::ops::Range{start: -1.0, end: 0.5}, std::ops::Range{start: -1.0, end: 1.0}, limit3)));
    let mand_multi = Fun::new("multi", move |b, _i| b.iter(|| renderer_multi.render(std::ops::Range{start: -1.0, end: 0.5}, std::ops::Range{start: -1.0, end: 1.0}, limit2)));
    let mand_opencl = Fun::new("opencl", move |b, _i| b.iter(|| renderer_opencl.render(std::ops::Range{start: -1.0, end: 0.5}, std::ops::Range{start: -1.0, end: 1.0}, limit)));
    let mand_simd = Fun::new("simd", move |b, _i| b.iter(|| renderer_simd.render(std::ops::Range{start: -1.0, end: 0.5}, std::ops::Range{start: -1.0, end: 1.0}, limit4)));


    let functions = vec![mand_single, mand_multi, mand_simd, mand_opencl];

    c.bench_functions("Mandelbrot", functions, 10);
}

//pub fn criterion_benchmark(c: &mut Criterion) {
//
//    c.bench_function("escapes", |b| b.iter(|| test_escape()));
//}

fn setup() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = benches;
    config = setup();
    targets = compare_escapes
}
criterion_main!(benches);