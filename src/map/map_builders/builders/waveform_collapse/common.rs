use super::GameTile;

#[derive(Eq, PartialEq, Clone)]
pub struct MapChunk {
    pub has_exits: bool,
    pub exits: [Vec<bool>; 4],
    pub pattern: Vec<GameTile>,
    pub compatible_with: [Vec<usize>; 4],
}

pub fn tile_idx_in_chunk(chunk_size: i32, x: i32, y: i32) -> usize { ((y * chunk_size) + x) as usize }
