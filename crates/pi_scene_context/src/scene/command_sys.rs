
use pi_scene_shell::prelude::*;

use crate::{
    cullings::prelude::*, flags::*, geometry::prelude::*, meshes::prelude::*, pass::*, renderers::prelude::*, transforms::{command_sys::{ActionTransformNode, BundleTreeNode}, prelude::*}
};

use super::{prelude::*, environment::{brdf::*, environment_texture::{EnvIrradiance, EnvTexture, EnvSampler, EnvTextureSlot}}, pass_render_target::*};

pub fn sys_create_scene(
    mut cmds: ResMut<ActionListSceneCreate>,
    mut commands: Commands,
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
    mut meshstate: ResMut<ActionListMeshStateModify>,
    // mut alter: Alter<(), (), (BundleScene, SceneBoundingPool, SceneAnimationContext, BoundingBoxDisplay), ()>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneCreation(entity, pool)| {

        let id_left = commands.spawn_empty().id();
        let id_right = commands.spawn_empty().id();
        let bounding = commands.spawn_empty().id();
        let boundinggeo = commands.spawn_empty().id();

        if let Some(mut entitycmds) = commands.get_entity(entity) {
            meshcreate.push(OpsMeshCreation::ops(entity, bounding, BoundingBoxDisplay::mesh_state()));
            meshstate.push(OpsMeshStateModify::ops(bounding, EMeshStateModify::BoundingCullingMode( ECullingStrategy::None )));

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

            if let Some(bundle) = ActionScene::init(lightlimit.0, shadowlimit.0, &mut dynbuffer, &device, &asset_samp) {
                let bundle = (
                    bundle,
                    pool,
                    SceneAnimationContext::new(),
                    BoundingBoxDisplay { mesh: bounding, display: false }
                );
                entitycmds.insert(bundle);
                // alter.alter(entity, bundle);
            }
        } else {
            commands.entity(id_left).despawn();
            commands.entity(id_right).despawn();
            commands.entity(bounding).despawn();
            commands.entity(boundinggeo).despawn();
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


pub type BundleScene = (
    BundleTreeNode,
    BundleEntity,
    (
        Scene,
        SceneCoordinateSytem3D,
        SceneTime,
        SceneFog,
        AmbientColor,
        SceneMainCameraID,
        SceneAnimationEnable,
    ),
    (
        SceneDirectLightsQueue,
        ScenePointLightsQueue,
        SceneSpotLightsQueue,
        SceneHemiLightsQueue,
        SceneLightingInfosDirty,
        SceneShadowInfosDirty,
        SceneShadowQueue,
    ),
    (
        MainCameraOpaqueTarget,
        MainCameraDepthTarget,
        BatchParamOpaque,
        BatchParamTransparent,
        SceneShadowRenderTarget,
        BindSceneEffect,
        SceneLightingInfos,
        SceneShadowInfos,
    ),
    (
        BRDFSampler,
        BRDFTextureSlot,
        BRDFTexture,
        EnvTextureSlot,
        EnvIrradiance,
        EnvTexture,
        EnvSampler,
    )
);

pub struct ActionScene;
impl ActionScene {
    pub fn init(
        lightlimit: LightLimitInfo,
        shadowlimit: ShadowLimitInfo,
        dynbuffer: &mut BindBufferAllocator,
        device: &PiRenderDevice,
        asset_samp: &ShareAssetMgr<SamplerRes>, 
    ) -> Option<BundleScene> {

        let (bindeffect0, bindeffect1, bindeffect2) = match (BindSceneEffect::new(dynbuffer), SceneLightingInfos::new(dynbuffer, lightlimit), SceneShadowInfos::new(dynbuffer, lightlimit, shadowlimit)) {
            (Some(bindeffect0), Some(bindeffect1), Some(bindeffect2)) => {
                (bindeffect0, bindeffect1, bindeffect2)
            },
            _ => {
                return None;
            }
        };
        let brdfsampler = BRDFSampler::new(device, asset_samp);
        let slot = BRDFTextureSlot(EKeyTexture::Tex(KeyTexture::from( DefaultTexture::WHITE_2D )));
        Some((
            ActionTransformNode::init_for_tree(),
            ActionEntity::init(),
            (
                Scene::default(),
                SceneCoordinateSytem3D::default(),
                SceneTime::new(),
                SceneFog { param: FogParam::None, r: 1., g: 1., b: 1. },
                AmbientColor(1., 1., 1., 1.),
                SceneMainCameraID(None),
                SceneAnimationEnable::default(),
            ),
            (
                // .insert(AnimationGroups::default())
                SceneDirectLightsQueue(SceneItemsQueue::new(lightlimit.max_direct_light_count)),
                ScenePointLightsQueue(SceneItemsQueue::new(lightlimit.max_point_light_count)),
                SceneSpotLightsQueue(SceneItemsQueue::new(lightlimit.max_spot_light_count)),
                SceneHemiLightsQueue(SceneItemsQueue::new(lightlimit.max_hemi_light_count)),
                SceneLightingInfosDirty,
                SceneShadowInfosDirty,
                SceneShadowQueue(SceneItemsQueue::new(shadowlimit.max_count)),
            ),
            (
                MainCameraOpaqueTarget(None),
                MainCameraDepthTarget(None),
                BatchParamOpaque::default(),
                BatchParamTransparent::default(),
                SceneShadowRenderTarget(None),
                bindeffect0,
                bindeffect1,
                bindeffect2
            ),
            (
                brdfsampler,
                slot,
                BRDFTexture::default(),
                EnvTextureSlot::default(),
                EnvIrradiance::default(),
                EnvTexture::default(),
                EnvSampler::new(device, asset_samp),
            )
        ))
    }
}

