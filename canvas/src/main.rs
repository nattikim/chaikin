use canvas::*;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

#[macroquad::main("Chaikin in Rust", window_conf)]
async fn main() {
    let mut state = AppState::default();

    set_window_size(800, 800);

    loop {
        clear_background(DARKGRAY);

        draw_instructions();

        handle_input(&mut state);
        animation_step(&mut state);

        if is_key_pressed(KeyCode::R) {
            reset_state(&mut state);
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        draw_points(&state.control_points);
        draw_lines(&state.lines);

        draw_message(state.message_start_time);

        draw_counter(state.animation_step);

        next_frame().await;
    }
}
