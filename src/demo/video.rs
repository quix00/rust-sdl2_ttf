use std::path::Path;

use sdl2;
use sdl2_ttf;

static SCREEN_WIDTH : i32 = 800;
static SCREEN_HEIGHT : i32 = 600;

// fail when error
macro_rules! trying(
    ($e:expr) => (match $e { Ok(e) => e, Err(e) => panic!("failed: {}", e) })
);

// hadle the annoying Rect i32
macro_rules! rect(
    ($x:expr, $y:expr, $w:expr, $h:expr) => (
        sdl2::rect::Rect::new($x as i32, $y as i32, $w as i32, $h as i32)
    )
);

pub fn main(filename: &Path) {
    let context = sdl2::init(sdl2::INIT_VIDEO).unwrap();
    sdl2_ttf::init();

    let window = trying!(sdl2::video::Window::new(
            "rust-sdl2 demo: Video", sdl2::video::WindowPos::PosCentered,
            sdl2::video::WindowPos::PosCentered, SCREEN_WIDTH, SCREEN_HEIGHT, sdl2::video::OPENGL));

    let renderer = trying!(sdl2::render::Renderer::from_window(
            window, sdl2::render::RenderDriverIndex::Auto, sdl2::render::ACCELERATED));

    // Load a font
    let font = trying!(sdl2_ttf::Font::from_file(filename, 128));

    // render a surface, and convert it to a texture bound to the renderer
    let surface = trying!(font.render_str_blended("Hello Rust!", sdl2::pixels::Color::RGBA(255, 0, 0, 255)));
    let texture = trying!(renderer.create_texture_from_surface(&surface));

    let _ = renderer.drawer().set_draw_color(sdl2::pixels::Color::RGBA(195, 217, 255, 255));
    let _ = renderer.drawer().clear();

    let result = texture.query();
    let (w, h) = (result.width, result.height);
    let _ = renderer.drawer().copy(&texture, None, Some(rect!((SCREEN_WIDTH - w)/ 2, (SCREEN_HEIGHT - h)/ 2, w, h)));

    renderer.drawer().present();
    let mut event_pump = context.event_pump();

    'main : loop {
        'event : loop {
            for event in event_pump.poll_iter() {
                match event {
                    sdl2::event::Event::Quit{..} => break 'main,
                    sdl2::event::Event::KeyDown{keycode: key, ..} => {
                        if key == sdl2::keycode::KeyCode::Escape {
                            break 'main
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    sdl2_ttf::quit();
}
