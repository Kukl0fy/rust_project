use macroquad::prelude::*;

use crate::game::combat_stats::CombatStats;
use crate::game::state::State;

use super::assets::AssetManager;
use super::WINDOW_WIDTH;

const PANEL_BG: Color = Color::new(0.08, 0.06, 0.1, 0.92);
const BATTLE_BAR_H: f32 = 190.0;

pub struct BattleUi;

impl BattleUi {
    pub fn render(&self, state: &State, assets: &AssetManager) {
        let Some(monster) = state.combat_monster() else {
            return;
        };

        let bar_y = super::WINDOW_HEIGHT - BATTLE_BAR_H;

        draw_rectangle(0.0, 0.0, WINDOW_WIDTH, super::WINDOW_HEIGHT, Color::new(0.0, 0.0, 0.0, 0.35));
        draw_rectangle(0.0, bar_y, WINDOW_WIDTH, BATTLE_BAR_H, PANEL_BG);
        draw_line(0.0, bar_y, WINDOW_WIDTH, bar_y, 2.0, Color::new(0.5, 0.4, 0.6, 1.0));

        let title_y = bar_y + 22.0;
        let title = "BATTLE";
        let title_w = measure_text(title, None, 22, 1.0).width;
        draw_text(title, WINDOW_WIDTH / 2.0 - title_w / 2.0, title_y, 22.0, YELLOW);

        draw_stats_panel(
            20.0,
            bar_y + 40.0,
            230.0,
            "PLAYER",
            &state.player().stats,
            GREEN,
        );

        draw_stats_panel(
            WINDOW_WIDTH - 250.0,
            bar_y + 40.0,
            230.0,
            &monster.monster_type.name().to_uppercase(),
            &monster.stats,
            RED,
        );

        draw_action_hints(assets, bar_y + 50.0);

        let msg = state.status_message();
        let msg_w = measure_text(msg, None, 16, 1.0).width;
        draw_text(
            msg,
            WINDOW_WIDTH / 2.0 - msg_w / 2.0,
            bar_y + BATTLE_BAR_H - 18.0,
            16.0,
            LIGHTGRAY,
        );
    }
}

fn draw_stats_panel(x: f32, y: f32, width: f32, title: &str, stats: &CombatStats, hp_color: Color) {
    draw_rectangle(x, y, width, 130.0, Color::new(0.12, 0.1, 0.14, 0.95));
    draw_rectangle_lines(x, y, width, 130.0, 2.0, Color::new(0.45, 0.38, 0.5, 1.0));

    let tx = x + 12.0;
    let mut ty = y + 22.0;
    let lh = 20.0;

    draw_text(title, tx, ty, 16.0, YELLOW);
    ty += lh + 4.0;

    draw_text(&format!("HP:  {}/{}", stats.hp, stats.max_hp), tx, ty, 15.0, hp_color);
    ty += lh;
    draw_text(&format!("ATK: {}", stats.attack), tx, ty, 15.0, Color::new(0.4, 0.9, 1.0, 1.0));
    ty += lh;
    draw_text(&format!("DEF: {}", stats.defense), tx, ty, 15.0, WHITE);
    ty += lh;
    draw_text(
        &format!("SP ATK: {}", stats.sp_attack),
        tx,
        ty,
        15.0,
        Color::new(1.0, 0.5, 1.0, 1.0),
    );
    ty += lh;
    draw_text(
        &format!("SP DEF: {}", stats.sp_defense),
        tx,
        ty,
        15.0,
        Color::new(0.8, 0.6, 1.0, 1.0),
    );
}

fn draw_action_hints(assets: &AssetManager, y: f32) {
    let center_x = WINDOW_WIDTH / 2.0;
    let gap = 150.0;
    let icon_size = 36.0;

    draw_action_button(
        assets,
        center_x - gap,
        y,
        icon_size,
        "arrow_up",
        0.0,
        "UP",
        "Attack",
        Color::new(0.4, 0.9, 1.0, 1.0),
    );
    draw_action_button(
        assets,
        center_x,
        y,
        icon_size,
        "arrow_down",
        0.0,
        "DOWN",
        "Special",
        Color::new(1.0, 0.5, 1.0, 1.0),
    );
    draw_action_button(
        assets,
        center_x + gap,
        y,
        icon_size,
        "arrow_left",
        0.0,
        "LEFT",
        "Flee",
        Color::new(1.0, 0.85, 0.4, 1.0),
    );
}

fn draw_action_button(
    assets: &AssetManager,
    center_x: f32,
    y: f32,
    icon_size: f32,
    icon_name: &str,
    rotation: f32,
    key_label: &str,
    action_label: &str,
    label_color: Color,
) {
    let box_w = 110.0;
    let box_h = 100.0;
    let x = center_x - box_w / 2.0;

    draw_rectangle(x, y, box_w, box_h, Color::new(0.16, 0.12, 0.18, 0.95));
    draw_rectangle_lines(x, y, box_w, box_h, 1.5, Color::new(0.5, 0.42, 0.55, 1.0));

    assets.draw_icon(icon_name, center_x - icon_size / 2.0, y + 10.0, icon_size, rotation);

    let key_w = measure_text(key_label, None, 14, 1.0).width;
    draw_text(key_label, center_x - key_w / 2.0, y + 58.0, 14.0, WHITE);

    let act_w = measure_text(action_label, None, 16, 1.0).width;
    draw_text(action_label, center_x - act_w / 2.0, y + 82.0, 16.0, label_color);
}
