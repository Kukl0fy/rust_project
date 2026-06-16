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

const FLOOR_A: TileCoord = TileCoord::new(6, 0);
const FLOOR_B: TileCoord = TileCoord::new(6, 1);
const FLOOR_C: TileCoord = TileCoord::new(7, 0);

const WALL_NW: TileCoord = TileCoord::new(0, 0);
const WALL_N: TileCoord = TileCoord::new(1, 0);
const WALL_N_MID: TileCoord = TileCoord::new(2, 0);
const WALL_NE: TileCoord = TileCoord::new(3, 0);

const WALL_W_TOP: TileCoord = TileCoord::new(0, 1);
const WALL_W: TileCoord = TileCoord::new(0, 2);
const WALL_W_BOT: TileCoord = TileCoord::new(0, 3);

const WALL_E_TOP: TileCoord = TileCoord::new(5, 0);
const WALL_E: TileCoord = TileCoord::new(5, 1);
const WALL_E_BOT: TileCoord = TileCoord::new(5, 3);

const WALL_SW: TileCoord = TileCoord::new(3, 5);
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
        Some(Tile::Wall) => wall_tile(map, pos).tile,
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
    if s && e && !n && !w {
        return WallDraw::new(WALL_NW, false);
    }
    if s && w && !n && !e {
        return WallDraw::new(WALL_NE, false);
    }
    if n && e && !s && !w {
        return WallDraw::new(WALL_SW, false);
    }
    if n && w && !s && !e {
        return WallDraw::new(WALL_SE, false);
    }

    if s && !n {
        return pick_n_wall(map, pos);
    }
    if n && !s {
        return pick_s_wall(map, pos);
    }
    if e && !w {
        return pick_w_wall(map, pos);
    }
    if w && !e {
        return pick_e_wall(map, pos);
    }

    if n && s && !e && !w {
        return pick_s_wall(map, pos);
    }
    if e && w && !n && !s {
        return pick_e_wall(map, pos);
    }

    if s && e && w && !n {
        return WallDraw::new(WALL_N, false);
    }
    if n && e && w && !s {
        return WallDraw::new(WALL_S, false);
    }
    if n && s && w && !e {
        return pick_e_wall(map, pos);
    }
    if n && s && e && !w {
        return pick_w_wall(map, pos);
    }

    WallDraw::new(WALL_N, false)
}

fn pick_n_wall(map: &Map, pos: Position) -> WallDraw {
    let continues_w = same_north_wall(map, pos, -1, 0);
    let continues_e = same_north_wall(map, pos, 1, 0);

    let tile = if !continues_w {
        WALL_N
    } else if !continues_e {
        WALL_N_MID
    } else {
        WALL_N_MID
    };
    WallDraw::new(tile, false)
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

