use std::cell::RefCell;
use std::rc::Rc;

struct Player {
    pub ship: Rc<RefCell<Ship>>,
}

impl Player {
    pub fn run(&mut self) {
        // Does some computing with self.ship.x/self.ship.y
    }
}

struct Ship {
    pub players: Vec<Player>,
    pub x: f64,
    pub y: f64,
}

impl Ship {
    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}

fn main() {
    let mut ship = Rc::new(RefCell::new(Ship {
        players: vec![],
        x: 0.0,
        y: 0.0,
    }));

    // At some point create a player for the ship
    let player = Player { ship: Rc::clone(&ship) };
    ship.borrow_mut().add_player(player); // <- Forbidden
}