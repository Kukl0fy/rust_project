use crate::game::position::Position;
use crate::game::state::{GameMode, State};

use std::io::{stdout, Write};

use crossterm::{
    cursor::MoveTo,
    execute,
    terminal,
};

const STATUS_LINES: usize = 4;

pub struct View {
    buffer: Vec<Vec<char>>,
    last_term_width: usize,
    last_term_height: usize,
}

impl View {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            last_term_width: 0,
            last_term_height: 0,
        }
    }

    pub fn render(&mut self, state: &State) -> std::io::Result<()> {
        let (term_w_u16, term_h_u16) = terminal::size()?;
        let term_w = term_w_u16 as usize;
        let term_h = term_h_u16 as usize;
        let map_area_h = term_h.saturating_sub(STATUS_LINES).max(1);

        if term_w != self.last_term_width || term_h != self.last_term_height {
            self.buffer = vec![vec![' '; term_w]; term_h];
            self.last_term_width = term_w;
            self.last_term_height = term_h;
        }

        let map_w = state.map().width();
        let map_h = state.map().height();
        let player_pos = state.player().pos();

        let (camera_x, camera_y) = self.camera_offset(
            map_w,
            map_h,
            term_w,
            map_area_h,
            player_pos,
        );

        let mut frame = vec![vec![' '; term_w]; term_h];

        for screen_y in 0..map_area_h {
            let map_y = camera_y + screen_y;
            if map_y >= map_h {
                break;
            }
            for screen_x in 0..term_w {
                let map_x = camera_x + screen_x;
                if map_x >= map_w {
                    break;
                }
                frame[screen_y][screen_x] = state.char_at(Position {
                    x: map_x as i32,
                    y: map_y as i32,
                });
            }
        }

        let status_lines = self.build_status_lines(state, term_w);
        for (i, line) in status_lines.iter().enumerate() {
            let screen_y = map_area_h + i;
            if screen_y >= term_h {
                break;
            }
            for (x, ch) in line.chars().enumerate() {
                if x < term_w {
                    frame[screen_y][x] = ch;
                }
            }
        }

        let mut stdout = stdout();
        for y in 0..term_h {
            if self.buffer[y] != frame[y] {
                let row: String = frame[y].iter().collect();
                execute!(stdout, MoveTo(0, y as u16))?;
                write!(stdout, "{row}")?;
                self.buffer[y] = frame[y].clone();
            }
        }

        stdout.flush()?;
        Ok(())
    }

    fn camera_offset(
        &self,
        map_w: usize,
        map_h: usize,
        view_w: usize,
        view_h: usize,
        player_pos: Position,
    ) -> (usize, usize) {
        let camera_x = if map_w > view_w {
            (player_pos.x as usize)
                .saturating_sub(view_w / 2)
                .min(map_w - view_w)
        } else {
            0
        };

        let camera_y = if map_h > view_h {
            (player_pos.y as usize)
                .saturating_sub(view_h / 2)
                .min(map_h - view_h)
        } else {
            0
        };

        (camera_x, camera_y)
    }

    fn build_status_lines(&self, state: &State, width: usize) -> Vec<String> {
        let mut lines = match state.mode() {
            GameMode::Exploration => vec![
                format!(
                    " HP: {}/{}",
                    state.player().stats.hp,
                    state.player().stats.max_hp
                ),
                state.status_message().to_string(),
                " WASD = ruch   Esc = wyjscie".to_string(),
            ],
            GameMode::Combat { monster_index, .. } => {
                let monster = &state.entities()[*monster_index];
                vec![
                    " === WALKA ===".to_string(),
                    format!(
                        " Ty: {}/{} HP  |  {}: {}/{} HP",
                        state.player().stats.hp,
                        state.player().stats.max_hp,
                        monster.monster_type.name(),
                        monster.stats.hp,
                        monster.stats.max_hp,
                    ),
                    state.status_message().to_string(),
                    " W = atak   S = atak spec.   A = ucieczka".to_string(),
                ]
            }
        };

        while lines.len() < STATUS_LINES {
            lines.push(String::new());
        }
        lines.truncate(STATUS_LINES);

        for line in &mut lines {
            if line.len() < width {
                line.push_str(&" ".repeat(width - line.len()));
            } else {
                line.truncate(width);
            }
        }

        lines
    }
}
