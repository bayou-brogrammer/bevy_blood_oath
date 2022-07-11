use super::*;

mod entrance;
pub use entrance::build_entrance;

mod colonist;
pub use colonist::spawn_random_colonist;

fn all_space(layer: &mut Layer) {
    layer.tiles.iter_mut().for_each(|t| {
        *t = Tile::empty();
    });
}
