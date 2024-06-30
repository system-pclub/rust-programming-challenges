struct Player<'a> {
    pub ship: &'a Ship<'a>,
}

impl<'a> Player<'a> {
    pub fn run(&mut self) {
        println!("Ship's position: ({}, {})", self.ship.x, self.ship.y);
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
        x: 10.0,
        y: 20.0,
    };

    let player = Player { ship: &ship };
    ship.add_player(player);
}