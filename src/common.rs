use crate::action::cases::common::ActionType;
use frame_support::pallet_prelude::*;
use frame_support::{Deserialize, Serialize};
use scale_info::TypeInfo;
use sp_std::vec::Vec;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Eq, PartialEq, Decode, Encode, RuntimeDebug, TypeInfo)]
pub enum AutoBattlerCoreVersion {
    V1,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Eq, PartialEq, Decode, Encode, RuntimeDebug, TypeInfo)]
pub struct Winner {
    pub command: Option<u8>,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Decode, Encode, RuntimeDebug, TypeInfo)]
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

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Decode, Encode, RuntimeDebug, TypeInfo)]
pub struct Skill {
    pub action_type: ActionType,
    pub level: u16,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Decode, Encode, RuntimeDebug, TypeInfo)]
pub struct Nft {
    pub characteristics: Characteristics,
    pub skills: Vec<Skill>,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Decode, Encode, RuntimeDebug, TypeInfo)]
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

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Decode, Encode, RuntimeDebug, TypeInfo)]
pub struct GameContext {
    pub auto_battler_core_version: AutoBattlerCoreVersion,
    pub max_turns: u64,
    pub max_actions_per_turn: u8,
    pub players_initial: [Vec<Player>; 2],
    pub turns: Vec<TurnState>,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Decode, Encode, PartialEq, Eq, RuntimeDebug, TypeInfo, Clone)]
pub struct GameResult {
    pub winner: Winner,
    pub is_timeout: bool,
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Decode, Encode, RuntimeDebug, TypeInfo)]
pub struct ActionState {
    pub players: [Vec<Player>; 2],
    pub action: ActionType,
    pub origin: (u8, u8), // command_id, player_id
}

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Decode, Encode, RuntimeDebug, TypeInfo)]
pub struct TurnState {
    pub command_turn: u8,
    pub player_turn: [u8; 2],
    pub actions: Vec<ActionState>,
    pub is_overflow: bool,
    pub winner: Winner,
}
