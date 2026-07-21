use piston_window::graphics::{rectangle, Context, Graphics};
type Color = [f32; 4];

const BLOCK_SIZE: f64 = 25.0;
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: i32) -> u32 {
    to_coord(game_coord) as u32
}

pub fn draw_block<G: Graphics>(color: Color, x: i32, y: i32, context: &Context, g: &mut G) {
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(color, [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE], context.transform, g);
}

pub fn draw_rectangle<G: Graphics>(color: Color, x: f64, y: f64, width: f64, height: f64, context: &Context, g: &mut G) {
    rectangle(color, [x, y, width, height], context.transform, g);
}
