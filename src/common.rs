use crate::action::cases::common::ActionType;
#[cfg(feature = "std")]
use js_sys::Array;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "std")]
use sp_std::vec;
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "std", wasm_bindgen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub struct Winner {
    pub command: Option<u8>,
}

#[cfg_attr(feature = "std", wasm_bindgen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
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

#[cfg_attr(feature = "std", wasm_bindgen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy)]
pub struct Skill {
    pub action_type: ActionType,
    pub level: u16,
}

#[cfg(not(feature = "std"))]
#[derive(Clone)]
pub struct Nft {
    pub characteristics: Characteristics,
    pub skills: Vec<Skill>,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Nft {
    pub characteristics: Characteristics,
    #[wasm_bindgen(skip)]
    pub skills: Vec<Skill>,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
impl Nft {
    #[wasm_bindgen(getter)]
    pub fn skills(&self) -> Array {
        self.skills
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
    }
}

#[cfg(not(feature = "std"))]
#[derive(Clone)]
pub struct Player {
    pub command: u8,
    pub player_id: u8,
    pub nft: Nft,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub command: u8,
    pub player_id: u8,
    #[wasm_bindgen(skip)]
    pub nft: Nft,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(getter)]
    pub fn nft(&self) -> JsValue {
        return JsValue::from(self.nft.clone());
    }
}

#[cfg(not(feature = "std"))]
#[derive(Clone)]
pub struct GameContext {
    pub max_turns: u64,
    pub max_actions_per_turn: u8,
    pub players_initial: [Vec<Player>; 2],
    pub turns: Vec<TurnState>,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct GameContext {
    pub max_turns: u64,
    pub max_actions_per_turn: u8,
    #[wasm_bindgen(skip)]
    pub players_initial: [Vec<Player>; 2],
    #[wasm_bindgen(skip)]
    pub turns: Vec<TurnState>,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
impl GameContext {
    #[wasm_bindgen(getter)]
    pub fn players_initial(&self) -> Array {
        self.players_initial
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn turns(&self) -> Array {
        self.turns
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
    }
}

#[cfg_attr(feature = "std", wasm_bindgen)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Clone, Copy)]
pub struct GameResult {
    pub winner: Option<Winner>,
    pub is_timeout: bool,
}

#[cfg(not(feature = "std"))]
#[derive(Clone)]
pub struct ActionState {
    pub players: [Vec<Player>; 2],
    pub action: ActionType,
    pub origin: (u8, u8), // command_id, player_id
}

#[cfg(feature = "std")]
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct ActionState {
    #[wasm_bindgen(skip)]
    pub players: [Vec<Player>; 2],
    pub action: ActionType,
    #[wasm_bindgen(skip)]
    pub origin: (u8, u8), // command_id, player_id
}

#[cfg(feature = "std")]
#[wasm_bindgen]
impl ActionState {
    #[wasm_bindgen(getter)]
    pub fn players(&self) -> Array {
        self.players
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
    }
    #[wasm_bindgen(getter)]
    pub fn origin(&self) -> Array {
        vec![self.origin.0, self.origin.1]
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
    }
}

#[cfg(not(feature = "std"))]
#[derive(Clone)]
pub struct TurnState {
    pub command_turn: u8,
    pub player_turn: [u8; 2],
    pub actions: Vec<ActionState>,
    pub is_overflow: bool,
    pub winner: Option<Winner>,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct TurnState {
    pub command_turn: u8,
    #[wasm_bindgen(skip)]
    pub player_turn: [u8; 2],
    #[wasm_bindgen(skip)]
    pub actions: Vec<ActionState>,
    pub is_overflow: bool,
    pub winner: Option<Winner>,
}

#[cfg(feature = "std")]
#[wasm_bindgen]
impl TurnState {
    #[wasm_bindgen(getter)]
    pub fn player_turn(&self) -> Array {
        self.player_turn
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
    }

    #[wasm_bindgen(getter)]
    pub fn actions(&self) -> Array {
        self.actions
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
    }
}
