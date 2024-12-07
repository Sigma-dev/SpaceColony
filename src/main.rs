mod looping_float;
mod occupables {
    pub mod button_value;
    pub mod occupable;
    pub mod occupable_counter;
}

use background::BackgroundPlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use blinking_sprite::BlinkingSpritePlugin;
use color_correction::{PostProcessPlugin, PostProcessSettings};
use looping_float::LoopingFloat;
use mouse_position::MousePositionPlugin;
use natural_resource::{spawn_bush, spawn_tree, NaturalResourcePlugin};
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

use bevy::{
    prelude::*, sprite::Material2dPlugin, window::PresentMode
};
use planet::{PlanetMaterial, PlanetSettings, PlanetWater, Planets};
use planet_placing::CircleMaterial;
use planet_sticker::PlanetSticker;
use planet_villager::spawn_villager;
use resources::ResourcesPlugin;
use scaling_sprite::ScalingSpritePlugin;
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
        .add_plugins(NoisyShaderPlugin)
        .add_plugins(PostProcessPlugin)
        .add_plugins(PanCamPlugin::default())
        .add_plugins(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
        .add_plugins((UiMaterialPlugin::<ui::ProgressBarMaterial>::default(), Material2dPlugin::<background::StarsMaterial>::default(), Material2dPlugin::<CircleMaterial>::default()))
        .add_systems(Startup, setup)
        .add_event::<occupable::OccupancyChange>()
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut planets: ResMut<Planets>,
    mut planet_materials: ResMut<Assets<PlanetMaterial>>,
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
    for tree_index in 0..1 {
        spawn_tree(&mut commands, &asset_server, main_planet, tree_index as f32 * 180.);
    }
    for bush_index in 0..1 {
        spawn_bush(&mut commands, &asset_server, main_planet, (bush_index + 1) as f32 * 33.)
    }
    for villager_index in 0..1 {
        spawn_villager(&mut commands, &asset_server, main_planet, 30. + 45. * (villager_index as f32), villager_index.to_string())
    }
    commands.spawn({(
        PlanetSticker {
            planet: Some(main_planet),
            position_degrees: LoopingFloat::new(67.5),
            size_degrees: Some(45.)
        },
        PlanetWater{},
        Name::new("Water")
    )
    });
    
    /* 
    commands.spawn({(
        PlanetSticker {
            planet: Some(main_planet),
            position_degrees: LoopingFloat::new(200.),
            size_degrees: Some(45.)
        },
        PlanetWater{},)
    });

    commands.spawn({(
        PlanetSticker {
            planet: Some(main_planet),
            position_degrees: LoopingFloat::new(270.),
            size_degrees: Some(45.)
        },
        PlanetWater{},)
    });
    */

     
}