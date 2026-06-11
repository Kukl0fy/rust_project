mod game;
mod gui;
mod terminal;
mod ui;

use game::character_class::CharacterClass;
use game::input::InputAction;
use game::level_generator::LevelGenerator;
use game::player::Player;
use game::state::State;
use terminal::TerminalGuard;
use ui::render::View;

use crate::game::input;

fn main() -> std::io::Result<()> {
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
    let _terminal = TerminalGuard::new()?;
    let mut state = State::new(
        map,
        Player::new(player_start, CharacterClass::Warrior),
        monsters,
        chests,
    );
    let mut view = View::new();
    loop {
        view.render(&state)?;
        match input::read_input()? {
            InputAction::Move(direction) => state.move_player(direction),
            InputAction::Quit => break,
            InputAction::None => {}
        }
    }

    Ok(())
}
