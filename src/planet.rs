
use bevy::{
    math::VectorSpace, prelude::*, reflect::Array, render::{
        mesh::CircleMeshBuilder,
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
    }, scene::ron::de, sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle, Mesh2dHandle}
};

use crate::{looping_float::LoopingFloat, planet_sticker::PlanetSticker};

#[derive(Component, PartialEq)]
pub struct PlanetWater {
}

#[derive(Component, Default)]
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
                mesh: Mesh2dHandle(meshes.add(Rectangle{half_size: Vec2::splat(radius)})),
                material: planet_materials.add(PlanetMaterial { settings: PlanetSettings { 
                    hole_array: [Vec4::splat(0.); 8]
                }}),
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
        app.insert_resource(Planets::default())
        .add_plugins(Material2dPlugin::<PlanetMaterial>::default())
        .add_systems(Update, (update_water));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
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

fn update_water(
    mut planets: Query<(Entity, &Handle<PlanetMaterial>), With<Planet>>,
    waters_query: Query<&PlanetSticker, With<PlanetWater>>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
) {
    for (planet, handle) in planets.iter_mut() {
        if let Some(material) = materials.get_mut(handle) {
            let mut waters: [Vec4; 8] = [Vec4::splat(0.); 8];
            let mut index = 0;
            for water_sticker in waters_query.iter() {
                let Some(water_planet) = water_sticker.planet else { continue; };
                let Some(size) = water_sticker.size_degrees else { continue; };
                if water_planet != planet {continue;};
                waters[index] = Vec4::new(water_sticker.position_degrees.to_f32() - size / 2., water_sticker.position_degrees.to_f32() + size / 2., 0., 0.);
                index += 1;
                if index == 8 {break;};
            }
            material.settings.hole_array = waters;
        }
    }
}