use macroquad::prelude::*;

pub fn draw_lines(lines: &[(f32, f32)]) {
    for i in 0..lines.len() / 2 {
        draw_line(
            lines[i * 2].0,
            lines[i * 2].1,
            lines[i * 2 + 1].0,
            lines[i * 2 + 1].1,
            1.0,
            WHITE,
        );
    }
}

pub fn draw_points(control_points: &[(f32, f32)]) {
    for point in control_points {
        draw_circle_lines(point.0, point.1, 5.0, 1.0, GREEN);
    }
}

pub fn draw_instructions() {
    draw_text(
        "Place points and press Enter. Press R to restart.",
        15.0,
        20.0,
        20.0,
        GRAY,
    );
}

pub fn draw_counter(animation_step: i32) {
    draw_text(
        format!("Step: {}", animation_step).as_str(),
        720.0,
        750.0,
        20.0,
        WHITE,
    );
}
