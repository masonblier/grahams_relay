use crate::audio::{AudioEvent,AudioEventAction};
use crate::game_state::GameState;
use crate::inputs::{CursorLockState,KeyInputState,MouseCamera,MouseLookState};
use crate::loading::AudioAssets;
use crate::movement::{CharacterState,CharacterAnimations};
use crate::settings::SettingsAsset;
use crate::world::{AnimatableEvent,AnimatableEventAction,DoorEvent,DoorEventAction,WorldState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const MOVE_SPEED: f32 = 10.0;
const CAMERA_FLY_MOVE_SPEED: f32 = 10.0;

const FORWARD_ANIMATION_IDX: usize = 0;
const IDLE_ANIMATION_IDX: usize = 4;
const TOGGLE_SWITCH_ANIMATION_IDX: usize = 1;
const TOGGLE_SWITCH_ANIMATION_DURATION: f32 = 1.2;

// system state
#[derive(Default)]
pub struct MovementState {
    pub noclip: bool,
    pub playing_animation: Option<usize>,
    pub toggle_switch_rmn: f32,
}

// marks the rigid body of the player character
#[derive(Clone,Component,Copy,Default)]
pub struct Mover {
    pub third_person: bool,
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
            .with_system(update_mouse_interaction)
        );
    }
}

fn update_movement(
    time: Res<Time>,
    key_state: Res<KeyInputState>,
    mouse_look: Res<MouseLookState>,
    mut movement_state: ResMut<MovementState>,
    mut mover_query: Query<(&mut ExternalImpulse, &mut Mover)>,
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

    let mouse_forward = (mouse_look.forward * Vec3::new(1.0, 0.0, 1.0)).normalize();
    let mouse_right = (mouse_look.right * Vec3::new(1.0, 0.0, 1.0)).normalize();
    let wish_move = MOVE_SPEED * time.delta_seconds() * (
        if key_state.forward { mouse_forward } else { Vec3::ZERO } +
        if key_state.backward { -mouse_forward } else { Vec3::ZERO } +
        if key_state.right { mouse_right } else { Vec3::ZERO } +
        if key_state.left { -mouse_right } else { Vec3::ZERO }
    );

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
    cursor_lock_state: Res<CursorLockState>,
    time: Res<Time>,
    key_state: Res<KeyInputState>,
    character_state: Res<CharacterState>,
    mut movement_state: ResMut<MovementState>,
    mouse_look: Res<MouseLookState>,
    mut animation_players: Query<(&Parent, &mut AnimationPlayer)>,
    settings: Res<SettingsAsset>,
) {
    if !cursor_lock_state.enabled || movement_state.noclip {
        return;
    }

    // rotate character with camera
    let mut mover_parent_transform = mover_parent_query.single_mut();
    let mover_parent_translation = mover_parent_transform.translation.clone();
    let mouse_forward = (mouse_look.forward * Vec3::new(1.0, 0.0, 1.0)).normalize();
    mover_parent_transform.look_at(mover_parent_translation - mouse_forward, Vec3::Y);

    // skip gltf in colliders mode
    if settings.graphics_settings.render_mode.as_str() == "colliders" {
        if movement_state.toggle_switch_rmn > 0.0001 {
            movement_state.toggle_switch_rmn = 0.0;
        }
        return;
    }

    // update animation state
    for (parent, mut player) in animation_players.iter_mut() {
        if character_state.character_anim_entity.is_some() && character_state.character_anim_entity.unwrap() == parent.0 {
            if movement_state.toggle_switch_rmn > 0.0001 {
                movement_state.toggle_switch_rmn -= time.delta_seconds();
                if !(movement_state.playing_animation.is_some() && movement_state.playing_animation.unwrap() == TOGGLE_SWITCH_ANIMATION_IDX) {
                    movement_state.playing_animation = Some(TOGGLE_SWITCH_ANIMATION_IDX);
                    player.play(animations.0[TOGGLE_SWITCH_ANIMATION_IDX].clone_weak()).repeat();
                }
            } else if key_state.forward {
                if !(movement_state.playing_animation.is_some() && movement_state.playing_animation.unwrap() == FORWARD_ANIMATION_IDX) {
                    movement_state.playing_animation = Some(FORWARD_ANIMATION_IDX);
                    player.play(animations.0[FORWARD_ANIMATION_IDX].clone_weak()).repeat();
                    // todo start/stop audio in sync with animation
                    // audio_events.send(AudioEvent {
                    //     action: AudioEventAction::PlayOnce,
                    //     source: Some(audio_assets.steps_snow_dry.clone()),
                    // });
                }
            } else {
                if !(movement_state.playing_animation.is_some() && movement_state.playing_animation.unwrap() == IDLE_ANIMATION_IDX) {
                    movement_state.playing_animation = Some(IDLE_ANIMATION_IDX);
                    player.play(animations.0[IDLE_ANIMATION_IDX].clone_weak()).repeat();
                }
            }
        }
    }
}

fn update_mouse_interaction(
    cursor_lock_state: Res<CursorLockState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut movement_state: ResMut<MovementState>,
    mouse_look: Res<MouseLookState>,
    rapier_context: Res<RapierContext>,
    camera_query: Query<&GlobalTransform, With<MouseCamera>>,
    mut animatable_events: EventWriter<AnimatableEvent>,
    mut door_events: EventWriter<DoorEvent>,
    mut audio_events: EventWriter<AudioEvent>,
    audio_assets: Res<AudioAssets>,
    world_state: Res<WorldState>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    // check for mouse button press
    if mouse_button_input.just_pressed(MouseButton::Left) && movement_state.toggle_switch_rmn <= 0.0001 {
        // get interactable ray from player state
        let camera_transform = camera_query.single();
        let ray_pos = camera_transform.translation.clone();
        let ray_len = 1.7;
        let ray_dir = mouse_look.forward * ray_len;
        let ray_groups = InteractionGroups::new(0b0100, 0b0100);
        let ray_filter = QueryFilter { groups: Some(ray_groups), ..Default::default()};

        // cast for interactables
        if let Some((entity, _toi)) = rapier_context.cast_ray(
            ray_pos, ray_dir, 1.0, true, ray_filter
        ) {
            if let Some(interactable) = world_state.interactable_states.get(&entity) {

                for action in interactable.interaction.actions.iter() {
                    match action.0.as_str() {
                        "audio_playonce" => {
                            audio_events.send(AudioEvent {
                                action: AudioEventAction::PlayOnce,
                                source: Some(audio_assets.big_switch.clone()),
                            });
                        },
                        "animate" => {
                            let parts = action.1.split(".").collect::<Vec<&str>>();
                            let animatable_name = parts[0].to_string();
                            let animation_name = parts[1].to_string();
                            animatable_events.send(AnimatableEvent {
                                action: AnimatableEventAction::PlayOnce,
                                name: animatable_name,
                                animation: animation_name,
                            });
                        },
                        "toggle_door" => {
                            door_events.send(DoorEvent {
                                action: DoorEventAction::Toggle,
                                door: action.1.to_string(),
                            });
                        },
                        _ => {
                            println!("Unknown interaction! {:?} -> {:?}", entity, action);
                        },
                    }
                }
            }

            // start button press animation
            movement_state.toggle_switch_rmn = TOGGLE_SWITCH_ANIMATION_DURATION;
        } else {
            // println!("No Collide {:?} {:?}", ray_pos, ray_dir);
        }
    }
}
