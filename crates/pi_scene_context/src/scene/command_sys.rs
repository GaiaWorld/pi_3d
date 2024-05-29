
use pi_scene_shell::{add_component, prelude::{pi_world::editor::EntityEditor, *}};

use crate::{
    cullings::prelude::*, flags::*, geometry::prelude::*, meshes::prelude::*, pass::*, renderers::prelude::*, transforms::{command_sys::ActionTransformNode, prelude::*}
};

use super::{prelude::*, environment::{brdf::*, environment_texture::{EnvIrradiance, EnvTexture, EnvSampler, EnvTextureSlot}}, pass_render_target::*};

pub fn sys_create_scene(
    mut cmds: ResMut<ActionListSceneCreate>,
    // mut commands: Commands,
    mut editor: EntityEditor,
    mut dynbuffer: ResMut<ResBindBufferAllocator>,
    lightlimit: Res<SceneLightLimit>,
    shadowlimit: Res<SceneShadowLimit>,
    device: Res<PiRenderDevice>,
    asset_samp: Res<ShareAssetMgr<SamplerRes>>, 
    mut meshcreate: ResMut<ActionListMeshCreate>,

    // mut meshpolygin: ResMut<ActionListPolyginMode>,
    mut meshprimitivestate: ResMut<ActionListPrimitiveState>,

    // mut meshdepthwrite: ResMut<ActionListDepthWrite>,
    // mut meshdepthtest: ResMut<ActionListDepthCompare>,
    // mut meshdepthstate: ResMut<ActionListDepthState>,

    mut meshrenderqueue: ResMut<ActionListRenderQueue>,
    mut geocreate: ResMut<ActionListGeometryCreate>,
    mut meshboundingmode: ResMut<ActionListMeshBoundingCullingMode>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneCreation(entity, pool)| {

        let id_left = editor.alloc_entity();
        let id_right = editor.alloc_entity();
        let bounding = editor.alloc_entity();
        let boundinggeo = editor.alloc_entity();

        if editor.contains_entity(entity) {
            meshcreate.push(OpsMeshCreation::ops(entity, bounding, BoundingBoxDisplay::mesh_state()));
            meshboundingmode.push(OpsMeshBoundingCullingMode::ops(bounding, ECullingStrategy::None));

            // meshpolygin.push(OpsPolygonMode::ops(bounding, PolygonMode::Line));
            meshprimitivestate.push(OpsPrimitiveState::ops(bounding, PassTag::PASS_TAG_01, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
            meshprimitivestate.push(OpsPrimitiveState::ops(bounding, PassTag::PASS_TAG_02, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
            meshprimitivestate.push(OpsPrimitiveState::ops(bounding, PassTag::PASS_TAG_03, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
            meshprimitivestate.push(OpsPrimitiveState::ops(bounding, PassTag::PASS_TAG_04, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
            meshprimitivestate.push(OpsPrimitiveState::ops(bounding, PassTag::PASS_TAG_05, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
            meshprimitivestate.push(OpsPrimitiveState::ops(bounding, PassTag::PASS_TAG_06, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
            meshprimitivestate.push(OpsPrimitiveState::ops(bounding, PassTag::PASS_TAG_07, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
            meshprimitivestate.push(OpsPrimitiveState::ops(bounding, PassTag::PASS_TAG_08, EPrimitiveState::CPolygonMode(PolygonMode::Line)));

            meshrenderqueue.push(OpsRenderQueue::ops(bounding, i32::MAX, i32::MAX));
            geocreate.push(OpsGeomeryCreate::ops(bounding, boundinggeo, pi_mesh_builder::cube::CubeBuilder::attrs_meta(), Some(pi_mesh_builder::cube::CubeBuilder::indices_meta())));

            ActionScene::init(entity, &mut editor, id_left, id_right, lightlimit.0, shadowlimit.0, &mut dynbuffer, &device, &asset_samp);
            let components = [editor.init_component::<SceneBoundingPool>(), editor.init_component::<SceneAnimationContext>(),editor.init_component::<BoundingBoxDisplay>(),];

            editor.add_components(entity, &components).unwrap();
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = pool; 
            *editor.get_component_unchecked_mut_by_id(entity, components[1]) = SceneAnimationContext::new(); 
            *editor.get_component_unchecked_mut_by_id(entity, components[2]) = BoundingBoxDisplay { mesh: bounding, display: false };
        } else {
            editor.destroy(id_left).unwrap();
            editor.destroy(id_right).unwrap();
            editor.destroy(bounding).unwrap();
            editor.destroy(boundinggeo).unwrap();
            // commands.entity(id_left).despawn();
            // commands.entity(id_right).despawn();
            // commands.entity(bounding).despawn();
            // commands.entity(boundinggeo).despawn();
            return;
        };
    });
}

pub fn sys_act_scene_ambient(
    mut cmds: ResMut<ActionListSceneTime>,
    mut scenes: Query<&mut SceneTime>,
    mut cmds_ambient: ResMut<ActionListSceneAmbientColor>,
    mut scenes_ambient: Query<&mut AmbientColor>,
    mut cmds_fog: ResMut<ActionListSceneFogParam>,
    mut scenes_fog: Query<&mut SceneFog>,
    mut cmds_anime: ResMut<ActionListSceneAnimationEnable>,
    mut scenes_anime: Query<&mut SceneAnimationEnable>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneTime(entity, val)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            comp.reset(val as u64);
        }
    });
    cmds_ambient.drain().drain(..).for_each(|OpsSceneAmbientColor(entity, val)| {
        if let Ok(mut comp) = scenes_ambient.get_mut(entity) {
            match val {
                ESceneAmbientOps::Color(r, g, b) => { comp.0 = r; comp.1 = g; comp.2 = b; },
                ESceneAmbientOps::Intensity(val) => { comp.3 = val; },
            }
        }
    });
    cmds_fog.drain().drain(..).for_each(|OpsSceneFogParam(entity, val)| {
        if let Ok(mut comp) = scenes_fog.get_mut(entity) {
            match val {
                EFogOps::Color(r, g, b) =>  { comp.r = r; comp.g = g; comp.b = b; },
                EFogOps::Param(param) => comp.param = param,
            }
        }
    });
    cmds_anime.drain().drain(..).for_each(|OpsSceneAnimationEnable(entity, val, count)| {
        if let Ok(mut comp) = scenes_anime.get_mut(entity) {
            *comp = SceneAnimationEnable(val);
        } else if count < 2 {
            cmds_anime.push(OpsSceneAnimationEnable(entity, val, count + 1))
        }
    });
}

pub fn sys_act_scene_render(
    mut cmds: ResMut<ActionListSceneBRDF>,
    mut scenes: Query<&mut BRDFTextureSlot>,
    mut opaquetarget_cmds: ResMut<ActionListSceneOpaqueTexture>,
    mut opaquetarget_scenes: Query<&mut MainCameraOpaqueTarget>,
    mut depthtarget_cmds: ResMut<ActionListSceneDepthTexture>,
    mut depthtarget_scenes: Query<&mut MainCameraDepthTarget>,
    targets: Res<CustomRenderTargets>,
    mut env_cmds: ResMut<ActionListSceneEnvTexture>,
    mut env_scenes: Query<&mut EnvTextureSlot>,
    mut shadow_cmds: ResMut<ActionListSceneShadowMap>,
    mut shadow_scenes: Query<&mut SceneShadowRenderTarget>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneBRDF(entity, val, compressed)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = BRDFTextureSlot(EKeyTexture::Image(KeyImageTextureView::new( KeyImageTexture { url: val, srgb: false, file: true, compressed, ..Default::default() }, TextureViewDesc::default() ) ));
        } else {
            cmds.push(OpsSceneBRDF(entity, val, compressed));
        }
    });
    opaquetarget_cmds.drain().drain(..).for_each(|OpsSceneOpaqueTexture(entity, key)| {
        if let Ok(mut comp) = opaquetarget_scenes.get_mut(entity) {
            comp.0 = targets.get(key);
        }
    });
    depthtarget_cmds.drain().drain(..).for_each(|OpsSceneDepthTexture(entity, key)| {
        if let Ok(mut comp) = depthtarget_scenes.get_mut(entity) {
            comp.0 = targets.get(key);
        }
    });
    env_cmds.drain().drain(..).for_each(|OpsSceneEnvTexture(entity, path, data_is_image)| {
        if let Ok(mut comp) = env_scenes.get_mut(entity) {
            comp.0 = path;
            comp.1 = data_is_image;
        }
    });
    shadow_cmds.drain().drain(..).for_each(|OpsSceneShadowMap(entity, path)| {
        if let Ok(mut comp) = shadow_scenes.get_mut(entity) {
            comp.0 = path;
        }
    });
}

pub type ActionSceneBundle = (
    Scene,
    SceneCoordinateSytem3D,
    SceneTime,
    SceneFog,
    AmbientColor,
    TreeLeftRoot,
    TreeRightRoot,
    // AnimationGroups::default())
    SceneMainCameraID,
    SceneAnimationEnable,
    SceneDirectLightsQueue,
    ScenePointLightsQueue,
    SceneSpotLightsQueue,
    SceneHemiLightsQueue,
    SceneLightingInfosDirty,
    SceneShadowInfosDirty,
    SceneShadowQueue,
    MainCameraOpaqueTarget,
    MainCameraDepthTarget,
    BatchParamOpaque,
    BatchParamTransparent,
    SceneShadowRenderTarget,
    BRDFSampler,
    BRDFTextureSlot,
    BRDFTexture,
    EnvTextureSlot, 
    EnvIrradiance, 
    EnvTexture, 
    EnvSampler
);
pub struct ActionScene;
impl ActionScene {
    pub fn init(
        entity: Entity,
        editor: &mut EntityEditor,
        
        // entitycmds: &mut EntityCommands,

        id_left: Entity,
        id_right: Entity,
        lightlimit: LightLimitInfo,
        shadowlimit: ShadowLimitInfo,
        dynbuffer: &mut BindBufferAllocator,
        device: &PiRenderDevice,
        asset_samp: &ShareAssetMgr<SamplerRes>, 
    ) {
        ActionTransformNode::init_for_tree(entity, editor);
        ActionEntity::init(entity, editor);

        let brdfsampler = BRDFSampler::new(device, asset_samp);
        let slot = BRDFTextureSlot(EKeyTexture::Tex(KeyTexture::from( DefaultTexture::WHITE_2D )));

        let components = [
            editor.init_component::<Scene>(),
            editor.init_component::<SceneCoordinateSytem3D>(),
            editor.init_component::<SceneTime>(),
            editor.init_component::<SceneFog>(),
            editor.init_component::<AmbientColor>(),
            editor.init_component::<TreeLeftRoot>(),
            editor.init_component::<TreeRightRoot>(),
            // AnimationGroups::default(),
            editor.init_component::<SceneMainCameraID>(),
            editor.init_component::<SceneAnimationEnable>(),
            editor.init_component::<SceneDirectLightsQueue>(),
            editor.init_component::<ScenePointLightsQueue>(),
            editor.init_component::<SceneSpotLightsQueue>(),
            editor.init_component::<SceneHemiLightsQueue>(),
            editor.init_component::<SceneLightingInfosDirty>(),
            editor.init_component::<SceneShadowInfosDirty>(),
            editor.init_component::<SceneShadowQueue>(),
            editor.init_component::<MainCameraOpaqueTarget>(),
            editor.init_component::<MainCameraDepthTarget>(),
            editor.init_component::<BatchParamOpaque>(),
            editor.init_component::<BatchParamTransparent>(),
            editor.init_component::<SceneShadowRenderTarget>(),
            editor.init_component::<BRDFSampler>(),
            editor.init_component::<BRDFTextureSlot>(),
            editor.init_component::<BRDFTexture>(),
            editor.init_component::<EnvTextureSlot>(),
            editor.init_component::<EnvIrradiance>(),
            editor.init_component::<EnvTexture>(),
            editor.init_component::<EnvSampler>(),
        ];

            editor.add_components(entity, &components);

        
            *editor.get_component_unchecked_mut_by_id(entity, components[0]) = Scene;
            *editor.get_component_unchecked_mut_by_id(entity, components[1]) = SceneCoordinateSytem3D::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[2]) = SceneTime::new();
            *editor.get_component_unchecked_mut_by_id(entity, components[3]) = SceneFog { param: FogParam::None, r: 1., g: 1., b: 1. };
            *editor.get_component_unchecked_mut_by_id(entity, components[4]) = AmbientColor(1., 1., 1., 1.);
            *editor.get_component_unchecked_mut_by_id(entity, components[5]) = TreeLeftRoot::new(id_left);
            *editor.get_component_unchecked_mut_by_id(entity, components[6]) = TreeRightRoot::new(id_right);
            // AnimationGroups::default(),
            *editor.get_component_unchecked_mut_by_id(entity, components[7]) = SceneMainCameraID(None);
            *editor.get_component_unchecked_mut_by_id(entity, components[8]) = SceneAnimationEnable::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[9]) = SceneDirectLightsQueue(SceneItemsQueue::new(lightlimit.max_direct_light_count));
            *editor.get_component_unchecked_mut_by_id(entity, components[10])= ScenePointLightsQueue(SceneItemsQueue::new(lightlimit.max_point_light_count));
            *editor.get_component_unchecked_mut_by_id(entity, components[11])=SceneSpotLightsQueue(SceneItemsQueue::new(lightlimit.max_spot_light_count));
            *editor.get_component_unchecked_mut_by_id(entity, components[12])=SceneHemiLightsQueue(SceneItemsQueue::new(lightlimit.max_hemi_light_count));
            *editor.get_component_unchecked_mut_by_id(entity, components[13])=SceneLightingInfosDirty;
            *editor.get_component_unchecked_mut_by_id(entity, components[14])=SceneShadowInfosDirty;
            *editor.get_component_unchecked_mut_by_id(entity, components[15])=SceneShadowQueue(SceneItemsQueue::new(shadowlimit.max_count));
            *editor.get_component_unchecked_mut_by_id(entity, components[16])=MainCameraOpaqueTarget(None);
            *editor.get_component_unchecked_mut_by_id(entity, components[17])=MainCameraDepthTarget(None);
            *editor.get_component_unchecked_mut_by_id(entity, components[18])=BatchParamOpaque::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[19])=BatchParamTransparent::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[20])=SceneShadowRenderTarget(None);
            *editor.get_component_unchecked_mut_by_id(entity, components[21])=brdfsampler;
            *editor.get_component_unchecked_mut_by_id(entity, components[22])=slot;
            *editor.get_component_unchecked_mut_by_id(entity, components[23])=BRDFTexture::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[24])=EnvTextureSlot::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[25])=EnvIrradiance::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[26])=EnvTexture::default();
            *editor.get_component_unchecked_mut_by_id(entity, components[27])=EnvSampler::new(device, asset_samp);
  

        if let Some(bindeffect) = BindSceneEffect::new(dynbuffer) {
            // alter4.alter(entity, (bindeffect,));
            add_component(editor, entity, bindeffect);
        }
        if let Some(bindeffect) = SceneLightingInfos::new(dynbuffer, lightlimit) {
            // alter5.alter(entity, (bindeffect,));
            add_component(editor, entity, bindeffect);
        }
        if let Some(bindeffect) = SceneShadowInfos::new(dynbuffer, lightlimit, shadowlimit) {
            // alter6.alter(entity, (bindeffect,));
            add_component(editor, entity, bindeffect);
        }
    }

    pub(crate) fn add_to_scene(
        entity: Entity,
        editor: &mut EntityEditor,
        scene: Entity,
    ) {
        // tree.push(OpsTransformNodeParent::ops(commands.id(), scene));
        let index = editor.init_component::<SceneID>();
        let _ = editor.add_components(entity, &[index]);
        *editor.get_component_unchecked_mut_by_id(entity, index) = SceneID(scene)
    }
}
