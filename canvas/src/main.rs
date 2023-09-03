use canvas::*;
use chaikin::chaikin;
use macroquad::miniquad::window::set_window_size;
use macroquad::prelude::*;

#[macroquad::main("Chaikin in Rust", window_conf)]
async fn main() {
    set_window_size(800, 800);

    let mut control_points: Vec<(f32, f32)> = Vec::new();
    let mut init_points: Vec<(f32, f32)> = Vec::new();
    let mut points: Vec<(f32, f32)> = vec![];
    let mut lines: Vec<(f32, f32)> = Vec::new();

    let mut animation_step = 0;
    let mut animation_timer = 0.0;
    let animation_speed = 1.0;
    let mut start = false;

    loop {
        clear_background(DARKGRAY);

        draw_instructions();

        if is_mouse_button_pressed(MouseButton::Left) && !start {
            let mouse_position = mouse_position();
            control_points.push(mouse_position);
        }

        if is_key_pressed(KeyCode::Enter) {
            start = true;
            init_points = control_points.clone();
        }

        animation_timer += get_frame_time();

        if start && init_points.len() > 1 && animation_step < 7 && animation_timer > animation_speed
        {
            animation_timer = 0.0;

            points = chaikin(&init_points);

            init_points = points.clone();
            animation_step += 1;

            lines.clear();
            for i in 0..points.len() - 1 {
                lines.push(points[i]);
                lines.push(points[i + 1]);
            }

            draw_points(&control_points)
        }
        draw_lines(&lines);

        if is_key_pressed(KeyCode::R) {
            animation_step = 0;
            animation_timer = 0.0;
            control_points.clear();
            init_points.clear();
            points.clear();
            lines.clear();
            start = false;
        }

        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        //if !start {
        draw_points(&control_points);
        //}

        draw_counter(animation_step);

        next_frame().await;
    }
}
