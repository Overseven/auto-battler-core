#![no_std]

pub mod action;
pub mod common;
pub mod randomizer;

use crate::action::cases::common::{get_last_action_state, ActionType};
use crate::action::cases::{action_can_be_processed, process_action};
use crate::randomizer::Randomizer;
pub use common::*;
use sp_std::vec;
use sp_std::vec::Vec;

pub fn create_game(params: InitGameState) -> (GameContext, Randomizer) {
    let ctx = GameContext {
        players_initial: params.players,
        max_turns: params.max_turns,
        max_actions_per_turn: params.max_actions_per_turn,
        turns: vec![],
    };
    let randomizer = Randomizer {
        seed: params.seed,
        index: 0,
    };
    (ctx, randomizer)
}

fn get_command_with_more_total_hp(ctx: &GameContext) -> Option<u8> {
    if let Some(state) = get_last_action_state(&ctx) {
        let sides_total_health: Vec<u64> = state
            .players
            .iter()
            .map(|side| {
                side.iter()
                    .fold(0, |acc, x| acc + x.nft.characteristics.health)
            })
            .collect();
        if sides_total_health.is_empty() {
            return None;
        }
        if sides_total_health
            .iter()
            .all(|x| x == &sides_total_health[0])
        {
            return None;
        }
        let winner_side = sides_total_health.iter().enumerate().max();
        if let Some((side, _)) = winner_side {
            return Some(side as u8);
        }
    }
    None
}

fn roll_first_turn(randomizer: &mut Randomizer, players: u8) -> u8 {
    randomizer.random(players as u32) as u8 % players
}

pub fn process_full_game(ctx: &mut GameContext, randomizer: &mut Randomizer) -> GameResult {
    for _ in 0..ctx.max_turns {
        ctx.turns.push(process_turn(ctx, randomizer));
        if let Some(winner) = ctx.turns.last().unwrap().winner {
            return GameResult {
                winner: Some(winner),
                is_timeout: false,
            };
        }
    }
    // Timeout
    let winner = match get_command_with_more_total_hp(ctx) {
        None => Some(Winner::Nobody),
        Some(x) => Some(Winner::Command(x)),
    };

    return GameResult {
        winner,
        is_timeout: true,
    };
}

fn roll_action(
    command_id: u8,
    player_id: u8,
    mut skills: Vec<Skill>,
    ctx: &GameContext,
    current_turn: &TurnState,
    randomizer: &mut Randomizer,
) -> ActionType {
    while !skills.is_empty() {
        let random = randomizer.random(skills.len() as u32) as usize;
        let action = skills[random].action_type;
        if action_can_be_processed(action, command_id, player_id, &ctx, current_turn) {
            return action;
        } else {
            skills.remove(random);
        }
    }
    ActionType::PunchAction
}

fn init_turn(ctx: &GameContext, randomizer: &mut Randomizer) -> TurnState {
    let command_turn: u8;
    let mut player_turn = [0, 0];
    if ctx.turns.is_empty() {
        command_turn = roll_first_turn(randomizer, 2);
        for i in 0..ctx.players_initial.len() {
            player_turn[i] = roll_first_turn(randomizer, ctx.players_initial[i].len() as u8);
        }
    } else {
        let prev_turn = ctx.turns.last().unwrap();
        command_turn = (prev_turn.command_turn + 1) % ctx.players_initial.len() as u8;
        let mut next_player_turn = player_turn[command_turn as usize];
        if let Some(action) = prev_turn.actions.last() {
            let mut it = action.players[command_turn as usize].iter().cycle();
            for _ in 0..action.players[command_turn as usize].len() + 1 {
                if let Some(player) = it.next() {
                    if player.nft.characteristics.health > 0 {
                        next_player_turn = player.player_id;
                        break;
                    }
                }
            }
            player_turn[command_turn as usize] = next_player_turn;
        }
    }

    TurnState {
        player_turn,
        command_turn,
        actions: vec![],
        is_overflow: false,
        winner: None,
    }
}

pub fn process_turn(ctx: &GameContext, randomizer: &mut Randomizer) -> TurnState {
    let mut new_turn = init_turn(ctx, randomizer);

    let command_turn = new_turn.command_turn;
    let player_turn = new_turn.player_turn[new_turn.command_turn as usize];

    // todo: зачем тут while?
    while new_turn.actions.len() < ctx.max_actions_per_turn as usize {
        let action_type = roll_action(
            command_turn,
            player_turn,
            ctx.players_initial[command_turn as usize][player_turn as usize]
                .nft
                .skills
                .clone(),
            ctx,
            &new_turn,
            randomizer,
        );

        let (mut actions, a_winner) = process_action(
            action_type,
            command_turn,
            player_turn,
            &ctx,
            &new_turn,
            randomizer,
        );

        new_turn.actions.append(&mut actions);

        if a_winner.is_some() {
            new_turn.winner = a_winner.clone();
            return new_turn;
        }

        // todo: проверка на ответное действие ?
        let end_turn = true; // check_response();
        if end_turn {
            break;
        }
    }
    return new_turn;
}
