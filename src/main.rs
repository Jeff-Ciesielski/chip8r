mod chip8;
extern crate sdl2;

use sdl2::gfx::primitives::DrawRenderer;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::Canvas;
use sdl2::rect::Point;
use std::time::Duration;
use std::{env};
use std::fs::File;
use std::io::prelude::*;
use std::process;

const SCREEN_WIDTH: usize = 1280;
const SCREEN_HEIGHT: usize = 640;
const SCALING_FACTOR: usize = SCREEN_WIDTH / chip8::SCREEN_X;

fn draw_frame_buffer(canvas: &mut Canvas<sdl2::video::Window>, pixels: [u8; 256]) {
    for x in 0..(chip8::SCREEN_X / 8) {
        for y in 0..chip8::SCREEN_Y {
            let bit_row = pixels[x * chip8::SCREEN_Y + y];
            if bit_row == 0 {
                continue;
            }

            for i in 0..8 {
                let mask = 0x80 >> i;
                if bit_row & mask > 0 {
                    canvas.pixel((8 * x + i) as i16, y as i16, Color::RGB(0, 255, 0));
                }
            }
        }
    }
}

fn main() {

    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Please provide a rom file to load");
        process::exit(1);
    }

    let mut f = File::open(&args[1]).expect("file not found");

    let mut contents: Vec<u8> = Vec::new();
    f.read_to_end(&mut contents)
        .expect("something went wrong reading the file");

    let mut core = chip8::Core::new();
    core.load_rom(&contents);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();


    let window = video_subsystem.window("CHIP_8 Emulator",
                                        SCREEN_WIDTH as u32,
                                        SCREEN_HEIGHT as u32)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_scale(SCALING_FACTOR as f32, SCALING_FACTOR as f32);
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
        core.tick();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_frame_buffer(& mut canvas, core.frame_buffer);
        canvas.present();
    }



}
