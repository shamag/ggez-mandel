//! The simplest possible example that does something.
mod constants;
mod lib;

use ggez;
use ggez::event;
use ggez::graphics;
use ggez::timer;
use ggez::nalgebra as na;
use ggez::{conf::*, Context, GameResult, mint,  graphics::*};
use constants::*;
use rayon::prelude::*;
use num::Complex;
use lib::escapes;





fn get_color(count: &Option<usize>, palette: u8) -> Vec<u8> {
    let palette1 = vec![
        vec![0x18, 0x4d, 0x68, 255],
        vec![0x31, 0x80, 0x9f, 255],
        vec![0xfb, 0x9c, 0x6c, 255],
        vec![0xd5, 0x51, 0x21, 255],
        vec![0xcf, 0xe9, 0x90, 255],
        vec![0xea, 0xfb, 0xc5, 255]
    ];
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

    match count {
        None => match palette {
            1 => palette1[0].clone(),
            2 => palette2[0].clone(),
            _  => palette1[0].clone()
        },
        Some(count) => {
            match palette {
                1 => {
                    match count {
                        0..=2 => palette1[5].clone(),
                        3..=5 => palette1[4].clone(),
                        6..=100 => palette1[3].clone(),
                        101..=150 => palette1[2].clone(),
                        _ => palette1[1].clone()
                    }
                    // palette1[ 5- (count) *5/LIMIT as usize].clone()
                },
                2 => {
                    let x = *count as f64/LIMIT as f64;
                    let tr = x * 14.0 + 1.0;
                    palette2[tr as usize].clone()
                },
                _ => palette1[count % 6].clone()
            }
        }
    }



}

struct MainState {
    fractal_buffer: Vec<u8>,
    fractal_rendered: bool,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let initial_buffer = Vec::with_capacity((WINDOW_WIDTH as usize * WINDOW_HEIGHT as usize * 4) as usize);
        let s = MainState { fractal_buffer: initial_buffer, fractal_rendered: false };
        Ok(s)
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
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());
        let ratio = WINDOW_WIDTH as f64 / WINDOW_HEIGHT as f64;
        let zoom = ZOOM;
        let center_x = FRACTAL_CENTER_X;
        let center_y = 0.0- FRACTAL_CENTER_Y;
        let min_x = center_x - (zoom / 2.0);
        let min_y = center_y - (zoom / 2.0 / ratio);
        let iterations = LIMIT;
        if !self.fractal_rendered {
            let colors = (0..(constants::WINDOW_WIDTH  * constants::WINDOW_HEIGHT) as usize)
                .into_par_iter()
                .map(|idx| {
                    let x = idx % (constants::WINDOW_WIDTH as usize );
                    let y = idx / (constants::WINDOW_WIDTH as usize);
                    let point = Complex{re: min_x + x as f64 / constants::WINDOW_WIDTH as f64 * zoom, im: min_y + y as f64 / constants::WINDOW_HEIGHT as f64 * zoom / ratio};
                    escapes(
                        point,
                        iterations as u32,
                    )
                })
                .collect::<Vec<Option<usize>>>();
            let mut max = 0 as usize;
            let mut min = std::usize::MAX;

            colors.iter().for_each(|color| {
                match color {
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
                get_color(item, 2)
            }).collect::<Vec<u8>>();

            self.fractal_buffer = buffer;
        }
        self.fractal_rendered = true;
        let fractal = graphics::Image::from_rgba8(
            ctx,
            constants::WINDOW_WIDTH as u16,
            constants::WINDOW_HEIGHT as u16,
            //&buffer
            &self.fractal_buffer
        ).unwrap();
        let scale: mint::Vector2<f32> = mint::Vector2 { x: 1.0, y: 1.0};
        let point: na::Point2<f32> = na::Point2::new(0.0, 0.0);
        graphics::draw(ctx, &fractal, DrawParam::default().scale(scale).dest(point))?;

        graphics::present(ctx)?;
        Ok(())
    }
}


pub fn main() -> GameResult {
    let app_config = ggez::conf::Conf {
        window_mode: WindowMode {
            width: constants::WINDOW_WIDTH as f32,
            height: constants::WINDOW_HEIGHT as f32,
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


