struct Player<'a> {
    pub ship: &'a Ship<'a>,
}

impl<'a> Player<'a> {
    pub fn run(&mut self) {
        // Does some computing with self.ship.x/self.ship.y
    }
}

struct Ship<'a> {
    pub players: Vec<Player<'a>>,
    pub x: f64,
    pub y: f64,
}

impl<'a> Ship<'a> {
    pub fn add_player(&mut self, player: Player<'a>) {
        self.players.push(player);
    }
}

fn main() {
    let mut ship = Ship {
        players: vec![],
        x: 0.0,
        y: 0.0,
    };

    // At some point create a player for the ship
    let player = Player { ship: &ship };
    ship.add_player(player); // <- Forbidden
}