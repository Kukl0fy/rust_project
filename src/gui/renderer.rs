use macroquad::prelude::*;

use crate::game::map::Tile;
use crate::game::position::Position;
use crate::game::state::State;

use super::assets::AssetManager;
use super::sprite_entities::{monster_sprite, player_sprite, CHARACTER_TILE_PX};
use super::sprite_map::{floor_tile, floor_under_wall, wall_draw};
use super::TILE_SIZE;

const WINDOW_WIDTH: f32 = 1280.0;
const WINDOW_HEIGHT: f32 = 720.0;
const MAP_VIEW_HEIGHT: f32 = 680.0;

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

        // Pass 1: void + podloga (takze pod scianami)
        for screen_y in 0..view_tiles_y {
            for screen_x in 0..view_tiles_x {
                let map_x = start_x + screen_x;
                let map_y = start_y + screen_y;

                if map_x < 0 || map_y < 0 || map_x >= map.width() as i32 || map_y >= map.height() as i32
                {
                    continue;
                }

                let pos = Position { x: map_x, y: map_y };
                let dest_x = map_x as f32 * TILE_SIZE - cam_x;
                let dest_y = map_y as f32 * TILE_SIZE - cam_y;

                match map.tile_at(pos) {
                    Some(Tile::Void) | None => {
                        draw_rectangle(dest_x, dest_y, TILE_SIZE, TILE_SIZE, VOID_COLOR);
                    }
                    Some(Tile::Floor) | Some(Tile::Exit) | Some(Tile::Ladder) => {
                        draw_floor(assets, floor_tile(pos), dest_x, dest_y);
                    }
                    Some(Tile::Wall) => {
                        draw_floor(assets, floor_under_wall(map, pos), dest_x, dest_y);
                    }
                }
            }
        }

        // Pass 2: sciany na wierzchu
        for screen_y in 0..view_tiles_y {
            for screen_x in 0..view_tiles_x {
                let map_x = start_x + screen_x;
                let map_y = start_y + screen_y;

                if map_x < 0 || map_y < 0 || map_x >= map.width() as i32 || map_y >= map.height() as i32
                {
                    continue;
                }

                let pos = Position { x: map_x, y: map_y };
                if !matches!(map.tile_at(pos), Some(Tile::Wall)) {
                    continue;
                }

                let dest_x = map_x as f32 * TILE_SIZE - cam_x;
                let dest_y = map_y as f32 * TILE_SIZE - cam_y;
                let wall = wall_draw(map, pos);
                assets.draw_sprite(
                    "tileset",
                    wall.tile,
                    dest_x,
                    dest_y,
                    TILE_SIZE,
                    super::sprite_map::TILESET_TILE_PX,
                    wall.flip_x,
                );
            }
        }

        let ladder = state.ladder_pos();
        if matches!(map.tile_at(ladder), Some(Tile::Ladder)) {
            let dest_x = ladder.x as f32 * TILE_SIZE - cam_x;
            let dest_y = ladder.y as f32 * TILE_SIZE - cam_y;

            draw_rectangle_lines(
                dest_x + 1.0,
                dest_y + 1.0,
                TILE_SIZE - 2.0,
                TILE_SIZE - 2.0,
                1.5,
                Color::new(1.0, 0.85, 0.2, 0.7),
            );
            assets.draw_full_texture("ladder", dest_x, dest_y, TILE_SIZE);
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

fn draw_floor(assets: &AssetManager, tile: super::sprite_map::TileCoord, dest_x: f32, dest_y: f32) {
    assets.draw_tile(
        "tileset",
        tile,
        dest_x,
        dest_y,
        TILE_SIZE,
        super::sprite_map::TILESET_TILE_PX,
    );
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
        false,
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
