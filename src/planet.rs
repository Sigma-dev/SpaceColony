
use bevy::{
    prelude::*, 
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType}, sprite::{Material2d, Material2dPlugin},
};

use crate::planet_sticker::PlanetSticker;

#[derive(Component, PartialEq)]
pub struct PlanetWater {
}

#[derive(Component, Default)]
pub struct Planet {
    pub radius: f32,
}

#[derive(Resource, Default)]
pub struct Planets {
    pub main: Option<Entity>,
    pub all: Vec<Entity>,
}

pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Planets::default())
        .add_plugins(Material2dPlugin::<PlanetMaterial>::default())
        .add_systems(Update, update_water);
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct PlanetMaterial {
    #[uniform(0)]
    pub settings: PlanetSettings,
}

impl Material2d for PlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/planet.wgsl".into()
    }
}

#[derive(ShaderType, Debug, Clone)]
pub struct PlanetSettings {
    pub hole_array: [Vec4; 8],
}


fn update_water(
    mut planets: Query<(Entity, &MeshMaterial2d<PlanetMaterial>), With<Planet>>,
    waters_query: Query<&PlanetSticker, With<PlanetWater>>,
    mut materials: ResMut<Assets<PlanetMaterial>>,
) {
    for (planet, handle) in planets.iter_mut() {
        if let Some(material) = materials.get_mut(handle.id()) {
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