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

pub fn tile_for(map: &Map, pos: Position) -> TileCoord {
    match map.tile_at(pos) {
        Some(Tile::Floor) | Some(Tile::Exit) => floor_tile(pos),
        Some(Tile::Wall) => wall_tile(map, pos),
        Some(Tile::Void) | None => FLOOR_A,
    }
}

fn floor_tile(pos: Position) -> TileCoord {
    let variants = [FLOOR_A, FLOOR_B, FLOOR_C];
    let index = ((pos.x * 17 + pos.y * 31).unsigned_abs() as usize) % variants.len();
    variants[index]
}

fn wall_tile(map: &Map, pos: Position) -> TileCoord {
    let n = map.is_floor_neighbor(pos, 0, -1);
    let s = map.is_floor_neighbor(pos, 0, 1);
    let e = map.is_floor_neighbor(pos, 1, 0);
    let w = map.is_floor_neighbor(pos, -1, 0);
    // Outer corners
    if s && e && !n && !w {
        return WALL_NW;
    }
    if s && w && !n && !e {
        return WALL_NE;
    }
    if n && e && !s && !w {
        return WALL_SW;
    }
    if n && w && !s && !e {
        return WALL_SE;
    }

    // North edge — floor to the south (front-facing brick)
    if s && !n {
        if e && w {
            return pick_n_wall(map, pos);
        }
        if !e && !w {
            return pick_n_wall(map, pos);
        }
    }

    // South edge — floor to the north (top-down wall cap)
    if n && !s {
        if e && w {
            return pick_s_wall(map, pos);
        }
        if !e && !w {
            return pick_s_wall(map, pos);
        }
    }

    // West edge — floor to the east
    if e && !w {
        if n && s {
            return pick_w_wall(map, pos);
        }
        if !n && !s {
            return pick_w_wall(map, pos);
        }
    }

    // East edge — floor to the west
    if w && !e {
        if n && s {
            return pick_e_wall(map, pos);
        }
        if !n && !s {
            return pick_e_wall(map, pos);
        }
    }

    // Wall sandwiched between two floor rows (shared horizontal boundary)
    if n && s && !e && !w {
        return pick_s_wall(map, pos);
    }

    // Wall sandwiched between two floor columns (shared vertical boundary)
    if e && w && !n && !s {
        return pick_e_wall(map, pos);
    }

    // T-junctions: open side faces void
    if s && e && w && !n {
        return WALL_N;
    }
    if n && e && w && !s {
        return WALL_S;
    }
    if n && s && w && !e {
        return pick_e_wall(map, pos);
    }
    if n && s && e && !w {
        return pick_w_wall(map, pos);
    }

    WALL_N
}

fn pick_n_wall(map: &Map, pos: Position) -> TileCoord {
    let continues_w = same_north_wall(map, pos, -1, 0);
    let continues_e = same_north_wall(map, pos, 1, 0);

    if !continues_w {
        WALL_N
    } else if !continues_e {
        WALL_N_MID
    } else {
        WALL_N_MID
    }
}

fn pick_s_wall(map: &Map, pos: Position) -> TileCoord {
    let continues_w = same_south_wall(map, pos, -1, 0);
    let continues_e = same_south_wall(map, pos, 1, 0);

    if !continues_w {
        WALL_S
    } else if !continues_e {
        WALL_S_RIGHT
    } else {
        WALL_S_MID
    }
}

fn pick_w_wall(map: &Map, pos: Position) -> TileCoord {
    let continues_n = same_west_wall(map, pos, 0, -1);
    let continues_s = same_west_wall(map, pos, 0, 1);

    if !continues_n {
        WALL_W_TOP
    } else if !continues_s {
        WALL_W_BOT
    } else {
        WALL_W
    }
}

fn pick_e_wall(map: &Map, pos: Position) -> TileCoord {
    let continues_n = same_east_wall(map, pos, 0, -1);
    let continues_s = same_east_wall(map, pos, 0, 1);

    if !continues_n {
        WALL_E_TOP
    } else if !continues_s {
        WALL_E_BOT
    } else {
        WALL_E
    }
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

