use std::cmp::min;

use bevy::prelude::*;

use crate::looping_float::LoopingFloat;
use crate::planet::Planet;

#[derive(Component, Default)]
pub struct PlanetSticker {
    pub planet: Option<Entity>,
    pub position_degrees: LoopingFloat<360>,
    pub size_degrees: Option<f32>,
}

pub trait IsCollidingWith {
    fn is_colliding_with(&self, other: &PlanetSticker) -> bool;
    fn is_colliding_with_pos(&self, other_pos: f32, other_size: f32) -> bool;
}

impl IsCollidingWith for PlanetSticker {
    fn is_colliding_with(&self, other: &PlanetSticker) -> bool {
        let Some(planet) = self.planet else { return false };
        let Some(other_planet) = other.planet else { return false };
        let Some(other_size) = other.size_degrees else { return false };
        let Some(size) = self.size_degrees else { return false };
        if planet != other_planet {
                return false;
        }
        if (size > other_size) {
            return self.is_colliding_with_pos(other.position_degrees.to_f32(), other_size)
        } else {
            return other.is_colliding_with_pos(self.position_degrees.to_f32(), size)
        }
    }

    fn is_colliding_with_pos(&self, other_pos: f32, other_size: f32) -> bool {
        let Some(size) = self.size_degrees else { return false };
        let left_self = self.position_degrees - size / 2.;
        let right_self = self.position_degrees + size / 2.;
        let pos_self = Vec2::new((self.position_degrees - size / 2.).to_f32(), (self.position_degrees + size / 2.).to_f32());
        let pos_other = Vec2::new(other_pos - other_size / 2., other_pos + other_size / 2.);
        /* 
        if pos_self.x < pos_other.x && pos_self.y > pos_other.x {
            return true;
        }
        if pos_self.x < pos_other.y && pos_self.y > pos_other.y {
            return true;
        }
        */
        if (left_self.is_in_between(pos_other.x, right_self.to_f32(), true)) {
            return true;
        }
        if (left_self.is_in_between(pos_other.y, right_self.to_f32(), true)) {
            return true;
        }
        return false;
    }
}

pub trait Contains {
    fn contains(&self, other: f32) -> bool;
}

impl Contains for PlanetSticker {
    fn contains(&self, pos: f32) -> bool {
        let Some(size) = self.size_degrees else { return false; };
        if self.position_degrees.distance(pos) > size / 2. {
            return false;
        }
        return true;
    }
}

pub trait EdgeDistanceTo {
    fn edge_distance_to(&self, pos: f32) -> f32;
}

impl EdgeDistanceTo for PlanetSticker {
    fn edge_distance_to(&self, pos: f32) -> f32 {
        let size = self.size_degrees.unwrap_or(0.);
        let left_pos = self.position_degrees - size / 2.;
        let right_pos = self.position_degrees + size / 2.;
        return left_pos.distance(pos).min(right_pos.distance(pos));
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
        if let Some(planet) = sticker.planet {
            if let Ok((planet_transform, planet)) = targets.get(planet) {
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
}
