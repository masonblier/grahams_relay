use crate::inputs::{CursorLockState};
use crate::game_state::GameState;
use crate::loading::{FontAssets,TextureAssets};
use bevy::prelude::*;

// system state
#[derive(Default)]
pub struct InventoryState {
    pub fuse_small_count: usize,
}


pub enum InventoryItem {
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
) {
    // fuse small icon
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexStart,
                padding: UiRect::new(Val::Percent(1.),Val::Percent(1.),Val::Percent(1.),Val::Percent(1.)),
                ..default()
            },
            color: Color::NONE.into(),
            visibility: Visibility { is_visible: false },
            ..default()
        })
        .insert(InventoryFuseSmallNode)
        .with_children(|parent| {
            parent.spawn_bundle(ImageBundle {
                style: Style {
                    size: Size::new(Val::Percent(8.), Val::Auto),
                    ..default()
                },
                image: texture_assets.fuse_small_icon.clone().into(),
                ..default()
            });
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "x 1".to_string(),
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
}

fn update_inventory_interaction(
    cursor_lock_state: Res<CursorLockState>,
    mut inventory_state: ResMut<InventoryState>,
    mut inventory_events: EventReader<InventoryEvent>,
    mut node_query: Query<&mut Visibility, With<InventoryFuseSmallNode>>,
    mut text_query: Query<&mut Text, With<InventoryFuseSmallCountText>>,
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
            },
        }

        // update ui
        let mut text = text_query.single_mut();
        text.sections[0].value = format!("x {}", inventory_state.fuse_small_count);
        let mut vis = node_query.single_mut();
        vis.is_visible = inventory_state.fuse_small_count > 0;
        vis.set_changed();
    }
}

fn exit_inventory_interaction(
    mut commands: Commands,
    node_query: Query<Entity, With<InventoryFuseSmallNode>>,
) {
    for ent in node_query.iter() {
        commands.entity(ent).despawn_recursive();();
    }
}