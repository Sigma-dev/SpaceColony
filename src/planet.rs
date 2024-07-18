use bevy::{
    prelude::*,
    render::{mesh::CircleMeshBuilder, render_resource::{AsBindGroup, ShaderRef}},
    sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle},
};

#[derive(Component)]
pub struct Planet {
    pub radius: f32,
}

#[derive(Bundle)]
pub struct PlanetBundle {
    pub mesh: MaterialMesh2dBundle<PlanetMaterial>,
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
        meshes: &mut ResMut<Assets<Mesh>>,
        planet_materials: ResMut<Assets<PlanetMaterial>>,
    ) -> Self;
}

impl NewPlanet for PlanetBundle {
    fn new(
        radius: f32,
        meshes: &mut ResMut<Assets<Mesh>>,
        mut planet_materials: ResMut<Assets<PlanetMaterial>>,
    ) -> Self {
        Self {
            mesh: MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(CircleMeshBuilder {
                    circle: Circle { radius },
                    resolution: 64,
                })),
                material: planet_materials.add(PlanetMaterial { }),
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

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
} 