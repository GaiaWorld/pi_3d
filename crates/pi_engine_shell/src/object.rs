use std::mem::replace;

use pi_ecs::{prelude::{Id, ResMut, EntityDelete, StageBuilder, Setup}, world::World};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTreeMut;
use pi_slotmap_tree::Storage;

#[derive(Debug, Clone, Copy, Default)]
pub struct GameObject;

pub type ObjectID = Id<GameObject>;

pub trait InterfaceObject {
    fn new_object(
        &self
    ) -> ObjectID;
    fn remove_object(
        &self,
        id: ObjectID,
    ) -> &Self;
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

    fn remove_object(
        &self,
        id: ObjectID,
    ) -> &Self {
        let world = self.world();

        world.get_resource_mut::<SingleObjectCommand>().unwrap().0.push(id);

        self
    }
}

#[derive(Debug, Default)]
pub(crate) struct SingleObjectCommand(Vec<ObjectID>);

struct SysObject;
#[setup]
impl SysObject {
    #[system]
    pub fn sys(
        mut cmds: ResMut<SingleObjectCommand>,
        mut tree: EntityTreeMut<GameObject>,
        mut delete: EntityDelete<GameObject>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|id| {
            let mut begin = 0;
            let mut end = 1;
            let mut count = 1;
            let mut loopcount = 0;
            let mut temp = vec![];
            temp.push(id);

            loop {
                if count == 0 || loopcount >= 65535 {
                    break;
                }
                count = 0;
                loopcount += 1;

                for i in begin..end {
                    if let Some(item) = tree.get_down(temp.get(i).unwrap().clone()) {
                        tree.iter(item.head).for_each(|item| {
                            count += 1;
                            temp.push(item);
                        });
                    }
                }
                begin = end;
                end = begin + count;
            }



            loop {
                if let Some(id) = temp.pop() {
                    delete.despawn(id);
                } else {
                    break;
                }
            }

            tree.remove(id);
        });
    }
}

pub struct PluginObject;
impl PluginObject {
    pub fn init(
        &mut self,
        world: &mut World,
        stage: &mut StageBuilder,
    ) {
        world.insert_resource(SingleObjectCommand::default());
        
        SysObject::setup(world, stage);
    }
}