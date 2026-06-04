mod game;
mod ui;
mod terminal;
use game::map::Map;
use game::map::Tile;
use game::player::Player;
use game::position::Position;
use game::state::State;
use ui::render::View;

use crate::game::direction;
use crate::game::input;
use crate::game::input::InputAction;
use crate::game::player;
use crate::terminal::TerminalGuard;
fn main() -> std::io::Result<()> {
    let map = Map::new(vec![
        vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
        vec![
            Tile::Wall,
            Tile::Floor,
            Tile::Floor,
            Tile::Floor,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Floor,
            Tile::Floor,
            Tile::Floor,
            Tile::Wall,
        ],
        vec![
            Tile::Wall,
            Tile::Floor,
            Tile::Floor,
            Tile::Floor,
            Tile::Wall,
        ],
        vec![Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall, Tile::Wall],
    ]);
    let _terminal = TerminalGuard::new()?;
    let mut state = State::new(map, Player::new(Position { x: 2, y: 2 }));
    let view = View;
    loop {
        View.render(&state)?;
        match input::read_input()?{
            InputAction::Move(direction) => state.move_player(direction),
            InputAction::Quit => break,
            InputAction::None => {}
        }
    }
    Ok(())
    
}
