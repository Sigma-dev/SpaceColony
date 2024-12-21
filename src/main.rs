mod looping_float;
mod occupables {
    pub mod button_value;
    pub mod occupable;
    pub mod occupable_counter;
}

use background::BackgroundPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use blinking_sprite::{BlinkingSprite, BlinkingSpritePlugin};
use color_correction::{PostProcessPlugin, PostProcessSettings};
use looping_float::LoopingFloat;
use mouse_position::MousePositionPlugin;
use natural_resource::{spawn_bush, spawn_tree, NaturalResource, NaturalResourcePlugin};
use noisy_bevy::NoisyShaderPlugin;
use occupable::*;
use occupables::*;
mod planet;
mod planet_sticker;
mod planet_villager;
mod planet_placing;
mod spritesheet_animator;
mod resources;
mod ui;
mod villager_spawn;
mod background;
mod mouse_position;
mod blinking_sprite;
mod natural_resource;
mod scaling_sprite;
mod color_correction;
mod planet_queries;
mod storage;
mod spawn_building;

use bevy::{
    prelude::*, sprite::{Anchor, Material2dPlugin}, utils::hashbrown::HashMap, window::PresentMode
};
use planet::{PlanetMaterial, PlanetSettings, PlanetWater, Planets};
use planet_placing::CircleMaterial;
use planet_queries::PlanetQueries;
use planet_sticker::{PlanetCollider, PlanetSticker};
use planet_villager::spawn_villager;
use resources::ResourcesPlugin;
use scaling_sprite::ScalingSpritePlugin;
use spawn_building::SpawnBuildingPlugin;
use storage::{SpaceResource, Storage};
use ui::CustomUiPlugin;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            planet_sticker::PlanetStickerPlugin,
            planet_villager::PlanetVillagerPlugin,
            planet_placing::PlanetPlacingPlugin,
            occupable::OccupablePlugin,
            occupable_counter::OccupableCounterPlugin,
            spritesheet_animator::SpritesheetAnimatorPlugin,
            ResourcesPlugin,
            CustomUiPlugin,
            villager_spawn::VillagerSpawnPlugin,
            planet::PlanetsPlugin,
            BackgroundPlugin,
            MousePositionPlugin,
            BlinkingSpritePlugin,
            NaturalResourcePlugin,
            ScalingSpritePlugin,
        ))
        .add_plugins(SpawnBuildingPlugin)
        .add_plugins(NoisyShaderPlugin)
        .add_plugins(PostProcessPlugin)
        .add_plugins(PanCamPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins((UiMaterialPlugin::<ui::ProgressBarMaterial>::default(), Material2dPlugin::<background::StarsMaterial>::default(), Material2dPlugin::<CircleMaterial>::default()))
        .add_systems(Startup, (setup, post_setup).chain())
        .add_event::<occupable::OccupancyChange>()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut planets: ResMut<Planets>,
    mut planet_materials: ResMut<Assets<PlanetMaterial>>,
    mut planet_queries: PlanetQueries,
) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.4,
            ..OrthographicProjection::default_2d()
        },
        PostProcessSettings {
            white_color: Vec3::new(1., 1., 1.),
            black_color: Vec3::new(0.024, 0.025, 0.028),
        },
        PanCam {
            enabled: true,
            ..default()
        },
        Name::new("Camera"),
        Msaa::Off
    ));

    let main_planet = commands.spawn((
        Mesh2d(meshes.add(Rectangle{half_size: Vec2::splat(100.)})),
        MeshMaterial2d(
            planet_materials.add(PlanetMaterial { 
                settings: PlanetSettings {
                    hole_array: [Vec4::splat(0.); 8]
                }
            })),
        Transform::from_xyz(0.0, 0.0, -10.0),
        planet::Planet { radius: 100. },
        Name::new("MainPlanet")
    )).id();
    planets.main = Some(main_planet);
    planets.all.push(main_planet);
    
    commands.spawn((
        Sprite { 
            image: asset_server.load("buildings/silo.png"),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        PlanetSticker {
            planet: main_planet,
            position_degrees: LoopingFloat::new(40.),
        },
        PlanetCollider {
            size_degrees: 8.
        },
        Storage {
            resources: HashMap::from([(SpaceResource::Wood, 10)]),
            max_amount: 200
        }
    ));
}

fn post_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    planets: ResMut<Planets>,
    mut planet_queries: PlanetQueries,
) {
    let main_planet = planets.main.unwrap();
    for _tree_index in 0..15 {
        //spawn_tree(&mut commands, &asset_server, main_planet, tree_index as f32 * 180.);
        place_trees_randomly(&mut commands, &asset_server, main_planet, &mut planet_queries);
    }
}

fn place_trees_randomly(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    planet: Entity,
    planet_queries: &mut PlanetQueries,
) {
    let tree_size = 8.;
    let Some(pos) = planet_queries.get_random_valid_placement(planet, tree_size, 10) else { return };
    commands.spawn((
        Sprite {
            image: asset_server.load("environment/tree.png"),
            anchor: Anchor::BottomCenter,
            ..default()
        },
        BlinkingSprite::new(false),
        PlanetSticker {
            planet,
            position_degrees: LoopingFloat::new(pos),
        },
        PlanetCollider {
            size_degrees: tree_size
        },
        NaturalResource {
            produced_resource: SpaceResource::Wood
        },
    )).with_child((
        Transform::from_translation(Vec3::new(0., 20., 0.)),
        Text2d::default(),
        TextFont {
            font_size: 5.,
            font_smoothing: bevy::text::FontSmoothing::None,
            font: asset_server.load("fonts/pixel.ttf")
        },
        TextLayout::new_with_justify(JustifyText::Center),
        Visibility::Hidden,
    ));
}