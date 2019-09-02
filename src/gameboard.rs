const SIZE: usize = 9;
pub struct Gameboard {
    pub cells: [[u8; SIZE]; SIZE],
}
impl Gameboard {
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }
    pub fn get_size() -> usize {
        SIZE
    }
}
