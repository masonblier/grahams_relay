use bevy::{prelude::*, input::mouse::{MouseMotion,MouseWheel}};
use crate::game_state::GameState;

pub struct MouseSettings {
    pub sensitivity: f32,
    pub zoom_sensitivity: f32,
}

impl Default for MouseSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.002,
            zoom_sensitivity: 0.02,
        }
    }
}

const PITCH_BOUND: f32 = std::f32::consts::FRAC_PI_2 - 1E-3;

#[derive(Default)]
pub struct CursorLockState {
    pub enabled: bool,
}


#[derive(Clone, Copy, Default)]
pub struct MouseLookState {
    pub yaw_pitch_roll: Vec3,
    pub forward: Vec3,
    pub right: Vec3,
    pub up: Vec3,
    pub zoom: f32,
}

#[derive(Clone, Copy, Component, Default)]
pub struct MouseCamera {
}

pub struct MouseInputPlugin;

/// demo
impl Plugin for MouseInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CursorLockState>();
        app.init_resource::<MouseLookState>();
        app.init_resource::<MouseSettings>();
        app.add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_cursor_lock)
            .with_system(input_to_look)
        );
    }
}

pub fn update_cursor_lock(
    key_input: Res<Input<KeyCode>>,
    mouse_btn_input: Res<Input<MouseButton>>,
    mut cursor_lock_controls: ResMut<CursorLockState>,
    mut windows: ResMut<Windows>,
) {
    let window = windows.get_primary_mut().unwrap();
    if key_input.just_pressed(KeyCode::Escape) {
        window.set_cursor_lock_mode(false);
        window.set_cursor_visibility(true);
        cursor_lock_controls.enabled = false;
    } else if mouse_btn_input.pressed(MouseButton::Left) {
        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
        cursor_lock_controls.enabled = true;
    }
}

pub fn input_to_look(
    mut mouse_motion_events: EventReader<MouseMotion>,
    mut mouse_wheel_events: EventReader<MouseWheel>,
    mut mouse_look: ResMut<MouseLookState>,
    settings: Res<MouseSettings>,
    cursor_lock: Res<CursorLockState>,
) {
    let mut delta = Vec2::ZERO;
    for motion in mouse_motion_events.iter() {
        delta -= motion.delta;
    }
    let mut wheel_delta_y = 0.0;
    for wheel_motion in mouse_wheel_events.iter() {
        wheel_delta_y -= wheel_motion.y;
    }
    if !cursor_lock.enabled {
        return;
    }
    if delta.length_squared() > 1E-6 {
        delta *= settings.sensitivity;
        mouse_look.yaw_pitch_roll += delta.extend(0.0);
        if mouse_look.yaw_pitch_roll.y > PITCH_BOUND {
            mouse_look.yaw_pitch_roll.y = PITCH_BOUND;
        }
        if mouse_look.yaw_pitch_roll.y < -PITCH_BOUND {
            mouse_look.yaw_pitch_roll.y = -PITCH_BOUND;
        }

        let x_rotation = Quat::from_euler(
            EulerRot::XYZ,
            0.0,
            mouse_look.yaw_pitch_roll.x,
            0.0,
        );
        let y_rotation = Quat::from_euler(
            EulerRot::XYZ,
            mouse_look.yaw_pitch_roll.y,
            0.0, 0.0
        );
        let rotation = x_rotation * y_rotation;
        mouse_look.forward = rotation * -Vec3::Z;
        mouse_look.right = rotation * Vec3::X;
        mouse_look.up = rotation * Vec3::Y;
    }
    mouse_look.zoom += wheel_delta_y * settings.zoom_sensitivity;
}
