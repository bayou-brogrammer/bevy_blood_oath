use super::*;

const CANCEL: &str = "[ Cancel ]";

#[derive(Debug)]
pub enum EquipmentActionModeResult {
    Cancelled,
    DropEquipment(Entity),
    RemoveEquipment(Entity),
}

#[derive(Debug)]
enum SubSection {
    Actions,
    Cancel,
}

#[allow(clippy::enum_variant_names)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum EquipmentAction {
    DropEquipment,
    RemoveEquipment,
}

impl EquipmentAction {
    pub fn from_key(key: VirtualKeyCode) -> Option<Self> {
        match key {
            VirtualKeyCode::A => Some(EquipmentAction::RemoveEquipment),
            VirtualKeyCode::D => Some(EquipmentAction::DropEquipment),
            _ => None,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            EquipmentAction::DropEquipment => "Drop",
            EquipmentAction::RemoveEquipment => "Remove",
        }
    }

    fn label(&self) -> &'static str {
        match self {
            EquipmentAction::DropEquipment => "[ Drop ]",
            EquipmentAction::RemoveEquipment => "[ Remove ]",
        }
    }
}

#[derive(Debug)]
pub struct EquipmentActionMode {
    item_id: Entity,
    inner_width: i32,
    selection: usize,
    subsection: SubSection,
    item_desc: (Glyph, String),
    actions: Vec<EquipmentAction>,
}

/// Show a menu of actions for an item currently equipped by the player.
impl EquipmentActionMode {
    pub fn new(world: &World, item_id: Entity, default_action: Option<EquipmentAction>) -> Self {
        let actions = [EquipmentAction::RemoveEquipment, EquipmentAction::DropEquipment].to_vec();
        let subsection = if actions.is_empty() { SubSection::Cancel } else { SubSection::Actions };

        let selection =
            default_action.and_then(|d_act| actions.iter().position(|a| *a == d_act)).unwrap_or(0);

        let item_width = world.get::<Naming>(item_id).unwrap().0.len();
        let inner_width = 2 + item_width
            .max(CANCEL.len())
            .max(actions.iter().map(|a| a.label().len()).max().unwrap_or(0));

        let item_glyph = *world.get::<Glyph>(item_id).unwrap();
        let item_name = world.get::<Naming>(item_id).unwrap().0.clone();

        Self {
            item_id,
            actions,
            selection,
            subsection,
            inner_width: inner_width as i32,
            item_desc: (item_glyph, item_name),
        }
    }

    fn confirm_action(&self) -> (ModeControl, ModeUpdate) {
        let result = match self.subsection {
            SubSection::Cancel => EquipmentActionModeResult::Cancelled,
            SubSection::Actions => match self.actions[self.selection as usize] {
                EquipmentAction::RemoveEquipment => {
                    EquipmentActionModeResult::RemoveEquipment(self.item_id)
                }
                EquipmentAction::DropEquipment => EquipmentActionModeResult::DropEquipment(self.item_id),
            },
        };

        (ModeControl::Pop(result.into()), ModeUpdate::Immediate)
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        _app: &mut App,
        _pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::Escape => {
                    return (
                        ModeControl::Pop(EquipmentActionModeResult::Cancelled.into()),
                        ModeUpdate::Immediate,
                    )
                }
                VirtualKeyCode::Down => match self.subsection {
                    SubSection::Actions => {
                        if self.selection < self.actions.len() - 1 {
                            self.selection += 1;
                        } else {
                            self.subsection = SubSection::Cancel;
                        }
                    }
                    SubSection::Cancel => {
                        if !self.actions.is_empty() {
                            self.subsection = SubSection::Actions;
                            self.selection = 0;
                        }
                    }
                },
                VirtualKeyCode::Up => match self.subsection {
                    SubSection::Actions => {
                        if self.selection > 0 {
                            self.selection -= 1;
                        } else {
                            self.subsection = SubSection::Cancel;
                        }
                    }
                    SubSection::Cancel => {
                        if !self.actions.is_empty() {
                            self.subsection = SubSection::Actions;
                            self.selection = self.actions.len() - 1;
                        }
                    }
                },
                VirtualKeyCode::Return => return self.confirm_action(),
                key @ VirtualKeyCode::R | key @ VirtualKeyCode::D => {
                    if let Some(equip_action) = EquipmentAction::from_key(key) {
                        if let Some(action_pos) = self.actions.iter().position(|a| *a == equip_action) {
                            if matches!(self.subsection, SubSection::Actions)
                                && self.selection == action_pos
                            {
                                return self.confirm_action();
                            } else {
                                self.subsection = SubSection::Actions;
                                self.selection = action_pos;
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _app: &mut App, _active: bool) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(0);

        let box_rect = center_box(
            &mut draw_batch,
            (MAP_PANEL_WIDTH, MAP_PANEL_HEIGHT),
            BoxConfig::new((self.inner_width, 10), ColorPair::new(WHITE, BLACK), true, false),
        );

        let x = box_rect.x1 + 1;
        let mut y = box_rect.y1 + 1;
        let (item_glyph, item_name) = &self.item_desc;

        draw_batch.set(Point::new(x, y), item_glyph.color, item_glyph.glyph);
        draw_batch.print(Point::new(x + 2, y), item_name);

        y += 2;
        for (i, action) in self.actions.iter().enumerate() {
            let bg = if matches!(self.subsection, SubSection::Actions) && i == self.selection {
                SELECTED_BG
            } else {
                BLACK
            };

            draw_batch.print_color_centered_at(
                Point::new(x + box_rect.width() / 2, y + i as i32),
                action.label(),
                ColorPair::new(WHITE, bg),
            );
        }

        draw_batch.print_color_centered_at(
            Point::new(x + box_rect.width() / 2, y + 3),
            CANCEL,
            ColorPair::new(
                WHITE,
                if matches!(self.subsection, SubSection::Cancel) { SELECTED_BG } else { BLACK },
            ),
        );

        draw_batch.submit(BATCH_UI_INV + 1000).expect("Batch error"); // On top of everything
    }
}
