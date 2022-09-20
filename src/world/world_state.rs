use crate::world::{AnimatableStatePlugin,DoorStatePlugin,InteractableStatePlugin,
    InventoryStatePlugin,LightsStatePlugin,SoundsStatePlugin,TrainsStatePlugin,
    WorldFlagsStatePlugin,WorldInteraction};
use bevy::prelude::*;
use std::collections::HashMap;

// todo break apart into modules, no need for unified world state
#[derive(Default)]
pub struct WorldState {
    pub interactable_states: HashMap<Entity, InteractableState>,
    pub animatables: HashMap<String, AnimatableState>,
    pub animatable_lights: HashMap<String, Entity>,
    pub animatable_sounds: HashMap<String, WorldSoundState>,
    pub animatable_trains: HashMap<String, WorldTrainState>,
    pub doors: HashMap<String, DoorState>,
    pub active_train: Option<String>,
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

#[derive(Clone, Debug, Default)]
pub struct WorldSoundState {
    pub sound: String,
    pub position: Vec3,
    pub panning: f32,
    pub volume: f32,
    pub paused: bool,
}

#[derive(Clone, Debug, Default)]
pub struct WorldTrainState {
    pub parent_entity: Option<Entity>,
    pub running: bool,
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
        .add_plugin(LightsStatePlugin)
        .add_plugin(SoundsStatePlugin)
        .add_plugin(TrainsStatePlugin)
        .add_plugin(WorldFlagsStatePlugin)
        ;
    }
}
