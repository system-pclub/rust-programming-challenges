struct Piece {
    x: u32,
    y: u32,
    name: &'static str,
}

impl Piece {
    fn exec(&self, target: &mut Piece) -> String {
        format!("{} -> {}", self.name, target.name)
    }
}

struct Board {
    pieces: Vec<Piece>,
}

fn do_exec(board: &mut Board) -> String {
    if let Some((a, b)) = board.pieces.split_first_mut() {
        a.exec(b.first_mut().unwrap())
    } else {
        "".to_string()
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task() {
        let mut board = Board {
            pieces: vec![
                Piece {
                    x: 0,
                    y: 0,
                    name: "A",
                },
                Piece {
                    x: 1,
                    y: 1,
                    name: "B",
                },
            ],
        };
        assert_eq!(do_exec(&mut board), "A -> B");
    }
}