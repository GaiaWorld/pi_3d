use pi_scene_shell::prelude::*;

use crate::prelude::{DepthState, ModelBlend, PrimitiveState, StencilState};

use super::{command::*, pass_object::*};


pub fn sys_create_pass_object(
    mut cmds: ResMut<ActionListPassObject>,
    mut commands: Alter<
    (), 
    (), 
    (
        PassBindEffectValue, 
        PassBindEffectTextures, 
        PassBindGroupScene, 
        PassBindGroupModel, 
        PassBindGroupTextureSamplers, 
        PassBindGroupLightingShadow, 
        PassBindGroups, 
        PassEffectReady, 
        PassShader, 
        PassPipeline, 
        PassDraw, 
        PassModelID, 
        PassMaterialID, 
        PassReset
    ), 
    ()>,
    models: Query<& PassIDs>,
) {
    cmds.drain().drain(..).for_each(|OpsPassObject(idmodel, idmaterial, pass)| {
        if let Ok(passid) = models.get(idmodel) {
            let id_pass = passid.0[pass.index()];

            // log::warn!("sys_create_pass_object ");

            if  commands.get(id_pass).is_ok() {
                ActionPassObject::reset(id_pass, &mut commands, idmodel, idmaterial);
            }
        }
    });
}

pub type ActionPassObjectInitBundle = (
    PassModelID,
    PassSceneID,
    PassSceneForSet3,
    PassViewerID,
    PassMaterialID,
    PassGeometryID,
    PassRendererID,
    PassPipelineStateDirty,
    PassDrawDirty,
    PrimitiveState,
    DepthState,
    StencilState,
    ModelBlend
);

pub type ActionPassObjectResetBundle = (
    PassBindEffectValue,
    PassBindEffectTextures,
    PassBindGroupScene,
    PassBindGroupModel,
    PassBindGroupTextureSamplers,
    PassBindGroupLightingShadow,
    PassBindGroups,
    PassEffectReady,
    PassShader,
    PassPipeline,
    PassDraw,
    PassModelID,
    PassMaterialID,
    PassReset
);
pub struct ActionPassObject;
impl ActionPassObject {
    pub fn init(
        entity: Entity,
        entitycmds: &mut Alter<
        (), 
        (), 
        ActionPassObjectInitBundle, 
        ()>,
        empty: Entity,
        idmodel: Entity,
        idscene: Entity,
    ) {
        entitycmds.alter(entity, (
            PassModelID(idmodel),
            PassSceneID(idscene),
            PassSceneForSet3(idscene),
            PassViewerID(empty),
            PassMaterialID(empty),
            PassGeometryID(empty),
            PassRendererID(empty),
            PassPipelineStateDirty,
            PassDrawDirty,
            PrimitiveState::default(),
            DepthState::default(),
            StencilState::default(),
            ModelBlend::default()
        ));
    }
    pub fn reset(
        entity: Entity,
        entitycmds: &mut Alter<
        (), 
        (), 
        ActionPassObjectResetBundle, 
        ()>,
        idmodel: Entity,
        material: Entity,
    ) {
        entitycmds.alter(entity, 
            (PassBindEffectValue(None),
            PassBindEffectTextures(None),
            PassBindGroupScene(None),
            PassBindGroupModel(None),
            PassBindGroupTextureSamplers(None),
            PassBindGroupLightingShadow(None),
            PassBindGroups(None),
            PassEffectReady(None),
            PassShader(None),
            PassPipeline(None),
            PassDraw(None),
            PassModelID(idmodel),
            PassMaterialID(material),
            PassReset)
        );
    }
}
