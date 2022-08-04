use super::*;

const INVENTORY_BASE_WIDTH: i32 = 25;
const INVENTORY_BASE_HEIGHT: i32 = 4;
const INVENTORY_EQUIPMENT_OFFSET: i32 = 9;

struct InventoryHandle {
    equipment: Equipment,
    inv_selection: usize,
    subsection: SubSection,
    dimensions: (i32, i32),
    inventory: Vec<(Entity, String)>,
}

#[derive(Debug)]
struct Equipment {
    armor: Option<(Entity, String, Glyph)>,
    weapon: Option<(Entity, String, Glyph)>,
}

#[derive(Debug)]
enum SubSection {
    Inventory,
    EquipArmor,
    EquipWeapon,
}

impl_new!(
    InventoryHandle,
    inv_selection: usize,
    dimensions: (i32, i32),
    subsection: SubSection,
    inventory: Vec<(Entity, String)>,
    equipment: Equipment
);

impl_new!(Equipment, weapon: Option<(Entity, String, Glyph)>, armor: Option<(Entity, String, Glyph)>);

////////////////////////////////////////////////////////////////////////
// Equipment Subsection
////////////////////////////////////////////////////////////////////////

fn setup_inventory(
    mut commands: Commands,
    memory: Res<MenuMemory>,
    player_q: Query<Entity, With<Player>>,
    handle: Option<Res<InventoryHandle>>,
    backpack_q: Query<(Entity, &Naming, &InBackpack), With<Item>>,
    equipped_q: Query<(Entity, &Naming, &Glyph, &Equipped), With<Item>>,
) {
    let player = player_q.single();

    if handle.is_none() {
        let equipment = equipped_q.iter().filter(|(_, _, _, b)| b.owner == player).collect::<Vec<_>>();

        let weapon = equipment
            .iter()
            .find(|(_, _, _, equip)| equip.slot == EquipmentSlot::Melee)
            .map(|(e, name, glyph, _)| (*e, name.0.clone(), **glyph));

        let armor = equipment
            .iter()
            .find(|(_, _, _, equip)| equip.slot == EquipmentSlot::Shield)
            .map(|(e, name, glyph, _)| (*e, name.0.clone(), **glyph));

        let inventory = backpack_q
            .iter()
            .filter(|(_, _, b)| b.owner == player)
            .map(|b| (b.0, b.1 .0.clone()))
            .collect::<Vec<_>>();

        let inv_width = if !inventory.is_empty() {
            i32::max(
                INVENTORY_BASE_WIDTH,
                (inventory.iter().map(|s| s.1.len()).max().unwrap() + 8) as i32,
            )
        } else {
            INVENTORY_BASE_WIDTH // Base width for empty menu
        };

        let inv_height = if !inventory.is_empty() {
            i32::max(INVENTORY_BASE_HEIGHT, inventory.len() as i32 + 3)
        } else {
            INVENTORY_BASE_HEIGHT // Base height for empty menu
        } as i32;

        let inv_selection = memory[MenuMemory::INVENTORY].min(inventory.len().saturating_sub(1));

        commands.insert_resource(InventoryHandle::new(
            inv_selection,
            (inv_width, inv_height),
            SubSection::Inventory,
            inventory,
            Equipment::new(weapon, armor),
        ));
    }
}

fn draw_equipment(handle: Res<InventoryHandle>) -> Rect {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_TEXT);

    let (inv_width, inv_height) = handle.dimensions;
    let weapon_bg_color =
        if matches!(handle.subsection, SubSection::EquipWeapon) { SELECTED_BG } else { BLACK };

    let armor_bg_color =
        if matches!(handle.subsection, SubSection::EquipArmor) { SELECTED_BG } else { BLACK };

    let start_x = (MAP_PANEL_WIDTH / 2) - (inv_width / 2);
    let start_y = (MAP_PANEL_HEIGHT / 2) - (inv_height / 2);
    let equipment_box = box_with_title(
        &mut draw_batch,
        Point::new(start_x, start_y),
        BoxConfigWithTitle::new(
            BoxConfig::new((inv_width, 5), ColorPair::new(BOX_GRAY, BLACK), false, false),
            TextConfig::new("< Equipment >", ColorPair::new(CYAN, BLACK), Alignment::Left),
        ),
    );

    let eq_x = equipment_box.x1 + 1;
    let mut eq_y = equipment_box.y1 + 2;

    // Weapon
    draw_batch.print_color(Point::new(eq_x, eq_y), "Weapon:", ColorPair::new(WHITE, BLACK));
    if let Some(weapon) = &handle.equipment.weapon {
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
    if let Some(armor) = &handle.equipment.armor {
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

    draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything

    equipment_box
}

////////////////////////////////////////////////////////////////////////
// Inventory Subsection
////////////////////////////////////////////////////////////////////////
fn draw_inventory(In(equipment_box): In<Rect>, handle: Res<InventoryHandle>) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(LAYER_TEXT);

    let (inv_width, inv_height) = handle.dimensions;
    let bg_color = if matches!(handle.subsection, SubSection::Inventory) { SELECTED_BG } else { BLACK };

    let inv_box = box_with_title(
        &mut draw_batch,
        Point::new(equipment_box.x1, equipment_box.y2 + 1),
        BoxConfigWithTitle::new(
            BoxConfig::new((inv_width, inv_height), ColorPair::new(BOX_GRAY, BLACK), false, false),
            TextConfig::with_footer(
                "< Inventory >",
                "[Esc] to cancel",
                ColorPair::new(CYAN, BLACK),
                ColorPair::new(YELLOW, BLACK),
                Alignment::Left,
            ),
        ),
    );

    let x = inv_box.x1;
    let mut y = inv_box.y1;

    if handle.inventory.is_empty() {
        draw_batch.print_color_centered_at(
            Point::new(x + inv_box.width() / 2, y + inv_box.height() / 2),
            "-- Empty --",
            ColorPair::new(WHITE, bg_color),
        );
    } else {
        for (j, item) in handle.inventory.iter().enumerate() {
            menu_option(
                &mut draw_batch,
                x + 1,
                y + 2,
                97 + j as FontCharType,
                &item.1,
                matches!(handle.subsection, SubSection::Inventory) && handle.inv_selection == j,
            );
            y += 1;
        }
    }

    draw_batch.submit(BATCH_UI_INV).expect("Batch error"); // On top of everything
}

