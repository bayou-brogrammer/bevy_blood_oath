use crate::prelude::*;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Player;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Monster;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct BlocksTile;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Item;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Consumable;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Dead;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Hidden;

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Door(pub bool);

#[derive(Component, Debug, Serialize, Deserialize, Clone)]
pub struct Bystander {}
