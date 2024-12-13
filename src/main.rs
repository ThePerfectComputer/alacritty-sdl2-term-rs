use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::Font;
use sdl2::video::Window;
use sdl2::Sdl;

mod Matrix;

const SCREEN_WIDTH: u32 = 924;
const SCREEN_HEIGHT: u32 = 600;
const NUM_COLS: u32 = 80;
const NUM_ROWS: u32 = 24;
const CELL_WIDTH: u32 = SCREEN_WIDTH/NUM_COLS;
const CELL_HEIGHT: u32 = SCREEN_HEIGHT/NUM_ROWS;
const FONT_SIZE: u16 = 16;

// struct TermDisplay {
//     sdl_context: Sdl,
//     video_subsystem: sdl2::VideoSubsystem,
//     ttf_context: sdl2::ttf::Sdl2TtfContext,
//     font: Font,
//     window: Window,
// }

fn main() -> Result<(), String> {
    // Initialize SDL2
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // Initialize SDL2 TTF
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font = ttf_context.load_font(
        "/System/Library/Fonts/Supplemental/Courier New.ttf",
        FONT_SIZE)?;

    // Create a window
    let window = video_subsystem
        .window("SDL2 Term", SCREEN_WIDTH, SCREEN_HEIGHT)
        .position_centered()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::BLACK);
    canvas.clear();

    // Draw the grid
    let texture_creator = canvas.texture_creator();

    for row in 0..(NUM_ROWS) {
        for col in 0..(NUM_COLS) {
            let character = if (row + col) % 2 == 0 { 'A' } else { 'B' };

            let surface = font
                .render(&character.to_string())
                .blended(Color::WHITE)
                .map_err(|e| e.to_string())?;

            let texture = texture_creator
                .create_texture_from_surface(&surface)
                .map_err(|e| e.to_string())?;

            let target = Rect::new(
                ((col+2) * CELL_WIDTH) as i32,
                (row * CELL_HEIGHT) as i32,
                CELL_WIDTH,
                CELL_HEIGHT,
            );

            canvas.copy(&texture, None, target)?;
        }
    }

    canvas.present();

    // Event pump to keep the window open
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit { .. }
                | sdl2::event::Event::KeyDown {
                    keycode: Some(sdl2::keyboard::Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
    }

    Ok(())
}
