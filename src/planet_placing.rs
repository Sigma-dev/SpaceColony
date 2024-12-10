use approx::AbsDiffEq;
use bevy::{
    prelude::*, render::render_resource::{AsBindGroup, ShaderRef, ShaderType}, sprite::{AlphaMode2d, Anchor, Material2d}
};
use crate::{mouse_position::MousePosition, planet::Planet, planet_queries::{PlanetQueries, StickerCollider}, planet_sticker::{PlanetCollider, PlanetSticker}, ResourceType};

#[derive(Component, Debug)]
pub struct PlanetPlacingGhost {
    state: GhostState
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BuildingType {
    Sawmill = 32
}

pub struct BuildingInfo {
    pub exploited_resource: ResourceType,
    pub range: f32,
    pub image_path: String,
    pub size: f32,
}

pub trait GetBuildingInfo {
    fn get_building_info(&self) -> BuildingInfo;
}

impl GetBuildingInfo for BuildingType {
    fn get_building_info(&self) -> BuildingInfo
    {
        match self {
            BuildingType::Sawmill => BuildingInfo { exploited_resource: ResourceType::Wood, range: 64., image_path: "buildings/sawmill.png".to_string(), size: 8. },
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

    fn alpha_mode(&self) -> AlphaMode2d {
         AlphaMode2d::Blend
    }
}

#[derive(Event)]
struct ToggleBuilding {
    building_type: BuildingType
}

#[derive(Resource)]
struct BuildingSelection {
    buildings: Vec<BuildingType>
}

pub struct PlanetPlacingPlugin;

impl Plugin for PlanetPlacingPlugin {
    fn build(&self, app: &mut App) {
        app
        .insert_resource(PlanetPlacing::default())
        .insert_resource(BuildingSelection {buildings: vec![BuildingType::Sawmill]})
        .add_event::<ToggleBuilding>()
        .add_systems(Startup, spawn_ghost)
        .add_systems(Update, (handle_selection, handle_currently_building, handle_circle, handle_ghost, compute_ghost_state));
    }
}

fn handle_selection(
    keys: Res<ButtonInput<KeyCode>>,
    building_selection: Res<BuildingSelection>,
    mut building_writer: EventWriter<ToggleBuilding>
) {
    let mut maybe_selection: Option<usize> = None;
    if keys.just_pressed(KeyCode::Digit1) {
        maybe_selection = Some(0);
    }
    else if keys.just_pressed(KeyCode::Digit2) {
        maybe_selection = Some(1);
    }
    else if keys.just_pressed(KeyCode::Digit3) {
        maybe_selection = Some(2);
    }
    else if keys.just_pressed(KeyCode::Digit4) {
        maybe_selection = Some(3);
    }
    else if keys.just_pressed(KeyCode::Digit5) {
        maybe_selection = Some(4);
    }
    else if keys.just_pressed(KeyCode::Digit6) {
        maybe_selection = Some(5);
    }
    else if keys.just_pressed(KeyCode::Digit7) {
        maybe_selection = Some(6);
    }
    else if keys.just_pressed(KeyCode::Digit8) {
        maybe_selection = Some(7);
    }
    else if keys.just_pressed(KeyCode::Digit9) {
        maybe_selection = Some(8);
    }
    else if keys.just_pressed(KeyCode::Digit0) {
        maybe_selection = Some(9);
    }
    let Some(selection) = maybe_selection else { return };
    if let Some(building_type) = building_selection.buildings.get(selection) {
        building_writer.send(ToggleBuilding { building_type: *building_type });
    }
}

fn handle_currently_building(
    mut toggle_building_reader: EventReader<ToggleBuilding>,
    mut planet_placing: ResMut<PlanetPlacing>
) {
    let Some(event) = toggle_building_reader.read().last() else { return };
    if Some(event.building_type) == planet_placing.building_type {
        planet_placing.building_type = None
    } else {
        planet_placing.building_type = Some(event.building_type)
    }
}

fn spawn_ghost(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut circle_materials: ResMut<Assets<CircleMaterial>>,
) {
    commands.spawn((
        Sprite {
            anchor: Anchor::BottomCenter,
            ..default()
        },
        PlanetPlacingGhost { state: GhostState::Hidden },
        Name::new("PlacingGhost")
    )).with_children(|parent| {
        parent.spawn((
            Mesh2d(meshes.add(Rectangle { half_size: Vec2::new(100., 100.)})),
            MeshMaterial2d(circle_materials.add(CircleMaterial { settings: CircleSettings { radius: 50. }})),
            Name::new("PlacingGhostCircle")
        ));
    });
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

#[derive(Debug)]
enum GhostState {
    Hidden,
    Detached(BuildingType, Vec2),
    AttachedInvalid(BuildingType, Entity, f32),
    AttachedValid(BuildingType, Entity, f32),
}

impl GhostState {
    pub fn get_attached(&self) -> Option<(BuildingType, Entity, f32)> {
        match self {
            GhostState::Hidden | GhostState::Detached(_, _) => None,
            GhostState::AttachedInvalid(b, e, p) => Some((*b, *e, *p)),
            GhostState::AttachedValid(b, e, p) => Some((*b, *e, *p)),
        }
    }

    pub fn get_visuals(&self) -> Option<(f32, BuildingType)> {
        match self {
            GhostState::Hidden => None,
            GhostState::Detached(b, _) => Some((0.5, *b)),
            GhostState::AttachedInvalid(b, _, _) => Some((0.5, *b)),
            GhostState::AttachedValid(b, _, _) => Some((1., *b)),
        }
    }
}

fn compute_ghost_state(
    planet_placing: ResMut<PlanetPlacing>,
    mut ghost_query: Query<&mut PlanetPlacingGhost>,
    mouse_position: Res<MousePosition>,
    planet_queries: PlanetQueries
) {
    let mut ghost_state = ghost_query.single_mut();
    let mouse_pos = mouse_position.world_position;
    
    if let Some(building_type) = planet_placing.building_type {
        if let Some(closest) = planet_queries.find_closest_surface(mouse_pos) {
            if closest.distance < 20. {
                let sc = StickerCollider { sticker: PlanetSticker::new(closest.planet, closest.pos_degrees), collider: PlanetCollider::new(building_type.get_building_info().size)};
                if planet_queries.overlaps_anything(sc) {
                    ghost_state.state = GhostState::AttachedInvalid(building_type, closest.planet, closest.pos_degrees);
                } else {
                    ghost_state.state = GhostState::AttachedValid(building_type, closest.planet, closest.pos_degrees);
                }
                return;
            }
        }
        ghost_state.state = GhostState::Detached(building_type, mouse_pos);
    } else {
        ghost_state.state = GhostState::Hidden;
    }
}

fn handle_ghost(
    mut commands: Commands,
    mouse_position: Res<MousePosition>,
    mut ghost_query: Query<(Entity, &mut Transform, &mut Visibility, &mut Sprite, &PlanetPlacingGhost)>,
    asset_server: Res<AssetServer>,
) {
    let (ghost_entity, mut ghost_transform, mut ghost_visibility, mut ghost_sprite, ghost_state) = ghost_query.single_mut();
    let state = &ghost_state.state;

    if let Some((building_type, planet, pos)) = state.get_attached() {
        commands.entity(ghost_entity).insert((
            PlanetSticker::new(planet, pos),
            PlanetCollider::new(building_type.get_building_info().size)
        ));
    } else {
        commands.entity(ghost_entity).remove::<(PlanetSticker, PlanetCollider)>();
        ghost_transform.translation = mouse_position.world_position.extend(0.);
        ghost_transform.rotation = Quat::default();
    }

    if let Some((alpha, building_type)) = state.get_visuals() {
        *ghost_visibility = Visibility::Visible;
        ghost_sprite.color.set_alpha(alpha);
        ghost_sprite.image = asset_server.load(building_type.get_building_info().image_path);
    } else {
        *ghost_visibility = Visibility::Hidden;
    }
} 

/* fn blink_resource_in_range(
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
        let Ok(planet) = planets_query.get(ghost.planet) else { continue; };
        let info = building_type.get_building_info();
        let arc_distance = resource_sticker.position_degrees.arc_distance(ghost.position_degrees.to_f32(), planet.radius);
        if arc_distance <= info.range && natural_resource.produced_resource == info.exploited_resource {
            blinking.enabled = true;
        }
        if let Some((planet_entity, angle)) = find_closest_surface(mouse_position.world_position, &planets.all, &planets_query, 20.) {
    }
} */
/*
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
*/

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