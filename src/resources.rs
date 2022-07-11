#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NewState {
    NoChange,
    Wait,
    // Tick,
    Player,
    Enemy,
    LeftMap,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum TurnState {
    WaitingForInput,
    PlayerTurn,
    EnemyTurn,
    // Ticking,
    GameOverLeft,
    Modal { title: String, body: String },
}
