use macroquad::prelude::*;
use crate::game::state::State;
use super::WINDOW_HEIGHT;

pub struct Hud {
    position_x: f32,
    position_y: f32,
}

impl Hud {
    pub fn new(position_x: f32, position_y: f32) -> Self {
        Self { position_x, position_y }
    }

    pub fn render(&self, state: &State) {
        if state.is_in_combat() {
            return;
        }

        let player = state.player();
        
        // Semi-transparent background for HUD
        draw_rectangle(
            self.position_x,
            self.position_y,
            250.0,
            285.0,
            Color::new(0.1, 0.1, 0.1, 0.7),
        );

        // Border
        draw_rectangle_lines(
            self.position_x,
            self.position_y,
            250.0,
            285.0,
            2.0,
            WHITE,
        );

        let text_x = self.position_x + 15.0;
        let mut text_y = self.position_y + 15.0;
        let line_height = 25.0;

        // Title
        draw_text("PLAYER STATS", text_x, text_y, 18.0, YELLOW);
        text_y += line_height;

        let level_text = format!("POZIOM: {}", state.level_depth());
        draw_text(&level_text, text_x, text_y, 16.0, ORANGE);
        text_y += line_height;

        // Health
        let health_text = format!("HP: {}/{}", player.stats.hp, player.stats.max_hp);
        draw_text(&health_text, text_x, text_y, 16.0, GREEN);
        text_y += line_height;

        // Attack
        let attack_text = format!("ATK: {}", player.stats.attack);
        draw_text(&attack_text, text_x, text_y, 16.0, Color::new(0.0, 1.0, 1.0, 1.0));
        text_y += line_height;

        // Defense
        let defense_text = format!("DEF: {}", player.stats.defense);
        draw_text(&defense_text, text_x, text_y, 16.0, WHITE);
        text_y += line_height;

        // Special attack / defense
        let sp_attack_text = format!("SP ATK: {}", player.stats.sp_attack);
        draw_text(&sp_attack_text, text_x, text_y, 16.0, Color::new(1.0, 0.5, 1.0, 1.0));
        text_y += line_height;

        let sp_defense_text = format!("SP DEF: {}", player.stats.sp_defense);
        draw_text(&sp_defense_text, text_x, text_y, 16.0, Color::new(0.8, 0.6, 1.0, 1.0));
        text_y += line_height;

        draw_text("---", text_x, text_y, 14.0, GRAY);
        text_y += line_height;

        let status = state.status_message();
        draw_text(status, text_x, text_y, 14.0, LIGHTGRAY);

        Self::render_loot_panel(state);
    }

    pub fn render_loot(state: &State) {
        Self::render_loot_panel(state);
    }

    fn render_loot_panel(state: &State) {
        let Some((name, effect)) = state.last_loot() else {
            return;
        };

        let panel_w = 280.0;
        let panel_h = 72.0;
        let x = 10.0;
        let y = WINDOW_HEIGHT - panel_h - 10.0;

        draw_rectangle(x, y, panel_w, panel_h, Color::new(0.1, 0.1, 0.1, 0.85));
        draw_rectangle_lines(x, y, panel_w, panel_h, 2.0, Color::new(1.0, 0.85, 0.3, 1.0));

        let text_x = x + 12.0;
        draw_text("OSTATNI LOOT", text_x, y + 20.0, 14.0, YELLOW);
        draw_text(name, text_x, y + 40.0, 16.0, WHITE);
        draw_text(&format!("Efekt: {effect}"), text_x, y + 60.0, 14.0, LIME);
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum MenuState {
    MainMenu,
    Playing,
    PauseMenu,
    GameOver,
}

pub struct Menu {
    state: MenuState,
}

impl Menu {
    pub fn new() -> Self {
        Self {
            state: MenuState::MainMenu,
        }
    }

    pub fn current_state(&self) -> MenuState {
        self.state
    }

    pub fn set_state(&mut self, state: MenuState) {
        self.state = state;
    }

    pub fn render_main_menu(&self) {
        clear_background(BLACK);

        let screen_width = screen_width();
        let screen_height = screen_height();
        let center_x = screen_width / 2.0;
        let center_y = screen_height / 2.0;

        // Title
        let title = "PIXEL DUNGEON";
        let title_size = 60.0;
        let title_width = measure_text(title, None, title_size as u16, 1.0).width;
        draw_text(
            title,
            center_x - title_width / 2.0,
            center_y - 80.0,
            title_size,
            YELLOW,
        );

        // Menu options
        let options = vec!["[ENTER] Start Game", "[Q] Quit"];
        let option_y_start = center_y + 40.0;

        for (i, option) in options.iter().enumerate() {
            let option_width = measure_text(option, None, 24, 1.0).width;
            draw_text(
                option,
                center_x - option_width / 2.0,
                option_y_start + (i as f32) * 50.0,
                24.0,
                WHITE,
            );
        }

        // Instructions
        let instructions = "Navigate the dungeon, defeat monsters, find treasures";
        let inst_width = measure_text(instructions, None, 16, 1.0).width;
        draw_text(
            instructions,
            center_x - inst_width / 2.0,
            screen_height - 40.0,
            16.0,
            GRAY,
        );
    }

    pub fn render_pause_menu(&self) {
        // Semi-transparent overlay
        draw_rectangle(0.0, 0.0, screen_width(), screen_height(), Color::new(0.0, 0.0, 0.0, 0.7));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Title
        let title = "PAUSED";
        let title_width = measure_text(title, None, 50, 1.0).width;
        draw_text(title, center_x - title_width / 2.0, center_y - 60.0, 50.0, YELLOW);

        // Options
        let options = vec!["[P] Resume", "[M] Main Menu", "[Q] Quit"];
        let option_y_start = center_y + 20.0;

        for (i, option) in options.iter().enumerate() {
            let option_width = measure_text(option, None, 24, 1.0).width;
            draw_text(
                option,
                center_x - option_width / 2.0,
                option_y_start + (i as f32) * 50.0,
                24.0,
                WHITE,
            );
        }
    }

    pub fn render_game_over(&self, victory: bool) {
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Title
        let title = if victory { "VICTORY!" } else { "GAME OVER" };
        let title_color = if victory { LIME } else { RED };
        let title_width = measure_text(title, None, 60, 1.0).width;
        draw_text(title, center_x - title_width / 2.0, center_y - 60.0, 60.0, title_color);

        // Options
        let options = vec!["[ENTER] Main Menu", "[Q] Quit"];
        let option_y_start = center_y + 40.0;

        for (i, option) in options.iter().enumerate() {
            let option_width = measure_text(option, None, 24, 1.0).width;
            draw_text(
                option,
                center_x - option_width / 2.0,
                option_y_start + (i as f32) * 50.0,
                24.0,
                WHITE,
            );
        }
    }
}
