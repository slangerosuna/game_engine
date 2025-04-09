use crate::ecs::world::World;

/**
 * This is a trait for systems in the ECS
 *
 * It allows for systems to be run on the world
 * **It is not intended for the user to implement**
 * It will be implemented already by any system you
 * create
 */
pub trait System: Send + Sync {
    /**
     * This function is only to be called by the
     * scheduler. It is not intended to be called
     * directly by the user.
     */
    unsafe fn run(&mut self, world: *mut World);
}
