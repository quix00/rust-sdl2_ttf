#![crate_type = "bin"]

extern crate sdl2;
extern crate sdl2_ttf;

use std::os;

mod video;

fn main() {
    let args = os::args();
    let version = sdl2_ttf::get_linked_version();
    println!("linked sdl2_ttf: {}.{}.{}", version.major, version.minor, version.patch);
    if args.len() < 2 {
        println!("Usage: ./demo font.[ttf|ttc|fon]")
    } else {
        video::main(&Path::new(os::args()[1].to_string()));
    }
}
