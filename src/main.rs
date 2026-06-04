mod game;
mod ui;
mod terminal;
use game::map::Map;
use game::map::Tile;
use game::player::Player;
use game::position::Position;
use game::state::State;
use ui::render::View;
use game::map_generator::{MapGenerator,MapGeneratorConfig};

use crate::game::direction;
use crate::game::input;
use crate::game::input::InputAction;
use crate::game::player;
use crate::terminal::TerminalGuard;
fn main() -> std::io::Result<()> {
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
    let _terminal = TerminalGuard::new()?;
    let mut state = State::new(generator_result.map, Player::new(generator_result.player_start));
    let view = View;
    loop {
        view.render(&state)?;
        match input::read_input()?{
            InputAction::Move(direction) => state.move_player(direction),
            InputAction::Quit => break,
            InputAction::None => {}
        }
    }

    // view.render(&state)?;
    // std::thread::sleep(std::time::Duration::from_secs(30));

    Ok(())
    
}
