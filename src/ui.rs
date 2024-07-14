use bevy::prelude::*;
use bevy::*;
use node_bundles::NodeBundle;
use ui::*;

use crate::{occupable::ResourceType, resources::Resources, CustomMaterial};

pub struct CustomUiPlugin;

impl Plugin for CustomUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_resource_texts, update_resource_bars))
            .add_systems(Startup, spawn_ui);
    }
}

#[derive(Component)]
pub struct ResourceText {
    pub resource_type: ResourceType,
}

#[derive(Component)]
pub struct ResourceBar {
    pub resource_type: ResourceType,
}

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut custom_materials: ResMut<Assets<CustomMaterial>>,
) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Row,
                padding: UiRect::all(Val::Px(5.)),
                column_gap: Val::Px(8.),
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    ..default()
                },
                UiImage::new(asset_server.load("ui/icons/villager.png")),
            ));
            parent.spawn((
                MaterialNodeBundle {
                    style: Style {
                        width: Val::Px(160.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    material: custom_materials.add(CustomMaterial { progress: 0. }),
                    ..default()
                },
                UiImage::new(asset_server.load("ui/progress_bar/ProgressBar.png")),
                ResourceBar {
                    resource_type: ResourceType::Food,
                },
            ));
            parent.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                    ..default()
                },
                UiImage::new(asset_server.load("ui/icons/wood.png")),
            ));
            parent.spawn((
                TextBundle::from_section(
                    "0",
                    TextStyle {
                        font: asset_server.load("fonts/pixel.ttf"),
                        font_size: 30.0,
                        ..default()
                    },
                ),
                Label,
                ResourceText {
                    resource_type: ResourceType::Wood,
                },
            ));
        });
}

fn update_resource_texts(resources: Res<Resources>, mut texts: Query<(&mut Text, &ResourceText)>) {
    for (mut text, resource_text) in texts.iter_mut() {
        if let Some(amount) = resources.stored.get(&(resource_text.resource_type as i32)) {
            text.sections[0].value = amount.to_string();
        }
    }
}

fn update_resource_bars(
    resources: Res<Resources>,
    mut bars: Query<(&Handle<CustomMaterial>, &ResourceBar)>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for (handle, resource_text) in bars.iter_mut() {
        if let Some(amount) = resources.stored.get(&(resource_text.resource_type as i32)) {
            if let Some(material) = materials.get_mut(handle) {
                material.progress = (*amount as f32) / 10.0 as f32;
            }
        }
    }
}
