use bevy::prelude::*;

use crate::looping_float::LoopingFloat;
use crate::planet::Planet;

#[derive(Component, Clone, Copy)]
pub struct PlanetSticker {
    pub planet: Entity,
    pub position_degrees: LoopingFloat<360>,
}

#[derive(Component, Clone, Copy)]
pub struct PlanetCollider {
    pub size_degrees: f32,
}

impl PlanetCollider {
    pub fn new(size_degrees: f32) -> PlanetCollider {
        PlanetCollider { size_degrees }
    }
}

impl PlanetSticker {
    pub fn new(planet: Entity, position_degrees: f32) -> PlanetSticker {
        PlanetSticker { planet, position_degrees: LoopingFloat::new(position_degrees) }
    }

    pub fn to_segment(&self, width: f32) -> Vec2 {
        Vec2::new((self.position_degrees - width / 2.0).to_f32(), (self.position_degrees + width / 2.0).to_f32())
    }

    pub fn arc_distance(&self, pos: f32, planet: Entity, radius: f32, ) -> Option<f32> {
        if self.planet != planet { return None }
        let dist_degrees = self.position_degrees.distance(pos);
        return Some(radius * dist_degrees.to_radians());
    }

    pub fn arc_distance_to(&self, sticker: PlanetSticker, radius: f32, ) -> Option<f32> {
        if self.planet != sticker.planet { return None }
        let dist_degrees = self.position_degrees.distance(sticker.position_degrees.to_f32());
        return Some(radius * dist_degrees.to_radians());
    }
}

pub struct PlanetStickerPlugin;

impl Plugin for PlanetStickerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, stick_to_planet);
    }
}

fn stick_to_planet(
    mut sticker_query: Query<(&mut Transform, &PlanetSticker)>,
    targets: Query<(&GlobalTransform, &Planet)>,
) {
    for (mut transform, sticker) in sticker_query.iter_mut() {
        if let Ok((planet_transform, planet)) = targets.get(sticker.planet) {
            let center = planet_transform.translation();
            let pos_rad = sticker.position_degrees.to_f32().to_radians();
            let sink = 0.75;
            transform.translation.x = center.x + pos_rad.sin() * (planet.radius - sink);
            transform.translation.y = center.y + pos_rad.cos() * (planet.radius - sink);

            let direction = Vec2::new(pos_rad.sin(), pos_rad.cos());
            let rotation = Quat::from_rotation_arc(
                Vec3::Y,
                Vec3::new(direction.x, direction.y, 0.0).normalize(),
            );
            transform.rotation = rotation;
        }
    }
}
