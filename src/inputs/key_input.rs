use bevy::{prelude::*};
use crate::game_state::GameState;
use crate::inputs::{CursorLockState};

// maintains mappings from input actions to game-logic actions
#[derive(Debug)]
pub struct KeyInputMap {
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_jump: KeyCode,
    pub key_run: KeyCode,
    pub key_crouch: KeyCode,
    pub key_fly: KeyCode,
    pub key_toggleview: KeyCode,
    pub key_escape: KeyCode,
}

impl Default for KeyInputMap {
    fn default() -> Self {
        Self {
            key_forward: KeyCode::W,
            key_backward: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_jump: KeyCode::Space,
            key_run: KeyCode::LShift,
            key_crouch: KeyCode::LControl,
            key_fly: KeyCode::F,
            key_toggleview: KeyCode::T,
            key_escape: KeyCode::Escape,
        }
    }
}

// maintains per-frame state of mapped activations from input to game-logic actions
#[derive(Debug, Default)]
pub struct KeyInputState {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub run: bool,
    pub up: bool,
    pub down: bool,
    pub jump: bool,
    pub toggle_fly: bool,
    pub toggle_view: bool,
}

// Plugin for keyboard input systems

pub struct KeyInputPlugin;

impl Plugin for KeyInputPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<KeyInputMap>();
        app.init_resource::<KeyInputState>();
        app.add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(input_to_move)
        );
    }
}

// updates desired move velocity from keyboard input
pub fn input_to_move(
    keyboard_input: Res<Input<KeyCode>>,
    input_map: Res<KeyInputMap>,
    mut state: ResMut<KeyInputState>,
    cursor_lock: Res<CursorLockState>,
    mut game_state: ResMut<State<GameState>>,
) {
    // check esc
    if cursor_lock.enabled && keyboard_input.just_pressed(input_map.key_escape) {
        game_state.set(GameState::Paused).unwrap();
    }

    // update input state from key states
    state.run = cursor_lock.enabled && keyboard_input.pressed(input_map.key_run);
    state.toggle_fly = cursor_lock.enabled && keyboard_input.just_pressed(input_map.key_fly);
    state.toggle_view = cursor_lock.enabled && keyboard_input.just_pressed(input_map.key_toggleview);
    state.jump = cursor_lock.enabled && keyboard_input.just_pressed(input_map.key_jump);

    // update desired velocity from key states
    state.forward = cursor_lock.enabled && keyboard_input.pressed(input_map.key_forward);
    state.backward = cursor_lock.enabled && keyboard_input.pressed(input_map.key_backward);
    state.right = cursor_lock.enabled && keyboard_input.pressed(input_map.key_right);
    state.left = cursor_lock.enabled && keyboard_input.pressed(input_map.key_left);
    state.up = cursor_lock.enabled && keyboard_input.pressed(input_map.key_jump);
    state.down = cursor_lock.enabled && keyboard_input.pressed(input_map.key_crouch);
}
