use crate::game_state::GameState;
use crate::inputs::{KeyInputState,MouseCamera,MouseLookState};
use crate::movement::{CharacterState,CharacterAnimations};
use crate::settings::SettingsAsset;
use crate::world::{SoundsEvent,SoundsEventAction,WorldState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const MOVE_SPEED: f32 = 20.0;
const RUN_SPEED: f32 = MOVE_SPEED * 2.0;
const CAMERA_FLY_MOVE_SPEED: f32 = 10.0;

// const RESET_ANIMATION_IDX: usize = 1;
const IDLE_ANIMATION_IDX: usize = 0;
const TOGGLE_SWITCH_ANIMATION_IDX: usize = 4;
const FORWARD_ANIMATION_IDX: usize = 5;
const FORWARD_RUN_ANIMATION_IDX: usize = 2;
// const TAKE_ITEM_ANIMATION_IDX: usize = 3;

// system state
#[derive(Default)]
pub struct MovementState {
    pub noclip: bool,
    pub playing_animation: Option<usize>,
    pub toggle_switch_rmn: f32,
}

// marks the rigid body of the player character
#[derive(Clone,Component,Copy)]
pub struct Mover {
    pub third_person: bool,
    pub last_wish_move: Vec3,
}

impl Default for Mover {
    fn default() -> Self {
        Self {
            third_person: true,
            last_wish_move: Vec3::ZERO,
        }
    }
}

// marks the parent element of character gltf scene
#[derive(Clone,Component,Copy,Default)]
pub struct MoverParent {
}

pub struct MovementStatePlugin;

impl Plugin for MovementStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MovementState>();
        app.add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_movement)
            .with_system(update_camera)
            .with_system(update_character_state)
        );
    }
}

fn update_movement(
    time: Res<Time>,
    key_state: Res<KeyInputState>,
    mouse_look: Res<MouseLookState>,
    mut movement_state: ResMut<MovementState>,
    mut mover_query: Query<(&mut ExternalImpulse, &mut Mover)>,
    world_state: Res<WorldState>,
) {
    let (mut mover_impulse, mut mover) = mover_query.single_mut();

    // update state
    if key_state.toggle_fly {
        movement_state.noclip = !movement_state.noclip;
    }
    if key_state.toggle_view {
        mover.third_person = !mover.third_person;
    }

    if movement_state.noclip {
        return;
    }
    if world_state.active_train.is_some() {
        return;
    }

    let mouse_forward = (mouse_look.forward * Vec3::new(1.0, 0.0, 1.0)).normalize();
    let mouse_right = (mouse_look.right * Vec3::new(1.0, 0.0, 1.0)).normalize();
    let wish_move = (if key_state.run { RUN_SPEED } else { MOVE_SPEED }) * time.delta_seconds() * (
        if key_state.forward { mouse_forward } else { Vec3::ZERO } +
        if key_state.backward { -mouse_forward } else { Vec3::ZERO } +
        if key_state.right { mouse_right } else { Vec3::ZERO } +
        if key_state.left { -mouse_right } else { Vec3::ZERO }
    );

    // store last move
    mover.last_wish_move = wish_move;
    // apply move
    mover_impulse.impulse = wish_move;
}

// update camera position from movement
fn update_camera(
    time: Res<Time>,
    key_state: Res<KeyInputState>,
    movement_state: Res<MovementState>,
    mouse_look: Res<MouseLookState>,
    mover_query: Query<(&Transform, &Mover), Without<MouseCamera>>,
    mut query: Query<&mut Transform, With<MouseCamera>>,
) {
    let (mover_transform, mover) = mover_query.single();
    for mut camera in query.iter_mut() {
        if movement_state.noclip {
            let camera_move = CAMERA_FLY_MOVE_SPEED * time.delta_seconds() * (
                if key_state.forward { mouse_look.forward.clone() } else { Vec3::ZERO } +
                if key_state.backward { -mouse_look.forward.clone() } else { Vec3::ZERO } +
                if key_state.right { mouse_look.right.clone() } else { Vec3::ZERO } +
                if key_state.left { -mouse_look.right.clone() } else { Vec3::ZERO } +
                if key_state.up { mouse_look.up.clone() } else { Vec3::ZERO } +
                if key_state.down { -mouse_look.up.clone() } else { Vec3::ZERO }
            ) * (
                if key_state.run { 3.0 } else { 1.0 }
            );

            let next_position = camera.translation + camera_move;
            camera.translation = next_position.clone();
            camera.look_at(next_position + mouse_look.forward, Vec3::Y);
        } else {
            if mover.third_person {
                let mover_position = mover_transform.translation.clone() + 0.8 * Vec3::Y;
                camera.translation = mover_position - (mouse_look.forward * (1.0 + mouse_look.zoom));
                camera.look_at(mover_position, Vec3::Y);
            } else {
                let mouse_forward = (mouse_look.forward * Vec3::new(1.0, 0.0, 1.0)).normalize();
                let mover_position = mover_transform.translation.clone() + 0.8 * Vec3::Y + 0.15 * mouse_forward;
                camera.translation = mover_position;
                camera.look_at(mover_position + mouse_look.forward, Vec3::Y);
            }
        }
    }
}

