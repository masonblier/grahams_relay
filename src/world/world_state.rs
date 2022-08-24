use crate::world::{AnimatableStatePlugin,DoorStatePlugin,InteractableStatePlugin,
    InventoryStatePlugin,WorldFlagsStatePlugin,WorldInteraction};
use bevy::prelude::*;
use std::collections::HashMap;

// todo break apart into modules, no need for unified world state
#[derive(Default)]
pub struct WorldState {
    pub interactable_states: HashMap<Entity, InteractableState>,
    pub animatables: HashMap<String, AnimatableState>,
    pub doors: HashMap<String, DoorState>,
}

#[derive(Clone, Debug, Default)]
pub struct InteractableState {
    pub interaction: WorldInteraction,
}

#[derive(Debug, Default)]
pub struct AnimatableState {
    pub scene_entity: Option<Entity>,
    pub clips: Vec<Handle<AnimationClip>>,
}

#[derive(Debug, Default)]
pub struct DoorState {
    pub parent_entity: Option<Entity>,
    pub open: bool,
}

pub struct WorldStatePlugin;

impl Plugin for WorldStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(AnimatableStatePlugin)
        .add_plugin(DoorStatePlugin)
        .add_plugin(InteractableStatePlugin)
        .add_plugin(InventoryStatePlugin)
        .add_plugin(WorldFlagsStatePlugin)
        ;
    }
}
