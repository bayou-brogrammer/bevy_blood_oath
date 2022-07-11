use crate::prelude::*;

pub fn spawn_random_colonist(
    location: Point,
    layer: usize,
) -> (Colonist, Position, Glyph, Description, ColonistStatus) {
    (
        Colonist {},
        Position::with_pt(location, layer),
        Glyph {
            glyph: to_cp437('☺'),
            color: ColorPair::new(LIME_GREEN, BLACK),
        },
        Description("A squishy friend. You are here to rescue your squishies.".to_string()),
        ColonistStatus::Alive,
    )
}
