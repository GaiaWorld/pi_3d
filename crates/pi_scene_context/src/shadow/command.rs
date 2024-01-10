use pi_scene_shell::prelude::*;

use crate::{viewer::prelude::*, object::ActionEntity, scene::command_sys::ActionScene};

use super::base::*;

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
        commands: &mut EntityCommands,
        scene: Entity,
        active: bool,
    ) {
        commands
            .insert(ShadowMinZ::default())
            .insert(ShadowMaxZ::default())
            .insert(ShadowFrustumSize::default())
            .insert(ShadowAngle::default())
            // .insert(ShadowAtlasSize::default())
            .insert(ShadowDepthScale::default())
            .insert(ShadowBias::default())
            .insert(ShadowNormalBias::default())
            ;

        ActionEntity::init(commands);
        ActionScene::add_to_scene(commands, scene);
        ActionViewer::as_viewer(commands, active);
    }
}