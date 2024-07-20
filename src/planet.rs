
use bevy::{
    math::VectorSpace, prelude::*, reflect::Array, render::{
        mesh::CircleMeshBuilder,
        render_resource::{AsBindGroup, ShaderRef, ShaderType},
    }, scene::ron::de, sprite::{Material2d, MaterialMesh2dBundle, Mesh2dHandle}
};

use crate::{looping_float::LoopingFloat, planet_sticker::PlanetSticker};

#[derive(Component, PartialEq)]
pub struct PlanetWater {
}

#[derive(Component, Default)]
pub struct Planet {
    pub radius: f32,
    pub navigation_zone: NavigationZone
}

#[derive(Default)]
pub struct NavigationZone {
    domain: Vec2,
    children: Option<Box<(NavigationZone, NavigationZone)>>
}

pub trait IsAccessible {
    fn is_accessible(&self, start: f32, end: f32) -> Option<i32>;
}

impl IsAccessible for NavigationZone {
    fn is_accessible(&self, start: f32, end: f32) -> Option<i32> {
        if start < self.domain.x || start > self.domain.y || end < self.domain.x || end > self.domain.y {
            return None;
        }
        if self.children.is_none() {
            //return Some()
        }
        return None;
    }
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
                material: planet_materials.add(PlanetMaterial { settings: PlanetSettings { 
                    hole_array: [Vec4::splat(0.); 8]
                }}),
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            planet: Planet { radius: radius, navigation_zone: NavigationZone { domain: Vec2::new(0., 360.), ..default() } },
        }
    }
}

pub struct PlanetsPlugin;

impl Plugin for PlanetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Planets::default())
        .add_systems(Update, (update_water, update_nav));
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


fn update_nav(
    mut planets_query: Query<Entity, With<Planet>>,
    waters_query: Query<&PlanetSticker, With<PlanetWater>>,
) {
    for planet in planets_query.iter_mut() {
        let mut nav: NavigationZone = NavigationZone { domain: Vec2::new(0., 360.), ..default() };
        for water in waters_query.iter() {
            let Some(water_planet) = water.planet else { continue; };
            let Some(water_size) = water.size_degrees else { continue; };
            if planet != water_planet { continue; };
            let split = Vec2::new(water.position_degrees.to_f32() - water_size / 2., water.position_degrees.to_f32() + water_size / 2.);
            if nav.domain == Vec2::new(0., 360.) {
                nav.domain = Vec2::new(split.y, split.x);
                continue;
            }
            add_split(&mut nav, split);
        }
        //print_nav(&nav, 0);
    }
}

fn add_split(mut nav: &mut NavigationZone, split: Vec2) {
 /* if split.x < nav.domain.x || split.x > nav.domain.y || split.y < nav.domain.x || split.y > nav.domain.y {
        println!("{} {}", nav.domain, split);
        return
    }
     */
   // let a = !(LoopingFloat::<360>::new(nav.domain.x).is_in_between(split.x, nav.domain.y));
    //let b = !(LoopingFloat::<360>::new(nav.domain.x).is_in_between(split.y, nav.domain.y));
    //if a || b {
     //   return;
   // }
    if let Some(ref mut children) = nav.children {
        add_split(&mut children.0, split);
        add_split(&mut children.1, split);
        return
    }
    else {
        let child1 = NavigationZone { domain: Vec2::new(nav.domain.x, split.x), children: None };
        let child2 = NavigationZone { domain: Vec2::new(split.y, nav.domain.y ), children: None };
        nav.children = Some(Box::new((child1, child2))); 
    }
}

fn print_nav(nav: &NavigationZone, depth: i32) {
    println!("{}Domain: {}", "  ".repeat(depth as usize), nav.domain);
    if let Some(ref children) = nav.children {
        print_nav(&children.0, depth + 1);
        print_nav(&children.1, depth + 1);
    }
}