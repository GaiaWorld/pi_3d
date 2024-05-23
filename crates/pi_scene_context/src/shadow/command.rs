use pi_scene_shell::prelude::{pi_world::editor::EntityEditor, *};

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
        editor: &mut EntityEditor,
        scene: Entity,
        active: bool,
    ) {
        let components  = [
                        editor.init_component::<ShadowParam>(),
                        editor.init_component::<ShadowAngle>(),
                    ];
        editor.add_components(entity, &components);
        // alter1.alter(entity, ( ShadowParam::default(), ShadowAngle::default()));
        ActionEntity::init(entity, editor);
        ActionScene::add_to_scene(entity, editor, scene);
        ActionViewer::as_viewer(entity, editor, active);
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