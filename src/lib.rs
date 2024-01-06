use pi_3d_state::{PluginStateGlobal, StateResource};
use pi_engine_shell::{prelude::*, run_stage::PluginRunstage};
use default_render::PluginDefaultMaterial;
use pi_gltf2_load::{GLTFResLoader, GLTF};
use pi_node_materials::{PluginNodeMaterial, NodeMaterialBlocks};
use pi_particle_system::prelude::{ParticleSystemPerformance, ActionSetParticleSystem, ResourceParticleSystem};
use pi_scene_context::{
    prelude::*,
    scene::PluginScene,
    animation::PluginSceneAnimation,
    transforms::PluginGroupTransformNode,
    cameras::PluginCamera,
    meshes::PluginMesh,
    geometry::PluginGeometry,
    light::PluginLighting,
    layer_mask::PluginLayerMask,
    materials::PluginGroupMaterial,
    renderers::PluginRenderer,
    skeleton::PluginSkeleton, cullings::PluginCulling, viewer::PluginViewerBase, shadow::PluginShadowGenerator
};
use pi_shadow_mapping::PluginShadowMapping;
use pi_trail_renderer::{ActionSetTrailRenderer, ResTrailBuffer};

pub struct Limit(pub wgpu::Limits);
// impl TMemoryAllocatorLimit for Limit {
//     fn max_size(&self) -> u64 {
//         500 * 1024 * 1024
//     }
// }


pub fn sys_info_node(
    _scenes: Query<Entity, With<SceneTime>>,
    _states: Res<StateResource>,
) {
    // scenes.iter().for_each(|entity| {
    //     if let Some(state) = states.scenes.get(&entity) {
    //         log::warn!(
    //             "Scene: {:?}, Draw: {:?}, Vertex: {:?}, Transform: {:?}, Mesh: {:?}, InstanceMesh: {:?}, Camera: {:?}, Light: {:?}, Skeleton: {:?}, ParticleSys: {:?}, Trail: {:?}, AnimeGroup: {:?}",
    //             entity,
    //             state.count_drawobj,
    //             state.count_vertex,
    //             state.count_transform,
    //             state.count_mesh,
    //             state.count_instance,
    //             state.count_camera,
    //             state.count_light,
    //             state.count_skeleton,
    //             state.count_particlesys,
    //             state.count_trail,
    //             state.count_animationgroup,
    //         );
    //     }
    // });
}

pub fn sys_info_draw(
    draws: Query<(&PassBindGroupScene, &PassBindGroupModel, &PassBindEffectValue, &PassBindEffectTextures, &PassShader, &PassBindGroups, &PassPipeline, &PassDraw)>,
    geometries: Query<&RenderGeometryComp>,
    meshes: Query<&RenderGeometryEable>,
    viewers: Query<(&ModelList, &ForceIncludeModelList, &ModelListAfterCulling)>,
    statecamera: Res<StateCamera>,
    command: Query<Entity>,
) {
    let mut entitycount = 0;
    command.iter().for_each(|v| { entitycount += 1; });

    let mut count_set0 = 0;
    let mut count_set1 = 0;
    let mut count_effect = 0;
    let mut count_textures = 0;
    let mut count_bindgroups = 0;
    let mut count_shader = 0;
    let mut count_pipeline = 0;
    let mut count_draw = 0;
    draws.iter().for_each(|(bindgroup_scene, bindgroup_model, bindeffect, bindtextures, shader, bindgroups, pipeline, draw)| {
        if bindgroup_scene.is_some() { count_set0 += 1; }
        if bindgroup_model.is_some() { count_set1 += 1; }
        if bindeffect.0.is_some() { count_effect += 1; }
        if bindtextures.val().is_some() { count_textures += 1; }
        if bindgroups.0.is_some() { count_bindgroups += 1; }
        if shader.is_some() { count_shader += 1; }
        if pipeline.is_some() { count_pipeline += 1; }
        if draw.is_some() {
            count_draw += 1;
        }
    });

    let mut count_ready_geo = 0;
    geometries.iter().for_each(|item| {
        if item.is_some() {
            count_ready_geo += 1;
        }
    });
    
    let mut count_ready_geo_mesh = 0;
    meshes.iter().for_each(|item| {
        if item.0 {
            count_ready_geo_mesh += 1;
        }
    });

    let mut viewer_cullings = vec![];
    let mut viewer_includes = vec![];
    viewers.iter().for_each(|(models, forcemodels, item)| {
        viewer_cullings.push(item.0.len());
        viewer_includes.push(models.0.len() + forcemodels.0.len());
    });

    log::warn!(
        "Entity: {}, ReadyGeo: {:?}-{:?}, Cullings: {:?}-{:?}-{:?}, Set0: {:?}, Set1: {:?}, Eff: {:?}, Tex: {:?}, BindGroups: {:?}, Shader: {:?}, Pipeline: {:?}, Draw: {:?}",
        entitycount,
        count_ready_geo, count_ready_geo_mesh,
        viewer_includes, viewer_cullings, statecamera.culling_time,
        count_set0, count_set1, count_effect, count_textures, count_bindgroups, count_shader, count_pipeline, count_draw
    );
}

