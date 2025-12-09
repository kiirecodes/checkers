use super::prelude::*;

#[derive(Clone, Copy)]
pub struct Square {
    pub x: u8,
    pub y: u8,
    pub color: Color,
    pub is_occupied: bool,
}

impl Square {
    fn new(x: u8, y: u8) -> Self {
        let color = if (x + y) % 2 == 0 {
            Color::BLACK
        } else {
            Color::WHITE
        };

        let occupied = match color {
            Color::BLACK => {
                if y == 1 || y == 2 || y == 7 || y == 8 {
                    true
                } else {
                    false
                }
            }

            Color::WHITE => false,
        };
        Square {
            x,
            y,
            color: color,
            is_occupied: occupied,
        }
    }
}

pub struct Board {
    pub squares: Vec<Square>,
}

impl Board {
    pub fn init() -> Self {
        let mut squares = Vec::new();
        for x in 1..=8 {
            for y in 1..=8 {
                squares.push(Square::new(x, y));
            }
        }
        Board { squares }
    }
}
