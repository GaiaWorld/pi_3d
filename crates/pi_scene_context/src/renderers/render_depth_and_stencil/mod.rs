

use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;

use super::*;

#[derive(Clone, Component)]
pub struct DepthState {
    pub depth_write: bool,
    pub compare: CompareFunction,
    pub bias: DepthBiasState,
}
impl Default for DepthState {
    fn default() -> Self {
        Self {
            depth_write: true,
            compare: CompareFunction::Always,
            bias: DepthBiasState {
                constant: 0,
                slope_scale: 0,
                clamp: 0,
            },
        }
    }
}
pub enum EDepthState {
    Write(bool),
    Compare(CompareFunction),
    Bias(DepthBiasState),
}
pub struct OpsDepthState(pub(crate) Entity, pub(crate) PassTag, pub(crate) EDepthState);
impl OpsDepthState {
    pub fn ops(mesh: Entity, pass: PassTag, val: EDepthState) -> Self {
        Self(mesh, pass, val)
    }
}
pub type ActionListDepthState = ActionList<OpsDepthState>;
pub fn sys_act_depth_state(
    mut cmds: ResMut<ActionListDepthState>,
    models: Query<&PassIDs>,
    mut items: Query<&mut DepthState>,
) {
    cmds.drain().drain(..).for_each(|OpsDepthState(entity, tag, cmd)| {
        if let Ok(passids) = models.get(entity) {
            let passid = passids.0[tag.index()];

            if let Ok(mut item) = items.get_mut(passid) {
                match cmd {
                    EDepthState::Write(val) => item.depth_write = val,
                    EDepthState::Compare(val) => item.compare = val,
                    EDepthState::Bias(val) => item.bias = val,
                }
            }
        }
    });
}

#[derive(Clone, Component)]
pub struct StencilState {
    pub stencil_front: StencilFaceState,
    pub stencil_back: StencilFaceState,
    pub stencil_read: u32,
    pub stencil_write: u32,
}
impl Default for StencilState {
    fn default() -> Self {
        Self {
            stencil_front: StencilFaceState::IGNORE,
            stencil_back: StencilFaceState::IGNORE,
            stencil_read: 0,
            stencil_write: 0,
        }
    }
}
pub enum EStencilState {
    Front(StencilFaceState),
    Back(StencilFaceState),
    Read(u32),
    Write(u32),
}
pub struct OpsStencilState(pub(crate) Entity, pub(crate) PassTag, pub(crate) EStencilState);
impl OpsStencilState {
    pub fn ops(mesh: Entity, pass: PassTag, val: EStencilState) -> Self {
        Self(mesh, pass, val)
    }
}
pub type ActionListStencilState = ActionList<OpsStencilState>;
pub fn sys_act_stencil_state(
    mut cmds: ResMut<ActionListStencilState>,
    models: Query<&PassIDs>,
    mut items: Query<&mut StencilState>,
) {
    cmds.drain().drain(..).for_each(|OpsStencilState(entity, tag, cmd)| {
        if let Ok(passids) = models.get(entity) {
            let passid = passids.0[tag.index()];

            if let Ok(mut item) = items.get_mut(passid) {
                match cmd {
                    EStencilState::Front(val) => item.stencil_front = val,
                    EStencilState::Back(val) => item.stencil_back = val,
                    EStencilState::Read(val) => item.stencil_read = val,
                    EStencilState::Write(val) => item.stencil_write = val,
                }
            }
        }
    });
}

pub fn depth_stencil_state(
    format: wgpu::TextureFormat,
    depth: &DepthState,
    stencil: &StencilState,
) -> DepthStencilState {
    DepthStencilState {
        format,
        depth_write_enabled: depth.depth_write,
        depth_compare: depth.compare.val2(),
        stencil: wgpu::StencilState {
            front: stencil.stencil_front.val(),
            back: stencil.stencil_back.val(),
            read_mask: stencil.stencil_read,
            write_mask: stencil.stencil_write,
        },
        bias: depth.bias,
    }
}
