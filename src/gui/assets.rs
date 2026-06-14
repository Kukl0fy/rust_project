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

        manager.try_load(
            "tileset",
            "2D Pixel Dungeon Asset Pack/character and tileset/Dungeon_Tileset.png",
        )
        .await;
        manager.try_load(
            "characters",
            "2D Pixel Dungeon Asset Pack/character and tileset/Dungeon_Character.png",
        )
        .await;
        manager.try_load(
            "ui_square",
            "2D Pixel Dungeon Asset Pack/interface/square_left_1.png",
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
        self.draw_sprite(name, tile, dest_x, dest_y, dest_size, tile_px);
    }

    pub fn draw_sprite(
        &self,
        name: &str,
        tile: TileCoord,
        dest_x: f32,
        dest_y: f32,
        dest_size: f32,
        tile_px: f32,
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
                    flip_x: false,
                    flip_y: false,
                    pivot: None,
                },
            );
        }
    }
}