fn inventory_input(
    mut commands: Commands,
    key: Option<Res<VirtualKeyCode>>,
    mut handle: ResMut<InventoryHandle>,
) {
    if let Some(game_key) = key.as_deref().get_key() {
        match (&handle.subsection, game_key) {
            (_, GameKey::Escape) => return commands.insert_resource(TurnState::AwaitingInput),
            ////////////////////////////////////////////////////
            // Sub Section Weapon
            ////////////////////////////////////////////////////
            (SubSection::EquipWeapon, GameKey::Up) => {
                handle.subsection = SubSection::Inventory;
                handle.inv_selection =
                    if handle.inventory.is_empty() { 0 } else { handle.inventory.len() - 1 }
            }
            (SubSection::EquipWeapon, GameKey::Down) => {
                handle.subsection = SubSection::EquipArmor;
            }
            (SubSection::EquipWeapon, GameKey::Select) => {
                if let Some(weapon) = &handle.equipment.weapon {
                    // return (
                    //     ModeControl::Push(EquipmentActionMode::new(world, weapon.0, None).into()),
                    //     ModeUpdate::Update,
                    // );
                }
            }
            ////////////////////////////////////////////////////
            // Sub Section Armor
            ////////////////////////////////////////////////////
            (SubSection::EquipArmor, GameKey::Up) => {
                handle.subsection = SubSection::EquipWeapon;
            }
            (SubSection::EquipArmor, GameKey::Down) => {
                handle.subsection = SubSection::Inventory;
                handle.inv_selection = 0;
            }
            (SubSection::EquipArmor, GameKey::Select) => {
                if let Some(armor) = &handle.equipment.armor {
                    // return (
                    //     ModeControl::Push(EquipmentActionMode::new(world, armor.0, None).into()),
                    //     ModeUpdate::Update,
                    // );
                }
            }
            ////////////////////////////////////////////////////
            // Sub Section Inventory
            ////////////////////////////////////////////////////
            (SubSection::Inventory, GameKey::Up) => {
                if handle.inv_selection > 0 {
                    handle.inv_selection -= 1;
                } else {
                    handle.subsection = SubSection::EquipArmor;
                }
            }
            (SubSection::Inventory, GameKey::Down) => {
                if !handle.inventory.is_empty() && handle.inv_selection < handle.inventory.len() - 1 {
                    handle.inv_selection += 1;
                } else {
                    handle.subsection = SubSection::EquipWeapon;
                }
            }
            (SubSection::Inventory, GameKey::Select) => {
                if !handle.inventory.is_empty() {
                    let item = handle.inventory[handle.inv_selection as usize].0;
                    // return (
                    //     ModeControl::Push(InventoryActionMode::new(world, item, None).into()),
                    //     ModeUpdate::Update,
                    // );
                }
            }
            _ => {}
        }
    }
}

pub struct InventoryMenuPlugin;
impl Plugin for InventoryMenuPlugin {
    fn build(&self, app: &mut App) {
        // GUI Inventory Systems
        app.add_system_set(
            ConditionSet::new()
                .label("setup_inventory_menu")
                .run_if_resource_equals(TurnState::Inventory)
                .with_system(setup_inventory)
                .into(),
        );

        app.add_system_set(
            ConditionSet::new()
                .after("setup_inventory_menu")
                .run_if_resource_equals(TurnState::Inventory)
                .run_if_resource_exists::<InventoryHandle>()
                .with_system(draw_equipment.chain(draw_inventory))
                .with_system(inventory_input)
                .into(),
        );
    }
}
