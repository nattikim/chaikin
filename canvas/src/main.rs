use chaikin::chaikin;
use macroquad::prelude::*;

#[macroquad::main("Chaikin in Rust", window_conf)]
async fn main() {
    let mut control_points: Vec<(f32, f32)> = Vec::new();

    let mut points: Vec<(f32, f32)> = vec![];

    loop {
        clear_background(DARKGRAY);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            control_points.push(mouse_position);
        }

        if is_key_pressed(KeyCode::Enter) && control_points.len() > 1 {
            points = chaikin(&control_points);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        for point in &points {
            draw_circle(point.0, point.1, 3.0, BLUE);
        }

        for point in &control_points {
            draw_circle(point.0, point.1, 4.0, GREEN);
        }

        next_frame().await;
    }
}
