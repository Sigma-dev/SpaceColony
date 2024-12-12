use bevy::{prelude::*, utils::HashMap};

#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug)]
pub enum SpaceResource {
    Wood
}

impl std::fmt::Display for SpaceResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub type SpaceResources = HashMap<SpaceResource, u32>;

pub trait SpaceResourcesTrait {
    fn get_amount(&self, resource: SpaceResource) -> u32;
    fn contains(&self, resources: &SpaceResources) -> bool;
    fn combine(&self, resources: &SpaceResources) -> SpaceResources;
}

impl SpaceResourcesTrait for SpaceResources {
    fn contains(&self, resources: &SpaceResources) -> bool {
        for (resource, amount)  in resources.iter() {
            if self.get_amount(*resource) < *amount {
                return false
            }
        }
        true
    }

    fn combine(&self, resources: &SpaceResources) -> SpaceResources {
        let mut result: SpaceResources = self.clone();
        for (resource, amount)  in resources.iter() {
            result.insert(*resource, result.get_amount(*resource) + amount);
        }
        result
    }

    fn get_amount(&self, resource: SpaceResource) -> u32 {
        *self.get(&resource).unwrap_or(&0)
    }
}

#[derive(Component)]
pub struct Storage {
    pub resources: SpaceResources
}

impl Storage {
    pub fn get_amount(&self, resource: SpaceResource) -> u32 {
        self.resources.get_amount(resource)
    }
}