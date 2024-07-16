use bevy::{
    prelude::*,
    render::mesh::CircleMeshBuilder,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use crate::{mouse_position::MousePosition, OccupableType};

#[derive(Component)]
pub struct PlanetPlacingGhost;

#[derive(PartialEq)]
pub enum BuildingType {
    Sawmill
}

#[derive(Resource, Default)]
pub struct PlanetPlacing {
    building_type: Option<BuildingType>,
}

pub struct PlanetPlacingPlugin;

impl Plugin for PlanetPlacingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlanetPlacing::default())
        .add_systems(Startup, spawn_ghost)
        .add_systems(Update, handle_ghost);
    }
}

fn spawn_ghost(
    mut commands: Commands,
) {
    commands.spawn((SpriteBundle {
        ..default()
    },
    PlanetPlacingGhost
    ));
    commands.insert_resource(PlanetPlacing { building_type: None })
}

fn handle_ghost(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_position: Res<MousePosition>,
    asset_server: Res<AssetServer>,
    mut planet_placing: ResMut<PlanetPlacing>,
    mut ghost_query: Query<(&mut Transform, &mut Handle<Image>, &mut Visibility), With<PlanetPlacingGhost>>
) {
    let (mut ghost_transform, mut ghost_image, mut ghost_visibility) = ghost_query.single_mut();

    if keys.just_pressed(KeyCode::Space) {
        planet_placing.building_type = Some(BuildingType::Sawmill);
    }

    if let Some(building_type) = &planet_placing.building_type {
        *ghost_visibility = Visibility::Visible;
        let image: Handle<Image>;
        match building_type {
            BuildingType::Sawmill => image = asset_server.load("buildings/sawmill.png"),
        }
        *ghost_image = image;
        ghost_transform.translation = Vec3::new(mouse_position.world_position.x, mouse_position.world_position.y, 0.0);
    } else {
        //*ghost_visibility = Visibility::Hidden;
    }
}