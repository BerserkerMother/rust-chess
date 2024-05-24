use std::{fmt::Display, io};

mod piece;
use piece::{Bishop, King, Knight, Pawn, Queen, Rook, Side};

// Define Visual elements
const HORIZANTAL_LINE: &str = "_________________"; // 17 _

fn main() {
    let mut board = Board::default();
    print!("\x1B[2J\x1B[1;1H");
    println!("{}", board);
    loop {
        let mut user_move = String::new();
        io::stdin()
            .read_line(&mut user_move)
            .expect("failed to get input!");
        user_move.pop();
        let user_move: Vec<usize> = user_move
            .split(" ")
            .map(|cor| cor.parse::<usize>().unwrap() - 1)
            .collect();
        match board.get_tile(user_move[0], user_move[1]) {
            Some(Position::Filled(piece)) => {
                if !piece.is_valid_move(
                    &board,
                    user_move[0],
                    user_move[1],
                    user_move[2],
                    user_move[3],
                ) {
                    continue;
                }
            }
            _ => (),
        }
        board.execute(user_move[0], user_move[1], user_move[2], user_move[3]);
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", board);
    }
}

pub enum PieceSide {
    White,
    Black,
}

enum Position {
    Empty,
    Filled(Box<dyn Piece>),
}

trait Piece: Display {
    fn is_valid_move(&self, board: &Board, xs: usize, ys: usize, xd: usize, yd: usize) -> bool {
        true
    }
}

struct Board {
    state: Vec<Position>,
}

impl Board {
    pub fn get_tile(&self, x: usize, y: usize) -> Option<&Position> {
        self.state.get(self.get_index(x, y))
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * 8 + x
    }
    pub fn execute(&mut self, xs: usize, ys: usize, xd: usize, yd: usize) -> bool {
        match self.get_tile(xs, ys) {
            Some(Position::Filled(piece)) => (),
            _ => {
                return false;
            }
        };

        let index_s = self.get_index(xs, ys);
        let index_d = self.get_index(xd, yd);

        self.state[index_d] = Position::Empty; // empty the destination tile
        self.state.swap(index_s, index_d);
        true
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut state = vec![
            Position::Filled(Box::new(Rook(Side::Black))),
            Position::Filled(Box::new(Knight(Side::Black))),
            Position::Filled(Box::new(Bishop(Side::Black))),
            Position::Filled(Box::new(Queen(Side::Black))),
            Position::Filled(Box::new(King(Side::Black))),
            Position::Filled(Box::new(Bishop(Side::Black))),
            Position::Filled(Box::new(Knight(Side::Black))),
            Position::Filled(Box::new(Rook(Side::Black))),
            Position::Filled(Box::new(Pawn(Side::Black))),
            Position::Filled(Box::new(Pawn(Side::Black))),
            Position::Filled(Box::new(Pawn(Side::Black))),
            Position::Filled(Box::new(Pawn(Side::Black))),
            Position::Filled(Box::new(Pawn(Side::Black))),
            Position::Filled(Box::new(Pawn(Side::Black))),
            Position::Filled(Box::new(Pawn(Side::Black))),
            Position::Filled(Box::new(Pawn(Side::Black))),
        ];
        for _ in 0..4 {
            for _ in 0..8 {
                state.push(Position::Empty)
            }
        }
        state.extend(
            vec![
                Position::Filled(Box::new(Rook(Side::White))),
                Position::Filled(Box::new(Knight(Side::White))),
                Position::Filled(Box::new(Bishop(Side::White))),
                Position::Filled(Box::new(Queen(Side::White))),
                Position::Filled(Box::new(King(Side::White))),
                Position::Filled(Box::new(Bishop(Side::White))),
                Position::Filled(Box::new(Knight(Side::White))),
                Position::Filled(Box::new(Rook(Side::White))),
                Position::Filled(Box::new(Pawn(Side::White))),
                Position::Filled(Box::new(Pawn(Side::White))),
                Position::Filled(Box::new(Pawn(Side::White))),
                Position::Filled(Box::new(Pawn(Side::White))),
                Position::Filled(Box::new(Pawn(Side::White))),
                Position::Filled(Box::new(Pawn(Side::White))),
                Position::Filled(Box::new(Pawn(Side::White))),
                Position::Filled(Box::new(Pawn(Side::White))),
            ]
            .into_iter()
            .rev(),
        );

        Board { state }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // let mut chess_visual = HORIZANTAL_LINE.to_string();
        for i in 0..8 {
            // chess_visual.push('\n');
            // chess_visual.push_str("|");
            for j in 0..8 {
                let char = self
                    .get_tile(j, i)
                    .map(|position| match position {
                        Position::Filled(piece) => {
                            write!(f, "{}", piece.to_string().as_str())
                            // chess_visual.push_str(format!("{}|", piece.to_string()).as_str())
                        }
                        Position::Empty => write!(f, "*"),
                    })
                    .unwrap(); // if ther position is empty just panic
            }
            writeln!(f);
            // chess_visual.push('\n');
            // chess_visual.push_str(HORIZANTAL_LINE);
        }
        // write!(f, "{}", chess_visual)
        Ok(())
    }
}

// why do I have to do this bro!
