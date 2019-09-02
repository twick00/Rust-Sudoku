use piston::input::{Button, GenericEvent, MouseButton};

use crate::Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
    pub selected_cell: Option<[usize; 2]>,
    cursor_pos: [f64; 2],
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        GameboardController {
            gameboard: gameboard,
            selected_cell: None,
            cursor_pos: [0.0; 2],
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, pos: [f64; 2], size: [f64; 2], e: &E) {
        if let Some(pos) = e.mouse_cursor_args() {
            self.cursor_pos = pos;
        }
        if let Some(Button::Mouse(MouseButton::Left)) = e.press_args() {
            let x = self.cursor_pos[0] - pos[0];
            let y = self.cursor_pos[1] - pos[1];
            if x >= 0.0 && x < size[0] && y >= 0.0 && y < size[1] {
                // Compute the cell position.
                let cell_x = (x / size[0] * 9.0) as usize;
                let cell_y = (y / size[1] * 9.0) as usize;
                self.selected_cell = Some([cell_x, cell_y]);
                println!("{:?}", &self.selected_cell)
            }
        }
    }
}
