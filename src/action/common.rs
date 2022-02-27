use crate::common::*;
use crate::Randomizer;
use sp_std::vec::Vec;

pub(crate) trait Action: Clone {
    fn can_be_processed(
        command_id: u8,
        player_id: u8,
        ctx: &GameContext,
        current_turn: &TurnState,
    ) -> bool;

    fn process(
        command_id: u8,
        player_id: u8,
        ctx: &GameContext,
        current_turn: &TurnState,
        randomizer: &mut Randomizer,
    ) -> Vec<ActionState>;
}
