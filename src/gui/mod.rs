use macroquad::prelude::*;
use crate::game::state::State;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const TILE_SIZE: f32 = 16.0;

pub fn conf() -> Conf {
    Conf {
        window_title: "Rust Pixel Dungeon".to_owned(),
        window_width: WINDOW_WIDTH as i32,
        window_height: WINDOW_HEIGHT as i32,
        window_resizable: false,
        ..Default::default()
    }
}

pub struct GuiRenderer;

impl GuiRenderer {
    pub fn new() -> Self {
        Self
    }

    pub fn render(&self, state: &State) {
        clear_background(BLACK);

        let map = state.map();
        
        // Draw map tiles
        for y in 0..map.height() {
            for x in 0..map.width() {
                let screen_x = x as f32 * TILE_SIZE;
                let screen_y = y as f32 * TILE_SIZE;
                
                // Draw a simple tile representation
                let char = state.char_at(crate::game::position::Position {
                    x: x as i32,
                    y: y as i32,
                });
                
                match char {
                    '#' => draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, GRAY),
                    '.' => draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, DARKGRAY),
                    '@' => draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, GREEN),
                    'G' => draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, RED),
                    _ => draw_rectangle(screen_x, screen_y, TILE_SIZE, TILE_SIZE, BLACK),
                }
                
                // Draw text character for now
                draw_text(&char.to_string(), screen_x + 2.0, screen_y + 12.0, 14.0, WHITE);
            }
        }
    }
}
