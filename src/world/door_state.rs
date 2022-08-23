use std::f32::consts::PI;

use crate::inputs::{CursorLockState};
use crate::game_state::GameState;
use crate::world::WorldState;
use bevy::prelude::*;

pub struct DoorStatePlugin;

pub enum DoorEventAction {
    Toggle,
}

pub struct DoorEvent {
    pub action: DoorEventAction,
    pub door: String,
}

impl Plugin for DoorStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_event::<DoorEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_door_interaction)
        );
    }
}


fn update_door_interaction(
    cursor_lock_state: Res<CursorLockState>,
    mut world_state: ResMut<WorldState>,
    mut door_events: EventReader<DoorEvent>,
    mut door_transforms: Query<(Entity, &mut Transform)>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    for door_event in door_events.iter() {
        if let Some(door_state) = world_state.doors.get_mut(&door_event.door) {
            let mut door_transform = door_transforms.get_mut(door_state.parent_entity.unwrap()).unwrap().1;
            if matches!(door_event.action, DoorEventAction::Toggle) {
                // todo fix door pivot
                if door_state.open {
                    let door_center = door_transform.translation.clone() - 0.5 * Vec3::X;
                    door_transform.rotate_around(door_center, Quat::from_rotation_y(-3.*PI/2.));
                    door_state.open = false;
                } else {
                    let door_center = door_transform.translation.clone() + 0.5 * Vec3::Z;
                    door_transform.rotate_around(door_center, Quat::from_rotation_y(3.*PI/2.));
                    door_state.open = true;
                }
            }
        }
    }
}
