use crate::inputs::{CursorLockState};
use crate::game_state::GameState;
use bevy::prelude::*;
use std::collections::HashMap;


// events
pub enum WorldFlagsEventAction {
    // Disable,
    Enable,
}

pub struct WorldFlagsEvent {
    pub action: WorldFlagsEventAction,
    pub flag: String,
}

// system state
#[derive(Default)]
pub struct WorldFlagsState {
    pub flags: HashMap<String, bool>,
}

pub struct WorldFlagsStatePlugin;

impl Plugin for WorldFlagsStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(WorldFlagsState::default())
        .add_event::<WorldFlagsEvent>()
        .add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_world_flags_interaction)
        );
    }
}

fn update_world_flags_interaction(
    cursor_lock_state: Res<CursorLockState>,
    mut world_flags_state: ResMut<WorldFlagsState>,
    mut world_flags_events: EventReader<WorldFlagsEvent>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    for world_flags_event in world_flags_events.iter() {
        match world_flags_event.action {
            // WorldFlagsEventAction::Disable => {
            //     world_flags_state.flags.insert(world_flags_event.flag.clone(), false);
            // },
            WorldFlagsEventAction::Enable => {
                world_flags_state.flags.insert(world_flags_event.flag.clone(), true);
            },
        }
    }
}
