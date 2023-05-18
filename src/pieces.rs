#![allow(dead_code)]
const RANKS: [&'static str; 8] = ["a", "b", "c", "d", "e", "f", "g", "h"];
const FILES: [&'static str; 8] = ["1", "2", "3", "4", "5", "6", "7", "8"];

#[derive(Clone)]
#[derive(Copy)]
pub struct Piece {
    rank: usize,
    file: usize
}
impl Piece { 
    pub fn get_pos(&mut self) -> String {
        let mut pos = String::new();
        pos.push_str(RANKS[self.rank]);
        pos.push_str(FILES[self.file]);
        pos
    }
}