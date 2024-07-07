mod looping_float;
mod occupables {
    pub mod button_value;
    pub mod occupable;
    pub mod occupable_counter;
}
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
    window::PresentMode,
};
use bevy_mod_picking::prelude::*;
use looping_float::LoopingFloat;
use planet_sticker::PlanetSticker;

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
            spritesheet_animator::SpritesheetAnimatorPlugin
        ))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, (setup))
        .add_event::<OccupancyChange>()
        .insert_resource(Msaa::Off)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

#[derive(Event)]
struct OccupancyChange {
    occupable: Entity,
    change: i32,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
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

    //  commands.spawn(Camera2dBundle::default());
    let rad: f32 = 100.;
    let main_planet = commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: rad })),
                material: materials.add(Color::hsl(1., 1., 1.)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            planet::Planet { radius: rad },
        ))
        .id();
    for tree_index in 0..2 {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    anchor: bevy::sprite::Anchor::BottomCenter,
                    ..default()
                },
                texture: asset_server.load("environment/tree/tree.png"),
                ..default()
            },
            planet_sticker::PlanetSticker {
                planet: main_planet,
                position_degrees: LoopingFloat::new(0. + tree_index as f32 * 180.),
            },
            occupable::Occupable {
                selected: false,
                workers: Vec::new(),
                max_workers: 1,
                occupable_type: occupable::OccupableType::Cutting,
            },
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
            spritesheet_animator::SpritesheetAnimator::new( UVec2{x: 16, y: 16}, vec![
                vec![0.6; 2], 
                vec![0.2; 2],
                vec![0.2; 4],
            ]),
            planet_sticker::PlanetSticker {
                planet: main_planet,
                position_degrees: LoopingFloat::new(45. + 45. * (villager_index as f32)),
            },
            planet_villager::PlanetVillager {
                name: format!("Villager{villager_index}")
            },
            planet_villager::VillagerWandering::default(),
        ));
    }
}
