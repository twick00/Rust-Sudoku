//! Gameboard view.

use graphics::types::{Color, Rectangle};
use graphics::{types, Context, Graphics};

use crate::gameboard_controller::GameboardController;
use graphics::math::Scalar;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size_x: f64,
    pub size_y: f64,
    /// Background color.
    pub background_color: Color,
    /// Border color.
    pub border_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge color between the 3x3 sections.
    pub section_edge_color: Color,
    /// Edge color between cells.
    pub cell_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge radius between the 3x3 sections.
    pub section_edge_radius: f64,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,

    pub selected_cell_background_color: Color,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [10.0; 2],
            size_x: 400.0,
            size_y: 400.0,
            background_color: [0.8, 0.8, 1.0, 1.0],
            border_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            section_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            section_edge_radius: 2.0,
            cell_edge_radius: 1.0,
            selected_cell_background_color: [0.9, 0.9, 1.0, 1.0],
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

#[derive(Debug, Copy, Clone)]
struct BoardRect {
    x: Scalar,
    y: Scalar,
    w: Scalar,
    h: Scalar,
}

impl BoardRect {
    fn new(x: Scalar, y: Scalar, w: Scalar, h: Scalar) -> BoardRect {
        BoardRect { x, y, w, h }
    }
}

impl Into<types::Rectangle> for BoardRect {
    fn into(self) -> [f64; 4] {
        return [self.x, self.y, self.w, self.h];
    }
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView { settings: settings }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};
        let ref settings = self.settings;
        let board_rect = BoardRect::new(
            settings.position[0],
            settings.position[1],
            settings.size_x,
            settings.size_y,
        );
        Rectangle::new(settings.background_color).draw(board_rect, &c.draw_state, c.transform, g);

        if let Some(ind) = controller.selected_cell {
            let cell_size_x = settings.size_x / 9.0;
            let cell_size_y = settings.size_y / 9.0;
            let pos = [ind[0] as f64 * cell_size_x, ind[1] as f64 * cell_size_y];
            let cell_rect = [
                settings.position[0] + pos[0],
                settings.position[1] + pos[1],
                cell_size_x,
                cell_size_y,
            ];
            Rectangle::new(settings.selected_cell_background_color).draw(
                cell_rect,
                &c.draw_state,
                c.transform,
                g,
            );
        }

        let cell_edge = Line::new(settings.cell_edge_color, settings.cell_edge_radius);
        let section_edge = Line::new(settings.section_edge_color, settings.section_edge_radius);

        for i in 0..9 {
            let x = settings.position[0] + i as f64 / 9.0 * settings.size_x;
            let y = settings.position[1] + i as f64 / 9.0 * settings.size_y;
            let x2 = settings.position[0] + settings.size_x;
            let y2 = settings.position[1] + settings.size_y;
            if (i % 3) == 0 {
                //Thicker lines separating quadrants
                let vline = [x, settings.position[1], x, y2];
                section_edge.draw(vline, &c.draw_state, c.transform, g);

                let hline = [settings.position[0], y, x2, y];
                section_edge.draw(hline, &c.draw_state, c.transform, g);
            } else {
                //Thin lines separating cells
                let vline = [x, settings.position[1], x, y2];
                cell_edge.draw(vline, &c.draw_state, c.transform, g);

                let hline = [settings.position[0], y, x2, y];
                cell_edge.draw(hline, &c.draw_state, c.transform, g);
            }
        }
        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius).draw(
            board_rect,
            &c.draw_state,
            c.transform,
            g,
        );
    }
}
