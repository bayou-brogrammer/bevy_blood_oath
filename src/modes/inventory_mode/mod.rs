use super::{menu_memory::MenuMemory, *};

pub mod equipment_action;
pub mod inventory_action;

pub use equipment_action::*;
pub use inventory_action::*;

const INVENTORY_BASE_WIDTH: i32 = 25;
const INVENTORY_BASE_HEIGHT: i32 = 4;
const INVENTORY_EQUIPMENT_OFFSET: i32 = 9;

#[derive(Debug)]
pub enum InventoryModeResult {
    DoNothing,
    DropItem(Entity),
    EquipItem(Entity),
    DropEquipment(Entity),
    RemoveEquipment(Entity),
    UseItem(Entity, Option<Point>),
}

#[derive(Debug, Default)]
enum SubSection {
    #[default]
    Inventory,
    EquipArmor,
    EquipWeapon,
}

#[derive(Debug, Default)]
struct Equipment {
    armor: Option<(Entity, String, Glyph)>,
    weapon: Option<(Entity, String, Glyph)>,
}

#[derive(Debug, Default)]
pub struct InventoryMode {
    equipment: Equipment,
    inv_selection: usize,
    dimensions: (i32, i32),
    subsection: SubSection,
    inventory: Vec<(Entity, String)>,
}

impl_new!(Equipment, weapon: Option<(Entity, String, Glyph)>, armor: Option<(Entity, String, Glyph)>);

/// Show a screen with items carried by the player, and allow them to be manipulated.
impl InventoryMode {
    pub fn new(world: &mut World) -> Self {
        let mut system_state: SystemState<(
            Res<Entity>,
            Query<(Entity, &Naming, &InBackpack), With<Item>>,
            Query<(Entity, &Naming, &Glyph, &Equipped), With<Item>>,
        )> = SystemState::new(world);

        let (player, backpack_q, equipped_q) = system_state.get(world);

        let inventory = backpack_q
            .iter()
            .filter(|(_, _, b)| b.owner == *player)
            .map(|b| (b.0, b.1 .0.clone()))
            .collect::<Vec<_>>();

        let equipment = equipped_q.iter().filter(|(_, _, _, b)| b.owner == *player).collect::<Vec<_>>();

        let weapon = equipment
            .iter()
            .find(|(_, _, _, equip)| equip.slot == EquipmentSlot::Melee)
            .map(|(e, name, glyph, _)| (*e, name.0.clone(), **glyph));

        let armor = equipment
            .iter()
            .find(|(_, _, _, equip)| equip.slot == EquipmentSlot::Shield)
            .map(|(e, name, glyph, _)| (*e, name.0.clone(), **glyph));

        let inv_selection =
            world.resource::<MenuMemory>()[MenuMemory::INVENTORY].min(inventory.len().saturating_sub(1));

        let inv_width = if !inventory.is_empty() {
            i32::max(INVENTORY_BASE_WIDTH, (inventory.iter().map(|s| s.1.len()).max().unwrap() + 8) as i32)
        } else {
            INVENTORY_BASE_WIDTH // Base width for empty menu
        };

        let inv_height = if !inventory.is_empty() {
            i32::max(INVENTORY_BASE_HEIGHT, inventory.len() as i32 + 3)
        } else {
            INVENTORY_BASE_HEIGHT // Base height for empty menu
        } as i32;

        Self {
            inventory,
            inv_selection,
            subsection: SubSection::Inventory,
            dimensions: (inv_width, inv_height),
            equipment: Equipment::new(weapon, armor),
        }
    }

