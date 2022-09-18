use std::f32::consts::PI;

use crate::inputs::{CursorLockState};
use crate::game_state::GameState;
use crate::world::WorldState;
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
        if let Some(train_state) = world_state.animatable_trains.get_mut(&train_event.train) {
            if matches!(train_event.action, TrainsEventAction::StartControl) {
                world_state.active_train = Some(train_event.train.clone());
            }
        }
    }
}

fn update_trains_movement(
    cursor_lock_state: Res<CursorLockState>,
    world_state: Res<WorldState>,
    mut train_events: EventReader<TrainsEvent>,
    mut train_transforms: Query<(Entity, &mut Transform)>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    if world_state.active_train.is_some() {
        if let Some(train_state) = world_state.animatable_trains.get(world_state.active_train.as_ref().unwrap()) {
            let mut train_transform = train_transforms.get_mut(train_state.parent_entity.unwrap()).unwrap().1;
            if keyboard_input.pressed(KeyCode::W) {
                train_transform.translation += 0.01 * Vec3::X;
            }
        }
    }
}
