use crate::action::cases::common::ActionType::HealAction;
use crate::action::cases::common::{find_skill, get_last_action_state};
use crate::action::common::Action;
use crate::{ActionState, GameContext, Randomizer};
use TurnState;

// 1 level of heal skill == +10 HP per use
pub const HP_PER_LEVEL: u64 = 10;

#[derive(Clone)]
pub(crate) struct Heal {}

impl Action for Heal {
    fn can_be_processed(
        command_id: u8,
        player_id: u8,
        ctx: &GameContext,
        current_turn: &TurnState,
    ) -> bool {
        if let Some(last_action) = current_turn.actions.last() {
            let player_state = &last_action.players[command_id as usize][player_id as usize];
            let current_health = player_state.nft.characteristics.health;
            let initial_health = ctx.players_initial[command_id as usize][player_id as usize]
                .nft
                .characteristics
                .health;
            // heal only if current health is lower than 60% of maximum HP
            return current_health < initial_health * 6 / 10;
        }

        false
    }

    fn process(
        command_id: u8,
        player_id: u8,
        ctx: &GameContext,
        current_turn: &TurnState,
        _randomizer: &mut Randomizer,
    ) -> Vec<ActionState> {
        if let Some(last_action) = current_turn.actions.last() {
            let player = &last_action.players[command_id as usize][player_id as usize];
            let current_health = player.nft.characteristics.health;
            let max_health = ctx.players_initial[command_id as usize][player_id as usize]
                .nft
                .characteristics
                .health;
            if let Some(skill) = find_skill(&player.nft.skills, HealAction) {
                let heal_size = skill.level as u64 * HP_PER_LEVEL;
                let new_hp = current_health + heal_size;
                let current_health = if new_hp > max_health {
                    max_health
                } else {
                    new_hp
                };
                let mut new_action = ActionState {
                    players: last_action.players.clone(),
                    action: HealAction,
                    origin: (command_id, player_id),
                };
                new_action.players[command_id as usize][player_id as usize]
                    .nft
                    .characteristics
                    .health = current_health;
                return vec![new_action];
            }
        }
        vec![]
    }
}
