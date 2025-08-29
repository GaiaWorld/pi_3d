use pi_scene_shell::prelude::*;
use serde_derive::{Deserialize, Serialize};

use crate::{object::ActionEntity, viewer::prelude::*};

use super::base::*;

pub struct OpsShadowGenerator(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity, pub(crate) PassTag, pub(crate) Entity);
impl OpsShadowGenerator {
    pub fn ops(entity: Entity, scene: Entity, light: Entity, pass: PassTag, in_graph: Entity) -> Self {
        Self(entity, scene, light, pass, in_graph)
    }
}
pub type ActionListShadowGenerator = ActionList<OpsShadowGenerator>;

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum EShadowGeneratorParam {
    ShadowMinz(f32),
    ShadowMaxz(f32),
    ShadowFrustumSize(f32),
    Bias(f32),
    NormalBias(f32),
    DepthScale(f32),
    // AtlasSize(Entity, u32),
    // ShadowEnable(Entity, bool),
}

pub struct OpsShadowGeneratorParam(pub Entity, pub EShadowGeneratorParam);
impl OpsShadowGeneratorParam {
    pub fn ops(shadow: Entity, val: EShadowGeneratorParam) -> Self {
        Self(shadow, val)
    }
}
pub type ActionListShadowGeneratorParam = ActionList<OpsShadowGeneratorParam>;

pub type ShadowGeneratorBundle = (
    (
        ShadowParam,
        ShadowAngle,
        SceneID
    ),
    BundleEntity,
    ViewerBundle,
);

pub struct ActionShadow;
impl ActionShadow {
    pub(crate) fn as_shadow_generator(
        scene: Entity,
        active: bool,
    ) -> ShadowGeneratorBundle {
        (
            (
                ShadowParam::default(),
                ShadowAngle::default(),
                SceneID(scene)
            ),
            ActionEntity::init(),
            ActionViewer::as_viewer(active),
        )

    }
}

// pub struct BundleShadowCaster(
//     RendererID,
//     ShadowParam,
//     ShadowAngle,
// );

// pub struct BundleShadowCasterDirect(
//     ShadowLinkedLightID,
//     DirectionalShadowDirection,
//     DirectionalShadowProjection,
// );

// pub struct BundleShadowCasterSpot(
//     ShadowLinkedLightID,
//     DirectionalShadowDirection,
//     SpotShadowProjection,
// );