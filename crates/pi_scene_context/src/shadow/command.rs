use pi_scene_shell::prelude::*;

use crate::{object::ActionEntity, prelude::RendererID, scene::command_sys::ActionScene, viewer::prelude::*};

use super::{base::*, DirectionalShadowDirection, DirectionalShadowProjection, SpotShadowProjection};

pub struct OpsShadowGenerator(pub(crate) Entity, pub(crate) Entity, pub(crate) Entity, pub(crate) PassTag);
impl OpsShadowGenerator {
    pub fn ops(entity: Entity, scene: Entity, light: Entity, pass: PassTag) -> Self {
        Self(entity, scene, light, pass)
    }
}
pub type ActionListShadowGenerator = ActionList<OpsShadowGenerator>;

pub enum OpsShadowGeneratorParam {
    ShadowMinz(Entity, f32),
    ShadowMaxz(Entity, f32),
    ShadowFrustumSize(Entity, f32),
    Bias(Entity, f32),
    NormalBias(Entity, f32),
    DepthScale(Entity, f32),
    // AtlasSize(Entity, u32),
    // ShadowEnable(Entity, bool),
}
pub type ActionListShadowGeneratorParam = ActionList<OpsShadowGeneratorParam>;

pub struct ActionShadow;
impl ActionShadow {
    pub(crate) fn as_shadow_generator(
        entity: Entity,
        alter1: &mut Alter<(), (), (ShadowParam, ShadowAngle), ()>,
        alter2: &mut Alter<(), (), (DisposeReady, DisposeCan), ()>,
        alter3: &mut Alter<(),(),(SceneID, ), ()>,
        alter4: &mut Alter<(), (), ActionViewerBundle>,
        scene: Entity,
        active: bool,
    ) {
        alter1.alter(entity, ( ShadowParam::default(), ShadowAngle::default()));
        ActionEntity::init(entity, alter2);
        ActionScene::add_to_scene(entity, alter3, scene);
        ActionViewer::as_viewer(entity, alter4, active);
    }
}

pub struct BundleShadowCaster(
    RendererID,
    ShadowParam,
    ShadowAngle,
);

pub struct BundleShadowCasterDirect(
    ShadowLinkedLightID,
    DirectionalShadowDirection,
    DirectionalShadowProjection,
);

pub struct BundleShadowCasterSpot(
    ShadowLinkedLightID,
    DirectionalShadowDirection,
    SpotShadowProjection,
);