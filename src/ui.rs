use bevy::prelude::*;
use bevy::*;
use render::render_resource::{AsBindGroup, ShaderRef};

use crate::{occupable::ResourceType, resources::Resources};

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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct ProgressBarMaterial {
    #[uniform(0)]
    progress: f32,
}

impl UiMaterial for ProgressBarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/progress_bar/shader.wgsl".into()
    }
} 

fn spawn_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut custom_materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    commands
        .spawn(Node {
            width: Val::Percent(100.),
            flex_direction: FlexDirection::Row,
            padding: UiRect::all(Val::Px(5.)),
            column_gap: Val::Px(8.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                ImageNode::new(asset_server.load("ui/icons/villager.png")),
            ));
            parent.spawn((
                MaterialNode(custom_materials.add(ProgressBarMaterial { progress: 0. })),
                ImageNode::new(asset_server.load("ui/progress_bar/ProgressBar.png")),
                Node{
                        width: Val::Px(160.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                ResourceBar {
                    resource_type: ResourceType::Food,
                },
            ));
            parent.spawn((
                Node {
                        width: Val::Px(32.0),
                        height: Val::Px(32.0),
                        ..default()
                    },
                ImageNode::new(asset_server.load("ui/icons/wood.png")),
            ));
            parent.spawn((
                Label,
                Text::new(""),
                TextFont {
                    font: asset_server.load("fonts/pixel.ttf"),
                    font_size: 30.0,
                    ..default()
                },
                ResourceText {
                    resource_type: ResourceType::Wood,
                },
            ));
        });
}

fn update_resource_texts(resources: Res<Resources>, mut texts: Query<(&mut Text, &ResourceText)>) {
    for (mut text, resource_text) in texts.iter_mut() {
        if let Some(amount) = resources.stored.get(&(resource_text.resource_type as i32)) {
            text.0 = amount.to_string();
        }
    }
}

fn update_resource_bars(
    resources: Res<Resources>,
    mut bars: Query<(&MaterialNode<ProgressBarMaterial>, &ResourceBar)>,
    mut materials: ResMut<Assets<ProgressBarMaterial>>,
) {
    for (handle, resource_text) in bars.iter_mut() {
        if let Some(amount) = resources.stored.get(&(resource_text.resource_type as i32)) {
            if let Some(material) = materials.get_mut(handle.id()) {
                material.progress = (*amount as f32) / 10.0 as f32;
            }
        }
    }
}
