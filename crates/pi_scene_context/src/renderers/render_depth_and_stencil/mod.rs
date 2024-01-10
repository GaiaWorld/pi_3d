

use pi_bevy_render_plugin::constant::render_state::{CompareFunction, StencilFaceState};
use pi_scene_shell::prelude::*;
use pi_render::renderer::pipeline::{DepthStencilState, DepthBiasState};

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct DepthWrite(pub bool);
impl Default for DepthWrite {
    fn default() -> Self {
        Self(true)
    }
}
pub struct OpsDepthWrite(pub(crate) Entity, pub(crate) DepthWrite, pub(crate) u16);
impl OpsDepthWrite {
    pub fn ops(mesh: Entity, val: bool) -> Self {
        Self(mesh, DepthWrite(val), 0)
    }
}
pub type ActionListDepthWrite = ActionList<OpsDepthWrite>;

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct DepthCompare(pub CompareFunction);
impl Default for DepthCompare {
    fn default() -> Self {
        Self(CompareFunction::LessEqual)
    }
}
pub struct OpsDepthCompare(pub(crate) Entity, pub(crate) DepthCompare, pub(crate) u16);
impl OpsDepthCompare {
    pub fn ops(mesh: Entity, val: CompareFunction) -> Self {
        Self(mesh, DepthCompare(val), 0)
    }
}
pub type ActionListDepthCompare = ActionList<OpsDepthCompare>;

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct DepthBias(pub DepthBiasState);
impl Default for DepthBias {
    fn default() -> Self {
        Self(
            DepthBiasState {
                constant: 0,
                slope_scale: 0,
                clamp: 0,
            }
        )
    }
}

pub struct OpsDepthBias(pub(crate) Entity, pub(crate) DepthBias, pub(crate) u16);
impl OpsDepthBias {
    pub fn ops(mesh: Entity, constant: i32, slope_scale: i32, clamp: i32) -> Self {
        Self(mesh, DepthBias(DepthBiasState {
            constant,
            slope_scale,
            clamp,
        }), 0)
    }
}
pub type ActionListDepthBias = ActionList<OpsDepthBias>;

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct StencilFront(pub StencilFaceState);
impl Default for StencilFront {
    fn default() -> Self {
        Self(StencilFaceState::IGNORE)
    }
}
pub struct OpsStencilFront(pub(crate) Entity, pub(crate) StencilFront, pub(crate) u16);
impl OpsStencilFront {
    pub fn ops(
        mesh: Entity,
        compare: CompareFunction,
        fail_op: StencilOperation,
        depth_fail_op: StencilOperation,
        pass_op: StencilOperation,
    ) -> Self {
        Self(mesh, StencilFront(StencilFaceState { compare, fail_op, depth_fail_op, pass_op }), 0)
    }
}
pub type ActionListStencilFront = ActionList<OpsStencilFront>;

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct StencilBack(pub StencilFaceState);
impl Default for StencilBack {
    fn default() -> Self {
        Self(StencilFaceState::IGNORE)
    }
}
pub struct OpsStencilBack(pub(crate) Entity, pub(crate) StencilBack, pub(crate) u16);
impl OpsStencilBack {
    pub fn ops(
        mesh: Entity,
        compare: CompareFunction,
        fail_op: StencilOperation,
        depth_fail_op: StencilOperation,
        pass_op: StencilOperation,
    ) -> Self {
        Self(mesh, StencilBack(StencilFaceState { compare, fail_op, depth_fail_op, pass_op }), 0)
    }
}
pub type ActionListStencilBack = ActionList<OpsStencilBack>;

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct StencilRead(pub u32);
impl Default for StencilRead {
    fn default() -> Self {
        Self(0)
    }
}
pub struct OpsStencilRead(pub(crate) Entity, pub(crate) StencilRead, pub(crate) u16);
impl OpsStencilRead {
    pub fn ops(mesh: Entity, val: u32) -> Self {
        Self(mesh, StencilRead(val), 0)
    }
}
pub type ActionListStencilRead = ActionList<OpsStencilRead>;

#[derive(Debug, Clone, Component, Deref, DerefMut)]
pub struct StencilWrite(pub u32);
impl Default for StencilWrite {
    fn default() -> Self {
        Self(0)
    }
}
pub struct OpsStencilWrite(pub(crate) Entity, pub(crate) StencilWrite, pub(crate) u16);
impl OpsStencilWrite {
    pub fn ops(mesh: Entity, val: u32) -> Self {
        Self(mesh, StencilWrite(val), 0)
    }
}
pub type ActionListStencilWrite = ActionList<OpsStencilWrite>;

pub fn depth_stencil_state(
    format: wgpu::TextureFormat,
    depth_write: &DepthWrite,
    compare: &DepthCompare,
    bias: &DepthBias,
    stencil_front: &StencilFront,
    stencil_back: &StencilBack,
    stencil_read: &StencilRead,
    stencil_write: &StencilWrite,
) -> DepthStencilState {
    DepthStencilState {
        format,
        depth_write_enabled: depth_write.0,
        depth_compare: compare.0.val2(),
        stencil: wgpu::StencilState {
            front: stencil_front.val(),
            back: stencil_back.val(),
            read_mask: stencil_read.0,
            write_mask: stencil_write.0,
        },
        bias: bias.0,
    }
}

pub fn sys_act_depth_write(
    mut cmds: ResMut<ActionListDepthWrite>,
    mut items: Query<&mut DepthWrite>,
) {
    cmds.drain().drain(..).for_each(|OpsDepthWrite(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsDepthWrite(entity, val, count + 1));
        }
    });
}

pub fn sys_act_depth_compare(
    mut cmds: ResMut<ActionListDepthCompare>,
    mut items: Query<&mut DepthCompare>,
) {
    cmds.drain().drain(..).for_each(|OpsDepthCompare(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsDepthCompare(entity, val, count + 1));
        }
    });
}

pub fn sys_act_depth_bias(
    mut cmds: ResMut<ActionListDepthBias>,
    mut items: Query<&mut DepthBias>,
) {
    cmds.drain().drain(..).for_each(|OpsDepthBias(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsDepthBias(entity, val, count + 1));
        }
    });
}

pub fn sys_act_stencil_front(
    mut cmds: ResMut<ActionListStencilFront>,
    mut items: Query<&mut StencilFront>,
) {
    cmds.drain().drain(..).for_each(|OpsStencilFront(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsStencilFront(entity, val, count + 1));
        }
    });
}


pub fn sys_act_stencil_back(
    mut cmds: ResMut<ActionListStencilBack>,
    mut items: Query<&mut StencilBack>,
) {
    cmds.drain().drain(..).for_each(|OpsStencilBack(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsStencilBack(entity, val, count + 1));
        }
    });
}


pub fn sys_act_stencil_read(
    mut cmds: ResMut<ActionListStencilRead>,
    mut items: Query<&mut StencilRead>,
) {
    cmds.drain().drain(..).for_each(|OpsStencilRead(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsStencilRead(entity, val, count + 1));
        }
    });
}
pub fn sys_act_stencil_write(
    mut cmds: ResMut<ActionListStencilWrite>,
    mut items: Query<&mut StencilWrite>,
) {
    cmds.drain().drain(..).for_each(|OpsStencilWrite(entity, val, count)| {
        if let Ok(mut item) = items.get_mut(entity) {
            *item = val;
            return;
        }

        if count < ACTION_WAIT_FRAME {
            cmds.push(OpsStencilWrite(entity, val, count + 1));
        }
    });
}