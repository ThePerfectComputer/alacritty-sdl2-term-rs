use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::Sdl;
use std::sync::OnceLock;

use crate::Matrix::Matrix;

const SCREEN_WIDTH: u32 = 924;
const SCREEN_HEIGHT: u32 = 600;
const NUM_COLS: u32 = 80;
const NUM_ROWS: u32 = 24;
const CELL_WIDTH: u32 = SCREEN_WIDTH / NUM_COLS;
const CELL_HEIGHT: u32 = SCREEN_HEIGHT / NUM_ROWS;
const FONT_SIZE: u16 = 16;

static CELL: OnceLock<Sdl2TtfContext> = OnceLock::new();

pub struct TermDisplay {
    sdl_context: Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    ttf_context: &'static Sdl2TtfContext,
    font: Font<'static, 'static>,
    canvas: Canvas<Window>,
    matrix: Matrix
}

impl TermDisplay {
    pub fn new() -> Result<Self, String> {
        // Initialize SDL2
        let sdl_context = sdl2::init()?;
        let video_subsystem = sdl_context.video()?;

        let ttf_context = CELL.get_or_init(|| {
            let ttf_context = sdl2::ttf::init().unwrap();
            ttf_context
        });
        let font = ttf_context
                .load_font(
                    "/System/Library/Fonts/Supplemental/Courier New.ttf",
                    FONT_SIZE)
                .map_err(|e| e.to_string())?;

        // Create a window
        let window = video_subsystem
            .window("SDL2 Term", SCREEN_WIDTH, SCREEN_HEIGHT)
            .position_centered()
            .build()
            .map_err(|e| e.to_string())?;

        let canvas = window
            .into_canvas()
            .present_vsync()
            .build()
            .map_err(|e| e.to_string())?;
        let mut matrix = Matrix::new(NUM_ROWS, NUM_COLS);
        matrix.set_to_content2();

        Ok(TermDisplay {
            sdl_context,
            video_subsystem,
            ttf_context,
            font,
            canvas,
            matrix
            }
        )
    }

    pub fn test_render(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(Color::BLACK);
        self.canvas.clear();

        let texture_creator = self.canvas.texture_creator();

        for row in 0..NUM_ROWS {
            for col in 0..NUM_COLS {
                let character = self.matrix.content[row as usize][col as usize]
                    .map_or(' ', |c| c);

                let surface = self
                    .font
                    .render(&character.to_string())
                    .blended(Color::WHITE)
                    .map_err(|e| e.to_string())?;

                let texture = texture_creator
                    .create_texture_from_surface(&surface)
                    .map_err(|e| e.to_string())?;

                let target = Rect::new(
                    (col * CELL_WIDTH) as i32,
                    (row * CELL_HEIGHT) as i32,
                    CELL_WIDTH,
                    CELL_HEIGHT,
                );

                self.canvas.copy(&texture, None, target)?;
            }
        }

        self.canvas.present();
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), String> {
        let mut event_pump = self.sdl_context.event_pump()?;
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
}