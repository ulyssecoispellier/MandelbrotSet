use macroquad::input::KeyCode::Escape;
use macroquad::prelude::*;
use MandelbrotSet::frame_coord_to_math_coord::frame_coord_to_math_coord;
use MandelbrotSet::init_axis::init_axis;
use MandelbrotSet::mandelbrot_calc::mandelbrot;
use MandelbrotSet::MAX_ITER;

fn window_conf() -> Conf {
    Conf {
        window_title: "Mandelbrot Set".to_string(),
        high_dpi: true,
        fullscreen: true,
        ..Default::default()
    }
}

fn fps(x: f32, y: f32) {
    let fps = get_fps();
    let text = format!("FPS: {}", fps);
    draw_text(&text, x, y, 25.0, RED);
}

#[macroquad::main(window_conf)]
async fn main() {
    'main_loop: loop {
        // On key press "ESCAPE" program stops
        if is_key_down(Escape) {
            break 'main_loop;
        }

        clear_background(BLACK);

        // Calculates then draws all the mandelbrot set
        mandelbrot(MAX_ITER);

        // Draws axis on screen
        init_axis(RED);

        // Draws current fps
        fps(25.0, 25.0);

        // Draws at the mouse position it's mathematical coordinates
        let current_mouse_math_coord = format!("{} | {}", frame_coord_to_math_coord((mouse_position().0, mouse_position().1)).0, frame_coord_to_math_coord((mouse_position().0, mouse_position().1)).1);
        draw_text(&current_mouse_math_coord, mouse_position().0, mouse_position().1, 25.0, GREEN);

        next_frame().await;
    }
}