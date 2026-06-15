use crate::game::map::{Map, Tile};
use crate::game::position::Position;

pub const TILESET_TILE_PX: f32 = 16.0;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TileCoord {
    pub col: u32,
    pub row: u32,
}

impl TileCoord {
    pub const fn new(col: u32, row: u32) -> Self {
        Self { col, row }
    }
}

// Room template from Dungeon_Tileset.png (top-left, 16px grid).
const FLOOR_A: TileCoord = TileCoord::new(1, 1);
const FLOOR_B: TileCoord = TileCoord::new(2, 2);
const FLOOR_C: TileCoord = TileCoord::new(3, 3);

const WALL_NW: TileCoord = TileCoord::new(0, 0);
const WALL_N: TileCoord = TileCoord::new(1, 0);
const WALL_N_MID: TileCoord = TileCoord::new(2, 0);
const WALL_NE: TileCoord = TileCoord::new(3, 0);
const WALL_NW_SHADOW: TileCoord = TileCoord::new(0, 5);
const WALL_N_SHADOW: TileCoord = TileCoord::new(1, 5);
const WALL_N_MID_SHADOW: TileCoord = TileCoord::new(2, 5);
const WALL_NE_SHADOW: TileCoord = TileCoord::new(3, 5);

const WALL_W_TOP: TileCoord = TileCoord::new(0, 1);
const WALL_W: TileCoord = TileCoord::new(0, 2);
const WALL_W_BOT: TileCoord = TileCoord::new(0, 3);

const WALL_E_TOP: TileCoord = TileCoord::new(4, 1);
const WALL_E: TileCoord = TileCoord::new(4, 2);
const WALL_E_BOT: TileCoord = TileCoord::new(4, 3);

const WALL_SW: TileCoord = TileCoord::new(0, 4);
const WALL_S: TileCoord = TileCoord::new(1, 4);
const WALL_S_MID: TileCoord = TileCoord::new(2, 4);
const WALL_S_RIGHT: TileCoord = TileCoord::new(3, 4);
const WALL_SE: TileCoord = TileCoord::new(4, 4);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WallDraw {
    pub tile: TileCoord,
    pub flip_x: bool,
}

impl WallDraw {
    const fn new(tile: TileCoord, flip_x: bool) -> Self {
        Self { tile, flip_x }
    }
}

pub fn wall_draw(map: &Map, pos: Position) -> WallDraw {
    wall_tile(map, pos)
}

pub fn tile_for(map: &Map, pos: Position) -> TileCoord {
    match map.tile_at(pos) {
        Some(Tile::Floor) | Some(Tile::Exit) | Some(Tile::Ladder) => floor_tile(pos),
        Some(Tile::Wall) => wall_draw(map, pos).tile,
        Some(Tile::Void) | None => FLOOR_A,
    }
}
pub fn floor_tile(pos: Position) -> TileCoord {
    let variants = [FLOOR_A, FLOOR_B, FLOOR_C];
    let index = ((pos.x * 17 + pos.y * 31).unsigned_abs() as usize) % variants.len();
    variants[index]
}

/// Podloga pod kafelkiem sciany — bierze wzor z sasiedniej podlogi.
pub fn floor_under_wall(map: &Map, pos: Position) -> TileCoord {
    for (dx, dy) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
        let neighbor = offset(pos, dx, dy);
        if map.is_floor_neighbor(pos, dx, dy) {
            return floor_tile(neighbor);
        }
    }
    floor_tile(pos)
}

fn wall_tile(map: &Map, pos: Position) -> WallDraw {
    let n = map.is_floor_neighbor(pos, 0, -1);
    let s = map.is_floor_neighbor(pos, 0, 1);
    let e = map.is_floor_neighbor(pos, 1, 0);
    let w = map.is_floor_neighbor(pos, -1, 0);
    let void_n = is_void_neighbor(map, pos, 0, -1);

    let mask = (n as u8) | ((s as u8) << 1) | ((e as u8) << 2) | ((w as u8) << 3);

    match mask {
        // Zewnetrzne narozniki (n=bit0, s=bit1, e=bit2, w=bit3)
        0b0110 => shadow_n(WallDraw::new(WALL_NW, false), void_n), // poludnie + wschod
        0b1010 => shadow_n(WallDraw::new(WALL_NW, true), void_n), // poludnie + zachod (NE)
        0b0101 => WallDraw::new(WALL_SW, false),                    // polnoc + wschod
        0b1001 => WallDraw::new(WALL_SW, true),                     // polnoc + zachod (SE)

        // Proste krawedzie
        0b0010 => shadow_n(pick_n_wall(map, pos), void_n),
        0b0001 => pick_s_wall(map, pos),
        0b0100 => pick_w_wall(map, pos),
        0b1000 => pick_e_wall(map, pos),

        // Dzielnik poziomy (podloga na polnocy i poludniu)
        0b0011 => pick_s_wall(map, pos),

        // Dzielnik pionowy (podloga na wschodzie i zachodzie)
        0b1100 => pick_e_wall(map, pos),

        // T-rozwidlelnia i wiecej sasiedow
        0b1110 => shadow_n(WallDraw::new(WALL_N, false), void_n),
        0b0111 => shadow_n(WallDraw::new(WALL_N, false), void_n),
        0b1011 => shadow_n(WallDraw::new(WALL_N, false), void_n),
        0b1101 => WallDraw::new(WALL_S, false),
        0b1111 => WallDraw::new(WALL_S, false),

        _ => shadow_n(WallDraw::new(WALL_N, false), void_n),
    }
}

