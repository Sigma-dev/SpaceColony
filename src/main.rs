mod looping_float;
mod occupables {
    pub mod button_value;
    pub mod occupable;
    pub mod occupable_counter;
    // You can also add other modules here if needed
    // pub mod button_value;
    // pub mod occupable_constants;
    // pub mod looping_float;
}
use occupables::*;
mod planet;
mod planet_sticker;
mod planet_villager;

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
        ))
        .add_plugins(DefaultPickingPlugins)
        .add_systems(Startup, (setup))
        .insert_resource(Msaa::Off)
        .run();
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
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
    commands
        .spawn((
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
                position_degrees: LoopingFloat::new(0.),
            },
            occupable::Occupable {
                selected: false,
                number_of_workers: 0,
            },
            On::<Pointer<Click>>::target_component_mut::<occupable::Occupable>(
                |drag, occupable| occupable.selected = true,
            ),
        ))
        .with_children(|parent: &mut ChildBuilder| {
            parent.spawn((
                SpriteSheetBundle {
                    texture: asset_server.load("ui/symbols.png"),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                            Vec2::new(8.0, 8.0),
                            10,
                            2,
                            None,
                            None,
                        )),
                        index: 0,
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: 0.,
                            y: 24.,
                            z: 0.,
                        },
                        ..Default::default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                occupable_counter::OccupableCounter,
            ));
            parent.spawn((
                SpriteSheetBundle {
                    texture: asset_server.load("ui/symbols.png"),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                            Vec2::new(8.0, 8.0),
                            10,
                            2,
                            None,
                            None,
                        )),
                        index: 11,
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: -16.,
                            y: 24.,
                            z: 0.,
                        },
                        ..Default::default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                button_value::Buttonvalue { value: -1 },
                On::<Pointer<Click>>::run(change_value),
            ));
            parent.spawn((
                SpriteSheetBundle {
                    texture: asset_server.load("ui/symbols.png"),
                    atlas: TextureAtlas {
                        layout: texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
                            Vec2::new(8.0, 8.0),
                            10,
                            2,
                            None,
                            None,
                        )),
                        index: 10,
                    },
                    transform: Transform {
                        translation: Vec3 {
                            x: 16.,
                            y: 24.,
                            z: 0.,
                        },
                        ..Default::default()
                    },
                    visibility: Visibility::Hidden,
                    ..default()
                },
                button_value::Buttonvalue { value: 1 },
                On::<Pointer<Click>>::run(change_value),
            ));
        });

    let layout = TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 2, 2, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices { first: 2, last: 3 };
    commands.spawn((
        SpriteSheetBundle {
            texture: asset_server.load("player/player.png"),
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            sprite: Sprite {
                anchor: bevy::sprite::Anchor::BottomCenter,
                ..default()
            },
            ..default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
        planet_sticker::PlanetSticker {
            planet: main_planet,
            position_degrees: LoopingFloat::new(45.),
        },
        planet_villager::PlanetVillager {
            current_state: planet_villager::PlanetVillagerState::Running,
            current_destination: LoopingFloat::new(0.),
        },
    ));
}

fn change_value(
    event: Listener<Pointer<Click>>,
    button_query: Query<(&button_value::Buttonvalue, &Parent)>,
    mut occupable_query: Query<&mut occupable::Occupable>,
) {
    let Ok((button, parent)) = button_query.get(event.target) else {
        return;
    };
    let Ok(mut occupable) = occupable_query.get_mut(parent.get()) else {
        return;
    };
    let new: i32 = occupable.number_of_workers + button.value;
    if new < 0 || new > 9 { return; }
    occupable.number_of_workers = new;
}