    pub fn tick(
        &mut self,
        ctx: &mut BTerm,
        app: &mut App,
        pop_result: &Option<ModeResult>,
    ) -> (ModeControl, ModeUpdate) {
        if let Some(result) = pop_result {
            return match result {
                ////////////////////////////////////////////////////////////////////////////////////////
                // Inventory Action
                ////////////////////////////////////////////////////////////////////////////////////////
                ModeResult::InventoryActionModeResult(result) => match result {
                    InventoryActionModeResult::Cancelled => (ModeControl::Stay, ModeUpdate::Update),
                    InventoryActionModeResult::UseItem(item_id, pt) => (
                        ModeControl::Pop(InventoryModeResult::UseItem(*item_id, *pt).into()),
                        ModeUpdate::Immediate,
                    ),
                    InventoryActionModeResult::DropItem(item_id) => (
                        ModeControl::Pop(InventoryModeResult::DropItem(*item_id).into()),
                        ModeUpdate::Immediate,
                    ),
                    InventoryActionModeResult::EquipItem(item_id) => (
                        ModeControl::Pop(InventoryModeResult::EquipItem(*item_id).into()),
                        ModeUpdate::Immediate,
                    ),
                },
                ////////////////////////////////////////////////////////////////////////////////////////
                // Equipment Action
                ////////////////////////////////////////////////////////////////////////////////////////
                ModeResult::EquipmentActionModeResult(result) => match result {
                    EquipmentActionModeResult::Cancelled => (ModeControl::Stay, ModeUpdate::Update),
                    EquipmentActionModeResult::RemoveEquipment(item_id) => (
                        ModeControl::Pop(InventoryModeResult::RemoveEquipment(*item_id).into()),
                        ModeUpdate::Immediate,
                    ),
                    EquipmentActionModeResult::DropEquipment(item_id) => (
                        ModeControl::Pop(InventoryModeResult::DropEquipment(*item_id).into()),
                        ModeUpdate::Immediate,
                    ),
                },
                _ => unreachable!("InventoryMode::tick: Unexpected ModeResult"),
            };
        }

        if let Some(key) = ctx.key {
            match (&self.subsection, key) {
                (_, VirtualKeyCode::Escape) => {
                    return (ModeControl::Pop(InventoryModeResult::DoNothing.into()), ModeUpdate::Update)
                }
                ////////////////////////////////////////////////////
                // Sub Section Weapon
                ////////////////////////////////////////////////////
                (SubSection::EquipWeapon, VirtualKeyCode::Up) => {
                    self.subsection = SubSection::Inventory;
                    self.inv_selection = if self.inventory.is_empty() { 0 } else { self.inventory.len() - 1 }
                }
                (SubSection::EquipWeapon, VirtualKeyCode::Down) => {
                    self.subsection = SubSection::EquipArmor;
                }
                (SubSection::EquipWeapon, VirtualKeyCode::Return) => {
                    if let Some(weapon) = &self.equipment.weapon {
                        return (
                            ModeControl::Push(EquipmentActionMode::new(&app.world, weapon.0, None).into()),
                            ModeUpdate::Update,
                        );
                    }
                }
                ////////////////////////////////////////////////////
                // Sub Section Armor
                ////////////////////////////////////////////////////
                (SubSection::EquipArmor, VirtualKeyCode::Up) => {
                    self.subsection = SubSection::EquipWeapon;
                }
                (SubSection::EquipArmor, VirtualKeyCode::Down) => {
                    self.subsection = SubSection::Inventory;
                    self.inv_selection = 0;
                }
                (SubSection::EquipArmor, VirtualKeyCode::Return) => {
                    if let Some(armor) = &self.equipment.armor {
                        return (
                            ModeControl::Push(EquipmentActionMode::new(&app.world, armor.0, None).into()),
                            ModeUpdate::Update,
                        );
                    }
                }
                ////////////////////////////////////////////////////
                // Sub Section Inventory
                ////////////////////////////////////////////////////
                (SubSection::Inventory, VirtualKeyCode::Up) => {
                    if self.inv_selection > 0 {
                        self.inv_selection -= 1;
                    } else {
                        self.subsection = SubSection::EquipArmor;
                    }
                }
                (SubSection::Inventory, VirtualKeyCode::Down) => {
                    if !self.inventory.is_empty() && self.inv_selection < self.inventory.len() - 1 {
                        self.inv_selection += 1;
                    } else {
                        self.subsection = SubSection::EquipWeapon;
                    }
                }
                (SubSection::Inventory, VirtualKeyCode::Return) => {
                    if !self.inventory.is_empty() {
                        let item = self.inventory[self.inv_selection as usize].0;
                        return (
                            ModeControl::Push(InventoryActionMode::new(&app.world, item, None).into()),
                            ModeUpdate::Update,
                        );
                    }
                }
                (SubSection::Inventory, key)
                    if matches!(key, VirtualKeyCode::E | VirtualKeyCode::A | VirtualKeyCode::D) =>
                {
                    if let Some(item_id) = self.inventory.get(self.inv_selection as usize) {
                        if let Some(inv_action) = InventoryAction::from_key(key) {
                            if InventoryAction::item_supports_action(&app.world, item_id.0, inv_action) {
                                return (
                                    ModeControl::Push(
                                        InventoryActionMode::new(&app.world, item_id.0, Some(inv_action))
                                            .into(),
                                    ),
                                    ModeUpdate::Update,
                                );
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        (ModeControl::Stay, ModeUpdate::Update)
    }

    pub fn draw(&self, _ctx: &mut BTerm, _world: &mut World, _active: bool) {
        let mut draw_batch = DrawBatch::new();
        draw_batch.target(LAYER_TEXT);

        let equipment_box = self.draw_equipment(&mut draw_batch);
        self.draw_inventory(&mut draw_batch, &equipment_box);

        draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything
    }
}

impl InventoryMode {
    fn draw_equipment(&self, draw_batch: &mut DrawBatch) -> Rect {
        ////////////////////////////////////////////////////////////////////////
        // Equipment Subsection
        ////////////////////////////////////////////////////////////////////////

        let weapon_bg_color =
            if matches!(self.subsection, SubSection::EquipWeapon) { SELECTED_BG } else { BLACK };

        let armor_bg_color =
            if matches!(self.subsection, SubSection::EquipArmor) { SELECTED_BG } else { BLACK };

        let (inv_width, inv_height) = self.dimensions;
        let start_x = (MAP_PANEL_WIDTH / 2) - (inv_width / 2);
        let start_y = (MAP_PANEL_HEIGHT / 2) - (inv_height);
        let equipment_box = box_with_title(
            draw_batch,
            Point::new(start_x, start_y),
            BoxConfigWithTitle::new(
                BoxConfig::new((inv_width, 5), ColorPair::new(BOX_GRAY, BLACK), false, false),
                TextConfig::new("< Equipment >", ColorPair::new(CYAN, BLACK), Alignment::Left, false),
            ),
        );

        let eq_x = equipment_box.x1 + 1;
        let mut eq_y = equipment_box.y1 + 2;

        // Weapon
        draw_batch.print_color(Point::new(eq_x, eq_y), "Weapon:", ColorPair::new(WHITE, BLACK));
        if let Some(weapon) = &self.equipment.weapon {
            draw_batch.set(
                Point::new(eq_x + INVENTORY_EQUIPMENT_OFFSET - 1, eq_y),
                weapon.2.color,
                weapon.2.glyph,
            );
            draw_batch.print_color(
                Point::new(eq_x + INVENTORY_EQUIPMENT_OFFSET + 1, eq_y),
                weapon.1.clone(),
                ColorPair::new(WHITE, weapon_bg_color),
            );
        } else {
            draw_batch.print_color(
                Point::new(eq_x + INVENTORY_EQUIPMENT_OFFSET, eq_y),
                "-- No Weapon --",
                ColorPair::new(WHITE, weapon_bg_color),
            );
        }

        // Armor
        eq_y += 1;
        draw_batch.print_color(Point::new(eq_x, eq_y), "Armor:", ColorPair::new(WHITE, BLACK));
        if let Some(armor) = &self.equipment.armor {
            draw_batch.set(
                Point::new(eq_x + INVENTORY_EQUIPMENT_OFFSET - 1, eq_y),
                armor.2.color,
                armor.2.glyph,
            );
            draw_batch.print_color(
                Point::new(eq_x + INVENTORY_EQUIPMENT_OFFSET + 1, eq_y),
                armor.1.clone(),
                ColorPair::new(WHITE, armor_bg_color),
            );
        } else {
            draw_batch.print_color(
                Point::new(eq_x + INVENTORY_EQUIPMENT_OFFSET, eq_y),
                "-- No Armor --",
                ColorPair::new(WHITE, armor_bg_color),
            );
        }

        equipment_box
    }

    fn draw_inventory(&self, draw_batch: &mut DrawBatch, equipment_box: &Rect) {
        ////////////////////////////////////////////////////////////////////////
        // Inventory Subsection
        ////////////////////////////////////////////////////////////////////////

        let (inv_width, inv_height) = self.dimensions;
        let bg_color = if matches!(self.subsection, SubSection::Inventory) { SELECTED_BG } else { BLACK };

        let inv_box = box_with_title(
            draw_batch,
            Point::new(equipment_box.x1, equipment_box.y2 + 1),
            BoxConfigWithTitle::new(
                BoxConfig::new((inv_width, inv_height), ColorPair::new(BOX_GRAY, BLACK), false, false),
                TextConfig::with_footer(
                    "< Inventory >",
                    "[Esc] to cancel",
                    ColorPair::new(CYAN, BLACK),
                    ColorPair::new(YELLOW, BLACK),
                    Alignment::Left,
                    false,
                ),
            ),
        );

        let x = inv_box.x1;
        let mut y = inv_box.y1;

        if self.inventory.is_empty() {
            draw_batch.print_color_centered_at(
                Point::new(x + inv_box.width() / 2, y + inv_box.height() / 2),
                "-- Empty --",
                ColorPair::new(WHITE, bg_color),
            );
        } else {
            for (j, item) in self.inventory.iter().enumerate() {
                let selected = matches!(self.subsection, SubSection::Inventory) && self.inv_selection == j;
                menu_option(draw_batch, x + 1, y + 2, 97 + j as FontCharType, &item.1, selected);
                y += 1;
            }
        }
    }
}
