#![cfg(test)]

use crate::{create_game, process_full_game, Characteristics, InitGameState, Nft, Player, Winner};

#[cfg(test)]
#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}

// #[test]
// fn randomizer() {
//     Randomizer {
//         seed: Seed,
//         index: 0,
//     };
//     let result = 2 + 2;
//     assert_eq!(result, 4);
// }

#[test]
fn full_game_process() {
    let (mut ctx, mut randomizer) = create_game(InitGameState {
        players: [
            vec![Player {
                command: 0,
                player_id: 0,
                nft: Nft {
                    characteristics: Characteristics {
                        health: 50,
                        strength: 10,
                        lucky: 0,
                        critical_chance: 0,
                        critical_factor: 0,
                        agility: 0,
                        accuracy: 0,
                        survivability: 0,
                    },
                    skills: vec![Skill {
                        action_type: ActionType::HealAction,
                        level: 5,
                    }],
                },
            }],
            vec![Player {
                command: 1,
                player_id: 0,
                nft: Nft {
                    characteristics: Characteristics {
                        health: 45,
                        strength: 11,
                        lucky: 0,
                        critical_chance: 0,
                        critical_factor: 0,
                        agility: 0,
                        accuracy: 0,
                        survivability: 0,
                    },
                    skills: vec![],
                },
            }],
        ],
        max_turns: 10,
        max_actions_per_turn: 4,
        seed: b"c3b656f074ee7cbdc6b1a209694c958582b6ac7cdf419d9fde9639fdb434c579".to_vec(),
    });
    let result = process_full_game(&mut ctx, &mut randomizer);
    assert_eq!(result.is_timeout, false);
    assert!(result.winner == Some(Winner::Command(0)));
    assert_eq!(ctx.turns.len(), 9);
}
