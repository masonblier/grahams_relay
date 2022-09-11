use crate::audio::{AudioEvent,AudioEventAction};
use crate::inputs::{CursorLockState,MouseCamera,MouseLookState};
use crate::game_state::GameState;
use crate::loading::{AudioAssets,FontAssets};
use crate::movement::{MovementState,Mover};
use crate::world::{AnimatableEvent,AnimatableEventAction,DoorEvent,
    DoorEventAction,InteractableState,InventoryEvent,InventoryEventAction,
    InventoryItem,InventoryState,LightsEvent,LightsEventAction,
    SoundsEvent,SoundsEventAction,
    WorldFlagsEvent,WorldFlagsEventAction,WorldFlagsState,WorldState};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

const INITIAL_BLOCKED_DURATION: f32 = 0.4;
const INTERACTION_BLOCKED_DURATION: f32 = 1.2;

// system state
#[derive(Default)]
pub struct InteractablesState {
    pub active_interactable: Option<InteractableState>,
    pub active_interactable_entity: Option<Entity>,
    pub ui_entity: Option<Entity>,
    pub blocked_rmn: f32,
}

// Tag for UI component
#[derive(Component)]
struct InteractablesOverlayText;

pub struct InteractableStatePlugin;

impl Plugin for InteractableStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(InteractablesState::default())
        .add_system_set(SystemSet::on_enter(GameState::Running)
            .with_system(setup_interactable_interaction))
        .add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_interactable_interaction)
            .with_system(update_mouse_click_interaction)
        )
        .add_system_set(SystemSet::on_exit(GameState::Running)
            .with_system(exit_interactable_interaction))
        ;
    }
}

fn setup_interactable_interaction(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    mut interactables_state: ResMut<InteractablesState>,
) {
    interactables_state.ui_entity = Some(commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: UiRect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::NONE.into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: font_assets.fira_sans.clone(),
                            font_size: 24.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: TextAlignment::CENTER,
                },
                ..Default::default()
            })
            .insert(InteractablesOverlayText)
            ;
        })
        .id());

    // delay inputs when first entering Running state
    interactables_state.blocked_rmn = INITIAL_BLOCKED_DURATION;
}

fn update_interactable_interaction(
    cursor_lock_state: Res<CursorLockState>,
    inventory_state: Res<InventoryState>,
    world_flags_state: Res<WorldFlagsState>,
    mut interactables_state: ResMut<InteractablesState>,
    world_state: Res<WorldState>,
    camera_query: Query<&GlobalTransform, With<MouseCamera>>,
    mouse_look: Res<MouseLookState>,
    rapier_context: Res<RapierContext>,
    mut text_query: Query<&mut Text, With<InteractablesOverlayText>>,
    mover_query: Query<&Mover>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    // get interactable ray from player state
    let mover = mover_query.single();
    let camera_transform = camera_query.single();
    let ray_pos = camera_transform.translation();
    let ray_len = if mover.third_person { 2.5 } else { 1.7 } ;
    let ray_dir = mouse_look.forward * ray_len;
    let ray_groups = InteractionGroups::new(0b0100, 0b0100);
    let ray_filter = QueryFilter { groups: Some(ray_groups), ..Default::default()};

    // cast for interactables
    let (entity, interactable) = if interactables_state.blocked_rmn > 0.0001 {
        (None, None)
    } else if let Some((entity, _toi)) = rapier_context.cast_ray(
        ray_pos, ray_dir, 1.0, true, ray_filter
    ) {
        if let Some(interactable) = world_state.interactable_states.get(&entity) {
            (Some(entity), Some(interactable.clone()))
        } else { (None, None) }
    } else { (None, None) };

    // if active interactable changed
    if interactables_state.active_interactable_entity != entity {
        interactables_state.active_interactable_entity = entity;
        interactables_state.active_interactable = interactable;

        if let Some(interactable) = &interactables_state.active_interactable {
            // check blockers
            let blockers = check_blockers(interactable.interaction.blockers.clone(),
                inventory_state, world_flags_state);

            if let Some(first_blocker) = blockers.first() {
                // show blocker text
                let mut text = text_query.single_mut();
                text.sections[0].value = "\n\n\nx\n\n".to_string() + &first_blocker.1;
            } else {
                // show interaction text
                let mut text = text_query.single_mut();
                text.sections[0].value = "\n\n\n.\n\n".to_string() + &interactable.interaction.interaction_text;
            }
        } else {
            // hide interaction text
            let mut text = text_query.single_mut();
            text.sections[0].value = "".to_string();
        }
    }
}


