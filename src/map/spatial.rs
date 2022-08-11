use crate::prelude::*;
use parking_lot::Mutex;

struct SpatialMap {
    width: i32,
    height: i32,
    opaque: Vec<bool>,
    blocked: Vec<(bool, bool)>,
    tile_content: Vec<Vec<(Entity, bool, bool)>>,
}

impl SpatialMap {
    fn new() -> Self {
        Self { width: 0, height: 0, blocked: Vec::new(), tile_content: Vec::new(), opaque: Vec::new() }
    }

    fn _xy_idx(&self, x: i32, y: i32) -> usize { (y as usize * self.width as usize) + x as usize }

    fn point2d_to_index(&self, pt: Point) -> usize {
        let bounds = Point::new(self.width, self.height);
        ((pt.y * bounds.x) + pt.x).try_into().expect("Not a valid usize. Did something go negative?")
    }
}

lazy_static! {
    static ref SPATIAL_MAP: Mutex<SpatialMap> = Mutex::new(SpatialMap::new());
}

pub fn set_size(width: i32, height: i32) {
    let mut lock = SPATIAL_MAP.lock();
    lock.width = width;
    lock.height = height;

    let map_tile_count = (width * height) as usize;
    lock.opaque = vec![true; map_tile_count];
    lock.blocked = vec![(false, false); map_tile_count];
    lock.tile_content = vec![Vec::new(); map_tile_count];
}

pub fn index_entity(entity: Entity, idx: usize, blocks_tile: bool, blocks_visibility: bool) {
    let mut lock = SPATIAL_MAP.lock();
    lock.tile_content[idx].push((entity, blocks_tile, blocks_visibility));

    if blocks_tile {
        lock.blocked[idx].1 = true;
    }
    if blocks_visibility {
        lock.opaque[idx] = true;
    }
}

pub fn move_entity(entity: Entity, moving_from: usize, moving_to: usize) {
    let mut lock = SPATIAL_MAP.lock();
    let mut entity_blocks = false;
    let mut entity_opaque = false;
    lock.tile_content[moving_from].retain(|(e, blocks, opaque)| {
        if *e == entity {
            entity_blocks = *blocks;
            entity_opaque = *opaque;
            false
        } else {
            true
        }
    });
    lock.tile_content[moving_to].push((entity, entity_blocks, entity_opaque));

    // Recalculate blocks for both tiles
    let mut from_blocked = false;
    let mut to_blocked = false;
    let mut from_opaque = false;
    let mut to_opaque = false;
    lock.tile_content[moving_from].iter().for_each(|(_, blocks, opaque)| {
        if *blocks {
            from_blocked = true;
        }
        if *opaque {
            from_opaque = true;
        }
    });
    lock.tile_content[moving_to].iter().for_each(|(_, blocks, opaque)| {
        if *blocks {
            to_blocked = true;
        }
        if *opaque {
            to_opaque = true;
        }
    });

    lock.blocked[moving_from].1 = from_blocked;
    lock.blocked[moving_to].1 = to_blocked;
    lock.opaque[moving_from] = from_opaque;
    lock.opaque[moving_to] = to_opaque;
}

pub fn remove_entity(entity: Entity, idx: usize) {
    let mut lock = SPATIAL_MAP.lock();
    lock.tile_content[idx].retain(|(e, _, _)| *e != entity);

    let mut from_blocked = false;
    let mut from_opaque = false;
    lock.tile_content[idx].iter().for_each(|(_, blocks, opaque)| {
        if *blocks {
            from_blocked = true;
        }
        if *opaque {
            from_opaque = true;
        }
    });
    lock.blocked[idx].1 = from_blocked;
    lock.opaque[idx] = from_opaque;
}

///////////////////////////////////////////////////////////////////////////////
/// Blocked
///////////////////////////////////////////////////////////////////////////////

pub fn clear_blocked() {
    let mut lock = SPATIAL_MAP.lock();
    lock.blocked.iter_mut().for_each(|b| {
        b.0 = false;
        b.1 = false;
    });
}

pub fn populate_blocked_from_map(map: &Map) {
    let mut lock = SPATIAL_MAP.lock();
    for (i, tile) in map.tiles.iter().enumerate() {
        lock.blocked[i].0 = !tile.walkable;
    }
}

pub fn is_blocked(idx: usize) -> bool {
    let lock = SPATIAL_MAP.lock();
    lock.blocked[idx].0 || lock.blocked[idx].1
}

///////////////////////////////////////////////////////////////////////////////
/// Opaque
///////////////////////////////////////////////////////////////////////////////

pub fn clear_opaque() {
    let mut lock = SPATIAL_MAP.lock();
    lock.opaque.iter_mut().for_each(|o| {
        *o = false;
    });
}

pub fn populate_opaque_from_map(map: &Map) {
    let mut lock = SPATIAL_MAP.lock();
    for (i, tile) in map.tiles.iter().enumerate() {
        lock.opaque[i] = tile.opaque;
    }
}

pub fn is_opaque(idx: usize) -> bool {
    let lock = SPATIAL_MAP.lock();
    lock.opaque[idx]
}

///////////////////////////////////////////////////////////////////////////////
/// Tile Content
///////////////////////////////////////////////////////////////////////////////

pub fn clear_content_index() {
    let mut lock = SPATIAL_MAP.lock();
    for content in lock.tile_content.iter_mut() {
        content.clear();
    }
}

pub fn for_each_tile_content<F>(idx: usize, mut f: F)
where
    F: FnMut(Entity),
{
    let lock = SPATIAL_MAP.lock();
    for entity in lock.tile_content[idx].iter() {
        f(entity.0);
    }
}

pub fn for_each_tile_content_pt<F>(pt: Point, mut f: F)
where
    F: FnMut(Entity),
{
    let lock = SPATIAL_MAP.lock();
    let idx = lock.point2d_to_index(pt);
    for entity in lock.tile_content[idx].iter() {
        f(entity.0);
    }
}

pub fn for_each_tile_content_with_gamemode<F, S>(idx: usize, default: S, mut f: F) -> S
where
    F: FnMut(Entity) -> Option<S>,
{
    let lock = SPATIAL_MAP.lock();
    for entity in lock.tile_content[idx].iter() {
        if let Some(rs) = f(entity.0) {
            return rs;
        }
    }

    default
}

pub fn get_tile_content_clone(idx: usize) -> Vec<Entity> {
    let lock = SPATIAL_MAP.lock();
    lock.tile_content[idx].iter().map(|(e, _, _)| *e).collect()
}

pub fn get_tile_content_clone_pt(pt: Point) -> Vec<Entity> {
    let lock = SPATIAL_MAP.lock();
    let idx = lock.point2d_to_index(pt);
    lock.tile_content[idx].iter().map(|(e, _, _)| *e).collect()
}

pub fn get_tile_content_clone_filtered<F>(pt: Point, mut filter: F) -> Vec<Entity>
where
    F: FnMut(Entity) -> bool,
{
    let lock = SPATIAL_MAP.lock();
    let idx = lock.point2d_to_index(pt);
    lock.tile_content[idx].iter().filter(|(e, _, _)| filter(*e)).map(|(e, _, _)| *e).collect()
}
