use pi_engine_shell::prelude::*;

use super::{command::*, pass_object::*};


pub fn sys_create_pass_object(
    mut cmds: ResMut<ActionListPassObject>,
    mut commands: Commands,
    models: Query<(
        & PassID01, & PassID02, & PassID03, & PassID04, & PassID05, & PassID06, & PassID07, & PassID08,
        // & PassID09, & PassID10, & PassID11, & PassID12
    )>,
) {
    cmds.drain().drain(..).for_each(|OpsPassObject(idmodel, idmaterial, pass)| {
        if let Ok(passid) = models.get(idmodel) {
            let id_pass = match pass {
                PassTag::PASS_TAG_01 => { passid.0.0 },
                PassTag::PASS_TAG_02 => { passid.1.0 },
                PassTag::PASS_TAG_03 => { passid.2.0 },
                PassTag::PASS_TAG_04 => { passid.3.0 },
                PassTag::PASS_TAG_05 => { passid.4.0 },
                PassTag::PASS_TAG_06 => { passid.5.0 },
                PassTag::PASS_TAG_07 => { passid.6.0 },
                PassTag::PASS_TAG_08 => { passid.7.0 },
                // PassTag::PASS_TAG_09 => { passid.8.0 },
                // PassTag::PASS_TAG_10 => { passid.9.0 },
                // PassTag::PASS_TAG_11 => { passid.10.0 },
                // PassTag::PASS_TAG_12 => { passid.11.0 },
                _ => { passid.7.0 }
            };

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