pub fn sys_info_resource(
    states: Res<StateResource>,
    psperformance: Res<ParticleSystemPerformance>,
    performance: Res<Performance>,
) {
    log::warn!(
        "Materials: {:?}, BindBuffer: {:?}, VertexBuffer: {:?}, VertexBufferSize: {:?}, Shaders: {:?}, Pipeline: {:?}, ImageTexture: {:?},",
        states.count_material, states.count_bindbuffer, states.count_geometrybuffer, states.size_geometrybuffer, states.count_shader, states.count_pipeline, states.count_imgtexture
    );
    log::warn!(
        "PSCount: {:?}, PSPerformance: {:?}, PSEmitMatrix: {:?}, PSDirection: {:?}, PSUpdate: {:?}, PSUpdateTrail: {:?}, {:?}, {:?}, {:?}, {:?}",
        psperformance.particles, performance.particlesystem, psperformance.sys_emitmatrix, psperformance.sys_direction, psperformance.sys_update_buffer, psperformance.sys_update_buffer_trail
        , psperformance.sys_emission, psperformance.sys_emitter, psperformance.sys_ids, psperformance.sys_orbit_over_life_time
    );
}

pub struct PluginBundleDefault;
impl PluginGroup for PluginBundleDefault {
    fn build(self) -> PluginGroupBuilder {
        let mut group = PluginGroupBuilder::start::<Self>();
        
        group = group.add(PluginRunstage);
        group = group.add(PluginGlobalAnimation);
        group = group.add(PluginRenderBindGroup);
        group = group.add(PluginScene);
        group = group.add(PluginSceneAnimation);
        group = group.add(PluginFlags);
        group = group.add(PluginAnimeNodeEnable::new());
        group = group.add(PluginTypeAnimatorableFloat::new());
        group = group.add(PluginTypeAnimatorableVec2::new());
        group = group.add(PluginTypeAnimatorableVec3::new());
        group = group.add(PluginTypeAnimatorableVec4::new());
        group = group.add(PluginTypeAnimatorableUint::new());
        group = group.add(PluginTypeAnimatorableInt::new());
        group = PluginGroupTransformNode::add(group);
        group = group.add(PluginCamera)
            .add(PluginAnimeCameraFOV::new())
            .add(PluginAnimeCameraSize::new())
            .add(PluginMesh)
            // .add(PluginAnimeBoneOffset::new())
            .add(PluginAnimeRenderIndiceRange::new())
            .add(PluginGeometry)
            .add(PluginLighting)
            .add(PluginLayerMask)
            .add(PluginViewerBase)
            .add(PluginCulling);
        group = PluginGroupMaterial::add(group);
        group = group.add(PluginRenderer)
            .add(PluginPassObject)
            .add(PluginSkeleton)
            .add(PluginDefaultMaterial)
            .add(PluginDispose)
            .add(PluginNodeMaterial)
            .add(PluginStateGlobal)
            .add(PluginShadowGenerator)
            .add(PluginShadowMapping)
            ;

        group
    }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_engine_shell::engine_shell::EnginShell,
    //     stages: &mut pi_engine_shell::run_stage::RunStage,
    // ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     let device = world.get_resource::<RenderDevice>().unwrap();
    //     let limit = Limit(device.limits());
    //     // world.insert_resource(DynMergyBufferAllocator::new(&limit, 4 * 1024 * 1024));

