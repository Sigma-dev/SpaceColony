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
    pub resources: SpaceResources,
    pub max_amount: u32,
}

impl Storage {
    pub fn new(max_amount: u32) -> Storage {
        Storage { resources: SpaceResources::new(), max_amount }
    }

    pub fn get_amount(&self, resource: SpaceResource) -> u32 {
        self.resources.get_amount(resource)
    }

    pub fn remove_many(&mut self, resources: SpaceResources) -> SpaceResources {
        let mut not_removed = SpaceResources::new();
        for (k, v) in resources.iter() {
            let remaining = self.remove(*k, *v);
            if remaining > 0 {
                not_removed.insert(*k, remaining);
            }
        }
        not_removed
    }

    pub fn remove(&mut self, resource: SpaceResource, amount: u32) -> u32 {
        let stored = self.resources.get(&resource).unwrap_or(&0);
        let remaining = amount - stored.min(&amount);
        self.resources.insert(resource, stored - (amount - remaining));
        remaining
    }

    pub fn add(&mut self, resource: SpaceResource, amount: u32) {
        let stored = *self.resources.get(&resource).unwrap_or(&0);
        self.resources.insert(resource, stored + amount);
    }

    pub fn add_many(&mut self, resources: SpaceResources) {
        for (k, v) in resources.iter() {
            self.add(*k, *v);
        }
    }

    pub fn get_stored_amount(&self) -> u32 {
        self.resources.values().sum()
    }
}