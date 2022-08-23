use crate::audio::{AudioEvent,AudioEventAction};
use crate::inputs::{CursorLockState,MouseCamera,MouseLookState};
use crate::game_state::GameState;
use crate::loading::AudioAssets;
use crate::movement::{MovementState};
use crate::world::{AnimatableEvent,AnimatableEventAction,DoorEvent,
    DoorEventAction,InteractableState,WorldState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const INTERACTION_BLOCKED_DURATION: f32 = 1.2;

// system state
#[derive(Default)]
pub struct InteractablesState {
    pub active_interactable: Option<InteractableState>,
    pub blocked_rmn: f32,
}
pub struct InteractableStatePlugin;

impl Plugin for InteractableStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(InteractablesState::default())
        .add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_interactable_interaction)
            .with_system(update_mouse_click_interaction)
        );
    }
}


fn update_interactable_interaction(
    cursor_lock_state: Res<CursorLockState>,
    mut interactables_state: ResMut<InteractablesState>,
    world_state: Res<WorldState>,
    camera_query: Query<&GlobalTransform, With<MouseCamera>>,
    mouse_look: Res<MouseLookState>,
    rapier_context: Res<RapierContext>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    // get interactable ray from player state
    let camera_transform = camera_query.single();
    let ray_pos = camera_transform.translation();
    let ray_len = 1.7;
    let ray_dir = mouse_look.forward * ray_len;
    let ray_groups = InteractionGroups::new(0b0100, 0b0100);
    let ray_filter = QueryFilter { groups: Some(ray_groups), ..Default::default()};

    // cast for interactables
    interactables_state.active_interactable = if let Some((entity, _toi)) = rapier_context.cast_ray(
        ray_pos, ray_dir, 1.0, true, ray_filter
    ) {
        if let Some(interactable) = world_state.interactable_states.get(&entity) {
            Some(interactable.clone())
        } else { None }
    } else { None };
}


fn update_mouse_click_interaction(
    cursor_lock_state: Res<CursorLockState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut movement_state: ResMut<MovementState>,
    mut animatable_events: EventWriter<AnimatableEvent>,
    mut door_events: EventWriter<DoorEvent>,
    mut audio_events: EventWriter<AudioEvent>,
    audio_assets: Res<AudioAssets>,
    mut interactables_state: ResMut<InteractablesState>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    if interactables_state.blocked_rmn > 0.0001 {
        interactables_state.blocked_rmn = 0.0;
    }

    // check for mouse button press
    if mouse_button_input.just_pressed(MouseButton::Left) && interactables_state.blocked_rmn <= 0.0001 {
        if let Some(interactable) = &interactables_state.active_interactable {

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
                        println!("Unknown interaction! {:?}", action);
                    },
                }
            }

            // block interaction for time
            interactables_state.blocked_rmn = INTERACTION_BLOCKED_DURATION;
            // todo proper event or something
            movement_state.toggle_switch_rmn = INTERACTION_BLOCKED_DURATION;
        }
    }
}
