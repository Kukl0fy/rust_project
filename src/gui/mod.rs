pub mod assets;
pub mod battle_ui;
pub mod hud;
pub mod renderer;
pub mod sprite_entities;
pub mod sprite_map;

pub use assets::AssetManager;
pub use battle_ui::BattleUi;
pub use hud::{Hud, Menu, MenuState};
pub use renderer::{GuiRenderer, conf};

pub const WINDOW_WIDTH: f32 = 1280.0;
pub const WINDOW_HEIGHT: f32 = 720.0;
pub const TILE_SIZE: f32 = 32.0;
