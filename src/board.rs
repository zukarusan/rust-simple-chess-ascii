#![allow(dead_code)]
use super::pieces::Piece;
struct Board {
    squares: [[Option<Piece>; 8]; 8],
}
impl Board {
    pub fn new() -> Board {
        Board {
            squares: [[None; 8]; 8]
        }
    }
    pub fn reset_board(&mut self) {
        
    }
}