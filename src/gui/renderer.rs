use macroquad::prelude::*;

use crate::game::map::Tile;
use crate::game::position::Position;
use crate::game::state::State;

use super::assets::AssetManager;
use super::sprite_entities::{player_sprite, monster_sprite, CHARACTER_TILE_PX};
use super::sprite_map::tile_for;
use super::TILE_SIZE;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const MAP_VIEW_HEIGHT: f32 = 680.0;

// Dark purple from the tileset background — not a labeled tile from the sheet.
const VOID_COLOR: Color = Color::new(0.07, 0.04, 0.08, 1.0);

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

    pub fn render(&self, state: &State, assets: &AssetManager) {
        clear_background(BLACK);

        let map = state.map();
        let (cam_x, cam_y) = camera_offset(state.player().pos(), map.width(), map.height());

        let view_tiles_x = (WINDOW_WIDTH / TILE_SIZE).ceil() as i32 + 1;
        let view_tiles_y = (MAP_VIEW_HEIGHT / TILE_SIZE).ceil() as i32 + 1;

        let start_x = (cam_x / TILE_SIZE).floor() as i32;
        let start_y = (cam_y / TILE_SIZE).floor() as i32;

        for screen_y in 0..view_tiles_y {
            for screen_x in 0..view_tiles_x {
                let map_x = start_x + screen_x;
                let map_y = start_y + screen_y;

                if map_x < 0 || map_y < 0 || map_x >= map.width() as i32 || map_y >= map.height() as i32
                {
                    continue;
                }

                let pos = Position {
                    x: map_x,
                    y: map_y,
                };
                let dest_x = map_x as f32 * TILE_SIZE - cam_x;
                let dest_y = map_y as f32 * TILE_SIZE - cam_y;

                match map.tile_at(pos) {
                    Some(Tile::Void) | None => {
                        draw_rectangle(dest_x, dest_y, TILE_SIZE, TILE_SIZE, VOID_COLOR);
                    }
                    Some(Tile::Wall) | Some(Tile::Floor) | Some(Tile::Exit) => {
                        let tile = tile_for(map, pos);
                        assets.draw_tile(
                            "tileset",
                            tile,
                            dest_x,
                            dest_y,
                            TILE_SIZE,
                            super::sprite_map::TILESET_TILE_PX,
                        );
                    }
                }
            }
        }

        for chest in state.chests() {
            if chest.is_open() {
                continue;
            }
            let dest_x = chest.pos().x as f32 * TILE_SIZE - cam_x;
            let dest_y = chest.pos().y as f32 * TILE_SIZE - cam_y;
            assets.draw_full_texture("chest_closed", dest_x, dest_y, TILE_SIZE);
        }

        for monster in state.entities() {
            draw_entity(
                assets,
                monster.pos(),
                monster_sprite(monster.monster_type),
                cam_x,
                cam_y,
            );
        }

        draw_entity(
            assets,
            state.player().pos(),
            player_sprite(state.player().class()),
            cam_x,
            cam_y,
        );
    }
}

fn draw_entity(
    assets: &AssetManager,
    pos: Position,
    sprite: super::sprite_map::TileCoord,
    cam_x: f32,
    cam_y: f32,
) {
    let dest_x = pos.x as f32 * TILE_SIZE - cam_x;
    let dest_y = pos.y as f32 * TILE_SIZE - cam_y;
    assets.draw_sprite(
        "characters",
        sprite,
        dest_x,
        dest_y,
        TILE_SIZE,
        CHARACTER_TILE_PX,
    );
}

fn camera_offset(player: Position, map_w: usize, map_h: usize) -> (f32, f32) {
    let view_w = WINDOW_WIDTH;
    let view_h = MAP_VIEW_HEIGHT;

    let max_cam_x = (map_w as f32 * TILE_SIZE - view_w).max(0.0);
    let max_cam_y = (map_h as f32 * TILE_SIZE - view_h).max(0.0);

    let cam_x = (player.x as f32 * TILE_SIZE + TILE_SIZE / 2.0 - view_w / 2.0).clamp(0.0, max_cam_x);
    let cam_y = (player.y as f32 * TILE_SIZE + TILE_SIZE / 2.0 - view_h / 2.0).clamp(0.0, max_cam_y);

    (cam_x, cam_y)
}
