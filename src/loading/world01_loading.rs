use crate::{game_state::GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct World01LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for World01LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<World01Props>();
        app.add_loading_state(
            LoadingState::new(GameState::World01Loading)
            .with_collection::<World01Props>()
            .continue_to_state(GameState::WorldLoading),
        );
    }
}

#[derive(AssetCollection,Default)]
pub struct World01Props {
    #[asset(path = "props/cardboard_closed.glb#Scene0")]
    pub cardboard_closed: Handle<Scene>,
    #[asset(path = "props/cardboard_opened.glb#Scene0")]
    pub cardboard_opened: Handle<Scene>,
    #[asset(path = "props/cardboard_tube.glb#Scene0")]
    pub cardboard_tube: Handle<Scene>,
    #[asset(path = "props/door_blue.glb#Scene0")]
    pub door_blue: Handle<Scene>,
    #[asset(path = "props/fountain_round.glb#Scene0")]
    pub fountain_round: Handle<Scene>,
    #[asset(path = "props/office_desk01.glb#Scene0")]
    pub office_desk01: Handle<Scene>,
    #[asset(path = "props/house_roof01.glb#Scene0")]
    pub house_roof01: Handle<Scene>,
    #[asset(path = "props/house_woodside.glb#Scene0")]
    pub house_woodside: Handle<Scene>,
    #[asset(path = "props/office_desk02.glb#Scene0")]
    pub office_desk02: Handle<Scene>,
    #[asset(path = "props/office_chair.glb#Scene0")]
    pub office_chair: Handle<Scene>,
    #[asset(path = "props/pallet.glb#Scene0")]
    pub pallet: Handle<Scene>,
    #[asset(path = "props/world01_building01.glb#Scene0")]
    pub world01_building01: Handle<Scene>,
    #[asset(path = "props/world01_ground01.glb#Scene0")]
    pub world01_ground01: Handle<Scene>,
}
