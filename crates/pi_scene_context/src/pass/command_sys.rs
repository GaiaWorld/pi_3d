use pi_scene_shell::prelude::{pi_world::editor::EntityEditor, *};

use crate::prelude::{DepthState, ModelBlend, PrimitiveState, StencilState};

use super::{command::*, pass_object::*};


pub fn sys_create_pass_object(
    mut cmds: ResMut<ActionListPassObject>,
    mut editor: EntityEditor,
    models: Query<& PassIDs>,
) {
    cmds.drain().drain(..).for_each(|OpsPassObject(idmodel, idmaterial, pass)| {
        if let Ok(passid) = models.get(idmodel) {
            let id_pass = passid.0[pass.index()];

            // log::warn!("sys_create_pass_object ");

            if  editor.contains_entity(id_pass) {
                ActionPassObject::reset(id_pass, &mut editor, idmodel, idmaterial);
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
        editor: &mut EntityEditor,
        empty: Entity,
        idmodel: Entity,
        idscene: Entity,
    ) {
        let components = [
             editor.init_component::<PassModelID>(),
             editor.init_component::<PassSceneID>(),
             editor.init_component::<PassSceneForSet3>(),
             editor.init_component::<PassViewerID>(),
             editor.init_component::<PassMaterialID>(),
             editor.init_component::<PassGeometryID>(),
             editor.init_component::<PassRendererID>(),
             editor.init_component::<PassPipelineStateDirty>(),
             editor.init_component::<PassDrawDirty>(),
             editor.init_component::<PrimitiveState>(),
             editor.init_component::<DepthState>(),
             editor.init_component::<StencilState>(),
             editor.init_component::<ModelBlend>(),
             editor.init_component::<PassEffectReady>(),
             editor.init_component::<PassBindGroupScene>(),
        ];

        editor.add_components(entity, &components).unwrap(); 

        *editor.get_component_unchecked_mut_by_id(entity, components[0])   = PassModelID(idmodel);
        *editor.get_component_unchecked_mut_by_id(entity, components[1])   = PassSceneID(idscene);
        *editor.get_component_unchecked_mut_by_id(entity, components[2])   = PassSceneForSet3(idscene);
        *editor.get_component_unchecked_mut_by_id(entity, components[3])   = PassViewerID(empty);
        *editor.get_component_unchecked_mut_by_id(entity, components[4])   = PassMaterialID(empty);
        *editor.get_component_unchecked_mut_by_id(entity, components[5])   = PassGeometryID(empty);
        *editor.get_component_unchecked_mut_by_id(entity, components[6])   = PassRendererID(empty);
        *editor.get_component_unchecked_mut_by_id(entity, components[7])   = PassPipelineStateDirty;
        *editor.get_component_unchecked_mut_by_id(entity, components[8])   = PassDrawDirty;
        // *editor.get_component_unchecked_mut_by_id(entity, components[9])   = PrimitiveState::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[10])   = DepthState::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[11])   = StencilState::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[12])   = ModelBlend::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[13])   = PassEffectReady::default();
        // *editor.get_component_unchecked_mut_by_id(entity, components[14])   = PassBindGroupScene::default();

    }
    pub fn reset(
        entity: Entity,
        editor: &mut EntityEditor,
        idmodel: Entity,
        material: Entity,
    ) {
        let components = [
            editor.init_component::<PassBindEffectValue>(),
            editor.init_component::<PassBindEffectTextures>(),
            editor.init_component::<PassBindGroupScene>(),
            editor.init_component::<PassBindGroupModel>(),
            editor.init_component::<PassBindGroupTextureSamplers>(),
            editor.init_component::<PassBindGroupLightingShadow>(),
            editor.init_component::<PassBindGroups>(),
            editor.init_component::<PassEffectReady>(),
            editor.init_component::<PassShader>(),
            editor.init_component::<PassPipeline>(),
            editor.init_component::<PassDraw>(),
            editor.init_component::<PassModelID>(),
            editor.init_component::<PassMaterialID>(),
            editor.init_component::<PassReset>(),
        ];
        editor.add_components(entity, &components).unwrap();
        
        // *editor.get_component_unchecked_mut_by_id(entity, components[0])   = PassBindEffectValue(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[1])   = PassBindEffectTextures(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[2])   = PassBindGroupScene(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[3])   = PassBindGroupModel(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[4])   = PassBindGroupTextureSamplers(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[5])   = PassBindGroupLightingShadow(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[6])   = PassBindGroups(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[7])   = PassEffectReady(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[8])   = PassShader(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[9])   = PassPipeline(None);
        // *editor.get_component_unchecked_mut_by_id(entity, components[10])   = PassDraw(None);
        *editor.get_component_unchecked_mut_by_id(entity, components[11])   = PassModelID(idmodel);
        *editor.get_component_unchecked_mut_by_id(entity, components[12])   = PassMaterialID(material);
        *editor.get_component_unchecked_mut_by_id(entity, components[13])   = PassReset;
        
    }
}
