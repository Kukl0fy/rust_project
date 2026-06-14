mod game;
mod gui;
mod terminal;
mod ui;

use game::character_class::CharacterClass;
use game::level_generator::LevelGenerator;
use game::player::Player;
use game::state::State;
use gui::{conf, GuiRenderer, Hud, Menu, MenuState, AssetManager};
use macroquad::prelude::*;

#[macroquad::main(conf)]
async fn main() {
    let level_generator = LevelGenerator::new(
        80,  // width
        40,  // height
        4,   // room_min_width
        12,  // room_max_width
        4,   // room_min_height
        8,   // room_max_height
        25,  // max_rooms
        1,   // min_monsters_per_room
        4,   // max_monsters_per_room
        0.4, // chest_spawn_chance
    );

    let (map, monsters, chests, player_start) = level_generator.generate_level().into_parts();
    let mut state = State::new(
        map,
        Player::new(player_start, CharacterClass::Warrior),
        monsters,
        chests,
    );

    let renderer = GuiRenderer::new();
    let hud = Hud::new(gui::WINDOW_WIDTH - 270.0, 10.0);
    let mut menu = Menu::new();
    let asset_manager = AssetManager::new().await;

    loop {
        match menu.current_state() {
            MenuState::MainMenu => {
                menu.render_main_menu();
                if is_key_pressed(KeyCode::Enter) {
                    menu.set_state(MenuState::Playing);
                }
                if is_key_pressed(KeyCode::Q) {
                    break;
                }
            }
            MenuState::Playing => {
                renderer.render(&state, &asset_manager);
                hud.render(&state);

                if is_key_pressed(KeyCode::Up) {
                    state.move_player(game::direction::Direction::Up);
                } else if is_key_pressed(KeyCode::Down) {
                    state.move_player(game::direction::Direction::Down);
                } else if is_key_pressed(KeyCode::Left) {
                    state.move_player(game::direction::Direction::Left);
                } else if is_key_pressed(KeyCode::Right) {
                    state.move_player(game::direction::Direction::Right);
                }

                if is_key_pressed(KeyCode::P) {
                    menu.set_state(MenuState::PauseMenu);
                }

                if is_key_pressed(KeyCode::Q) {
                    break;
                }
            }
            MenuState::PauseMenu => {
                renderer.render(&state, &asset_manager);
                hud.render(&state);
                menu.render_pause_menu();

                if is_key_pressed(KeyCode::P) {
                    menu.set_state(MenuState::Playing);
                }
                if is_key_pressed(KeyCode::M) {
                    menu.set_state(MenuState::MainMenu);
                }
                if is_key_pressed(KeyCode::Q) {
                    break;
                }
            }
            MenuState::GameOver => {
                menu.render_game_over(false);
                if is_key_pressed(KeyCode::Enter) {
                    menu.set_state(MenuState::MainMenu);
                }
                if is_key_pressed(KeyCode::Q) {
                    break;
                }
            }
        }

        next_frame().await;
    }
}