    //     PluginFlags.init(engine, stages);
    //     PluginRenderBindGroup.init(engine, stages);
    //     PluginScene.init(engine, stages);
    //     PluginTransformNode.init(engine, stages);
    //     PluginMesh.init(engine, stages);
    //     PluginCamera.init(engine, stages);

    //     PluginCulling.init(engine, stages);
    //     PluginGeometry.init(engine, stages);

    //     PluginMaterial.init(engine, stages);
    //     PluginLayerMask.init(engine, stages);

    //     PluginDefaultMaterial.init(engine, stages);

    //     PluginRenderer.init(engine, stages);
    //     PluginBoundingOctTree.init(engine, stages);

    //     // PluginCubeBuilder.init(engine, stages);
    //     // PluginBallBuilder.init(engine, stages);
    //     Ok(())
    // }
}

#[derive(SystemParam)]
pub struct ActionSets<'w> {
    pub scene: ActionSetScene<'w>,
    pub scene_dispose: ResMut<'w, ActionListSceneDispose>,
    pub obj_dispose: ResMut<'w, ActionListDispose>,
    pub camera: ActionSetCamera<'w>,
    pub light: ActionSetLighting<'w>,
    pub shadow: ActionSetShadow<'w>,
    pub transform: ActionSetTransform<'w>,
    pub mesh: ActionSetMesh<'w>,
    pub skin: ActionSetSkeleton<'w>,
    pub abstructmesh: ActionSetAbstructMesh<'w>,
    pub instance: ActionSetInstanceMesh<'w>,
    pub geometry: ActionSetGeometry<'w>,
    pub material: ActionSetMaterial<'w>,
    pub anime: ActionSetAnimationGroup<'w>,
    pub anime_uniform: ResMut<'w, ActionListTargetAnimationUniform>,
    pub anime_instance: ResMut<'w, ActionListTargetAnimationAttribute>,
    pub renderer: ActionSetRenderer<'w>,
    pub trail: ActionSetTrailRenderer<'w>,
    pub parsys: ActionSetParticleSystem<'w>,
    pub property_targetanimation: ResMut<'w, ActionListPropertyTargetAnimation>,
}

#[derive(SystemParam)]
pub struct ResourceSets<'w> {
    pub default_mat: Res<'w, SingleIDBaseDefaultMaterial>,
    pub node_material_blocks: ResMut<'w, NodeMaterialBlocks>,
    pub imgtex_loader: ResMut<'w, ImageTextureLoader>,
    pub imgtex_loader_state: ResMut<'w, StateTextureLoader>,
    pub imgtex_asset: Res<'w, ShareAssetMgr<ImageTexture>>,
    pub imgtexview_asset: Res<'w, ShareAssetMgr<ImageTextureView>>,
    pub gltf2_asset: Res<'w, ShareAssetMgr<GLTF>>,
    pub gltf2_loader: ResMut<'w, GLTFResLoader>,
    pub device: Res<'w, PiRenderDevice>,
    pub queue: Res<'w, PiRenderQueue>,
    pub anime_assets: TypeAnimeAssetMgrs<'w>,
    pub anime_contexts: TypeAnimeContexts<'w>,
    pub render_targets: ResMut<'w, CustomRenderTargets>,
    pub asset_samp: Res<'w, ShareAssetMgr<SamplerRes>>,
    pub asset_atlas: Res<'w, PiSafeAtlasAllocator>,
    pub scene_lighting_limit: ResMut<'w, SceneLightLimit>,
    pub model_lighting_limit: ResMut<'w, ModelLightLimit>,
    pub scene_shadow_limit: ResMut<'w, SceneShadowLimit>,
    pub vb_mgr: Res<'w, ShareAssetMgr<EVertexBufferRange>>,
    pub vb_wait: ResMut<'w, VertexBufferDataMap3D>,
    pub shader_metas: Res<'w, ShareAssetMgr<ShaderEffectMeta>>,
    // pub anime_scene_ctxs: ResMut<'w, SceneAnimationContextMap>,
    pub anime_global: ResMut<'w, GlobalAnimeAbout>,
    pub anime_events: ResMut<'w, GlobalAnimeEvents>,
    pub trailbuffer: ResMut<'w, ResTrailBuffer>,
    pub particlesys: ResourceParticleSystem<'w>,
    pub error_record: ResMut<'w, ErrorRecord>,
}