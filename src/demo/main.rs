#![crate_type = "bin"]

extern crate sdl2;
extern crate sdl2_ttf;

use std::path::Path;
use std::env;

mod video;

fn main() {
    let mut args = env::args();
    println!("linked sdl2_ttf: {}", sdl2_ttf::get_linked_version());

    if args.len() < 2 {
        println!("Usage: ./demo font.[ttf|ttc|fon]")
    } else {
        video::main(&Path::new(&args.nth(1).unwrap()));
    }
}
