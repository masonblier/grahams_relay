

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameState {
    // During the loading State the LoadingPlugin will load our assets
    PreLoading,
    AssetLoading,
    WorldInit,
    World01Loading,
    World03Loading,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // During this State the actual game logic is executed
    Running,
    // Game paused, can resume
    Paused,
    // Character loading and init
    CharacterLoading,
    // World loading states, level specific assets
    WorldLoading,
}
