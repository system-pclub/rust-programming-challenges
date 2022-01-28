struct Card<'t> {
    id: String,
    content: &'t str
}

struct CardsConnectionEdge<'t> {
    cursor: String,
    node: Card<'t>
}

struct CardsConnection<'t> {
    edges: Vec<CardsConnectionEdge<'t>>
}

impl<'t> CardsConnection<'t> {
    fn new(cards: Vec<Card<'t>>) -> Self {
        CardsConnection {
            edges: cards.iter().map(|c| CardsConnectionEdge {
                cursor: c.id.clone(),
                node: *c
            }).collect::<Vec<CardsConnectionEdge>>()
        }
    }
}

fn main() {}