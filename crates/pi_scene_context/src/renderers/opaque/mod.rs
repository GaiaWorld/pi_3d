use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;

use crate::object::{ObjectID, GameObject};


#[derive(Debug, Clone, Copy)]
pub struct Opaque;

#[derive(Debug, Clone, Copy)]
pub enum OpaqueCommand {
    Apply(ObjectID),
    Undo(ObjectID),
}

#[derive(Debug, Default)]
pub struct SingleOpaqueCommandList {
    list: Vec<OpaqueCommand>
}

pub struct SysOpaqueCommandTick;
#[setup]
impl SysOpaqueCommandTick {
    #[system]
    pub fn tick(
        mut cmds: ResMut<SingleOpaqueCommandList>,
        mut meshes: Query<GameObject, Write<Opaque>>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                OpaqueCommand::Apply(mesh) => {
                    match meshes.get_mut(mesh) {
                        Some(mut mesh) => {
                            mesh.insert_no_notify(Opaque);
                        },
                        _ => {

                        }
                    }
                },
                OpaqueCommand::Undo(mesh) => {
                    match meshes.get_mut(mesh) {
                        Some(mut mesh) => {
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

pub trait InterfaceOpaque {
    fn as_opaque(
        &self,
        entity: ObjectID,
    ) -> &Self;

    fn not_opaque(
        &self,
        entity: ObjectID,
    ) -> &Self;
}

impl InterfaceOpaque for crate::engine::Engine {
    fn as_opaque(
        &self,
        entity: ObjectID,
    ) -> &Self {
        let cmomands = self.world().get_resource_mut::<SingleOpaqueCommandList>().unwrap();
        cmomands.list.push(OpaqueCommand::Apply(entity));

        self
    }

    fn not_opaque(
        &self,
        entity: ObjectID,
    ) -> &Self {
        let cmomands = self.world().get_resource_mut::<SingleOpaqueCommandList>().unwrap();
        cmomands.list.push(OpaqueCommand::Undo(entity));

        self
    }
}

pub struct PluginOpaque;
impl crate::Plugin for PluginOpaque {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleOpaqueCommandList::default());

        SysOpaqueCommandTick::setup(world, stages.command_stage());

        Ok(())
    }
}