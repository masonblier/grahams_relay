use crate::{game_state::GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct World03LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for World03LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World03Props>();
        app.add_loading_state(
            LoadingState::new(GameState::World03Loading)
            .with_collection::<World03Props>()
            .continue_to_state(GameState::WorldLoading),
        );
    }
}

#[derive(AssetCollection,Default)]
pub struct World03Props {
    #[asset(path = "props/refinery_column01.glb#Scene0")]
    pub refinery_column01: Handle<Scene>,
    #[asset(path = "props/refinery_desalter.glb#Scene0")]
    pub refinery_desalter: Handle<Scene>,
    #[asset(path = "props/refinery_scaffolding.glb#Scene0")]
    pub refinery_scaffolding: Handle<Scene>,
    #[asset(path = "props/refinery_sphere.glb#Scene0")]
    pub refinery_sphere: Handle<Scene>,
    #[asset(path = "props/refinery_tank01.glb#Scene0")]
    pub refinery_tank01: Handle<Scene>,
    #[asset(path = "props/refinery_warmer.glb#Scene0")]
    pub refinery_warmer: Handle<Scene>,
    #[asset(path = "props/world03_ground.glb#Scene0")]
    pub world03_ground: Handle<Scene>,
    #[asset(path = "props/world03_pipes.glb#Scene0")]
    pub world03_pipes: Handle<Scene>,
    #[asset(path = "props/world03_walkways.glb#Scene0")]
    pub world03_walkways: Handle<Scene>,
}
