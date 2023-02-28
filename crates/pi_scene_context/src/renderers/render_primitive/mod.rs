use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Setup, Commands, Query}, query::{Or, Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::{TSystemStageInfo, ERunStageChap};

use crate::{object::{ObjectID, GameObject}, scene::coordinate_system::ESceneCoordinateMode};

/// * 默认值 Back
#[derive(Debug, Clone, Copy)]
pub enum ECullMode {
    Off,
    Back,
    Front
}
impl ECullMode {
    pub fn mode(&self) -> Option<wgpu::Face> {
        match self {
            ECullMode::Off => None,
            ECullMode::Back => Some(wgpu::Face::Back),
            ECullMode::Front => Some(wgpu::Face::Front),
        }
    }
}

// #[derive(Debug, Clone, Copy)]
/// * 默认值 Fill
pub type EPolygonMode = wgpu::PolygonMode;

/// * 默认值 Ccw
pub type EFrontFace = wgpu::FrontFace;

#[derive(Debug, Clone, Copy)]
pub struct PrimitiveState {
    pub state: wgpu::PrimitiveState,
}
impl PrimitiveState {
    pub fn state(cull: &ECullMode, polygon: &EPolygonMode, face: &EFrontFace) -> wgpu::PrimitiveState {
        wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: face.clone(),
            polygon_mode: polygon.clone(),
            cull_mode: cull.mode(),
            // 不设置可能渲染出来黑的
            unclipped_depth: true,
            ..Default::default()
        }
    }
    pub fn new(cull: &ECullMode, polygon: &EPolygonMode, face: &EFrontFace) -> Self {
        Self {
            state: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: face.clone(),
                polygon_mode: polygon.clone(),
                cull_mode: cull.mode(),
                // 不设置可能渲染出来黑的
                unclipped_depth: true,
                ..Default::default()
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ERenderPrimitiveCommand {
    CullMode(ObjectID, ECullMode),
    PolygonMode(ObjectID, EPolygonMode),
    FrontFace(ObjectID, EFrontFace),
}

#[derive(Debug, Default)]
pub struct SingleRenderPrimitiveCreateCommandList {
    pub list: Vec<ERenderPrimitiveCommand>,
}

pub struct SysRenderPrimitiveCreateCommand;
impl TSystemStageInfo for SysRenderPrimitiveCreateCommand {

}
#[setup]
impl SysRenderPrimitiveCreateCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleRenderPrimitiveCreateCommandList>,
        mut cullmode: Commands<GameObject, ECullMode>,
        mut polygon: Commands<GameObject, EPolygonMode>,
        mut front: Commands<GameObject, EFrontFace>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                ERenderPrimitiveCommand::CullMode(entity, value) => {
                    cullmode.insert(entity, value);
                },
                ERenderPrimitiveCommand::PolygonMode(entity, value) => {
                    polygon.insert(entity, value);
                },
                ERenderPrimitiveCommand::FrontFace(entity, value) => {
                    front.insert(entity, value);
                },
            }
        });
    }
}

pub struct SysRenderPrimitiveCommand;
impl TSystemStageInfo for SysRenderPrimitiveCommand {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysRenderPrimitiveCreateCommand::key()
        ]
    }
}
#[setup]
impl SysRenderPrimitiveCommand {
    #[system]
    fn sys(
        mut item: Query<GameObject, (ObjectID, &ECullMode, &EPolygonMode, &EFrontFace), Or<(Changed<ECullMode>, Changed<EPolygonMode>, Changed<EFrontFace>)>>,
        mut cmd: Commands<GameObject, PrimitiveState>
    ) {
        item.iter().for_each(|(id_obj, cull, polygon, front)| {
            cmd.insert(id_obj, PrimitiveState::new(cull, polygon, front));
        });
    }
}

pub trait InterfaceRenderPrimitive {
    fn cull_mode(
        &self,
        entity: ObjectID,
        value: ECullMode,
    ) -> &Self;
    
    fn polygon_mode(
        &self,
        entity: ObjectID,
        value: EPolygonMode,
    ) -> &Self;
    
    fn front_face(
        &self,
        entity: ObjectID,
        value: EFrontFace,
    ) -> &Self;
}

impl InterfaceRenderPrimitive for crate::engine::Engine {
    fn cull_mode(
        &self,
        entity: ObjectID,
        value: ECullMode,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleRenderPrimitiveCreateCommandList>().unwrap();
        commands.list.push(ERenderPrimitiveCommand::CullMode(entity, value));

        self
    }
    
    fn polygon_mode(
        &self,
        entity: ObjectID,
        value: EPolygonMode,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleRenderPrimitiveCreateCommandList>().unwrap();
        commands.list.push(ERenderPrimitiveCommand::PolygonMode(entity, value));

        self
    }
    
    fn front_face(
        &self,
        entity: ObjectID,
        value: EFrontFace,
    ) -> &Self {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleRenderPrimitiveCreateCommandList>().unwrap();
        commands.list.push(ERenderPrimitiveCommand::FrontFace(entity, value));

        self
    }
}

pub struct PluginRenderPrimitive;
impl crate::Plugin for PluginRenderPrimitive {
    fn init(
        &mut self,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        world.insert_resource(SingleRenderPrimitiveCreateCommandList::default());

        SysRenderPrimitiveCreateCommand::setup(world, stages.query_stage::<SysRenderPrimitiveCreateCommand>(ERunStageChap::Initial));
        SysRenderPrimitiveCommand::setup(world, stages.query_stage::<SysRenderPrimitiveCommand>(ERunStageChap::Initial));

        Ok(())
    }
}