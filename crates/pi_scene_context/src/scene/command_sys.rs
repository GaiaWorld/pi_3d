
use pi_engine_shell::prelude::*;

use crate::{
    transforms::{prelude::*, command_sys::ActionTransformNode},
    flags::*,
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
) {
    cmds.drain().drain(..).for_each(|OpsSceneCreation(entity, pool)| {

        let id_left = commands.spawn_empty().id();
        let id_right = commands.spawn_empty().id();

        if let Some(mut entitycmds) = commands.get_entity(entity) {
            ActionScene::init(&mut entitycmds, id_left, id_right, lightlimit.0, shadowlimit.0, &mut dynbuffer, &device, &asset_samp);
            entitycmds.insert(pool);
            entitycmds.insert(SceneAnimationContext::new());
        } else {
            commands.entity(id_left).despawn();
            commands.entity(id_right).despawn();
            return;
        };
    });
}

pub fn sys_act_scene_time(
    mut cmds: ResMut<ActionListSceneTime>,
    mut scenes: Query<&mut SceneTime>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneTime(entity, val)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            comp.reset(val as u64);
        }
    });
}

pub fn sys_act_scene_ambientcolor(
    mut cmds: ResMut<ActionListSceneAmbientColor>,
    mut scenes: Query<&mut AmbientColor>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneAmbientColor(entity, r, g, b, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = AmbientColor(r, g, b);
        } else if count < 2 {
            cmds.push(OpsSceneAmbientColor(entity, r, g, b, count + 1))
        }
    });
}


pub fn sys_act_scene_ambientintensity(
    mut cmds: ResMut<ActionListSceneAmbientIntensity>,
    mut scenes: Query<&mut AmbientIntensity>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneAmbientIntensity(entity, val, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = AmbientIntensity(val);
        } else if count < 2 {
            cmds.push(OpsSceneAmbientIntensity(entity, val, count + 1))
        }
    });
}

pub fn sys_act_scene_fogcolor(
    mut cmds: ResMut<ActionListSceneFogColor>,
    mut scenes: Query<&mut SceneFogColor>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneFogColor(entity, r, g, b, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = SceneFogColor(r, g, b);
        } else if count < 2 {
            cmds.push(OpsSceneFogColor(entity, r, g, b, count + 1))
        }
    });
}


pub fn sys_act_scene_fogparam(
    mut cmds: ResMut<ActionListSceneFogParam>,
    mut scenes: Query<&mut SceneFogParam>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneFogParam(entity, val, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = SceneFogParam(val);
        } else if count < 2 {
            cmds.push(OpsSceneFogParam(entity, val, count + 1))
        }
    });
}

pub fn sys_act_scene_animation_enable(
    mut cmds: ResMut<ActionListSceneAnimationEnable>,
    mut scenes: Query<&mut SceneAnimationEnable>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneAnimationEnable(entity, val, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = SceneAnimationEnable(val);
        } else if count < 2 {
            cmds.push(OpsSceneAnimationEnable(entity, val, count + 1))
        }
    });
}

pub fn sys_act_scene_brdf(
    mut cmds: ResMut<ActionListSceneBRDF>,
    mut scenes: Query<&mut BRDFTextureSlot>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneBRDF(entity, val, compressed)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = BRDFTextureSlot(EKeyTexture::Image(KeyImageTextureView::new( KeyImageTexture { url: val, srgb: false, file: true, compressed, ..Default::default() }, TextureViewDesc::default() ) ));
        } else {
            cmds.push(OpsSceneBRDF(entity, val, compressed));
        }
    });
}

pub fn sys_act_scene_opaque_target(
    mut cmds: ResMut<ActionListSceneOpaqueTexture>,
    mut scenes: Query<&mut MainCameraOpaqueTarget>,
    targets: Res<CustomRenderTargets>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneOpaqueTexture(entity, key)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            comp.0 = targets.get(key);
        }
    });
}

