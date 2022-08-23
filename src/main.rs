// disable console on windows for release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::prelude::{App, ClearColor, Color, Msaa, WindowDescriptor};
use bevy::DefaultPlugins;
use bevy_rapier3d::prelude::*;

use grahams_relay::GamePlugin;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2)))
        .insert_resource(WindowDescriptor {
            width: 1280.,
            height: 720.,
            title: "Graham's Relay".to_string(), // ToDo
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(GamePlugin)
        .run();
}
