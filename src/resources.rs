#[derive(PartialEq, Eq)]
pub enum NewState {
    NoChange,
    Wait,
    Player,
    Enemy,
    LeftMap,
}

#[derive(PartialEq, Eq, Clone)]
pub enum TurnState {
    WaitingForInput,
    PlayerTurn,
    EnemyTurn,
    Modal { title: String, body: String },
    GameOverLeft,
}
