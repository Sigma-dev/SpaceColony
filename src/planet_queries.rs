use std::f32::consts::PI;

use approx::AbsDiffEq;
use bevy::{ecs::system::{lifetimeless::Read, SystemParam}, prelude::*, utils::HashMap};
use rand::Rng;

use crate::{looping_float::{self, LoopingFloat}, planet::Planet, planet_placing::PlanetPlacingGhost, planet_sticker::{PlanetCollider, PlanetSticker}, storage::{SpaceResource, SpaceResources, SpaceResourcesTrait, Storage}};

#[derive(SystemParam)]
pub struct PlanetQueries<'w, 's> {
    #[doc(hidden)]
    pub planet_query: Query<
        'w,
        's,
        (
            Entity,
            Read<Transform>,
            Read<Planet>,
        ),
        Without<PlanetSticker>
    >,
    #[doc(hidden)]
    pub stickers_query: Query<
        'w,
        's,
        (
            Entity,
            Read<PlanetSticker>,
            Read<PlanetCollider>,
            Option<Read<Storage>>
        ),
        (
            Without<PlanetPlacingGhost>,
            Without<Planet>,
        ),
    >,
}

pub struct StickerCollider {
    pub sticker: PlanetSticker,
    pub collider: PlanetCollider,
}

impl StickerCollider {
    pub fn is_colliding_with(&self, other: &StickerCollider) -> bool {
        self.sticker.position_degrees.distance(other.sticker.position_degrees.to_f32()) < self.collider.size_degrees / 2. + other.collider.size_degrees / 2.
    }
}

pub struct ClosestResult {
    pub planet: Entity,
    pub pos_degrees: f32,
    pub distance: f32,
}

impl<'w, 's> PlanetQueries<'w, 's> {
    pub fn overlaps_anything(
        &self,
        sc: StickerCollider
    ) -> bool {
        let (planet_entity, _planet_transform, _planet) = self.planet_query.get(sc.sticker.planet).unwrap();
        for (_, other_sticker, other_collider, _) in self.stickers_query.iter() {
            if other_sticker.planet != planet_entity { continue; }
            if sc.is_colliding_with(&StickerCollider { sticker: *other_sticker, collider: *other_collider }) {
                return true
            }
        }
        return false;
    }

    pub fn get_random_valid_placement(
        &self,
        planet: Entity,
        size_degrees: f32,
        max_tries: u32,
    ) -> Option<f32> {
        for _i in 0..max_tries {
            let pos = rand::thread_rng().gen_range(0.0..360.);
            let sc = StickerCollider { 
                sticker: PlanetSticker::new(planet, pos),
                collider: PlanetCollider::new(size_degrees)
            };
            if !self.overlaps_anything(sc) {
                return Some(pos)
            }
        }
        None
    }

    pub fn find_closest_surface(
        &self,
        pos: Vec2
    ) -> Option<ClosestResult> {
        let mut best: Option<ClosestResult> = None;
        for (planet_entity, planet_transform, planet, ) in self.planet_query.iter() {
            let planet_pos_2d = planet_transform.translation.xy();
            let dist = pos.distance(planet_pos_2d) - planet.radius;
            let diff = pos - planet_pos_2d;
            let up = planet_transform.up().xy();
            let angle = diff.angle_to(up).to_degrees();
            best = Some(ClosestResult { planet: planet_entity, pos_degrees: angle, distance: dist });
        }
        return best;
    }

    pub fn are_colliding(
        &self,
        e1: Entity,
        e2: Entity,
    ) -> bool {
        let [(_, s1, c1, _), (_, s2, c2, _)] = self.stickers_query.get_many([e1, e2]).unwrap();
        
        StickerCollider {
            sticker: *s1,
            collider: *c1
        }.is_colliding_with(&StickerCollider {
            sticker: *s2,
            collider: *c2
        })
    }

    pub fn get_resources_on_planet(
        &self,
        planet: Entity,
    ) -> SpaceResources {
        let mut resources = SpaceResources::new();
        for (_, storage_sticker, _, maybe_storage) in self.stickers_query.iter() {
            if storage_sticker.planet != planet { continue; }
            if let Some(storage) = maybe_storage {
                resources = resources.combine(&storage.resources)
            }
        }
        resources
    }

    pub fn can_afford_on_planet(
        &self,
        planet: Entity,
        resources: SpaceResources,
    ) -> bool {
        self.get_resources_on_planet(planet).contains(&resources)
    }
}