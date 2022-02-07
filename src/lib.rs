mod tests;

pub struct Characteristics {
    pub health: u64,
    pub strength: u64,
    pub lucky: u64,
    pub critical_chance: u64,
    pub critical_factor: u64,
    pub agility: u64, // уклонение
    pub accuracy: u64,
    pub survivability: u64
}

pub struct Skill {

}

pub struct Nft {
    pub characteristics: Characteristics,
    pub skills: Vec<Skill>
}

pub struct Player {
    pub id: u64,
    pub nft: Nft,

}

pub struct InitGameState {

}

pub struct GameContext {
    pub step_count: u64,
    pub player_turn: u64 // чей сейчас ход

}

pub struct GameResult {
    pub winner: u64
}

pub enum Action {
    Nothing,
    Punch(u64),
    Skill(Skill)
}

pub struct Step {
    pub player_source: Player,
    pub player_dest: Player,
    pub action: Action,
}

pub struct TurnState {
    pub steps: Vec<Step>
}

pub fn create_game_context(params: InitGameState) -> GameContext{
    // TODO
    GameContext{ step_count: 0, player_turn: 0 }
}

pub fn process_full_game(ctx: &GameContext) -> GameResult {
    // TODO
    GameResult{ winner: 0 }
}

pub fn process_step(ctx: &GameContext) -> TurnState {
    // TODO
    TurnState{ steps: vec![] }
}
