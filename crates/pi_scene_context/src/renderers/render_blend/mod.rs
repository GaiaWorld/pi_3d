use std::mem::replace;

use pi_engine_shell::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct ModelBlend {
    pub enable: bool,
    pub src_color: BlendFactor,
    pub dst_color: BlendFactor,
    pub src_alpha: BlendFactor,
    pub dst_alpha: BlendFactor,
    pub opt_color: BlendOperation,
    pub opt_alpha: BlendOperation,
}
impl Default for ModelBlend {
    fn default() -> Self {
        Self {
            enable: false,
            src_color: BlendFactor::One,
            dst_color: BlendFactor::OneMinusSrcAlpha,
            src_alpha: BlendFactor::One,
            dst_alpha: BlendFactor::OneMinusSrcAlpha,
            opt_color: BlendOperation::Add,
            opt_alpha: BlendOperation::Add,
        }
    }
}
impl ModelBlend {
    pub fn combine(&mut self) {
        self.enable = true;
    }
    pub fn one_one() -> Self {
        Self {
            enable: true,
            src_color: BlendFactor::One,
            dst_color: BlendFactor::One,
            src_alpha: BlendFactor::One,
            dst_alpha: BlendFactor::One,
            opt_color: BlendOperation::Add,
            opt_alpha: BlendOperation::Add,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum OpsRenderBlend {
    Disable(Entity),
    Blend(Entity, ModelBlend),
}
impl OpsRenderBlend {
    pub fn ops(mesh: Entity, mode: ModelBlend) -> Self {
        Self::Blend(mesh, mode)
    }
}

pub type ActionListBlend = ActionList<OpsRenderBlend>;
pub fn sys_act_model_blend(
    mut cmds: ResMut<ActionListBlend>,
    mut meshes: Query<&mut ModelBlend>,
) {
    cmds.drain().drain(..).for_each(|cmd| {
        match cmd {
            OpsRenderBlend::Disable(_) => todo!(),
            OpsRenderBlend::Blend(entity, value) => {
                if let Ok(mut mode) = meshes.get_mut(entity) {
                    *mode = value;
                } else {
                    cmds.push(OpsRenderBlend::Blend(entity, value));
                }
            },
        }
    });
}

// #[derive(Debug, Default)]
// pub struct SingleRenderBlendCommandList {
//     pub list: Vec<ERenderBlendCommand>,
// }

// pub struct SysRenderBlendCommand;
// impl TSystemStageInfo for SysRenderBlendCommand {

// }
// #[setup]
// impl SysRenderBlendCommand {
//     #[system]
//     pub fn cmd(
//         mut cmds: ResMut<SingleRenderBlendCommandList>,
//         mut items: Query<GameObject, &mut ModelBlend>,
//         mut blends: Commands<GameObject, ModelBlend>,
//     ) {
//         let mut list = replace(&mut cmds.list, vec![]);
//         list.drain(..).for_each(|cmd| {
//             match cmd {
//                 ERenderBlendCommand::Disable(entity) => {
//                     if let Some(mut item) = items.get_mut(entity) {
//                         item.enable = false;
//                     } else {
//                         blends.insert(entity, ModelBlend::default());
//                     }
//                 },
//                 ERenderBlendCommand::Blend(entity, value) => {
//                     blends.insert(entity, value);
//                 },
//             }
//         });
//     }
// }

// pub trait InterfaceRenderBlend {
//     fn blend(
//         &self,
//         entity: ObjectID,
//         blend: ModelBlend,
//     ) -> &Self;

//     fn disable_blend(
//         &self,
//         entity: ObjectID
//     ) -> &Self;
// }
// impl InterfaceRenderBlend for crate::engine::Engine {
//     fn blend(
//         &self,
//         entity: ObjectID,
//         value: ModelBlend,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleRenderBlendCommandList>().unwrap();
//         commands.list.push(ERenderBlendCommand::Blend(entity, value));

//         self
//     }

//     fn disable_blend(
//         &self,
//         entity: ObjectID
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleRenderBlendCommandList>().unwrap();
//         commands.list.push(ERenderBlendCommand::Disable(entity));

//         self
//     }
// }

// pub struct PluginRenderBlend;
// impl crate::Plugin for PluginRenderBlend {
//     fn init(
//         &mut self,
//         engine: &mut crate::engine::Engine,
//         stages: &mut crate::run_stage::RunStage,
//     ) -> Result<(), crate::plugin::ErrorPlugin> {
//         let world = engine.world_mut();

//         world.insert_resource(SingleRenderBlendCommandList::default());

//         SysRenderBlendCommand::setup(world, stages.query_stage::<SysRenderBlendCommand>(ERunStageChap::Initial));

//         Ok(())
//     }
// }