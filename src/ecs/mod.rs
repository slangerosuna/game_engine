pub mod mappings;
pub mod scheduler;
pub mod system;
pub mod task;
pub mod world;

use typeid::ConstTypeId;

use std::any::Any;
use std::collections::HashMap;
use std::sync::OnceLock;

pub use peano_derive::Component;
pub use peano_derive::Resource;

/**
 * This is a trait for components in the ECS
 *
 * It allows for components to be queried by Systems
 * An Entity can have multiple components of the same type
 */
pub trait Component: Any {
    fn get_type_id(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/**
 * This is a trait for resources in the ECS
 *
 * It allows for resources to be queried by Systems
 * You cannot have multiple resources of the same type
 */
pub trait Resource: Any {
    fn get_type_id(&self) -> usize;
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/**
 * This is a struct to handle the registration inventory
 *
 * It is intended to be used by the `Component` derive macro
 * It is not intended to be used directly
 */
pub struct ComponentRegistration {
    pub type_id: ConstTypeId,
    pub name: &'static str,
}

/**
 * This is a struct to handle the registration inventory
 *
 * It is intended to be used by the `Resource` derive macro
 * It is not intended to be used directly
 */
pub struct ResourceRegistration {
    pub type_id: ConstTypeId,
    pub name: &'static str,
}

inventory::collect!(ComponentRegistration);
inventory::collect!(ResourceRegistration);

static COMPONENT_IDS: OnceLock<HashMap<ConstTypeId, usize>> = OnceLock::new();
static RESOURCE_IDS: OnceLock<HashMap<ConstTypeId, usize>> = OnceLock::new();

fn build_component_ids() -> HashMap<ConstTypeId, usize> {
    let mut entries: Vec<_> = inventory::iter::<ComponentRegistration>
        .into_iter()
        .collect();
    entries.sort_by_key(|e| e.name);
    entries
        .into_iter()
        .enumerate()
        .map(|(i, r)| (r.type_id, i))
        .collect()
}

fn build_resource_ids() -> HashMap<ConstTypeId, usize> {
    let mut entries: Vec<_> = inventory::iter::<ResourceRegistration>
        .into_iter()
        .collect();
    entries.sort_by_key(|e| e.name);
    entries
        .into_iter()
        .enumerate()
        .map(|(i, r)| (r.type_id, i))
        .collect()
}

/**
 * Returns the component ID for the given type
 *
 * This function is for use by the `Component` derive macro.
 * It is not intended to be used directly.
 */
pub fn get_component_id<T: 'static>() -> usize {
    *COMPONENT_IDS
        .get_or_init(build_component_ids)
        .get(&ConstTypeId::of::<T>())
        .expect("Component not registered")
}

/**
 * Returns the resource ID for the given type
 *
 * This function is for use by the `Resource` derive macro.
 * It is not intended to be used directly.
 */
pub fn get_resource_id<T: 'static>() -> usize {
    *RESOURCE_IDS
        .get_or_init(build_resource_ids)
        .get(&ConstTypeId::of::<T>())
        .expect("Resource not registered")
}

/**
 * Represents an entity in the ECS.
 *
 * Each entity has a unique ID and a list of components.
 *
 */
pub struct Entity {
    pub id: u32,
    pub(crate) components: Vec<Option<Box<dyn Component>>>,
}

impl Entity {
    pub(crate) fn new(id: u32) -> Self {
        Self {
            id,
            components: Vec::with_capacity(COMPONENT_IDS.get().unwrap().len()),
        }
    }

    /**
     * Sets a component for the entity.
     *
     * Drops the previous component if it exists.
     */
    pub fn set_component(&mut self, component: Option<Box<dyn Component>>, id: usize) {
        self.components[id] = component;
    }

    /**
     * Adds a component to the entity.
     *
     * If the component is already present, it returns None.
     */
    pub fn add_component(&mut self, component: Box<dyn Component>) -> Option<()> {
        let id = component.get_type_id();

        if self.components[id].is_none() {
            self.components[id] = Some(component);
            Some(())
        } else {
            None
        }
    }

    /**
     * Gets a component from the entity.
     *
     * If the component is not present, it returns None.
     */
    pub fn get_component<T: Component>(&self) -> Option<&T> {
        let id = get_component_id::<T>() as usize;
        self.components[id]
            .as_ref()
            .and_then(|c| c.as_any().downcast_ref::<T>())
    }

    /**
     * Gets a mutable component from the entity.
     *
     * If the component is not present, it returns None.
     */
    pub fn get_component_mut<T: Component>(&mut self) -> Option<&mut T> {
        let id = get_component_id::<T>() as usize;
        self.components[id]
            .as_mut()
            .and_then(|c| c.as_any_mut().downcast_mut::<T>())
    }

    /**
     * Removes a component from the entity.
     *
     * If the component is not present, it returns None.
     */
    pub fn remove_component<T: Component>(&mut self) -> Option<Box<dyn Component>> {
        let id = get_component_id::<T>() as usize;
        self.components[id].take()
    }

    /**
     * Checks if the entity has a component.
     *
     * If the component is not present, it returns false.
     */
    pub fn has_component<T: Component>(&self) -> bool {
        let id = get_component_id::<T>() as usize;
        self.components[id].is_some()
    }
}
