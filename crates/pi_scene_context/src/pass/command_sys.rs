use pi_scene_shell::prelude::*;

use crate::prelude::{DepthState, ModelBlend, PrimitiveState, StencilState};

use super::{command::*, pass_object::*};


pub fn sys_create_pass_object(
    mut cmds: ResMut<ActionListPassObject>,
    mut commands: Commands,
    models: Query<& PassIDs>,
) {
    cmds.drain().drain(..).for_each(|OpsPassObject(idmodel, idmaterial, pass)| {
        if let Ok(passid) = models.get(idmodel) {
            let id_pass = passid.0[pass.index()];

            // log::warn!("sys_create_pass_object ");

            if let Some(mut entitycmds) = commands.get_entity(id_pass) {
                ActionPassObject::reset(&mut entitycmds, idmodel, idmaterial);
            }
        }
    });
}

pub struct ActionPassObject;
impl ActionPassObject {
    pub fn init(
        entitycmds: &mut EntityCommands,
        empty: Entity,
        idmodel: Entity,
        idscene: Entity,
    ) {
        entitycmds
            .insert(PassModelID(idmodel))
            .insert(PassSceneID(idscene))
            .insert(PassSceneForSet3(idscene))
            .insert(PassViewerID(empty))
            .insert(PassMaterialID(empty))
            .insert(PassGeometryID(empty))
            .insert(PassRendererID(empty))
            .insert(PassPipelineStateDirty)
            .insert(PassDrawDirty)
            .insert(PrimitiveState::default())
            .insert(DepthState::default())
            .insert(StencilState::default())
            .insert(ModelBlend::default())
        ;
    }
    pub fn reset(
        entitycmds: &mut EntityCommands,
        idmodel: Entity,
        material: Entity,
    ) {
        entitycmds
            .insert(PassBindEffectValue(None))
            .insert(PassBindEffectTextures(None))
            .insert(PassBindGroupScene(None))
            .insert(PassBindGroupModel(None))
            .insert(PassBindGroupTextureSamplers(None))
            .insert(PassBindGroupLightingShadow(None))
            .insert(PassBindGroups(None))
            .insert(PassEffectReady(None))
            .insert(PassShader(None))
            .insert(PassPipeline(None))
            .insert(PassDraw(None))
            .insert(PassModelID(idmodel))
            .insert(PassMaterialID(material))
            .insert(PassReset)
        ;
    }
}