fn update_mouse_click_interaction(
    mut commands: Commands,
    cursor_lock_state: Res<CursorLockState>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut movement_state: ResMut<MovementState>,
    mut animatable_events: EventWriter<AnimatableEvent>,
    mut door_events: EventWriter<DoorEvent>,
    mut inventory_events: EventWriter<InventoryEvent>,
    mut lights_events: EventWriter<LightsEvent>,
    mut sounds_events: EventWriter<SoundsEvent>,
    mut world_flags_events: EventWriter<WorldFlagsEvent>,
    mut audio_events: EventWriter<AudioEvent>,
    audio_assets: Res<AudioAssets>,
    mut interactables_state: ResMut<InteractablesState>,
    inventory_state: Res<InventoryState>,
    world_flags_state: Res<WorldFlagsState>,
    time: Res<Time>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    if interactables_state.blocked_rmn > 0.0001 {
        interactables_state.blocked_rmn -= time.delta_seconds();
    }

    // check for mouse button press
    if mouse_button_input.just_pressed(MouseButton::Left) && interactables_state.blocked_rmn <= 0.0001 {
        if let Some(interactable) = &interactables_state.active_interactable {

            // check blockers
            let blockers = check_blockers(interactable.interaction.blockers.clone(),
                inventory_state, world_flags_state);
            if blockers.len() > 0 {
                return;
            }

            // send action events
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
                    "toggle_light" => {
                        lights_events.send(LightsEvent {
                            action: LightsEventAction::Toggle,
                            name: action.1.to_string(),
                        });
                    },
                    "toggle_sound" => {
                        sounds_events.send(SoundsEvent {
                            action: SoundsEventAction::Toggle,
                            name: action.1.to_string(),
                        });
                    },
                    "consume_item" => {
                        let item = match action.1.as_str() {
                            "fuse_small" => InventoryItem::FuseSmall,
                            _ => {
                                println!("bad action {:?}", action);
                                InventoryItem::FuseSmall
                            },
                        };
                        inventory_events.send(InventoryEvent {
                            action: InventoryEventAction::RemoveItem,
                            item,
                        });
                    },
                    "pickup_item" => {
                        let item = match action.1.as_str() {
                            "fuse_small" => InventoryItem::FuseSmall,
                            _ => {
                                println!("bad action {:?}", action);
                                InventoryItem::FuseSmall
                            },
                        };
                        inventory_events.send(InventoryEvent {
                            action: InventoryEventAction::AddItem,
                            item,
                        });
                    },
                    "enable_flag" => {
                        world_flags_events.send(WorldFlagsEvent {
                            action: WorldFlagsEventAction::Enable,
                            flag: action.1.clone(),
                        });
                    },
                    "hide_prop" => {
                        animatable_events.send(AnimatableEvent {
                            action: AnimatableEventAction::Despawn,
                            name: action.1.clone(),
                            animation: "".to_string(),
                        });
                    },
                    "despawn_self" => {
                        commands.entity(
                            interactables_state.active_interactable_entity.unwrap()).despawn();
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

fn check_blockers(
    blockers: Vec<(String, String)>,
    inventory_state: Res<InventoryState>,
    world_flags_state: Res<WorldFlagsState>,
) -> Vec<(String,String)> {
    blockers.clone().into_iter().filter({|blocker|
        if blocker.0.starts_with("holding") {
            let item_name = blocker.0.split(".").collect::<Vec<&str>>()[1];
            match item_name {
                "fuse_small" => {
                    inventory_state.fuse_small_count <= 0
                },
                _ => {
                    println!("blocker: bad item {:?}", blocker);
                    false
                },
            }
        } else if blocker.0.starts_with("flag_enabled") {
            let flag_name = blocker.0.split(".").collect::<Vec<&str>>()[1];
            let flag_wrapped = world_flags_state.flags.get(&flag_name.to_string());
            !(flag_wrapped.is_some() && *flag_wrapped.unwrap())
        } else {
            println!("invalid blocker {:?}", blocker);
            false
        }
    }).collect::<Vec<(String,String)>>()
}

fn exit_interactable_interaction(
    mut commands: Commands,
    interactables_state: Res<InteractablesState>,
) {
    if let Some(ui_entity) = interactables_state.ui_entity {
        commands.entity(ui_entity).despawn_recursive();();
    }
}
