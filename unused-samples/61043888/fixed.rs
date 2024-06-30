use std::cell::RefCell;
use std::rc::Rc;

struct Player {
    pub ship: Rc<RefCell<Ship>>,
}

impl Player {
    pub fn update_position(&self) {
        let mut ship = self.ship.borrow_mut();
        ship.x += 1.0;
        ship.y += 1.0;
    }

    pub fn get_position(&self) -> String {
        let ship = self.ship.borrow();
        format!(
            "Ship's position: ({}, {})\n",
            ship.x,
            ship.y
        )
    }
}

struct Ship {
    pub players: Vec<Rc<RefCell<Player>>>,
    pub x: f64,
    pub y: f64,
}

impl Ship {
    pub fn add_player(&mut self, player: Rc<RefCell<Player>>) {
        self.players.push(player);
    }
}

fn task(n_players: usize) -> String {
    let mut ret = String::new();
    let ship = Rc::new(RefCell::new(Ship {
        players: vec![],
        x: 0.0,
        y: 0.0,
    }));

    for _ in 0..n_players {
        let player = Rc::new(RefCell::new(Player {
            ship: Rc::clone(&ship),
        }));
        ship.borrow_mut().add_player(player);
    }

    for player in ship.borrow().players.iter() {
        player.borrow().update_position();
    }

    for player in ship.borrow().players.iter() {
        let result = player.borrow().get_position();
        ret.push_str(&result);
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ship() {
        println!("{}", task(2));
    }
}
