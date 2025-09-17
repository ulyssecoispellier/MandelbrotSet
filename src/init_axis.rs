use macroquad::color::{Color};
use macroquad::prelude::{draw_line, draw_text};
use macroquad::window::{screen_height, screen_width};

/// Draws coordinate axes and markers on the screen for reference.
pub fn init_axis(colour: Color) {
    // Draw the horizontal X-axis line across the middle of the screen
    draw_line(0.0, screen_height()/2.0, screen_width()/1.0, screen_height()/2.0, 2.5, colour);

    // Draw the vertical Y-axis line shifted to the right (at 2/3 of the screen width)
    draw_line(screen_width()-screen_width()/3.0, 0.0, screen_width()-screen_width()/3.0, screen_height(), 2.5, colour);

    // Draw small vertical tick mark at x = -1 position on the axis
    draw_line((screen_width()-screen_width()/3.0)/2.0, screen_height()/2.0-10.0, (screen_width()-screen_width()/3.0)/2.0, screen_height()/2.0+10.0, 2.5, colour);

    // Draw small vertical tick mark at x = -2 position on the axis
    draw_line(0.0, screen_height()/2.0-10.0, 0.0, screen_height()/2.0+10.0, 2.5, colour);

    // Label the tick mark at x = -1
    draw_text("-1", (screen_width()-screen_width()/3.0)/2.0-25.0, screen_height()/2.0-15.0, 50.0, colour);

    // Label the tick mark at x = -2
    draw_text("-2", 0.0, screen_height()/2.0-15.0, 50.0, colour);
}