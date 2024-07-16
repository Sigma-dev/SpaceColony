use bevy::{
    prelude::*,
    render::mesh::CircleMeshBuilder,
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

#[derive(Resource, Default)]
pub struct Planets {
    pub main: Option<Entity>,
    pub all: Vec<Entity>
}

pub trait NewPlanet {
    fn new(
        radius: f32,
        meshes: ResMut<Assets<Mesh>>,
        materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self;
}

impl NewPlanet for PlanetBundle {
    fn new(
        radius: f32,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(CircleMeshBuilder {
                    circle: Circle { radius },
                    resolution: 64,
                })),
                material: materials.add(Color::hsl(1., 1., 1.)),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            planet: Planet { radius: radius },
        }
    }
}

pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Planets::default());
    }
}
