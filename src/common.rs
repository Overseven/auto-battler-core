use crate::action::cases::common::ActionType;
use sp_std::vec::Vec;

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum AutoBattlerCoreVersion {
    V1,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Winner {
    Nobody,
    Command(u8),
}

#[derive(Clone, Copy)]
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

#[derive(Clone, Copy)]
pub struct Skill {
    pub action_type: ActionType,
    // pub action: Rc<dyn Action>, // like that?
    pub level: u16,
}

#[derive(Clone)]
pub struct Nft {
    pub characteristics: Characteristics,
    pub skills: Vec<Skill>,
}

#[derive(Clone)]
pub struct Player {
    pub command: u8,
    pub player_id: u8,
    pub nft: Nft,
}

#[derive(Clone)]
pub struct InitGameState {
    pub auto_battler_core_version: AutoBattlerCoreVersion,
    pub players: [Vec<Player>; 2],
    pub max_turns: u64,
    pub max_actions_per_turn: u8,
    pub seed: Vec<u8>,
}

pub struct GameContext {
    pub auto_battler_core_version: AutoBattlerCoreVersion,
    pub max_turns: u64,
    pub max_actions_per_turn: u8,
    pub players_initial: [Vec<Player>; 2],
    pub turns: Vec<TurnState>,
}

#[derive(Clone)]
pub struct GameResult {
    pub winner: Option<Winner>,
    pub is_timeout: bool,
}

#[derive(Clone)]
pub struct ActionState {
    pub players: [Vec<Player>; 2],
    pub action: ActionType,
    pub origin: (u8, u8), // command_id, player_id
}

#[derive(Clone)]
pub struct TurnState {
    pub command_turn: u8,
    pub player_turn: [u8; 2],
    pub actions: Vec<ActionState>,
    pub is_overflow: bool,
    pub winner: Option<Winner>,
}
