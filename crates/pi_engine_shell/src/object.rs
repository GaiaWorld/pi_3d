use std::mem::replace;
use pi_bevy_ecs_extend::prelude::EntityTreeMut;
use crate::prelude::*;

#[derive(Debug, Clone, Copy, Default)]
pub struct GameObject;

pub type ObjectID = Entity;

pub trait InterfaceObject {
    fn new_object(
        &mut self
    ) -> ObjectID;
    fn remove_object(
        &mut self,
        id: ObjectID,
    ) -> &Self;
}

impl InterfaceObject for crate::engine_shell::EnginShell {
    fn new_object(
        &mut self
    ) -> ObjectID {
        self.world.spawn_empty().id()
    }

    fn remove_object(
        &mut self,
        id: ObjectID,
    ) -> &Self {
        self.world.despawn(id);

        self
    }
}

#[derive(Debug, Default, Resource)]
pub struct SingleObjectCommand(Vec<ObjectID>);

pub fn sys_object(
    mut cmds: ResMut<SingleObjectCommand>,
    mut tree: EntityTreeMut,
    mut delete: Commands,
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
                    tree.iter(item.head.0).for_each(|item| {
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
                if let Some(mut cmd) = delete.get_entity(id) {
                    cmd
                    .despawn();
                }
            } else {
                break;
            }
        }

        tree.remove(id);
    });
}

pub struct PluginObject;
impl Plugin for PluginObject {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.world.insert_resource(SingleObjectCommand::default());
        
        app.add_systems(Update, sys_object);
    }
}