mod game;
mod ui;
mod terminal;
mod gui;

use game::player::Player;
use game::state::State;
use game::map_generator::{MapGenerator, MapGeneratorConfig};
use game::character_class::CharacterClass;
use crate::game::direction;
use crate::gui::{conf, GuiRenderer};
use macroquad::prelude::*;

#[macroquad::main(conf)]
async fn main() {
    let config = MapGeneratorConfig::new(
        80, // width
        40, // height
        4,  // room_min_width
        12, // room_max_width
        4,  // room_min_height
        8,  // room_max_height
        25, // max_rooms
    );

    let generator = MapGenerator::new(config);
    let generator_result = generator.generate_map();
    let mut state = State::new(
        generator_result.map,
        Player::new(generator_result.player_start, CharacterClass::Warrior),
    );
    let renderer = GuiRenderer::new();

    loop {
        renderer.render(&state);

        // Handle keyboard input
        if is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Up) {
            state.move_player(direction::Direction::Up);
        } else if is_key_pressed(KeyCode::A) || is_key_pressed(KeyCode::Left) {
            state.move_player(direction::Direction::Left);
        } else if is_key_pressed(KeyCode::S) || is_key_pressed(KeyCode::Down) {
            state.move_player(direction::Direction::Down);
        } else if is_key_pressed(KeyCode::D) || is_key_pressed(KeyCode::Right) {
            state.move_player(direction::Direction::Right);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
