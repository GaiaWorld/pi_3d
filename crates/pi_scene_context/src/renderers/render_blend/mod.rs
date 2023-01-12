use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Setup, Commands}, };
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, ERunStageChap};

use crate::object::{ObjectID, GameObject};

#[derive(Debug, Clone, Copy)]
pub struct RenderBlend {
    pub enable: bool,
    pub src_color: wgpu::BlendFactor,
    pub dst_color: wgpu::BlendFactor,
    pub src_alpha: wgpu::BlendFactor,
    pub dst_alpha: wgpu::BlendFactor,
    pub opt_color: wgpu::BlendOperation,
    pub opt_alpha: wgpu::BlendOperation,
}
impl Default for RenderBlend {
    fn default() -> Self {
        Self {
            enable: false,
            src_color: wgpu::BlendFactor::One,
            dst_color: wgpu::BlendFactor::OneMinusSrcAlpha,
            src_alpha: wgpu::BlendFactor::One,
            dst_alpha: wgpu::BlendFactor::OneMinusSrcAlpha,
            opt_color: wgpu::BlendOperation::Add,
            opt_alpha: wgpu::BlendOperation::Add,
        }
    }
}
impl RenderBlend {
    pub fn combine(&mut self) {
        self.enable = true;
    }
    pub fn one_one() -> Self {
        Self {
            enable: true,
            src_color: wgpu::BlendFactor::One,
            dst_color: wgpu::BlendFactor::One,
            src_alpha: wgpu::BlendFactor::One,
            dst_alpha: wgpu::BlendFactor::One,
            opt_color: wgpu::BlendOperation::Add,
            opt_alpha: wgpu::BlendOperation::Add,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ERenderBlendCommand {
    Disable(ObjectID),
    Blend(ObjectID, RenderBlend),
}

#[derive(Debug, Default)]
pub struct SingleRenderBlendCommandList {
    pub list: Vec<ERenderBlendCommand>,
}

pub struct SysRenderBlendCommand;
impl TSystemStageInfo for SysRenderBlendCommand {

}
#[setup]
impl SysRenderBlendCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderBlendCommandList>,
        mut items: Query<GameObject, &mut RenderBlend>,
        mut blends: Commands<GameObject, RenderBlend>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);
        list.drain(..).for_each(|cmd| {
            match cmd {
                ERenderBlendCommand::Disable(entity) => {
                    if let Some(mut item) = items.get_mut(entity) {
                        item.enable = false;
                    } else {
                        blends.insert(entity, RenderBlend::default());
                    }
                },
                ERenderBlendCommand::Blend(entity, value) => {
                    blends.insert(entity, value);
                },
            }
        });
    }
}

pub trait InterfaceRenderBlend {
    fn blend(
        &self,
        entity: ObjectID,
        blend: RenderBlend,
    ) -> &Self;

    fn disable_blend(
        &self,
        entity: ObjectID
    ) -> &Self;
}
impl InterfaceRenderBlend for crate::engine::Engine {
    fn blend(
        &self,
        entity: ObjectID,
        value: RenderBlend,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleRenderBlendCommandList>().unwrap();
        commands.list.push(ERenderBlendCommand::Blend(entity, value));

        self
    }

    fn disable_blend(
        &self,
        entity: ObjectID
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleRenderBlendCommandList>().unwrap();
        commands.list.push(ERenderBlendCommand::Disable(entity));

        self
    }
}

pub struct PluginRenderBlend;
impl crate::Plugin for PluginRenderBlend {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleRenderBlendCommandList::default());

        SysRenderBlendCommand::setup(world, stages.query_stage::<SysRenderBlendCommand>(ERunStageChap::Command));

        Ok(())
    }
}