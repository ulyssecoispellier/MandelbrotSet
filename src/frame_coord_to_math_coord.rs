use macroquad::prelude::{screen_height, screen_width};

/// Converts screen frame coordinates (pixels) into corresponding math (Cartesian) coordinates.
pub fn frame_coord_to_math_coord(frame_coord: (f32, f32)) -> (f32, f32) {
    // Convert x: shift origin to vertical axis at 2/3 screen width and scale by 575
    let math_x = (frame_coord.0 - (screen_width()-screen_width()/3.0))/575.0;

    // Convert y: shift origin to horizontal axis at half screen height and scale by 500
    let math_y = (screen_height()/2.0 - frame_coord.1)/500.0;

    // Return math coordinates (x, y)
    (math_x, math_y)
}