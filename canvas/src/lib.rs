use chaikin::chaikin;
use macroquad::prelude::*;

pub struct AppState {
    pub control_points: Vec<(f32, f32)>,
    init_points: Vec<(f32, f32)>,
    points: Vec<(f32, f32)>,
    pub lines: Vec<(f32, f32)>,
    pub animation_step: i32,
    animation_timer: f32,
    animation_speed: f32,
    start: bool,
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
        }
    }
}
pub fn handle_input(state: &mut AppState) {
    if is_mouse_button_pressed(MouseButton::Left) && !state.start {
        let mouse_position = mouse_position();
        state.control_points.push(mouse_position);
    }

    if is_key_pressed(KeyCode::Enter) {
        state.start = true;
        state.init_points = state.control_points.clone();
    }
}

pub fn animation_step(state: &mut AppState) {
    state.animation_timer += get_frame_time();

    if state.start
        && state.init_points.len() > 1
        && state.animation_step < 7
        && state.animation_timer > state.animation_speed
    {
        state.animation_timer = 0.0;

        state.points = chaikin(&state.init_points);

        state.init_points = state.points.clone();
        state.animation_step += 1;

        state.lines.clear();
        for i in 0..state.points.len() - 1 {
            state.lines.push(state.points[i]);
            state.lines.push(state.points[i + 1]);
        }
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

pub fn draw_counter(animation_step: i32) {
    draw_text(
        format!("Step: {}", animation_step).as_str(),
        720.0,
        750.0,
        20.0,
        WHITE,
    );
}
