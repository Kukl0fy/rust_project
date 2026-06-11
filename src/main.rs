mod game;
mod gui;
mod terminal;
mod ui;

use game::character_class::CharacterClass;
use game::level_generator::LevelGenerator;
use game::player::Player;
use game::state::State;
use gui::GuiRenderer;
use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Rust Pixel Dungeon".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let level_generator = LevelGenerator::new(
        80,  // width
        40,  // height
        4,   // room_min_width
        12,  // room_max_width
        4,   // room_min_height
        8,   // room_max_height
        25,  // max_rooms
        15,  // monster_count
    );

    let (map, monsters, player_start) = level_generator.generate_level().into_parts();
    let mut state = State::new(
        map,
        Player::new(player_start, CharacterClass::Warrior),
        monsters,
    );
    let renderer = GuiRenderer::new();

    loop {
        // Handle input
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        // Handle arrow keys for movement
        if is_key_pressed(KeyCode::Up) {
            state.move_player(game::direction::Direction::Up);
        }
        if is_key_pressed(KeyCode::Down) {
            state.move_player(game::direction::Direction::Down);
        }
        if is_key_pressed(KeyCode::Left) {
            state.move_player(game::direction::Direction::Left);
        }
        if is_key_pressed(KeyCode::Right) {
            state.move_player(game::direction::Direction::Right);
        }

        // Render the game
        renderer.render(&state);

        next_frame().await;
    }
}
