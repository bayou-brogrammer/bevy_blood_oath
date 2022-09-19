use crate::prelude::*;

#[derive(Component, Debug, Serialize, Deserialize, Reflect, Default)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Serialize, Deserialize, Reflect)]
pub struct Monster;

#[derive(Component, Debug, Serialize, Deserialize, Reflect)]
pub struct BlocksTile;

#[derive(Component, Debug, Serialize, Deserialize, Reflect)]
pub struct Item;

#[derive(Component, Debug, Serialize, Deserialize, Reflect)]
pub struct Consumable;

#[derive(Component, Debug, Serialize, Deserialize, Reflect)]
pub struct Dead;

#[derive(Component, Debug, Serialize, Deserialize, Reflect)]
pub struct Hidden;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Door(pub bool);

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Bystander {}

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Prop {}
