use macroquad::prelude::*;
use std::collections::HashMap;

use super::sprite_map::TileCoord;

pub struct AssetManager {
    textures: HashMap<String, Texture2D>,
}

impl AssetManager {
    pub async fn new() -> Self {
        let mut manager = Self {
            textures: HashMap::new(),
        };

        manager
            .try_load(
                "tileset",
                "2D Pixel Dungeon Asset Pack/character and tileset/Dungeon_Tileset.png",
            )
            .await;
        manager
            .try_load(
                "characters",
                "2D Pixel Dungeon Asset Pack/character and tileset/Dungeon_Character.png",
            )
            .await;
        manager
            .try_load(
                "ui_square",
                "2D Pixel Dungeon Asset Pack/interface/square_left_1.png",
            )
            .await;
        manager
            .try_load(
                "arrow_up",
                "2D Pixel Dungeon Asset Pack/interface/arrow_2.png",
            )
            .await;
        manager
            .try_load(
                "arrow_down",
                "2D Pixel Dungeon Asset Pack/interface/arrow_1.png",
            )
            .await;
        manager
            .try_load(
                "arrow_left",
                "2D Pixel Dungeon Asset Pack/interface/arrow_3.png",
            )
            .await;
        manager
            .try_load(
                "chest_closed",
                "2D Pixel Dungeon Asset Pack/items and trap_animation/chest/chest_3.png",
            )
            .await;
        manager
            .try_load(
                "chest_open",
                "2D Pixel Dungeon Asset Pack/items and trap_animation/chest/chest_open_1.png",
            )
            .await;
        manager
            .try_load("ladder", "assets/sprites/ladder.png")
            .await;
        manager
            .try_load(
                "heal_potion",
                "2D Pixel Dungeon Asset Pack/items and trap_animation/flasks/flasks_1_2.png",
            )
            .await;

        manager
    }

    async fn try_load(&mut self, name: &str, path: &str) {
        if let Ok(texture) = load_texture(path).await {
            texture.set_filter(FilterMode::Nearest);
            self.textures.insert(name.to_string(), texture);
        }
    }

    pub fn draw_tile(
        &self,
        name: &str,
        tile: TileCoord,
        dest_x: f32,
        dest_y: f32,
        dest_size: f32,
        tile_px: f32,
    ) {
        self.draw_sprite(name, tile, dest_x, dest_y, dest_size, tile_px, false);
    }

    pub fn draw_sprite(
        &self,
        name: &str,
        tile: TileCoord,
        dest_x: f32,
        dest_y: f32,
        dest_size: f32,
        tile_px: f32,
        flip_x: bool,
    ) {
        if let Some(texture) = self.textures.get(name) {
            draw_texture_ex(
                texture,
                dest_x,
                dest_y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(dest_size)),
                    source: Some(Rect::new(
                        tile.col as f32 * tile_px,
                        tile.row as f32 * tile_px,
                        tile_px,
                        tile_px,
                    )),
                    rotation: 0.0,
                    flip_x,
                    flip_y: false,
                    pivot: None,
                },
            );
        }
    }

    pub fn draw_full_texture(&self, name: &str, x: f32, y: f32, size: f32) {
        if let Some(texture) = self.textures.get(name) {
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(size)),
                    source: None,
                    rotation: 0.0,
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        }
    }

    pub fn draw_icon(&self, name: &str, x: f32, y: f32, size: f32, rotation: f32) {
        if let Some(texture) = self.textures.get(name) {
            let pivot = Vec2::new(size / 2.0, size / 2.0);
            draw_texture_ex(
                texture,
                x,
                y,
                WHITE,
                DrawTextureParams {
                    dest_size: Some(Vec2::splat(size)),
                    source: None,
                    rotation,
                    flip_x: false,
                    flip_y: false,
                    pivot: Some(pivot),
                },
            );
        }
    }
}