fn shadow_n(draw: WallDraw, void_n: bool) -> WallDraw {
    if !void_n {
        return draw;
    }
    let tile = match draw.tile {
        WALL_NW => WALL_NW_SHADOW,
        WALL_N => WALL_N_SHADOW,
        WALL_N_MID => WALL_N_MID_SHADOW,
        WALL_NE => WALL_NE_SHADOW,
        _ => WALL_N_SHADOW,
    };
    WallDraw::new(tile, draw.flip_x)
}

fn pick_n_wall(map: &Map, pos: Position) -> WallDraw {
    let continues_w = same_north_wall(map, pos, -1, 0);
    let continues_e = same_north_wall(map, pos, 1, 0);

    if !continues_w {
        WallDraw::new(WALL_N, false)
    } else if !continues_e {
        WallDraw::new(WALL_NW, true) // prawy koniec -> odbicie NW
    } else {
        WallDraw::new(WALL_N_MID, false)
    }
}

fn pick_s_wall(map: &Map, pos: Position) -> WallDraw {
    let continues_w = same_south_wall(map, pos, -1, 0);
    let continues_e = same_south_wall(map, pos, 1, 0);

    if !continues_w {
        WallDraw::new(WALL_S, false)
    } else if !continues_e {
        WallDraw::new(WALL_S_RIGHT, false)
    } else {
        WallDraw::new(WALL_S_MID, false)
    }
}

fn pick_w_wall(map: &Map, pos: Position) -> WallDraw {
    let continues_n = same_west_wall(map, pos, 0, -1);
    let continues_s = same_west_wall(map, pos, 0, 1);

    if !continues_n {
        WallDraw::new(WALL_W_TOP, false)
    } else if !continues_s {
        WallDraw::new(WALL_W_BOT, false)
    } else {
        WallDraw::new(WALL_W, false)
    }
}

/// Sciana wschodnia = odbicie kafelkow zachodnich (kolumna 4 tilesetu jest pusta).
fn pick_e_wall(map: &Map, pos: Position) -> WallDraw {
    let west = pick_w_wall(map, pos);
    WallDraw::new(west.tile, true)
}

fn same_north_wall(map: &Map, pos: Position, dx: i32, dy: i32) -> bool {
    let neighbor = offset(pos, dx, dy);
    matches!(map.tile_at(neighbor), Some(Tile::Wall))
        && map.is_floor_neighbor(neighbor, 0, 1)
        && !map.is_floor_neighbor(neighbor, 0, -1)
}

fn same_south_wall(map: &Map, pos: Position, dx: i32, dy: i32) -> bool {
    let neighbor = offset(pos, dx, dy);
    matches!(map.tile_at(neighbor), Some(Tile::Wall))
        && map.is_floor_neighbor(neighbor, 0, -1)
        && !map.is_floor_neighbor(neighbor, 0, 1)
}

fn same_west_wall(map: &Map, pos: Position, dx: i32, dy: i32) -> bool {
    let neighbor = offset(pos, dx, dy);
    matches!(map.tile_at(neighbor), Some(Tile::Wall))
        && map.is_floor_neighbor(neighbor, 1, 0)
        && !map.is_floor_neighbor(neighbor, -1, 0)
}

fn same_east_wall(map: &Map, pos: Position, dx: i32, dy: i32) -> bool {
    let neighbor = offset(pos, dx, dy);
    matches!(map.tile_at(neighbor), Some(Tile::Wall))
        && map.is_floor_neighbor(neighbor, -1, 0)
        && !map.is_floor_neighbor(neighbor, 1, 0)
}

fn offset(pos: Position, dx: i32, dy: i32) -> Position {
    Position {
        x: pos.x + dx,
        y: pos.y + dy,
    }
}

fn is_void_neighbor(map: &Map, pos: Position, dx: i32, dy: i32) -> bool {
    matches!(map.tile_at(offset(pos, dx, dy)), Some(Tile::Void) | None)
}
