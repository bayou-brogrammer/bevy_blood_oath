use crate::prelude::*;

pub fn spawn_player(world: &mut World, start_pos: Point) -> Entity {
    world
        .create_entity()
        .with(Player)
        .with(Position::new(start_pos))
        .with(Glyph::new(to_cp437('@'), ColorPair::new(YELLOW, BLACK)))
        .with(Description(
            "Everybody's favorite Bracket Corp SecBot".to_string(),
        ))
        .with(Name("SecBot".to_string()))
        .with(FieldOfView::new(8))
        .with(Name("SecBot".to_string()))
        .with(CombatStats::new(30, 30, 2, 5))
        .build()
}

pub fn spawn_monster(world: &mut World, start_pos: Point, index: usize) -> Entity {
    let mut rng = crate::rng::RNG.write();

    let glyph: FontCharType;
    let name: String;
    match rng.roll_dice(1, 2) {
        1 => {
            glyph = to_cp437('g');
            name = "Goblin".to_string();
        }
        _ => {
            glyph = to_cp437('o');
            name = "Orc".to_string();
        }
    }

    world
        .create_entity()
        .with(Monster)
        .with(BlocksTile)
        .with(Position(start_pos))
        .with(Glyph::new(glyph, ColorPair::new(RED, BLACK)))
        .with(FieldOfView::new(6))
        .with(Name(format!("{} #{}", &name, index)))
        .with(CombatStats::new(16, 16, 1, 4))
        .build()
}
