use core::f32;
use std::f32::consts::PI;

use approx::AbsDiffEq;
use bevy::{ecs::system::{lifetimeless::{Read, Write}, SystemParam}, prelude::*, utils::HashMap};
use rand::Rng;

use crate::{looping_float::{self, LoopingFloat}, natural_resource::{self, NaturalResource}, planet::Planet, planet_placing::{PlanetPlacingGhost, UpdateSelection}, planet_sticker::{PlanetCollider, PlanetSticker}, storage::{SpaceResource, SpaceResources, SpaceResourcesTrait, Storage}};

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
            Option<Write<Storage>>,
            Option<Read<NaturalResource>>,
        ),
        (
            Without<PlanetPlacingGhost>,
            Without<Planet>,
        ),
    >,
    #[doc(hidden)]
    pub commands: Commands<'w, 's>,
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
        for (_, other_sticker, other_collider, _, _) in self.stickers_query.iter() {
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
        let [(_, s1, c1, _, _), (_, s2, c2, _, _)] = self.stickers_query.get_many([e1, e2]).unwrap();
        
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
        for (_, storage_sticker, _, maybe_storage, _) in self.stickers_query.iter() {
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

    pub fn try_place<T: Bundle>(&mut self, bundle: T, sticker: PlanetSticker, collider: PlanetCollider) -> Result<Entity, ()> {
        let sc = StickerCollider { sticker, collider };
        if self.overlaps_anything(sc) {
            return Err(());
        }
        let id = self.commands
        .spawn((bundle, sticker, collider))
        .observe(|trigger: Trigger<Pointer<Click>>, mut select_event: EventWriter<UpdateSelection>|{
            select_event.send(UpdateSelection::new(Some(trigger.entity())));
        }).id();
        Ok(id)
    }

    pub fn remove_resources(
        &mut self,
        planet: Entity,
        resources: SpaceResources
    ) {
        let mut remaining = resources.clone();
        for (_, storage_sticker, _, maybe_storage, _) in self.stickers_query.iter_mut() {
            if storage_sticker.planet != planet { continue; }
            if resources.is_empty() {
                return;
            }
            if let Some(mut storage) = maybe_storage {
                remaining = storage.remove_many(remaining);
            }
        }
        if !remaining.is_empty() {
            println!("{:?}", remaining);
            panic!("Remaining resources")
        }
    }
    
    pub fn get_natural_resource_in_range(&self, sticker: PlanetSticker, range: f32, resource: SpaceResource) -> Vec<Entity> {
        let (_, _, planet) = self.planet_query.get(sticker.planet).unwrap();
        let mut result = Vec::new();
        for (resource_entity, resource_sticker, _, _, maybe_natural_resource) in self.stickers_query.iter() {
            if maybe_natural_resource.map_or(true, |n| n.produced_resource != resource) { continue; }
            if sticker.arc_distance_to(*resource_sticker, planet.radius).unwrap_or(f32::MAX) <= range {
                result.push(resource_entity);
            }
        }
        result
    }
}