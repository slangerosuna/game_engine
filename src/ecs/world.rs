use super::{Entity, Resource, mappings::Mapping, system::System};

pub struct World {
    entities: Vec<Entity>,
    resources: Vec<Option<Box<dyn Resource>>>,
    mappings: Vec<Option<Box<dyn Mapping>>>,
    systems: Vec<Box<dyn System>>,
}
