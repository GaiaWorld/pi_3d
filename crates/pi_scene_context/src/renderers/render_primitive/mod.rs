

use pi_scene_shell::prelude::*;

/// * 默认值 Back
#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct CCullMode(pub CullMode);
pub struct OpsCullMode(pub(crate) Entity, pub(crate) CCullMode, pub u16);
impl OpsCullMode {
    pub fn ops(mesh: Entity, mode: CullMode) -> Self {
        Self(mesh, CCullMode(mode), 0)
    }
}
pub type ActionListCullMode = ActionList<OpsCullMode>;
pub fn sys_act_mesh_cull_mode(
    mut cmds: ResMut<ActionListCullMode>,
    mut items: Query<&mut CCullMode>,
) {
    cmds.drain().drain(..).for_each(|OpsCullMode(entity, mode, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = mode;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsCullMode(entity, mode, count + 1));
        }
    });
}

// #[derive(Debug, Clone, Copy)]
/// * 默认值 Fill
#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct CPolygonMode(pub PolygonMode);
pub struct OpsPolygonMode(pub(crate) Entity, pub(crate) CPolygonMode, pub u16);
impl OpsPolygonMode {
    pub fn ops(mesh: Entity, mode: PolygonMode) -> Self {
        Self(mesh, CPolygonMode(mode), 0)
    }
}
pub type ActionListPolyginMode = ActionList<OpsPolygonMode>;
pub fn sys_act_mesh_polygon_mode(
    mut cmds: ResMut<ActionListPolyginMode>,
    mut items: Query<&mut CPolygonMode>,
) {
    cmds.drain().drain(..).for_each(|OpsPolygonMode(entity, mode, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = mode;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsPolygonMode(entity, mode, count + 1));
        }
    });
}

// #[derive(Debug, Clone, Copy)]
/// * 默认值 Fill
#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct Topology(pub PrimitiveTopology);
pub struct OpsTopology(pub(crate) Entity, pub(crate) Topology, pub u16);
impl OpsTopology {
    pub fn ops(mesh: Entity, mode: PrimitiveTopology) -> Self {
        Self(mesh, Topology(mode), 0)
    }
}
pub type ActionListTopology = ActionList<OpsTopology>;
pub fn sys_act_mesh_topolygon(
    mut cmds: ResMut<ActionListTopology>,
    mut items: Query<&mut Topology>,
) {
    cmds.drain().drain(..).for_each(|OpsTopology(entity, mode, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = mode;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsTopology(entity, mode, count + 1));
        }
    });
}

// #[derive(Debug, Clone, Copy)]
/// * 默认值 Fill
#[derive(Debug, Clone, Copy, Component)]
pub struct CUnClipDepth(pub bool);
pub struct OpsUnClipDepth(pub(crate) Entity, pub(crate) bool, pub u16);
impl OpsUnClipDepth {
    pub fn ops(mesh: Entity, mode: bool) -> Self {
        Self(mesh, mode, 0)
    }
}
pub type ActionListUnClipDepth = ActionList<OpsUnClipDepth>;
pub fn sys_act_mesh_unclip_depth(
    mut cmds: ResMut<ActionListUnClipDepth>,
    mut items: Query<&mut CUnClipDepth>,
) {
    cmds.drain().drain(..).for_each(|OpsUnClipDepth(entity, mode, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = CUnClipDepth(mode);
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsUnClipDepth(entity, mode, count + 1));
        }
    });
}

/// * 默认值 Ccw
#[derive(Debug, Clone, Copy, Component, Deref, DerefMut)]
pub struct CFrontFace(pub FrontFace);
pub struct OpsFrontFace(pub(crate) Entity, pub(crate) CFrontFace, pub u16);
impl OpsFrontFace {
    pub fn ops(mesh: Entity, mode: FrontFace) -> Self {
        Self(mesh, CFrontFace(mode), 0)
    }
}
pub type ActionListFrontFace = ActionList<OpsFrontFace>;
pub fn sys_act_mesh_frontface(
    mut cmds: ResMut<ActionListFrontFace>,
    mut items: Query<&mut CFrontFace>,
) {
    cmds.drain().drain(..).for_each(|OpsFrontFace(entity, mode, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = mode;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsFrontFace(entity, mode, count + 1));
        }
    });
}

#[derive(Debug, Clone, Copy, Component)]
pub struct PrimitiveState {
    // pub state: wgpu::PrimitiveState,
}
impl PrimitiveState {
    pub fn state(cull: &CCullMode, topology: &Topology, polygon: &CPolygonMode, face: &CFrontFace, unclip_depth: &CUnClipDepth) -> wgpu::PrimitiveState {
        wgpu::PrimitiveState {
            topology: topology.val(),
            front_face: face.val(),
            polygon_mode: polygon.val(),
            cull_mode: cull.val(),
            // 不设置可能渲染出来黑的
            unclipped_depth: unclip_depth.0,
            ..Default::default()
        }
    }
    // pub fn new(cull: &ECullMode, polygon: &PolygonMode, face: &FrontFace) -> Self {
    //     Self {
    //         state: wgpu::PrimitiveState {
    //             topology: wgpu::PrimitiveTopology::TriangleList,
    //             front_face: face.val(),
    //             polygon_mode: polygon.val(),
    //             cull_mode: cull.mode(),
    //             // 不设置可能渲染出来黑的
    //             #[cfg(not(target_arch = "wasm32"))]
    //             unclipped_depth: true,
    //             ..Default::default()
    //         }
    //     }
    // }
}

#[derive(Debug, Clone, Copy)]
pub enum ERenderPrimitiveCommand {
    CullMode(CCullMode),
    PolygonMode(CPolygonMode),
    FrontFace(CFrontFace),
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