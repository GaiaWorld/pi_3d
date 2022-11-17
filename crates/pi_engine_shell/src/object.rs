use pi_ecs::{prelude::Id, world::World};

#[derive(Debug, Clone, Copy, Default)]
pub struct GameObject;

pub type ObjectID = Id<GameObject>;

pub trait InterfaceObject {
    fn new_object(
        &self
    ) -> ObjectID;
}

impl InterfaceObject for crate::engine_shell::EnginShell {
    fn new_object(
        &self
    ) -> ObjectID {
        unsafe { 
            let const_ptr = self.world() as *const World;
            let world: &mut World = &mut *(const_ptr as *mut World);
            ObjectID::new(world.archetypes_mut()[self.node_archetype_id()].reserve_entity()) 
        }
    }
}