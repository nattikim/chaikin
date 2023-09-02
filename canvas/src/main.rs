use macroquad::prelude::*;

#[macroquad::main("Chaikin in Rust", window_conf)]
async fn main() {
    let mut control_points: Vec<(f32, f32)> = Vec::new();

    loop {
        clear_background(DARKGRAY);

        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_position = mouse_position();
            control_points.push(mouse_position);
        }

        for point in &control_points {
            draw_circle(point.0, point.1, 3.0, GREEN);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
