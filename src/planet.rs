use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Planet {
    pub radius: f32,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub mesh: MaterialMesh2dBundle<ColorMaterial>,
    pub planet: Planet,
}

pub trait NewPlanet {
    fn new(radius: f32, meshes: ResMut<Assets<Mesh>>, materials: ResMut<Assets<ColorMaterial>>) -> Self;
}

impl NewPlanet for PlanetBundle {
    fn new(radius: f32, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) -> Self {
        Self {
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Circle { radius: radius })),
                material: materials.add(Color::hsl(1., 1., 1.)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            planet: Planet { radius: radius },
        }
    }
}
