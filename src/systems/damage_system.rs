use super::{CombatStats, Player, SufferDamage};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct DamageSystem {}

impl<'a> System<'a> for DamageSystem {
    type SystemData = (
        WriteStorage<'a, CombatStats>,
        WriteStorage<'a, SufferDamage>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut stats, mut damage) = data;

        for (mut stats, damage) in (&mut stats, &damage).join() {
            stats.hp -= damage.amount.iter().sum::<i32>();
        }

        damage.clear();
    }
}

pub fn delete_the_dead(ecs: &mut World) {
    let dead_entities;

    {
        let entities = ecs.entities();
        let players = ecs.read_storage::<Player>();
        let combat_stats = ecs.read_storage::<CombatStats>();

        dead_entities = (&entities, &combat_stats, (&players).maybe())
            .par_join()
            .filter(|(_, stats, player)| stats.hp < 1 && player.is_none())
            .map(|(ent, _, _)| ent)
            .collect::<Vec<Entity>>();
    }

    ecs.delete_entities(&dead_entities);
}

struct ScopeCall<F: FnMut()> {
    c: F,
}
impl<F: FnMut()> Drop for ScopeCall<F> {
    fn drop(&mut self) {
        (self.c)();
    }
}

macro defer($e:expr) {
    let _scope_call = ScopeCall {
        c: || -> () {
            $e;
        },
    };
}
