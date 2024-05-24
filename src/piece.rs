use crate::{Piece, Position};
use std::fmt::Display;

#[derive(Eq, PartialOrd, Ord, PartialEq)]
pub enum Side {
    Black,
    White,
}

pub struct Rook(pub Side);

impl Display for Rook {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Side::Black {
            write!(f, "♖")
        } else {
            write!(f, "♜")
        }
    }
}

impl Piece for Rook {
    fn is_valid_move(
        &self,
        board: &crate::Board,
        xs: usize,
        ys: usize,
        xd: usize,
        yd: usize,
    ) -> bool {
        if xs != xd && ys != yd {
            return false;
        }
        let (dx, dy) = (xd as isize - xs as isize, yd as isize - ys as isize);
        let (step_x, step_y) = (dx.signum(), dy.signum());
        let (mut cx, mut cy) = (xs as isize + step_x, ys as isize + step_y);

        while (cx != xd as isize) || (cy != yd as isize) {
            if let Some(Position::Filled(_)) = board.get_tile(cx as usize, cy as usize) {
                return false;
            }
            cx += step_x;
            cy += step_y;
        }
        true
    }
}

pub struct Knight(pub Side);

impl Display for Knight {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Side::Black {
            write!(f, "♘")
        } else {
            write!(f, "♞")
        }
    }
}

impl Piece for Knight {
    fn is_valid_move(
        &self,
        _board: &crate::Board,
        xs: usize,
        ys: usize,
        xd: usize,
        yd: usize,
    ) -> bool {
        let dx = isize::abs(xs as isize - xd as isize);
        let dy = isize::abs(ys as isize - yd as isize);
        (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
    }
}

pub struct Bishop(pub Side);

impl Display for Bishop {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Side::Black {
            write!(f, "♗")
        } else {
            write!(f, "♝")
        }
    }
}

impl Piece for Bishop {
    fn is_valid_move(
        &self,
        board: &crate::Board,
        xs: usize,
        ys: usize,
        xd: usize,
        yd: usize,
    ) -> bool {
        if isize::abs(xs as isize - xd as isize) != isize::abs(ys as isize - yd as isize) {
            return false;
        }
        let (dx, dy) = (xd as isize - xs as isize, yd as isize - ys as isize);
        let (step_x, step_y) = (dx.signum(), dy.signum());
        let (mut cx, mut cy) = (xs as isize + step_x, ys as isize + step_y);

        while (cx != xd as isize) || (cy != yd as isize) {
            if let Some(Position::Filled(_)) = board.get_tile(cx as usize, cy as usize) {
                return false;
            }
            cx += step_x;
            cy += step_y;
        }
        true
    }
}

pub struct Queen(pub Side);

impl Display for Queen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Side::Black {
            write!(f, "♕")
        } else {
            write!(f, "♛")
        }
    }
}

impl Piece for Queen {
    fn is_valid_move(
        &self,
        board: &crate::Board,
        xs: usize,
        ys: usize,
        xd: usize,
        yd: usize,
    ) -> bool {
        let dx = isize::abs(xs as isize - xd as isize);
        let dy = isize::abs(ys as isize - yd as isize);
        if dx != dy && xs != xd && ys != yd {
            return false;
        }
        let (step_x, step_y) = (
            (xd as isize - xs as isize).signum(),
            (yd as isize - ys as isize).signum(),
        );
        let (mut cx, mut cy) = (xs as isize + step_x, ys as isize + step_y);

        while (cx != xd as isize) || (cy != yd as isize) {
            if let Some(Position::Filled(_)) = board.get_tile(cx as usize, cy as usize) {
                return false;
            }
            cx += step_x;
            cy += step_y;
        }
        true
    }
}

pub struct King(pub Side);

impl Display for King {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Side::Black {
            write!(f, "♔")
        } else {
            write!(f, "♚")
        }
    }
}

impl Piece for King {
    fn is_valid_move(
        &self,
        _board: &crate::Board,
        xs: usize,
        ys: usize,
        xd: usize,
        yd: usize,
    ) -> bool {
        let dx = isize::abs(xs as isize - xd as isize);
        let dy = isize::abs(ys as isize - yd as isize);
        dx <= 1 && dy <= 1
    }
}

pub struct Pawn(pub Side);

impl Display for Pawn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.0 == Side::Black {
            write!(f, "♙")
        } else {
            write!(f, "♟︎")
        }
    }
}

impl Piece for Pawn {
    fn is_valid_move(
        &self,
        board: &crate::Board,
        xs: usize,
        ys: usize,
        xd: usize,
        yd: usize,
    ) -> bool {
        let direction = if self.0 == Side::Black { 1 } else { -1 };
        let dy = (yd as isize - ys as isize) * direction;
        let dx = isize::abs(xs as isize - xd as isize);
        dbg!(dy);

        match dy {
            1 => {
                if dx == 0 {
                    matches!(board.get_tile(xd, yd), Some(Position::Empty))
                } else if dx == 1 {
                    matches!(board.get_tile(xd, yd), Some(Position::Filled(_)))
                } else {
                    false
                }
            }
            2 => {
                xs == xd
                    && matches!(
                        board.get_tile(xs, ys + direction as usize),
                        Some(Position::Empty)
                    )
                    && ((self.0 == Side::White && ys == 6) || (self.0 == Side::Black && ys == 1))
            }
            _ => false,
        }
    }
}
