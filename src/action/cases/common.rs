use crate::{ActionState, GameContext, Player, Skill, TurnState};
use frame_support::pallet_prelude::*;
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;

#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Eq, PartialEq, Decode, Encode, RuntimeDebug, TypeInfo)]
pub enum ActionType {
    PunchAction = 1,
    HealAction = 2,
}

pub fn find_skill(skills: &Vec<Skill>, skill_type: ActionType) -> Option<Skill> {
    skills.iter().find_map(|x| {
        if x.action_type == skill_type {
            Some(x.clone())
        } else {
            None
        }
    })
}

pub fn get_last_action_state(ctx: &GameContext) -> Option<&ActionState> {
    if ctx.turns.is_empty() {
        return None;
    }
    if let Some(last_turn) = ctx.turns.last() {
        let last_turn_last_action = last_turn.actions.last();
        if last_turn_last_action.is_some() {
            return last_turn_last_action;
        }

        if let Some(prev_turn) = ctx.turns.get(ctx.turns.len()) {
            return prev_turn.actions.last();
        }
    }
    None
}

pub fn get_last_players_state(ctx: &GameContext, current_turn: &TurnState) -> [Vec<Player>; 2] {
    if let Some(a) = current_turn.actions.last() {
        return a.players.clone();
    }
    if let Some(last_turn) = ctx.turns.last() {
        if let Some(a) = last_turn.actions.last() {
            return a.players.clone();
        }
    }
    ctx.players_initial.clone()
}

// pub fn put_action_to_turn(action: ActionState, ctx: &mut GameContext) {
//     let turns_count = ctx.turns.len();
//     if let Some(last_turn) = ctx.turns.get_mut(turns_count) {
//         last_turn.actions.push(action);
//     }
// }
