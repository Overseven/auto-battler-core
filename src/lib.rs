mod randomizer;
mod tests;

pub type Seed = [u8; 32];

pub struct Characteristics {
    pub health: u64,
    pub strength: u64,
    pub lucky: u64,
    pub critical_chance: u64,
    pub critical_factor: u64,
    pub agility: u64, // уклонение
    pub accuracy: u64,
    pub survivability: u64,
}

pub struct Skill {}

pub struct Nft {
    pub characteristics: Characteristics,
    pub skills: Vec<Skill>,
}

pub struct Player {
    pub id: u64,
    pub nft: Nft,
}

pub struct InitGameState {
    pub player1: Player,
    pub player2: Player,
    pub max_step: u64,
    pub seed: Seed,
}

pub struct GameContext {
    pub step_count: u64,
    pub max_step: u64,
    pub player_turn: u8, // чей сейчас ход
    pub player1: Player,
    pub player2: Player,
    pub seed: Seed,
}

pub struct GameResult {
    pub winner: u8,
    pub is_timeout: bool,
}

pub enum Action {
    Nothing,
    Punch(u64),
    Skill(Skill),
}

pub struct Step {
    pub player_source: Player,
    pub player_dest: Player,
    pub action: Action,
}

pub struct TurnState {
    pub steps: Vec<Step>,
    pub winner: Option<u8>,
}

pub fn create_game_context(params: InitGameState) -> GameContext {
    GameContext {
        step_count: 0,
        player_turn: 0,
        player1: params.player1,
        player2: params.player2,
        max_step: params.max_step,
        seed: params.seed,
    }
}

fn check_who_have_more_hp(ctx: &GameContext) -> u8 {
    // todo
    unimplemented!()
}

fn roll_first_turn(seed: &Seed, players: u8) -> u8 {
    seed[2] % players
}

pub fn process_full_game(ctx: &mut GameContext) -> GameResult {
    ctx.player_turn = roll_first_turn(&ctx.seed, 2);

    for _ in 0..ctx.max_step {
        let turn = process_step(ctx);
        if let Some(winner) = turn.winner {
            return GameResult {
                winner,
                is_timeout: false,
            };
        }
    }
    // Timeout
    let winner = check_who_have_more_hp(ctx);
    return GameResult {
        winner,
        is_timeout: true,
    };
}

pub fn process_step(mut ctx: &GameContext) -> TurnState {
    // 1. определяется, будет ли использоваться скилл
    // 2.а если да:
    //    process_skill():
    //
    // 2.б если нет:
    //    process_punch():
    //
    // TODO
    TurnState {
        steps: vec![],
        winner: None,
    }
}
