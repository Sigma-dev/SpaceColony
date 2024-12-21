use std::str::FromStr;

use bevy::prelude::*;
use bevy::*;
use color::palettes::css::{self, RED};
use render::render_resource::{AsBindGroup, ShaderRef};
use utils::HashMap;

use crate::{
    occupable::ResourceType, planet_placing::UpdateSelection, resources::Resources, storage::{SpaceResource, SpaceResources}
};

pub struct CustomUiPlugin;

impl Plugin for CustomUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                button_system,
                update_resource_texts,
                update_resource_bars,
                update_planet_resources,
                update_selection_ui,
            ),
        )
        .add_systems(Startup, spawn_ui)
        .add_event::<PlanetResourcesUpdate>();
    }
}

#[derive(Component)]
pub struct ResourceText {
    pub resource_type: ResourceType,
}

#[derive(Component)]
pub struct PlanetResourcesText;

#[derive(Component)]
pub struct BuildingSelectionUI;

#[derive(Event)]
pub struct PlanetResourcesUpdate {
    pub maybe_resources: Option<SpaceResources>,
}

impl PlanetResourcesUpdate {
    pub fn off() -> PlanetResourcesUpdate {
        PlanetResourcesUpdate {
            maybe_resources: None,
        }
    }

    pub fn on(resources: SpaceResources) -> PlanetResourcesUpdate {
        PlanetResourcesUpdate {
            maybe_resources: Some(resources),
        }
    }
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
    commands.spawn((
        Text::new("5 Wood"),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(5.0),
            left: Val::Px(15.0),
            ..default()
        },
        PlanetResourcesText,
    ));

    commands
        .spawn((
            Node {
                bottom: Val::Px(15.0),
                right: Val::Px(15.0),
                position_type: PositionType::Absolute,
                flex_direction: FlexDirection::Column,
                align_self: AlignSelf::Stretch,
                ..default()
            },
            Visibility::Hidden,
            BuildingSelectionUI
        ))
        .with_children(|parent| {
            parent.spawn((Text::new("Informative Text"),));
            parent
                .spawn((
                    Button,
                    Node {
                        width: Val::Px(150.0),
                        height: Val::Px(65.0),
                        border: UiRect::all(Val::Px(4.0)),
                        // horizontally center child text
                        justify_content: JustifyContent::Center,
                        // vertically center child text
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BorderColor::from(Color::WHITE),
                    BorderRadius::MAX,
                ))
                .with_child((
                    Text::new("Upgrade"),
                    TextFont {
                        font: asset_server.load("fonts/pixel.ttf"),
                        font_size: 20.0,
                        ..default()
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

fn update_selection_ui(
    mut selection_events: EventReader<UpdateSelection>,
    mut query: Query<&mut Visibility, With<BuildingSelectionUI>>
) {
    let mut selection = query.single_mut();
    for event in selection_events.read() {
        match event.selected {
            Some(_) => {
                *selection = Visibility::Inherited
            },
            None => {
                *selection = Visibility::Hidden
            },
        }
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<(&mut Text, &mut TextColor)>,
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let (mut text, mut text_color) = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                *text_color = TextColor(Color::BLACK);
                *color = BackgroundColor(Color::WHITE);
            }
            Interaction::Hovered => {
                *text_color = TextColor(Color::WHITE);
                *color = BackgroundColor(Color::BLACK);
            }
            Interaction::None => {
                *text_color = TextColor(Color::WHITE);
                *color = BackgroundColor(Color::BLACK);
            }
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

fn update_planet_resources(
    mut resources_events: EventReader<PlanetResourcesUpdate>,
    mut bars: Query<&mut Text, With<PlanetResourcesText>>,
) {
    for event in resources_events.read() {
        for mut text in bars.iter_mut() {
            if let Some(resource) = &event.maybe_resources {
                let mut string = "Available Resources:\n".to_owned();
                string.push_str(
                    (resource
                        .iter()
                        .map(|(k, v)| format!("{} {}\n", v, k))
                        .collect::<Vec<_>>()
                        .join("\n")
                        + "\n")
                        .as_str(),
                );
                text.0 = string;
            } else {
                text.0 = "".to_owned()
            }
        }
    }
}
