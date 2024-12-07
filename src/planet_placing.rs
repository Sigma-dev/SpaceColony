use approx::AbsDiffEq;
use bevy::{
    prelude::*, render::render_resource::{AsBindGroup, ShaderRef, ShaderType}, sprite::{Anchor, Material2d}
};
use crate::{blinking_sprite::BlinkingSprite, looping_float::LoopingFloat, mouse_position::MousePosition, planet::{Planet, Planets}, planet_sticker::{IsCollidingWith, PlanetSticker}, spawn_building, natural_resource::NaturalResource, ResourceType};

#[derive(Component)]
pub struct PlanetPlacingGhost;

#[derive(PartialEq)]
pub enum BuildingType {
    Sawmill = 32
}

pub struct BuildingInfo {
    pub exploited_resource: ResourceType,
    pub range: f32,
}

pub trait GetBuildingInfo {
    fn get_building_info(&self) -> BuildingInfo;
}

impl GetBuildingInfo for BuildingType {
    fn get_building_info(&self) -> BuildingInfo
    {
        match self {
            BuildingType::Sawmill => BuildingInfo { exploited_resource: ResourceType::Wood, range: 64. },
        }
    }
}

#[derive(Resource, Default)]
pub struct PlanetPlacing {
    building_type: Option<BuildingType>,
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CircleMaterial {
    #[uniform(0)]
    settings: CircleSettings,
}

#[derive(ShaderType, Debug, Clone)]
struct CircleSettings {
    radius: f32,
}

impl Material2d for CircleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle.wgsl".into()
    }
}

pub struct PlanetPlacingPlugin;

impl Plugin for PlanetPlacingPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlanetPlacing::default())
        .add_systems(Startup, spawn_ghost)
        .add_systems(Update, (handle_ghost, handle_circle, blink_resource_in_range));
    }
}

fn spawn_ghost(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut circle_materials: ResMut<Assets<CircleMaterial>>,
) {
    commands.spawn((
        Sprite::default(),
        PlanetPlacingGhost,
        PlanetSticker {
            size_degrees: Some(16.),
            ..default()
        },
        Name::new("PlacingGhost")
    )).with_children(|parent| {
        parent.spawn((
            Mesh2d(meshes.add(Rectangle { half_size: Vec2::new(100., 100.)})),
            MeshMaterial2d(circle_materials.add(CircleMaterial { settings: CircleSettings { radius: 50. }})),
            Name::new("PlacingGhostCircle")
        ));
    });
    commands.insert_resource(PlanetPlacing { building_type: None })
}

fn handle_circle(
    circle_query: Query<&MeshMaterial2d<CircleMaterial>>,
    planet_placing: Res<PlanetPlacing>,
    mut circle_materials: ResMut<Assets<CircleMaterial>>,
) {
    let Some(building_type) = &planet_placing.building_type else { return; };
    for handle in circle_query.iter() {
        if let Some(material) = circle_materials.get_mut(handle.id()) {
            material.settings.radius = building_type.get_building_info().range;
        }
    }
}

fn blink_resource_in_range(
    planets_query: Query<&Planet>,
    ghost_query: Query<&PlanetSticker, With<PlanetPlacingGhost>>,
    mut natural_resource_query: Query<(&NaturalResource, &PlanetSticker, &mut BlinkingSprite), Without<PlanetPlacingGhost>>,
    planet_placing: Res<PlanetPlacing>,
) {
    let ghost = ghost_query.single();
    for (natural_resource, resource_sticker, mut blinking) in natural_resource_query.iter_mut() {
        blinking.enabled = false;
        let Some(building_type) = &planet_placing.building_type else { 
            continue;
        };
        let Some(planet_entity) = ghost.planet else { continue; };
        let Ok(planet) = planets_query.get(planet_entity) else { continue; };
        let info = building_type.get_building_info();
        let arc_distance = resource_sticker.position_degrees.arc_distance(ghost.position_degrees.to_f32(), planet.radius);
        if arc_distance <= info.range && natural_resource.produced_resource == info.exploited_resource {
            blinking.enabled = true;
        }
    }
}

fn handle_ghost(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    mouse_position: Res<MousePosition>,
    asset_server: Res<AssetServer>,
    planets: Res<Planets>,
    mut planet_placing: ResMut<PlanetPlacing>,
    mut ghost_query: Query<(&mut Transform, &mut Visibility, &mut PlanetSticker, &mut Sprite), With<PlanetPlacingGhost>>,
    planets_query: Query<(Entity, &Planet, &GlobalTransform)>,
    stickers_query: Query<&PlanetSticker, Without<PlanetPlacingGhost>>,
) {
    let (mut ghost_transform, mut ghost_visibility, mut ghost_sticker, mut ghost_sprite) = ghost_query.single_mut();

    if keys.just_pressed(KeyCode::Space) {
        planet_placing.building_type = Some(BuildingType::Sawmill);
    }
    ghost_sprite.color.set_alpha(1.);
    if let Some(building_type) = &planet_placing.building_type {
        *ghost_visibility = Visibility::Visible;
        ghost_sprite.color.set_alpha(0.5);
        let image: Handle<Image>;
        match building_type {
            BuildingType::Sawmill => image = asset_server.load("buildings/sawmill.png"),
        }
        ghost_sprite.image = image;
        if let Some((planet_entity, angle)) = find_closest_surface(mouse_position.world_position, &planets.all, &planets_query, 20.) {
            ghost_sticker.planet = Some(planet_entity);
            ghost_sticker.position_degrees = LoopingFloat::new(angle);
            ghost_sprite.anchor = Anchor::BottomCenter;
            let colliding = check_planet_collisions(ghost_sticker.as_ref(), &stickers_query);
            if colliding {
                ghost_sprite.color.set_alpha(0.1);
            } else {
                if mouse_buttons.just_pressed(MouseButton::Left) {
                    spawn_building(&mut commands, &asset_server, planet_entity, angle, BuildingType::Sawmill);
                    planet_placing.building_type = None;
                }
            }
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
            let planet_pos_2d = planet_transform.translation().xy();
            let dist = pos.distance(planet_pos_2d);
            if dist.abs_diff_eq(&planet.radius, threshold) {
                let diff = pos - planet_pos_2d;
                let up = planet_transform.up().xy();
                let angle = diff.angle_to(up).to_degrees();
                best = Some((planet_entity, angle));
            }
        }
    }
    return best;
}

fn check_planet_collisions(sticker: &PlanetSticker, stickers_query: &Query<&PlanetSticker, Without<PlanetPlacingGhost>>) -> bool{
    for other_sticker in stickers_query.iter() {
        if sticker.is_colliding_with(other_sticker) {
            return true
        }
    }
    return false;
}