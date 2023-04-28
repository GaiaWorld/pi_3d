use std::mem::replace;

use pi_engine_shell::prelude::*;


#[derive(Debug, Clone, Copy)]
pub enum TransparentCommand {
    Apply(),
    Undo(),
}

pub struct ActionRenderTransparent;
impl ActionRenderTransparent {
    pub fn modify(
        commands: &mut EntityCommands,
        val: TransparentCommand,
    ) {
        match val {
            TransparentCommand::Apply() => {
                meshes.insert(Transparent);
            },
            TransparentCommand::Undo() => {
                meshes.remove::<Transparent>();
            },
        }
    }
}

#[derive(Debug, Default, Resource)]
pub struct SingleTransparentCommandList {
    list: Vec<TransparentCommand>
}

pub struct SysTransparentCommandTick;
impl TSystemStageInfo for SysTransparentCommandTick {

}
#[setup]
impl SysTransparentCommandTick {
    #[system]
    pub fn tick(
        cmds: ResMut<SingleTransparentCommandList>,
        mut meshes: Commands<GameObject, Transparent>,
    ) {
        let list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                TransparentCommand::Apply(mesh) => {
                    meshes.insert(mesh, Transparent);
                },
                TransparentCommand::Undo(mesh) => {
                    meshes.delete(mesh);
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

        SysTransparentCommandTick::setup(engine.world_mut(), stages.query_stage::<SysTransparentCommandTick>());

        Ok(())
    }
}