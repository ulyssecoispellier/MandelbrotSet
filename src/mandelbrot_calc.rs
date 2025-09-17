use macroquad::color::{BLACK, WHITE};
use macroquad::prelude::{draw_rectangle, screen_height, screen_width};
use crate::frame_coord_to_math_coord::frame_coord_to_math_coord;

//////////////////////////////////// Variable explanation \\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\\
/*
    screen_w, screen_h   -> Cached screen dimensions in pixels.
    cr, ci               -> Real and imaginary parts of the complex coordinate c.
    cr_step              -> Increment in the real coordinate per pixel along a row.
    zr, zi               -> Real and imaginary parts of the current z in iteration.
    zr2, zi2             -> Squares of zr and zi, cached for efficiency.
    escaped              -> Flag indicating if the current point escaped the Mandelbrot set.
    cr0_f, ci_f          -> Mapped math coordinates for the first pixel in a row.
    cr1_f                -> Mapped real coordinate for the second pixel in a row (used for step).
    cr_p1                -> Helper variable (cr + 1.0) used for period-2 bulb test.
    cr_m_q               -> Helper variable (cr - 0.25) used for cardioid test.
    q                    -> Temporary value in cardioid membership calculation.
*/
//////////////////////////////////////////////////////////////////////////////////////////////////

/// Tests if c is in the main cardioid or in the 2nd period bulb to shortcut big and heavy calculations
fn in_main_cardioid_or_period2_bulb(cr: f64, ci: f64) -> bool {
    // Period-2 bulb: (cr + 1)^2 + ci^2 < 1/16
    let cr_p1 = cr + 1.0;
    if (cr_p1 * cr_p1 + ci * ci) < 0.0625 {
        return true;
    }

    // Main cardioid test:
    // q = (cr - 0.25)^2 + ci^2
    // q * (q + (cr - 0.25)) < 0.25 * ci^2
    let cr_m_q = cr - 0.25;
    let q = cr_m_q * cr_m_q + ci * ci;
    q * (q + cr_m_q) < 0.25 * ci * ci
}

#[inline]
pub fn mandelbrot(max_iter: i32) {
    // Cache screen dimensions to avoid repeated FFI calls
    let screen_w = screen_width() as i32;
    let screen_h = screen_height() as i32;

    // Iterate row-wise for better cache behavior
    for j in 0..screen_h {
        // Compute the complex-plane mapping for the first two pixels in this row
        // to derive a constant per-pixel step along the row. This avoids an expensive
        // call to `frame_coord_to_math_coord` for every pixel.
        let (cr0_f, ci_f) = frame_coord_to_math_coord((0.0, j as f32));
        let (cr1_f, _ci_unused) = frame_coord_to_math_coord((1.0, j as f32));
        let mut cr = cr0_f as f64;
        let cr_step = (cr1_f - cr0_f) as f64;
        let ci = ci_f as f64;

        for i in 0..screen_w {
            // Quick acceptance: interior of cardioid or period-2 bulb
            if in_main_cardioid_or_period2_bulb(cr, ci) {
                draw_rectangle(i as f32, j as f32, 1.0, 1.0, WHITE);
                cr += cr_step;
                continue;
            }

            // Manual complex iteration using f64s to avoid overhead of Complex ops.
            let mut zr: f64 = 0.0;
            let mut zi: f64 = 0.0;
            let mut zr2: f64 = 0.0; // zr^2
            let mut zi2: f64 = 0.0; // zi^2

            let mut escaped = false;
            for _ in 0..max_iter {
                // z <- z^2 + c where z = zr + i*zi
                // Compute imaginary first because it needs the previous zr, zi
                zi = 2.0 * zr * zi + ci;
                zr = zr2 - zi2 + cr;

                // Update squares
                zr2 = zr * zr;
                zi2 = zi * zi;

                // Escape check: |z|^2 = zr^2 + zi^2
                if zr2 + zi2 > 4.0 {
                    draw_rectangle(i as f32, j as f32, 1.0, 1.0, BLACK);
                    escaped = true;
                    break;
                }
            }

            if !escaped {
                draw_rectangle(i as f32, j as f32, 1.0, 1.0, WHITE);
            }

            // Advance to next pixel in the row
            cr += cr_step;
        }
    }
}