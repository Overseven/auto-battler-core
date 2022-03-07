use crate::action::cases::common::ActionType;
use js_sys::Array;
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;
use wasm_bindgen::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Winner {
    pub command: Option<u8>,
}

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
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

#[wasm_bindgen]
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Skill {
    pub action_type: ActionType,
    pub level: u16,
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Nft {
    pub characteristics: Characteristics,
    #[wasm_bindgen(skip)]
    pub skills: Vec<Skill>,
}

#[wasm_bindgen]
impl Nft {
    #[wasm_bindgen(getter)]
    pub fn skills(&self) -> Array {
        self.skills
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
        // self.skills.clone().into()
    }
}

#[wasm_bindgen]
#[derive(Clone, Serialize, Deserialize)]
pub struct Player {
    pub command: u8,
    pub player_id: u8,
    #[wasm_bindgen(skip)]
    pub nft: Nft,
}

#[wasm_bindgen]
impl Player {
    #[wasm_bindgen(getter)]
    pub fn nft(&self) -> JsValue {
        return JsValue::from(self.nft.clone());
    }
}

#[derive(Clone)]
pub struct InitGameState {
    pub players: [Vec<Player>; 2],
    pub max_turns: u64,
    pub max_actions_per_turn: u8,
    pub seed: Vec<u8>,
}

#[wasm_bindgen]
pub struct GameContext {
    pub max_turns: u64,
    pub max_actions_per_turn: u8,
    #[wasm_bindgen(skip)]
    pub players_initial: [Vec<Player>; 2],
    #[wasm_bindgen(skip)]
    pub turns: Vec<TurnState>,
}

#[wasm_bindgen]
impl GameContext {
    #[wasm_bindgen(getter)]
    pub fn players_initial(&self) -> Array {
        self.players_initial
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
        // self.players_initial.iter().map(JsValue::from).collect()
        // return JsValue::from(self.players_initial.clone());
    }
    #[wasm_bindgen(getter)]
    pub fn turns(&self) -> Array {
        self.turns
            .iter()
            .map(|x| JsValue::from_serde(x).unwrap())
            .collect()
        // self.turns.iter().map(JsValue::from).collect()
        // return JsValue::from(self.turns.clone());
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct GameResult {
    pub winner: Option<Winner>,
    pub is_timeout: bool,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ActionState {
    pub players: [Vec<Player>; 2],
    pub action: ActionType,
    pub origin: (u8, u8), // command_id, player_id
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TurnState {
    pub command_turn: u8,
    pub player_turn: [u8; 2],
    pub actions: Vec<ActionState>,
    pub is_overflow: bool,
    pub winner: Option<Winner>,
}
