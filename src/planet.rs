use bevy::{
    math::VectorSpace, prelude::*, reflect::Array, render::{
        mesh::CircleMeshBuilder,
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
    }, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
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
    pub all: Vec<Entity>,
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
                material: planet_materials.add(PlanetMaterial { settings: PlanetSettings { hole_array: [
                    Vec4::new(45., 90., 0., 0.),
                    Vec4::new(135., 190., 0., 0.),
                    Vec4::splat(0.),
                    Vec4::splat(0.),
                    Vec4::splat(0.),
                    Vec4::splat(0.),
                    Vec4::splat(0.),
                    Vec4::splat(0.),
                ] }}),
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
    // Parameters to the planet shader bound to uniform 0.
    #[uniform(0)]
    settings: PlanetSettings,
}

#[derive(ShaderType, Debug, Clone)]
struct PlanetSettings {
    hole_array: [Vec4; 8],
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}

fn update_planets(
    mut bars: Query<(&Handle<PlanetMaterial>, &Planet)>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
) {
    for (handle, resource_text) in bars.iter_mut() {
        if let Some(material) = materials.get_mut(handle) {

        }
    }
}
