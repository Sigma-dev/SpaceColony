mod looping_float;
mod occupables {
    pub mod button_value;
    pub mod occupable;
    pub mod occupable_counter;
}

use background::BackgroundPlugin;
use mouse_position::{MousePosition, MousePositionPlugin};
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
mod planet_water;

use bevy::{
    prelude::*, render::render_resource::{AsBindGroup, ShaderRef}, sprite::{Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle}, window::PresentMode
};
use bevy_mod_picking::prelude::*;
use planet::{NewPlanet, PlanetMaterial, Planets};
use planet_villager::spawn_villager;
use planet_water::WaterPlanetMaterial;
use resources::ResourcesPlugin;
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
            MousePositionPlugin
        ))
        .add_plugins(NoisyShaderPlugin)
        .add_plugins((DefaultPickingPlugins, UiMaterialPlugin::<ui::ProgressBarMaterial>::default(), Material2dPlugin::<background::StarsMaterial>::default(), Material2dPlugin::<WaterPlanetMaterial>::default(), Material2dPlugin::<PlanetMaterial>::default()))
        .add_systems(Startup, setup)
        .add_event::<occupable::OccupancyChange>()
        .insert_resource(Msaa::Off)
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
    mut planets: ResMut<Planets>,
    mut water_materials: ResMut<Assets<WaterPlanetMaterial>>,
    mut planet_materials: ResMut<Assets<PlanetMaterial>>,
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
        .spawn(planet::PlanetBundle::new(100., &mut meshes, planet_materials))
        .id();
    planets.main = Some(main_planet);
    planets.all.push(main_planet);
    for tree_index in 0..2 {
        spawn_tree(&mut commands, &asset_server, main_planet, tree_index as f32 * 180.);
    }
    for bush_index in 0..1 {
        spawn_bush(&mut commands, &asset_server, main_planet, (bush_index + 1) as f32 * 33.)
    }
    for villager_index in 0..2 {
        spawn_villager(&mut commands, &asset_server, main_planet, 45. + 45. * (villager_index as f32), villager_index.to_string())
    }
    commands.spawn({
        (MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle { half_size: Vec2 { x: 100., y: 100. } })),
            material: water_materials.add(planet_water::WaterPlanetMaterial { }),
            transform: Transform {
                translation: Vec3 { x: 0., y: 0., z: -10. },
                ..default()
            },
            ..default()
        },
    )
    });
}