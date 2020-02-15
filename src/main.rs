//! The simplest possible example that does something.
mod constants;
mod lib;
mod opencl;

use rgsl;
use ggez;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::nalgebra as na;
use ggez::{conf::*, Context, GameResult, mint,  graphics::*, event::*};
use constants::*;
use rayon::prelude::*;
use num::Complex;
use lib::{escapes, generate};
use rgsl::{Spline, InterpAccel};

struct Splines {
    r: Spline,
    g: Spline,
    b: Spline,
    ra: InterpAccel,
    ga: InterpAccel,
    ba: InterpAccel,
}

fn get_splines() -> Splines {
    let palette2 = vec![
        vec![0x00, 0x00, 0x00, 255],
        vec![0x03, 0x26, 0x28, 255],
        vec![0x07, 0x3e, 0x1e, 255],
        vec![0x18, 0x55, 0x08, 255],
        vec![0x5f, 0x6e, 0x0f, 255],
        vec![0x84, 0x50, 0x19, 255],
        vec![0x9b, 0x30, 0x22, 255],
        vec![0xb4, 0x92, 0x2f, 255],
        vec![0x94, 0xca, 0x3d, 255],
        vec![0x4f, 0xd5, 0x51, 255],
        vec![0x66, 0xff, 0xb3, 255],
        vec![0x82, 0xc9, 0xe5, 255],
        vec![0x9d, 0xa3, 0xeb, 255],
        vec![0xd7, 0xb5, 0xf3, 255],
        vec![0xfd, 0xd6, 0xf6, 255],
        vec![0xff, 0xf0, 0xf2, 255],
    ];
    let mut x : [f64; 16] = [0.0; 16];
    let mut r : [f64; 16] = [0f64; 16];
    let mut g : [f64; 16] = [0f64; 16];
    let mut b : [f64; 16] = [0f64; 16];

    for i in 0usize..16usize {
        x[i] = i as f64/15.0;
        r[i] = palette2[i][0] as f64;
        g[i] = palette2[i][1] as f64;
        b[i] = palette2[i][2] as f64;
    }

    let mut acc_r = rgsl::InterpAccel::new();
    let mut acc_g = rgsl::InterpAccel::new();
    let mut acc_b = rgsl::InterpAccel::new();
    let spline_r = rgsl::Spline::new(&rgsl::InterpType::cspline(), 16).unwrap();
    let spline_g = rgsl::Spline::new(&rgsl::InterpType::cspline(), 16).unwrap();
    let spline_b = rgsl::Spline::new(&rgsl::InterpType::cspline(), 16).unwrap();

    spline_r.init(&x, &r);
    spline_g.init(&x, &g);
    spline_b.init(&x, &b);
    return Splines{
        r: spline_r,
        g: spline_g,
        b: spline_b,
        ra: acc_r,
        ga: acc_g,
        ba: acc_b
    }
}


