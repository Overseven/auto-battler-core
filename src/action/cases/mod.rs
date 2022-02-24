use crate::action::cases::common::ActionType::{self, *};
use crate::action::cases::heal::Heal;
use crate::action::cases::punch::Punch;
use crate::action::common::Action;
use TurnState;

use crate::{get_last_action_state, ActionState, GameContext, Randomizer, Winner};

pub mod common;
pub mod heal;
pub mod punch;

pub fn action_can_be_processed(
    action_type: ActionType,
    command_id: u8,
    player_id: u8,
    ctx: &GameContext,
    current_turn: &TurnState,
) -> bool {
    match action_type {
        PunchAction => Punch::can_be_processed(command_id, player_id, ctx, current_turn),
        HealAction => Heal::can_be_processed(command_id, player_id, ctx, current_turn),
    }
}

/// Returns winner Option<command_id>
pub fn process_action(
    action_type: ActionType,
    command_id: u8,
    player_id: u8,
    ctx: &GameContext,
    current_turn: &TurnState,
    randomizer: &mut Randomizer,
) -> (Vec<ActionState>, Option<Winner>) {
    let actions = match action_type {
        PunchAction => Punch::process(command_id, player_id, ctx, current_turn, randomizer),
        HealAction => Heal::process(command_id, player_id, ctx, current_turn, randomizer),
    };

    let winner = check_winner(&actions);
    (actions, winner)
}

pub fn check_winner(last_actions: &Vec<ActionState>) -> Option<Winner> {
    if let Some(last_action) = last_actions.last() {
        let commands_health: Vec<u64> = last_action
            .players
            .iter()
            .map(|command| {
                command
                    .iter()
                    .map(|player| player.nft.characteristics.health)
                    .sum()
            })
            .collect();

        let non_zero: Vec<(usize, u64)> = commands_health
            .into_iter()
            .enumerate()
            .filter(|(_, command)| *command != 0)
            .collect();

        if non_zero.len() == 1 {
            return Some(Winner::Command(non_zero.first().unwrap().0 as u8));
        }
    }
    None
}
