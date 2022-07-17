use crate::modes::title::TitleMode;
use crate::prelude::*;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
pub enum TurnState {
    AwaitingInput,
    PlayerTurn,
    AITurn,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, StageLabel)]
pub enum GameStage {
    Update,
    PlayerStage,
    GenerateAIMoves,
    AIStage,
}

pub struct GameWorld {
    pub world: World,
    pub mode_stack: ModeStack,
    pub wait_for_event: bool,
}

impl GameWorld {
    pub fn new() -> Self {
        let world = World::new();
        let mode_stack = ModeStack::new(vec![TitleMode::new().into()]);

        Self {
            world,
            mode_stack,
            wait_for_event: false,
        }
    }

    pub fn update(&mut self, ctx: &mut BTerm) {
        match self.mode_stack.update(ctx, &mut self.world) {
            RunControl::Update => {}
            RunControl::WaitForEvent => self.wait_for_event = true,
            RunControl::Quit => ctx.quit(),
        }
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        if !self.wait_for_event {
            self.update(ctx);
        } else {
            // Wait for key event
            let mut is_event = false;

            match (ctx.key, ctx.left_click) {
                (None, false) => {}
                (None, true) => is_event = true,
                (Some(_), false) => is_event = true,
                (Some(_), true) => is_event = true,
            }

            if is_event {
                self.wait_for_event = false;
                self.update(ctx);
            }
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}
