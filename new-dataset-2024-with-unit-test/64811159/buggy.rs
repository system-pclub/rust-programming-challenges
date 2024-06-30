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
        let mut edges = Vec::<CardsConnectionEdge>::with_capacity(cards.len());

        for c in cards.iter() {
            edges.push(CardsConnectionEdge {
                cursor: c.id.clone(),
                node: *c
            })
        }

        CardsConnection {
            edges
        }
    }
}

fn main() {}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cards_connection() {
        let cards = vec![
            Card { id: "1".to_string(), content: "Card 1" },
            Card { id: "2".to_string(), content: "Card 2" },
            Card { id: "3".to_string(), content: "Card 3" }
        ];

        let connection = CardsConnection::new(cards);

        assert_eq!(connection.edges.len(), 3);
        assert_eq!(connection.edges[0].cursor, "1");
        assert_eq!(connection.edges[0].node.content, "Card 1");
        assert_eq!(connection.edges[1].cursor, "2");
        assert_eq!(connection.edges[1].node.content, "Card 2");
        assert_eq!(connection.edges[2].cursor, "3");
        assert_eq!(connection.edges[2].node.content, "Card 3");
    }
}