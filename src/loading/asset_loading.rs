use crate::{game_state::GameState, settings::SettingsAsset, world::WorldAsset};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct AssetLoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for AssetLoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::AssetLoading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<CharacterAssets>()
            .with_collection::<SettingsAssets>()
            .with_collection::<TextureAssets>()
            .with_collection::<WorldAssets>()
            .with_collection::<WorldProps>()
            .continue_to_state(GameState::Menu),
        );
    }
}

// the following asset collections will be loaded during the State `GameState::Loading`
// when done loading, they will be inserted as resources (see https://github.com/NiklasEi/bevy_asset_loader)

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/big_switch.ogg")]
    pub big_switch: Handle<AudioSource>,
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
    #[asset(path = "audio/steps_snow_dry.ogg")]
    pub steps_snow_dry: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct CharacterAssets {
    #[asset(path = "characters/graham_4action_v2.glb#Scene0")]
    pub graham: Handle<Scene>,
}

#[derive(AssetCollection)]
pub struct SettingsAssets {
    #[asset(path = "settings/default.settings")]
    pub default_settings: Handle<SettingsAsset>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(path = "textures/fuse_small_icon.png")]
    pub fuse_small_icon: Handle<Image>,
    #[asset(path = "textures/bevy.png")]
    pub texture_bevy: Handle<Image>,
}

#[derive(AssetCollection)]
pub struct WorldAssets {
    #[asset(path = "world/world01.world")]
    pub world01: Handle<WorldAsset>,
}

#[derive(AssetCollection)]
pub struct WorldProps {
    #[asset(path = "props/big_switch.glb#Scene0")]
    pub big_switch: Handle<Scene>,
    #[asset(path = "props/cardboard_closed.glb#Scene0")]
    pub cardboard_closed: Handle<Scene>,
    #[asset(path = "props/cardboard_opened.glb#Scene0")]
    pub cardboard_opened: Handle<Scene>,
    #[asset(path = "props/cardboard_tube.glb#Scene0")]
    pub cardboard_tube: Handle<Scene>,
    #[asset(path = "props/city_fence.glb#Scene0")]
    pub city_fence: Handle<Scene>,
    #[asset(path = "props/denki_train.glb#Scene0")]
    pub denki_train: Handle<Scene>,
    #[asset(path = "props/door_blue.glb#Scene0")]
    pub door_blue: Handle<Scene>,
    #[asset(path = "props/fuse_console.glb#Scene0")]
    pub fuse_console: Handle<Scene>,
    #[asset(path = "props/fuse_small.glb#Scene0")]
    pub fuse_small: Handle<Scene>,
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
    #[asset(path = "props/rail_track.glb#Scene0")]
    pub rail_track: Handle<Scene>,
    #[asset(path = "props/tunnel_entrance.glb#Scene0")]
    pub tunnel_entrance: Handle<Scene>,
    #[asset(path = "props/world01_building01.glb#Scene0")]
    pub world01_building01: Handle<Scene>,
    #[asset(path = "props/world01_ground01.glb#Scene0")]
    pub world01_ground01: Handle<Scene>,
    #[asset(path = "props/world01_ground02.glb#Scene0")]
    pub world01_ground02: Handle<Scene>,
    #[asset(path = "props/world01_ground03.glb#Scene0")]
    pub world01_ground03: Handle<Scene>,
}
