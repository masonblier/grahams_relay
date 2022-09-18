use crate::inputs::{CursorLockState};
use crate::game_state::GameState;
use crate::loading::{FontAssets,TextureAssets};
use bevy::prelude::*;

// system state
#[derive(Default)]
pub struct InventoryState {
    pub root_ent: Option<Entity>,
    pub fuse_small_count: usize,
    pub bottle_lightfuel_count: usize,
}


pub enum InventoryItem {
    BottleLightFuel,
    FuseSmall,
}

pub enum InventoryEventAction {
    AddItem,
    RemoveItem,
}
pub struct InventoryEvent {
    pub action: InventoryEventAction,
    pub item: InventoryItem,
}

// Tags for UI components
#[derive(Component)]
struct InventoryBottleLightFuelCountText;
#[derive(Component)]
struct InventoryBottleLightFuelNode;
#[derive(Component)]
struct InventoryFuseSmallCountText;
#[derive(Component)]
struct InventoryFuseSmallNode;

pub struct InventoryStatePlugin;

impl Plugin for InventoryStatePlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(InventoryState::default())
        .add_event::<InventoryEvent>()
        .add_system_set(SystemSet::on_enter(GameState::Running)
            .with_system(setup_inventory_interaction))
        .add_system_set(
            SystemSet::on_update(GameState::Running)
            .with_system(update_inventory_interaction)
        )
        .add_system_set(SystemSet::on_exit(GameState::Running)
            .with_system(exit_inventory_interaction))
        ;
    }
}

fn setup_inventory_interaction(
    mut commands: Commands,
    font_assets: Res<FontAssets>,
    texture_assets: Res<TextureAssets>,
    mut inventory_state: ResMut<InventoryState>,
) {
    inventory_state.root_ent = Some(commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(10.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                flex_direction: FlexDirection::Column,
                padding: UiRect::new(Val::Percent(1.),Val::Percent(1.),Val::Percent(5.),Val::Percent(5.)),
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
        .with_children(|parent| {
            // fuse small icon
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::new(Val::Percent(1.),Val::Percent(1.),Val::Percent(1.),Val::Percent(1.)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    visibility: Visibility { is_visible: inventory_state.fuse_small_count > 0 },
                    ..default()
                })
                .insert(InventoryFuseSmallNode)
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Auto),
                            ..default()
                        },
                        image: texture_assets.fuse_small_icon.clone().into(),
                        ..default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("x {}", inventory_state.fuse_small_count),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 24.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            alignment: TextAlignment::CENTER,
                        },
                        ..Default::default()
                    })
                    .insert(InventoryFuseSmallCountText)
                    ;
                });
            // bottle lightfuel icon
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::new(Val::Percent(1.),Val::Percent(1.),Val::Percent(1.),Val::Percent(1.)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    visibility: Visibility { is_visible: inventory_state.bottle_lightfuel_count > 0 },
                    ..default()
                })
                .insert(InventoryBottleLightFuelNode)
                .with_children(|parent| {
                    parent.spawn_bundle(ImageBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.), Val::Auto),
                            ..default()
                        },
                        image: texture_assets.bottle_lightfuel_icon.clone().into(),
                        ..default()
                    });
                    parent.spawn_bundle(TextBundle {
                        text: Text {
                            sections: vec![TextSection {
                                value: format!("x {}", inventory_state.bottle_lightfuel_count),
                                style: TextStyle {
                                    font: font_assets.fira_sans.clone(),
                                    font_size: 24.0,
                                    color: Color::rgb(0.9, 0.9, 0.9),
                                },
                            }],
                            alignment: TextAlignment::CENTER,
                        },
                        ..Default::default()
                    })
                    .insert(InventoryBottleLightFuelCountText)
                    ;
                });
            // empty
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                        justify_content: JustifyContent::FlexStart,
                        align_items: AlignItems::FlexStart,
                        padding: UiRect::new(Val::Percent(1.),Val::Percent(1.),Val::Percent(1.),Val::Percent(1.)),
                        ..default()
                    },
                    color: Color::NONE.into(),
                    ..default()
                });
    }).id());
}

fn update_inventory_interaction(
    cursor_lock_state: Res<CursorLockState>,
    mut inventory_state: ResMut<InventoryState>,
    mut inventory_events: EventReader<InventoryEvent>,
    mut text_query: ParamSet<(
        Query<&mut Text, With<InventoryBottleLightFuelCountText>>,
        Query<&mut Text, With<InventoryFuseSmallCountText>>,
        )>,
    mut node_query: ParamSet<(
        Query<&mut Visibility, With<InventoryBottleLightFuelNode>>,
        Query<&mut Visibility, With<InventoryFuseSmallNode>>,
        )>,
) {
    if !cursor_lock_state.enabled {
        return;
    }

    for inventory_event in inventory_events.iter() {
        // update state
        match &inventory_event.item {
            InventoryItem::FuseSmall => {
                match &inventory_event.action {
                    InventoryEventAction::AddItem => {
                        inventory_state.fuse_small_count += 1;
                    },
                    InventoryEventAction::RemoveItem => {
                        inventory_state.fuse_small_count -= 1;
                    }
                }
                // update ui
                let mut fuse_text_query = text_query.p1();
                let mut fuse_node_query = node_query.p1();
                let mut text = fuse_text_query.single_mut();
                text.sections[0].value = format!("x {}", inventory_state.fuse_small_count);
                let mut vis = fuse_node_query.single_mut();
                vis.is_visible = inventory_state.fuse_small_count > 0;
                vis.set_changed();
            },
            InventoryItem::BottleLightFuel => {
                match &inventory_event.action {
                    InventoryEventAction::AddItem => {
                        inventory_state.bottle_lightfuel_count += 1;
                    },
                    InventoryEventAction::RemoveItem => {
                        inventory_state.bottle_lightfuel_count -= 1;
                    }
                }
                // update ui
                let mut bottle_text_query = text_query.p0();
                let mut bottle_node_query = node_query.p0();
                let mut text = bottle_text_query.single_mut();
                text.sections[0].value = format!("x {}", inventory_state.bottle_lightfuel_count);
                let mut vis = bottle_node_query.single_mut();
                vis.is_visible = inventory_state.bottle_lightfuel_count > 0;
                vis.set_changed();
            },
        }

    }
}

fn exit_inventory_interaction(
    mut commands: Commands,
    inventory_state: Res<InventoryState>,
) {
    if let Some(root_ent) = inventory_state.root_ent {
        commands.entity(root_ent).despawn_recursive();
    }
}
