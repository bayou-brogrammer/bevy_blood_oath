use crate::{
    modes::{main_menu, ModeStack, RunControl},
    prelude::*,
};

pub struct GameWorld {
    pub mode_stack: ModeStack,
}

impl GameWorld {
    pub fn new() -> Self {
        Self { mode_stack: ModeStack::new(vec![main_menu::MainMenuMode::new().into()]) }
    }
}

impl GameState for GameWorld {
    fn tick(&mut self, ctx: &mut BTerm) {
        match self.mode_stack.tick(ctx) {
            RunControl::Quit => {
                ctx.quit();
            }
            RunControl::Update => {}
        }

        render_draw_buffer(ctx).expect("Render error");
    }
}
