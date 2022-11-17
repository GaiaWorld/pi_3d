use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;

use crate::object::{ObjectID, GameObject};


#[derive(Debug, Clone, Copy)]
pub enum TransparentCommand {
    Apply(ObjectID),
    Undo(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleTransparentCommandList {
    list: Vec<TransparentCommand>
}

pub struct SysTransparentCommandTick;
#[setup]
impl SysTransparentCommandTick {
    #[system]
    pub fn tick(
        cmds: ResMut<SingleTransparentCommandList>,
        meshes: Query<GameObject, Write<Transparent>>,
    ) {
        let list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                TransparentCommand::Apply(mesh) => {
                    match meshes.get_mut(mesh) {
                        Some(mesh) => {
                            mesh.insert_no_notify(Transparent);
                        },
                        _ => {

                        }
                    }
                },
                TransparentCommand::Undo(mesh) => {
                    match meshes.get_mut(mesh) {
                        Some(mesh) => {
                            mesh.remove();
                        },
                        _ => {

                        }
                    }
                },
            }
        });
    }
}

pub trait InterfaceTransparent {
    fn as_transparent(
        &self,
        entity: ObjectID,
    ) -> &Self;

    fn not_transparent(
        &self,
        entity: ObjectID,
    ) -> &Self;
}

impl InterfaceTransparent for crate::engine::Engine {
    fn as_transparent(
        &self,
        entity: ObjectID,
    ) -> &Self {
        let cmomands = self.world().get_resource_mut::<SingleTransparentCommandList>().unwrap();
        cmomands.list.push(TransparentCommand::Apply(entity));

        self
    }

    fn not_transparent(
        &self,
        entity: ObjectID,
    ) -> &Self {
        let cmomands = self.world().get_resource_mut::<SingleTransparentCommandList>().unwrap();
        cmomands.list.push(TransparentCommand::Undo(entity));

        self
    }
}

pub struct PluginTransparent;
impl crate::Plugin for PluginTransparent {
    fn init(
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        engine.world_mut().insert_resource(SingleTransparentCommandList::default());

        SysTransparentCommandTick::setup(engine.world_mut(), stages.command_stage());

        Ok(())
    }
}