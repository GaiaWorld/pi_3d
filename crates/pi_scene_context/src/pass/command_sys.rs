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

pub type PassObjBundle = (
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
    ModelBlend,
    PassReset,
);

pub struct ActionPassObject;
impl ActionPassObject {
    pub fn init(
        empty: Entity,
        idmodel: Entity,
        idscene: Entity,
    ) -> PassObjBundle {
        (
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
            ModelBlend::default(),
            PassReset,
        )
    }
    pub fn reset(
        entitycmds: &mut EntityCommands,
        idmodel: Entity,
        material: Entity,
    ) {
        entitycmds
        .insert((
            PassBindEffectValue(None),
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
            PassReset,
        ))
        ;
    }
}
