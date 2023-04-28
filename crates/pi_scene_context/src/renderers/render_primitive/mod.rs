use std::mem::replace;

use pi_engine_shell::prelude::*;

use crate::{scene::coordinate_system::ESceneCoordinateMode};

/// * 默认值 Back
#[derive(Debug, Clone, Copy, Component)]
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
pub struct OpsCullMode(Entity, ECullMode);
impl OpsCullMode {
    pub fn ops(mesh: Entity, mode: ECullMode) -> Self {
        Self(mesh, mode)
    }
}
pub type ActionListCullMode = ActionList<OpsCullMode>;
pub fn sys_act_mesh_cull_mode(
    mut cmds: ResMut<ActionListCullMode>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsCullMode(entity, mode)| {
        commands.entity(entity).insert(mode);
    });
}

// #[derive(Debug, Clone, Copy)]
/// * 默认值 Fill
#[derive(Debug, Clone, Copy, Component)]
pub enum PolygonMode {
    Fill = 0,
    /// Polygons are drawn as line segments
    Line = 1,
    /// Polygons are drawn as points
    Point = 2,
}
impl PolygonMode {
    pub fn val(&self) -> wgpu::PolygonMode {
        match self {
            PolygonMode::Fill => wgpu::PolygonMode::Fill,
            PolygonMode::Line => wgpu::PolygonMode::Line,
            PolygonMode::Point => wgpu::PolygonMode::Point,
        }
    }
}
pub struct OpsPolygonMode(Entity, PolygonMode);
impl OpsPolygonMode {
    pub fn ops(mesh: Entity, mode: PolygonMode) -> Self {
        Self(mesh, mode)
    }
}
pub type ActionListPolyginMode = ActionList<OpsPolygonMode>;
pub fn sys_act_mesh_polygon_mode(
    mut cmds: ResMut<ActionListPolyginMode>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsPolygonMode(entity, mode)| {
        commands.entity(entity).insert(mode);
    });
}

/// * 默认值 Ccw
#[derive(Debug, Clone, Copy, Component)]
pub enum FrontFace {
    Ccw = 0,
    /// Triangles with vertices in clockwise order are considered the front face.
    ///
    /// This is the default with left handed coordinate spaces.
    Cw = 1,
}
impl FrontFace {
    pub fn val(&self) -> wgpu::FrontFace {
        match self {
            FrontFace::Ccw => wgpu::FrontFace::Ccw,
            FrontFace::Cw => wgpu::FrontFace::Cw,
        }
    }
}
pub struct OpsFrontFace(Entity, FrontFace);
impl OpsFrontFace {
    pub fn ops(mesh: Entity, mode: FrontFace) -> Self {
        Self(mesh, mode)
    }
}
pub type ActionListFrontFace = ActionList<OpsFrontFace>;
pub fn sys_act_mesh_frontface(
    mut cmds: ResMut<ActionListFrontFace>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsFrontFace(entity, mode)| {
        commands.entity(entity).insert(mode);
    });
}

