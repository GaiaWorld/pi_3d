
use pi_engine_shell::prelude::*;
use pi_render::{renderer::bind_buffer::BindBufferAllocator};
use pi_scene_math::Vector3;

use crate::{
    viewer::{prelude::*, command_sys::ActionViewer},
    renderers::{
        prelude::*,
        command_sys::*,
    },
    materials::{
        prelude::*,
        command_sys::*
    },
    pass::{EPassTag, PassTagOrders},
    scene::command_sys::ActionScene,
    transforms::{command_sys::*, prelude::*},
    animation::command_sys::*, prelude::GlobalEnable,
};

use super::{
    base::{LightDirection, Light, LightingMode},
    point::ShadowAngle,
    shadow_generator::{
        base::{ShadowMinZ, ShadowMaxZ, ShadowFrustumSize, ShadowEnable, ShadowBias, ShadowNormalBias, ShadowDepthScale, ShadowAtlasSize, },
        ShaderShadowGenerator
    }
};

pub struct OpsLightCreate(pub(crate) Entity, pub(crate) Entity, pub(crate) String);
impl OpsLightCreate {
    pub fn ops(scene: Entity, light: Entity, name: String) -> Self {
        OpsLightCreate(scene, light, name)
    }
}

pub type ActionListLightCreate = ActionList<OpsLightCreate>;

pub type ActionListLightParam = ActionList<ELightModifyCommand>;

pub enum ELightModifyCommand {
    LightType(Entity, Light),
    LightingType(Entity, LightingMode),
    ShadowMinz(Entity, f32),
    ShadowMaxz(Entity, f32),
    ShadowFrustumSize(Entity, f32),
    Directional(Entity, Vector3),
    Bias(Entity, f32),
    NormalBias(Entity, f32),
    DepthScale(Entity, f32),
    AtlasSize(Entity, u32),
    ShadowEnable(Entity, bool),
}