fn update_character_state(
    animations: Res<CharacterAnimations>,
    mut mover_parent_query: Query<&mut Transform, With<MoverParent>>,
    mover_query: Query<&Mover>,
    time: Res<Time>,
    character_state: Res<CharacterState>,
    mut movement_state: ResMut<MovementState>,
    mut animation_players: Query<(&Parent, &mut AnimationPlayer)>,
    mut sounds_events: EventWriter<SoundsEvent>,
    settings: Res<SettingsAsset>,
    key_state: Res<KeyInputState>,
) {
    if movement_state.noclip {
        return;
    }

    // rotate character with camera
    let mover = mover_query.single();

    let mut mover_parent_transform = mover_parent_query.single_mut();
    let mover_parent_translation = mover_parent_transform.translation.clone();
    let move_forward = mover.last_wish_move.normalize();
    if move_forward.length_squared() > 0.0001 {
        let old_rotation = mover_parent_transform.rotation.clone();
        mover_parent_transform.look_at(mover_parent_translation - move_forward, Vec3::Y);
        if old_rotation.angle_between(mover_parent_transform.rotation) > 0.001 {
            mover_parent_transform.rotation = old_rotation.slerp(mover_parent_transform.rotation, 0.1);
        }
    }

    // skip gltf in colliders mode
    if settings.graphics_settings.render_mode.as_str() == "colliders" {
        if movement_state.toggle_switch_rmn > 0.0001 {
            movement_state.toggle_switch_rmn = 0.0;
        }
        return;
    }

    // update animation state
    for (parent, mut player) in animation_players.iter_mut() {
        if character_state.character_anim_entity.is_some() && character_state.character_anim_entity.unwrap() == parent.get() {
            if movement_state.toggle_switch_rmn > 0.0001 {
                movement_state.toggle_switch_rmn -= time.delta_seconds();
                if !(movement_state.playing_animation.is_some() && movement_state.playing_animation.unwrap() == TOGGLE_SWITCH_ANIMATION_IDX) {
                    movement_state.playing_animation = Some(TOGGLE_SWITCH_ANIMATION_IDX);
                    player.play(animations.0[TOGGLE_SWITCH_ANIMATION_IDX].clone_weak()).repeat();
                }
            } else if move_forward.length_squared() > 0.0001 {
                if key_state.run {
                    if !(movement_state.playing_animation.is_some() && movement_state.playing_animation.unwrap() == FORWARD_RUN_ANIMATION_IDX) {
                        movement_state.playing_animation = Some(FORWARD_RUN_ANIMATION_IDX);
                        player.play(animations.0[FORWARD_RUN_ANIMATION_IDX].clone_weak()).repeat();
                        // play footsteps
                        sounds_events.send(SoundsEvent {
                            action: SoundsEventAction::Resume,
                            name: "footsteps".into(),
                        });
                    }
                } else {
                    if !(movement_state.playing_animation.is_some() && movement_state.playing_animation.unwrap() == FORWARD_ANIMATION_IDX) {
                        movement_state.playing_animation = Some(FORWARD_ANIMATION_IDX);
                        player.play(animations.0[FORWARD_ANIMATION_IDX].clone_weak()).repeat();
                        // play footsteps
                        sounds_events.send(SoundsEvent {
                            action: SoundsEventAction::Resume,
                            name: "footsteps".into(),
                        });
                    }
                }
            } else {
                if !(movement_state.playing_animation.is_some() && movement_state.playing_animation.unwrap() == IDLE_ANIMATION_IDX) {
                    movement_state.playing_animation = Some(IDLE_ANIMATION_IDX);
                    player.play(animations.0[IDLE_ANIMATION_IDX].clone_weak()).repeat();
                    // pause footsteps
                    sounds_events.send(SoundsEvent {
                        action: SoundsEventAction::Pause,
                        name: "footsteps".into(),
                    });
                }
            }
        }
    }
}
