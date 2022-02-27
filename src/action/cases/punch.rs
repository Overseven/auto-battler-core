use crate::action::cases::common::get_last_players_state;
use crate::action::common::Action;
use crate::{ActionState, ActionType, GameContext, Nft, Player, Randomizer, TurnState};
use sp_std::vec;
use sp_std::vec::Vec;

#[derive(Clone)]
pub(crate) struct Punch {}

impl Action for Punch {
    fn can_be_processed(
        _command_id: u8,
        _player_id: u8,
        _ctx: &GameContext,
        _current_turn: &TurnState,
    ) -> bool {
        true
    }

    fn process(
        command_id: u8,
        player_id: u8,
        ctx: &GameContext,
        current_turn: &TurnState,
        randomizer: &mut Randomizer,
    ) -> Vec<ActionState> {
        let prev_state = get_last_players_state(ctx, current_turn);
        let who_beats = &prev_state[command_id as usize][player_id as usize];
        let who_get_hit = define_aim(who_beats, command_id, &prev_state, randomizer);
        let mut action = ActionState {
            players: prev_state,
            action: ActionType::PunchAction,
            origin: (command_id, player_id),
        };
        {
            let health = &mut action.players[who_get_hit.0 as usize][who_get_hit.1 as usize]
                .nft
                .characteristics
                .health;
            *health = health.saturating_sub(who_get_hit.2);
        }
        vec![action]
    }
}

fn calc_damage(who_beats: &Nft, _who_get_hit: &Nft, _randomizer: &mut Randomizer) -> u64 {
    // todo: more complex calculation
    who_beats.characteristics.strength
}

/// returns (command_id, player_id, damage)
fn define_aim(
    who_beats: &Player,
    command_id: u8,
    players: &[Vec<Player>; 2],
    randomizer: &mut Randomizer,
) -> (u8, u8, u64) {
    let damages = players
        .iter()
        .enumerate()
        .filter_map(|(cmd_id, command)| {
            if cmd_id == command_id as usize {
                return None;
            }
            return Some(
                command
                    .iter()
                    .enumerate()
                    .filter_map(|(player_id, player)| {
                        if players[cmd_id][player_id].nft.characteristics.health == 0 {
                            return None;
                        }
                        Some((
                            cmd_id as u8,
                            player_id as u8,
                            calc_damage(&who_beats.nft, &player.nft, randomizer),
                        ))
                    })
                    .collect::<Vec<(u8, u8, u64)>>(),
            );
        })
        .flatten()
        .collect::<Vec<(u8, u8, u64)>>();

    let who_may_died = damages
        .iter()
        .filter_map(|(cmd_id, pl_id, dmg)| {
            if players[*cmd_id as usize][*pl_id as usize]
                .nft
                .characteristics
                .health
                > *dmg
            {
                return None;
            }
            return Some((*cmd_id as u8, *pl_id as u8, *dmg));
        })
        .collect::<Vec<(u8, u8, u64)>>();

    if let Some(&aim) = who_may_died.first() {
        return aim;
    }

    damages[randomizer.random(damages.len() as u32) as usize]
}

#[cfg(test)]
#[test]
fn test_define_aim() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