struct MainState {
    fractal_buffer: Vec<u8>,
    fractal_rendered: bool,
    splines: Splines,
    zoom: f64,
    limit: f64,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let initial_buffer = Vec::with_capacity((WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize * 4) as usize);
        let s = MainState { fractal_buffer: initial_buffer, fractal_rendered: false, splines:  get_splines(), zoom: ZOOM, limit: LIMIT};
        Ok(s)
    }
    fn get_color(&mut self, count: &Option<usize>) -> Vec<u8>  {
        match count {
            None => vec![
                self.splines.r.eval(0.0, &mut self.splines.ra) as u8,
                self.splines.g.eval(0.0, &mut self.splines.ga) as u8,
                self.splines.b.eval(0.0, &mut self.splines.ba) as u8,
                255
            ].clone(),
            Some(count) => {
                let xi = 1 as f64 - (*count as f64/self.limit as f64);
                vec![
                    self.splines.r.eval(xi, &mut self.splines.ra) as u8,
                    self.splines.g.eval(xi,  &mut self.splines.ga) as u8,
                    self.splines.b.eval(xi, &mut self.splines.ba) as u8,
                    255
                ].clone()
            }
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if timer::ticks(ctx) % 100 == 0 {
            println!("Delta frame time: {:?} ", timer::delta(ctx));
            println!("Average FPS: {}", timer::fps(ctx));
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 0.0, 0.0, 1.0].into());
        let ratio = WINDOW_WIDTH as f64 / WINDOW_HEIGHT as f64;
        let zoom = self.zoom;
        let center_x = FRACTAL_CENTER_X;
        let center_y = 0.0 - FRACTAL_CENTER_Y;
        let width = zoom /2.0;
        let mut height = zoom /2.0/ratio;
        height = height;
        let min_x = center_x - (zoom / 2.0);
        let min_y = center_y - (zoom / 2.0 / ratio);
        let max_y = center_y + (zoom / 2.0 / ratio);
        let iterations = self.limit;
        if !self.fractal_rendered {
            let dims = (WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize);
            let xr = std::ops::Range{start: FRACTAL_CENTER_X - width, end: FRACTAL_CENTER_X + width};
            let yr = std::ops::Range{start: FRACTAL_CENTER_Y - height, end: FRACTAL_CENTER_Y + height};
            // let colors = generate(dims, xr, yr, iterations as usize);
            let colors = opencl::generate(WINDOW_WIDTH as usize, WINDOW_HEIGHT as usize).unwrap();
//            let colors = (0..(WINDOW_WIDTH  * WINDOW_HEIGHT) as usize)
//                .into_par_iter()
//                .map(|idx| {
//                    let x = idx % (WINDOW_WIDTH as usize );
//                    let y = idx / (WINDOW_WIDTH as usize);
//                    let point = Complex{re: min_x + x as f64 / WINDOW_WIDTH as f64 * zoom, im: min_y + y as f64 / WINDOW_HEIGHT as f64 * zoom / ratio};
//                    escapes(
//                        point,
//                        iterations as u32,
//                    )
//                })
//                .collect::<Vec<Option<usize>>>();
            let mut max = 0 as u64;
            let mut min = std::u64::MAX;

            colors.iter().for_each(|color| {
                let wrapped = Some(color);
                match wrapped {
                    None => {},
                    Some(num) => {
                        if *num > max {
                            max = *num;
                        }
                        if *num < min {
                            min = *num;
                        }
                    }
                }

            });
            println!("{}, {}", min, max);

            let buffer= colors.iter().flat_map(|item| {
                let max_iter = (iterations -1.0) as u64;
                let mut wrapped = if *item < (iterations-1.0) as u64{
                    Some(*item as usize)
                } else {
                    None
                };

                self.get_color(&wrapped)
            }).collect::<Vec<u8>>();

            self.fractal_buffer = buffer;
        }
        self.fractal_rendered = true;
        let fractal = graphics::Image::from_rgba8(
            ctx,
            WINDOW_WIDTH as u16,
            WINDOW_HEIGHT as u16,
            //&buffer
            &self.fractal_buffer
        ).unwrap();
        let scale: mint::Vector2<f32> = mint::Vector2 { x: 1.0, y: 1.0};
        let point: na::Point2<f32> = na::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &fractal, DrawParam::default().scale(scale).dest(point))?;

        graphics::present(ctx)?;
        Ok(())
    }
    fn key_up_event(&mut self, ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        if keycode == KeyCode::Z {
            self.zoom += 0.05 * self.zoom;
            self.fractal_rendered = false;
        }
        if keycode == KeyCode::X {
            self.zoom -= 0.05 * self.zoom;
            self.fractal_rendered = false;
        }
        if keycode == KeyCode::C {
            self.limit += 0.5 * self.limit;
            self.fractal_rendered = false;
        }
        if keycode == KeyCode::V {
            self.limit -= 0.5 * self.limit;
            self.fractal_rendered = false;
        }
    }
}


pub fn main() -> GameResult {
    let app_config = ggez::conf::Conf {
        window_mode: WindowMode {
            width: WINDOW_WIDTH as f32,
            height: WINDOW_HEIGHT as f32,
            borderless: false,
            fullscreen_type: FullscreenType::Windowed,
            resizable: false,
            maximized: false,
            ..WindowMode::default()
        },
        window_setup: WindowSetup {
            title: "Mandelbrot".to_string(),
            samples: NumSamples::Two,
            icon: "".to_owned(),
            vsync: false,
            srgb: true,
        },
        backend: Backend::default().gl().version(3, 2),
        modules: ModuleConf::default(),
    };
    let cb = ggez::ContextBuilder::new("super_simple", "ggez").conf(app_config);
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut MainState::new()?;
    event::run(ctx, event_loop, state)
}


