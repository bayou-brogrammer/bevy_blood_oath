use crate::prelude::*;

pub fn spawn_random_colonist(world: &mut World, location: Point, layer: usize) {
    world
        .create_entity()
        .with(Colonist)
        .with(Position::with_pt(location, layer))
        .with(Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(LIME_GREEN, BLACK),
        })
        .with(Description(
            "A squishy friend. You are here to rescue your squishies.".to_string(),
        ))
        .with(ColonistStatus::Alive)
        .build();
}
