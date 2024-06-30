struct Kingdom {
    gold: u32,
    soldiers: u32,
}

struct GameState {
    kingdoms: Vec<Kingdom>,
}

fn try_attack(game_state: &mut GameState, target_index: usize) {
    let player = game_state.kingdoms.first_mut().unwrap();

    let target = game_state
        .kingdoms
        .get(target_index)
        .unwrap();

    if target.soldiers < player.soldiers || target.gold < player.gold {
        player.gold += (target.gold as f64 * 0.1) as u32;
        player.soldiers += (target.soldiers as f64 * 0.05) as u32;
    }
}


fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_attack() {
        let mut game_state = GameState {
            kingdoms: vec![
                Kingdom {
                    gold: 100,
                    soldiers: 100,
                },
                Kingdom {
                    gold: 50,
                    soldiers: 50,
                },
            ],
        };

        try_attack(&mut game_state, 1);

        assert_eq!(game_state.kingdoms[0].gold, 105);
        assert_eq!(game_state.kingdoms[0].soldiers, 102);
    }
}