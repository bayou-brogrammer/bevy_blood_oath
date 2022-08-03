use crate::prelude::*;

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

impl CombatStats {
    pub fn new(max_hp: i32, hp: i32, defense: i32, power: i32) -> Self {
        assert!(max_hp > 0 && hp > 0 && hp <= max_hp);

        CombatStats { max_hp, hp, defense, power }
    }
}

// #[derive(Component, Debug)]
// pub struct SufferDamage {
//     pub amount: Vec<i32>,
// }

// impl SufferDamage {
//     pub fn new_damage(
//         commands: &mut Commands,
//         store: &mut Query<&mut SufferDamage>,
//         victim: Entity,
//         amount: i32,
//     ) {
//         if let Ok(mut suffering) = store.get_mut(victim) {
//             suffering.amount.push(amount);
//         } else {
//             let dmg = SufferDamage {
//                 amount: vec![amount],
//             };
//             commands.entity(victim).insert(dmg);
//         }
//     }
// }