mod looping_float;
mod occupables {
    pub mod button_value;
    pub mod occupable;
    pub mod occupable_counter;
}

use occupable::{NewOccupable, Occupable, OccupableBundle, OccupableType, ResourceType};
use occupables::*;
mod planet;
mod planet_sticker;
mod planet_villager;
mod spritesheet_animator;

#[macro_use]
extern crate approx;

use bevy::{
    diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    render::render_resource::FilterMode,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
    utils::{self, HashMap},
    window::PresentMode,
};
use bevy_mod_picking::prelude::*;
use looping_float::LoopingFloat;
use planet::NewPlanet;
use planet_sticker::PlanetSticker;

#[derive(Resource, Default)]
struct Resources {
    stored: HashMap<i32, i32>,
}

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
            occupable::OccupablePlugin,
            occupable_counter::OccupableCounterPlugin,
            spritesheet_animator::SpritesheetAnimatorPlugin,
        ))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, setup)
        .add_event::<occupable::OccupancyChange>()
        .insert_resource(Msaa::Off)
        .insert_resource(Resources::default())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            far: 1000.,
            near: -1000.,
            scale: 0.4,
            ..default()
        },
        ..default()
    });

    let main_planet = commands
        .spawn(planet::PlanetBundle::new(100., meshes, materials))
        .id();
    for tree_index in 0..2 {
        commands.spawn((
            OccupableBundle::new(
                asset_server.load("environment/tree.png"),
                main_planet,
                tree_index as f32 * 180.,
                OccupableType::Cutting,
                ResourceType::Wood,
            ),
            On::<Pointer<Click>>::target_component_mut::<occupable::Occupable>(|_, occupable| {
                occupable.selected = true
            }),
        ));
    }
    for bush_index in 0..2 {
        commands.spawn((
            OccupableBundle::new(
                asset_server.load("environment/bush.png"),
                main_planet,
                (bush_index + 1) as f32 * 33.,
                OccupableType::Foraging,
                ResourceType::Food,
            ),
            On::<Pointer<Click>>::target_component_mut::<occupable::Occupable>(|_, occupable| {
                occupable.selected = true
            }),
        ));
    }
    for villager_index in 0..2 {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("player/player.png"),
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..default()
                },
                ..default()
            },
            spritesheet_animator::SpritesheetAnimator::new(
                UVec2 { x: 16, y: 16 },
                vec![vec![0.6; 2], vec![0.2; 2], vec![0.2; 4], vec![0.2; 2]],
            ),
            planet_sticker::PlanetSticker {
                planet: main_planet,
                position_degrees: LoopingFloat::new(45. + 45. * (villager_index as f32)),
            },
            planet_villager::PlanetVillager {
                name: format!("Villager{villager_index}"),
            },
            planet_villager::VillagerWandering::default(),
        ));
    }
}