#[derive(Debug, Clone, Copy, Component)]
pub struct PrimitiveState {
    pub state: wgpu::PrimitiveState,
}
impl PrimitiveState {
    pub fn state(cull: &ECullMode, polygon: &PolygonMode, face: &FrontFace) -> wgpu::PrimitiveState {
        wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            front_face: face.val(),
            polygon_mode: polygon.val(),
            cull_mode: cull.mode(),
            // 不设置可能渲染出来黑的
            unclipped_depth: true,
            ..Default::default()
        }
    }
    pub fn new(cull: &ECullMode, polygon: &PolygonMode, face: &FrontFace) -> Self {
        Self {
            state: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                front_face: face.val(),
                polygon_mode: polygon.val(),
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
    CullMode(ECullMode),
    PolygonMode(PolygonMode),
    FrontFace(FrontFace),
}

pub struct ActionRenderPrimitive;
impl ActionRenderPrimitive {
    pub fn modify(
        commands: &mut EntityCommands,
        val: ERenderPrimitiveCommand,
    ) {
        match val {
            ERenderPrimitiveCommand::CullMode(value) => {
                commands.insert(value);
            },
            ERenderPrimitiveCommand::PolygonMode(value) => {
                commands.insert(value);
            },
            ERenderPrimitiveCommand::FrontFace(value) => {
                commands.insert(value);
            },
        }
    }
}

// #[derive(Debug, Default)]
// pub struct SingleRenderPrimitiveCreateCommandList {
//     pub list: Vec<ERenderPrimitiveCommand>,
// }

// pub struct SysRenderPrimitiveCreateCommand;
// impl TSystemStageInfo for SysRenderPrimitiveCreateCommand {

// }
// #[setup]
// impl SysRenderPrimitiveCreateCommand {
//     #[system]
//     pub fn cmd(
//         mut cmds: ResMut<SingleRenderPrimitiveCreateCommandList>,
//         mut cullmode: Commands<GameObject, ECullMode>,
//         mut polygon: Commands<GameObject, EPolygonMode>,
//         mut front: Commands<GameObject, EFrontFace>,
//     ) {
//         let mut list = replace(&mut cmds.list, vec![]);

//         list.drain(..).for_each(|cmd| {
//             match cmd {
//                 ERenderPrimitiveCommand::CullMode(entity, value) => {
//                     cullmode.insert(entity, value);
//                 },
//                 ERenderPrimitiveCommand::PolygonMode(entity, value) => {
//                     polygon.insert(entity, value);
//                 },
//                 ERenderPrimitiveCommand::FrontFace(entity, value) => {
//                     front.insert(entity, value);
//                 },
//             }
//         });
//     }
// }

// pub struct SysRenderPrimitiveCommand;
// impl TSystemStageInfo for SysRenderPrimitiveCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysRenderPrimitiveCreateCommand::key()
//         ]
//     }
// }
// #[setup]
// impl SysRenderPrimitiveCommand {
//     #[system]
    pub fn sys_render_primitive_modify(
        mut item: Query<(ObjectID, &ECullMode, &PolygonMode, &FrontFace), Or<(Changed<ECullMode>, Changed<PolygonMode>, Changed<FrontFace>)>>,
        mut commands: Commands,
    ) {
        item.iter().for_each(|(id_obj, cull, polygon, front)| {
            commands.entity(id_obj).insert(PrimitiveState::new(cull, polygon, front));
        });
    }
// }

// pub trait InterfaceRenderPrimitive {
//     fn cull_mode(
//         &self,
//         entity: ObjectID,
//         value: ECullMode,
//     ) -> &Self;
    
//     fn polygon_mode(
//         &self,
//         entity: ObjectID,
//         value: EPolygonMode,
//     ) -> &Self;
    
//     fn front_face(
//         &self,
//         entity: ObjectID,
//         value: EFrontFace,
//     ) -> &Self;
// }

// impl InterfaceRenderPrimitive for crate::engine::Engine {
//     fn cull_mode(
//         &self,
//         entity: ObjectID,
//         value: ECullMode,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleRenderPrimitiveCreateCommandList>().unwrap();
//         commands.list.push(ERenderPrimitiveCommand::CullMode(entity, value));

//         self
//     }
    
//     fn polygon_mode(
//         &self,
//         entity: ObjectID,
//         value: EPolygonMode,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleRenderPrimitiveCreateCommandList>().unwrap();
//         commands.list.push(ERenderPrimitiveCommand::PolygonMode(entity, value));

//         self
//     }
    
//     fn front_face(
//         &self,
//         entity: ObjectID,
//         value: EFrontFace,
//     ) -> &Self {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleRenderPrimitiveCreateCommandList>().unwrap();
//         commands.list.push(ERenderPrimitiveCommand::FrontFace(entity, value));

//         self
//     }
// }

// pub struct PluginRenderPrimitive;
// impl crate::Plugin for PluginRenderPrimitive {
//     fn init(
//         &mut self,
//         engine: &mut crate::engine::Engine,
//         stages: &mut crate::run_stage::RunStage,
//     ) -> Result<(), crate::plugin::ErrorPlugin> {
//         let world = engine.world_mut();

//         world.insert_resource(SingleRenderPrimitiveCreateCommandList::default());

//         SysRenderPrimitiveCreateCommand::setup(world, stages.query_stage::<SysRenderPrimitiveCreateCommand>(ERunStageChap::Initial));
//         SysRenderPrimitiveCommand::setup(world, stages.query_stage::<SysRenderPrimitiveCommand>(ERunStageChap::Initial));

//         Ok(())
//     }
// }