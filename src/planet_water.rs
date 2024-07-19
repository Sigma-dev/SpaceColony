use bevy::{asset::Asset, prelude::Component, reflect::TypePath, render::render_resource::{AsBindGroup, ShaderRef}, sprite::Material2d};

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct WaterPlanetMaterial {
}

impl Material2d for WaterPlanetMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/water_planet.wgsl".into()
    }
} 

#[derive(Component, PartialEq)]
pub struct PlanetWater {
}