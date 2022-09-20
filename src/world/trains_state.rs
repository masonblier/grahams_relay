use crate::inputs::{CursorLockState};
use crate::game_state::GameState;
use crate::movement::{Mover};
use crate::world::{SoundsEvent,SoundsEventAction,WorldState};
use bevy::prelude::*;

pub struct TrainsStatePlugin;

pub enum TrainsEventAction {
    StartControl,
}

pub struct TrainsEvent {
    pub action: TrainsEventAction,
    pub train: String,
}

impl Plugin for TrainsStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<TrainsEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_trains_interaction)
            .with_system(update_trains_movement)
        );
    }
}


fn update_trains_interaction(
    cursor_lock_state: Res<CursorLockState>,
    mut world_state: ResMut<WorldState>,
    mut train_events: EventReader<TrainsEvent>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    for train_event in train_events.iter() {
        if world_state.animatable_trains.contains_key(&train_event.train) {
            if matches!(train_event.action, TrainsEventAction::StartControl) {
                world_state.active_train = Some(train_event.train.clone());
            }
        }
    }
}

fn update_trains_movement(
    cursor_lock_state: Res<CursorLockState>,
    mut world_state: ResMut<WorldState>,
    mut transforms: Query<(Entity, &mut Transform)>,
    mut sounds_events: EventWriter<SoundsEvent>,
    keyboard_input: Res<Input<KeyCode>>,
    mover_query: Query<(Entity, &Mover)>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    if world_state.active_train.is_some() {
        let train_name = world_state.active_train.as_ref().unwrap().clone();
        if let Some(train_state) = world_state.animatable_trains.get_mut(&train_name) {
            let mut train_transform = transforms.get_mut(train_state.parent_entity.unwrap()).unwrap().1;
            let mut amove = 0.0;
            if keyboard_input.pressed(KeyCode::W) {
                amove = -0.02;
            }
            if keyboard_input.pressed(KeyCode::S) {
                amove = 0.02;
            }
            train_transform.translation += amove * Vec3::Z;

            if amove.abs() >= f32::EPSILON {
                if !train_state.running {
                    train_state.running = true;
                    sounds_events.send(SoundsEvent {
                        action: SoundsEventAction::Resume,
                        name: "train".into(),
                    });
                }

                let (mover_ent, _mover) = mover_query.single();
                let mut mover_transform = transforms.get_mut(mover_ent).unwrap().1;
                mover_transform.translation += amove * Vec3::Z;
            }
            if amove.abs() < f32::EPSILON && train_state.running {
                train_state.running = false;
                sounds_events.send(SoundsEvent {
                    action: SoundsEventAction::Pause,
                    name: "train".into(),
                });
            }
        }
    }
}
