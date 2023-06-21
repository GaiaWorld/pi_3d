

use pi_engine_shell::prelude::*;


#[derive(Debug, Clone, Copy, Component)]
pub struct Opaque;

#[derive(Debug, Clone, Copy)]
pub enum OpaqueCommand {
    Apply(),
    Undo(),
}

pub struct ActionRenderOpaque;
impl ActionRenderOpaque {
    pub fn modify(
        commands: &mut EntityCommands,
        val: OpaqueCommand,
    ) {
        match val {
            OpaqueCommand::Apply() => {
                commands.insert(Opaque);
            },
            OpaqueCommand::Undo() => {
                commands.remove::<Opaque>();
            },
        }
    }
}

// #[derive(Debug, Default)]
// pub struct SingleOpaqueCommandList {
//     list: Vec<OpaqueCommand>
// }

// pub struct SysOpaqueCommandTick;
// impl TSystemStageInfo for SysOpaqueCommandTick {

// }
// #[setup]
// impl SysOpaqueCommandTick {
//     #[system]
//     pub fn tick(
//         mut cmds: ResMut<SingleOpaqueCommandList>,
//         mut meshes: Commands<GameObject, Opaque>,
//     ) {
//         let mut list = replace(&mut cmds.list, vec![]);

//         list.drain(..).for_each(|cmd| {
//             match cmd {
//                 OpaqueCommand::Apply(mesh) => {
//                     meshes.insert(mesh, Opaque);
//                 },
//                 OpaqueCommand::Undo(mesh) => {
//                     meshes.delete(mesh);
//                 },
//             }
//         });
//     }
// }

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

// impl InterfaceOpaque for crate::engine::Engine {
//     fn as_opaque(
//         &self,
//         entity: ObjectID,
//     ) -> &Self {
//         let cmomands = self.world().get_resource_mut::<SingleOpaqueCommandList>().unwrap();
//         cmomands.list.push(OpaqueCommand::Apply(entity));

//         self
//     }

//     fn not_opaque(
//         &self,
//         entity: ObjectID,
//     ) -> &Self {
//         let cmomands = self.world().get_resource_mut::<SingleOpaqueCommandList>().unwrap();
//         cmomands.list.push(OpaqueCommand::Undo(entity));

//         self
//     }
// }

// pub struct PluginOpaque;
// impl crate::Plugin for PluginOpaque {
//     fn init(
//         &mut self,
//         engine: &mut crate::engine::Engine,
//         stages: &mut crate::run_stage::RunStage,
//     ) -> Result<(), crate::plugin::ErrorPlugin> {
//         let world = engine.world_mut();

//         world.insert_resource(SingleOpaqueCommandList::default());

//         SysOpaqueCommandTick::setup(world, stages.query_stage::<SysOpaqueCommandTick>(ERunStageChap::Initial));

//         Ok(())
//     }
// }