use chaikin::chaikin;
use macroquad::prelude::*;
use std::time::{Duration, Instant};

pub struct AppState {
    pub control_points: Vec<(f32, f32)>,
    init_points: Vec<(f32, f32)>,
    points: Vec<(f32, f32)>,
    pub lines: Vec<(f32, f32)>,
    pub animation_step: i32,
    animation_timer: f32,
    animation_speed: f32,
    start: bool,
    message_start_time: Option<Instant>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            control_points: Vec::new(),
            init_points: Vec::new(),
            points: Vec::new(),
            lines: Vec::new(),
            animation_step: 0,
            animation_timer: 0.0,
            animation_speed: 1.0,
            start: false,
            message_start_time: None,
        }
    }
}
pub fn handle_input(state: &mut AppState) {
    if is_mouse_button_pressed(MouseButton::Left) && !state.start {
        let mouse_position = mouse_position();
        state.control_points.push(mouse_position);
    }

    if is_key_pressed(KeyCode::Enter) {
        if state.control_points.is_empty() {
            state.message_start_time = Some(Instant::now());
        }
        if state.control_points.len() > 1 {
            state.start = true;
            state.init_points = state.control_points.clone();
        }
    }

    if let Some(start_time) = state.message_start_time {
        let elapsed = start_time.elapsed();
        if elapsed < Duration::from_secs(3) {
            draw_message();
        } else {
            // After 3 seconds, clear the message and reset the start time
            state.message_start_time = None;
        }
    }
}

pub fn animation_step(state: &mut AppState) {
    state.animation_timer += get_frame_time();

    if state.start && state.init_points.len() == 2 {
        state.lines.push(state.init_points[0]);
        state.lines.push(state.init_points[1]);
    }

    if state.start
        && state.init_points.len() > 2
        && state.animation_step <= 7
        && state.animation_timer > state.animation_speed
    {
        if state.animation_step == 7 {
            state.animation_step = 0;
            state.init_points = state.control_points.clone();
        }

        state.animation_timer = 0.0;

        state.lines.clear();
        for i in 0..state.init_points.len() - 1 {
            state.lines.push(state.init_points[i]);
            state.lines.push(state.init_points[i + 1]);
        }

        state.points = chaikin(&state.init_points);
        state.init_points = state.points.clone();
        state.animation_step += 1;
    }
}

pub fn reset_state(state: &mut AppState) {
    state.animation_step = 0;
    state.animation_timer = 0.0;
    state.control_points.clear();
    state.init_points.clear();
    state.points.clear();
    state.lines.clear();
    state.start = false;
}

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
        177.0,
        30.0,
        20.0,
        GRAY,
    );
}

pub fn draw_message() {
    draw_text("You forgot to draw any points.", 250.0, 750.0, 25.0, GREEN);
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
