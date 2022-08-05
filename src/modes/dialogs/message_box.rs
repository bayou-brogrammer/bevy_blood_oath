use super::*;

#[derive(Debug)]
pub enum MessageBoxModeResult {
    Done,
    AppQuit,
}

#[derive(Debug, Default)]
pub struct MessageBoxMode {
    inner_width: i32,
    msg: Vec<String>,
}

/// Show a multi-line message box.
impl MessageBoxMode {
    pub fn new(msg: Vec<String>) -> Self {
        let inner_width = msg.iter().map(|m| m.to_string().chars().count()).max().unwrap_or(0) as i32;
        Self { msg, inner_width }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        _app: &mut App,
        _pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(key) = ctx.key {
            if matches!(key, VirtualKeyCode::Return | VirtualKeyCode::Escape) {
                return (ModeControl::Pop(MessageBoxModeResult::Done.into()), ModeUpdate::Immediate);
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _world: &mut World, _active: bool) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(0);

        center_box(
            &mut draw_batch,
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            BoxConfig::new((self.inner_width, 10), ColorPair::new(WHITE, BLACK), true, false),
        );

        for (y, msg) in self.msg.iter().enumerate() {
            draw_batch.print(Point::new(2, 2 + y as i32), msg);
        }

        draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything
    }
}