pub fn sys_act_scene_depth_target(
    mut cmds: ResMut<ActionListSceneDepthTexture>,
    mut scenes: Query<&mut MainCameraDepthTarget>,
    targets: Res<CustomRenderTargets>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneDepthTexture(entity, key)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            comp.0 = targets.get(key);
        }
    });
}

pub fn sys_act_scene_env_texture(
    mut cmds: ResMut<ActionListSceneEnvTexture>,
    mut scenes: Query<&mut EnvTextureSlot>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneEnvTexture(entity, path, data_is_image)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            comp.0 = path;
            comp.1 = data_is_image;
        }
    });
}

pub fn sys_act_scene_shadowmap(
    mut cmds: ResMut<ActionListSceneShadowMap>,
    mut scenes: Query<&mut SceneShadowRenderTarget>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneShadowMap(entity, path)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            comp.0 = path;
        }
    });
}

pub struct ActionScene;
impl ActionScene {
    pub fn init(
        entitycmds: &mut EntityCommands,
        id_left: Entity,
        id_right: Entity,
        lightlimit: LightLimitInfo,
        shadowlimit: ShadowLimitInfo,
        dynbuffer: &mut BindBufferAllocator,
        device: &PiRenderDevice,
        asset_samp: &ShareAssetMgr<SamplerRes>, 
    ) {
        ActionTransformNode::init_for_tree(entitycmds);
        ActionEntity::init(entitycmds);

        entitycmds
            .insert(Scene)
            .insert(SceneCoordinateSytem3D::default())
            .insert(SceneTime::new())
            .insert(SceneFogColor(1., 1., 1.))
            .insert(SceneFogParam(FogParam::None))
            .insert(AmbientColor(1., 1., 1.))
            .insert(AmbientIntensity(1.))
            .insert(TreeLeftRoot::new(id_left))
            .insert(TreeRightRoot::new(id_right))
            // .insert(AnimationGroups::default())
            .insert(SceneMainCameraID(None))
            .insert(SceneAnimationEnable::default())
            .insert(SceneDirectLightsQueue(SceneItemsQueue::new(lightlimit.max_direct_light_count)))
            .insert(ScenePointLightsQueue(SceneItemsQueue::new(lightlimit.max_point_light_count)))
            .insert(SceneSpotLightsQueue(SceneItemsQueue::new(lightlimit.max_spot_light_count)))
            .insert(SceneHemiLightsQueue(SceneItemsQueue::new(lightlimit.max_hemi_light_count)))
            .insert(SceneLightingInfosDirty)
            .insert(SceneShadowInfosDirty)
            .insert(SceneShadowQueue(SceneItemsQueue::new(shadowlimit.max_count)))
            .insert(MainCameraOpaqueTarget(None))
            .insert(MainCameraDepthTarget(None))
            ;

        entitycmds.insert(SceneShadowRenderTarget(None));

        if let Some(bindeffect) = BindSceneEffect::new(dynbuffer) {
            entitycmds.insert(bindeffect);
        }
        if let Some(bindeffect) = SceneLightingInfos::new(dynbuffer, lightlimit) {
            entitycmds.insert(bindeffect);
        }
        if let Some(bindeffect) = SceneShadowInfos::new(dynbuffer, lightlimit, shadowlimit) {
            entitycmds.insert(bindeffect);
        }
        

        let brdfsampler = BRDFSampler::new(device, asset_samp);
        entitycmds.insert(brdfsampler);

        let slot = BRDFTextureSlot(EKeyTexture::Tex(KeyTexture::from( DefaultTexture::WHITE_2D )));
        entitycmds.insert(slot);
        entitycmds.insert(BRDFTexture::default());

        entitycmds.insert(EnvTextureSlot::default());
        entitycmds.insert(EnvIrradiance::default());
        entitycmds.insert(EnvTexture::default());
        entitycmds.insert(EnvSampler::new(device, asset_samp));
    }

    pub(crate) fn add_to_scene(
        commands: &mut EntityCommands,
        scene: Entity,
    ) {
        // tree.push(OpsTransformNodeParent::ops(commands.id(), scene));
        commands
            .insert(SceneID(scene));
    }
}
