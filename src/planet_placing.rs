use approx::AbsDiffEq;
use bevy::{
    input::mouse, math::VectorSpace, prelude::*, render::mesh::CircleMeshBuilder, sprite::{Anchor, MaterialMesh2dBundle, Mesh2dHandle}
};
use crate::{looping_float::{self, LoopingFloat}, mouse_position::MousePosition, planet::{Planet, Planets}, planet_sticker::PlanetSticker, OccupableType};

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
    PlanetPlacingGhost,
    PlanetSticker::default()
    ));
    commands.insert_resource(PlanetPlacing { building_type: None })
}

fn handle_ghost(
    keys: Res<ButtonInput<KeyCode>>,
    mouse_position: Res<MousePosition>,
    asset_server: Res<AssetServer>,
    planets: Res<Planets>,
    mut planet_placing: ResMut<PlanetPlacing>,
    mut ghost_query: Query<(&mut Transform, &mut Handle<Image>, &mut Visibility, &mut PlanetSticker, &mut Sprite), With<PlanetPlacingGhost>>,
    planets_query: Query<(Entity, &Planet, &GlobalTransform)>
) {
    let (mut ghost_transform, mut ghost_image, mut ghost_visibility, mut ghost_sticker, mut ghost_sprite) = ghost_query.single_mut();

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
        if let Some((planet_entity, angle)) = find_closest_surface(mouse_position.world_position, &planets.all, &planets_query, 20.) {
            println!("{}", angle);
            ghost_sticker.planet = Some(planet_entity);
            ghost_sticker.position_degrees = LoopingFloat::new(angle);
            ghost_sprite.anchor = Anchor::BottomCenter;
        } else {
            ghost_sticker.planet = None;
            ghost_transform.translation = Vec3::new(mouse_position.world_position.x, mouse_position.world_position.y, 0.0);
            ghost_sprite.anchor = Anchor::Center;
            ghost_transform.rotation = Quat::from_axis_angle(Vec3::ZERO, 0.)
        }
    } else {
        *ghost_visibility = Visibility::Hidden;
    }
}

fn find_closest_surface(pos: Vec2, planets: &Vec<Entity>, planets_query: &Query<(Entity, &Planet, &GlobalTransform)>, threshold: f32) -> Option<(Entity, f32)> {
    let mut best: Option<(Entity, f32)> = None;
    for planet_id in planets {
        if let Ok((planet_entity, planet, planet_transform)) = planets_query.get(*planet_id) {
            let planet_pos_2d = vec3_to_vec2(planet_transform.translation());
            let dist = pos.distance(planet_pos_2d);
            if dist.abs_diff_eq(&planet.radius, threshold) {
                let diff = pos - planet_pos_2d;
                let up = vec3_to_vec2(*planet_transform.up());
                let angle = diff.angle_between(up).to_degrees();
                best = Some((planet_entity, angle));
            }
        }
    }
    return best;
}

fn vec2_to_vec3(source: Vec2) -> Vec3 {
    return Vec3{ x: source.x, y: source.y, z: 0. };
}

fn vec3_to_vec2(source: Vec3) -> Vec2 {
    return Vec2{ x: source.x, y: source.y };
}