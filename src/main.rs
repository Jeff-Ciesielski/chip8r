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
const CORE_FREQ: u64 = 840;

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
                Event::KeyDown { keycode: Some(Keycode::Q), ..} => core.set_key(0),
                Event::KeyDown { keycode: Some(Keycode::W), ..} => core.set_key(1),
                Event::KeyDown { keycode: Some(Keycode::E), ..} => core.set_key(2),
                Event::KeyDown { keycode: Some(Keycode::R), ..} => core.set_key(3),
                Event::KeyDown { keycode: Some(Keycode::A), ..} => core.set_key(4),
                Event::KeyDown { keycode: Some(Keycode::S), ..} => core.set_key(5),
                Event::KeyDown { keycode: Some(Keycode::D), ..} => core.set_key(6),
                Event::KeyDown { keycode: Some(Keycode::F), ..} => core.set_key(7),
                Event::KeyDown { keycode: Some(Keycode::Z), ..} => core.set_key(8),
                Event::KeyDown { keycode: Some(Keycode::X), ..} => core.set_key(9),
                Event::KeyDown { keycode: Some(Keycode::C), ..} => core.set_key(10),
                Event::KeyDown { keycode: Some(Keycode::V), ..} => core.set_key(11),
                Event::KeyDown { keycode: Some(Keycode::U), ..} => core.set_key(12),
                Event::KeyDown { keycode: Some(Keycode::I), ..} => core.set_key(13),
                Event::KeyDown { keycode: Some(Keycode::O), ..} => core.set_key(14),
                Event::KeyDown { keycode: Some(Keycode::P), ..} => core.set_key(15),
                Event::KeyUp { keycode: Some(Keycode::Q), ..} => core.clear_key(0),
                Event::KeyUp { keycode: Some(Keycode::W), ..} => core.clear_key(1),
                Event::KeyUp { keycode: Some(Keycode::E), ..} => core.clear_key(2),
                Event::KeyUp { keycode: Some(Keycode::R), ..} => core.clear_key(3),
                Event::KeyUp { keycode: Some(Keycode::A), ..} => core.clear_key(4),
                Event::KeyUp { keycode: Some(Keycode::S), ..} => core.clear_key(5),
                Event::KeyUp { keycode: Some(Keycode::D), ..} => core.clear_key(6),
                Event::KeyUp { keycode: Some(Keycode::F), ..} => core.clear_key(7),
                Event::KeyUp { keycode: Some(Keycode::Z), ..} => core.clear_key(8),
                Event::KeyUp { keycode: Some(Keycode::X), ..} => core.clear_key(9),
                Event::KeyUp { keycode: Some(Keycode::C), ..} => core.clear_key(10),
                Event::KeyUp { keycode: Some(Keycode::V), ..} => core.clear_key(11),
                Event::KeyUp { keycode: Some(Keycode::U), ..} => core.clear_key(12),
                Event::KeyUp { keycode: Some(Keycode::I), ..} => core.clear_key(13),
                Event::KeyUp { keycode: Some(Keycode::O), ..} => core.clear_key(14),
                Event::KeyUp { keycode: Some(Keycode::P), ..} => core.clear_key(15),
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::from_millis(1000/CORE_FREQ));

        // The rest of the game loop goes here...
        core.tick();
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();
        draw_frame_buffer(& mut canvas, core.frame_buffer);
        canvas.present();
    }



}
