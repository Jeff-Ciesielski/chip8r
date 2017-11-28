mod chip8;

use std::{thread, time, env};
use std::fs::File;
use std::io::prelude::*;
use std::process;


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

    loop {
        core.tick();
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }



}
