use super::board::{Board, Square};
use super::prelude::*;

pub struct Move {
    pre_move: Option<Box<Move>>,
    post_move: Option<Box<Move>>,
    count: u8,
    direction: Direction,
    piece: Piece,
}

impl Move {
    //TODO:    fn is_valid(&self) -> bool {}
    //TODO:    fn proceed(&self) -> bool {}
    //TODO:    fn process_error(&self) -> Result<(), String> {}
    //TODO:    pub fn make(&self, board::Board) -> Result<(), String> {}
    //TODO:    pub fn undo(&self) -> Result<(), String> {}
    //TODO:    pub fn redo(&self) -> Result<(), String> {}

    pub fn destination(&self) -> Box<Square> {
        let square = Box::new(self.piece.position.clone());
        match &self.direction {
            &Direction::LeftUpper => {
                square.x - self.count;
                square.y + self.count;
            }

            &Direction::LeftLower => {
                square.x - self.count;
                square.y - self.count;
            }

            &Direction::RightUpper => {
                square.x + self.count;
                square.y + self.count;
            }

            &Direction::RightLower => {
                square.x + self.count;
                square.y - self.count;
            }
        }

        square
    }
}

pub struct Piece {
    position: Square,
    is_queen: bool,
    color: Color,
}

enum Direction {
    LeftLower,
    LeftUpper,
    RightLower,
    RightUpper,
}
