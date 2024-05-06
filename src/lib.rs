
use std::sync::Arc;
use pi_scene_shell::{prelude::*, run_stage::PluginRunstage};
use pi_gltf2_load::{GLTFResLoader, GLTF};
use pi_node_materials::prelude::*;
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
use pi_trail_renderer::{ActionSetTrailRenderer, ResTrailBuffer};

pub struct Limit(pub wgpu::Limits);
// impl TMemoryAllocatorLimit for Limit {
//     fn max_size(&self) -> u64 {
//         500 * 1024 * 1024
//     }
// }

#[derive(Resource, Default)]
pub struct StateResource {
    pub debug: bool,
    pub count_gltf: usize,
    pub count_texture: usize,
    pub count_imgtexture: usize,
    pub mem_imgtexture: usize,
    pub count_bindgroup: usize,
    pub count_pipeline: usize,
    pub count_shadermeta: usize,
    pub mem_shadermeta: usize,
    pub count_shader: usize,
    pub mem_shader: usize,
    pub count_bindbuffer: usize,
    pub mem_bindbuffer: usize,
    pub count_geometrybuffer: usize,
    pub size_geometrybuffer: u64,
    pub count_passmat: u32,
    pub count_passtexs: u32,
    pub count_passset0: u32,
    pub count_passset1: u32,
    pub count_passset2: u32,
    pub count_passbindgroups: u32,
    pub count_passshader: u32,
    pub count_passpipeline: u32,
    pub count_passdraw: u32,
    pub count_rendergeometryenable: u32,
    pub count_material: u32,
    pub count_vertex: u32,
    // pub scenes: XHashMap<Entity, StateScene>,
}

pub fn sys_state_resource(
    asset_gltf: Res<ShareAssetMgr<GLTF>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    bindbuffers: Res<ResBindBufferAllocator>,
    vertexbuffers: Res<VertexBufferAllocator3D>,
    shaders: Res<ShareAssetMgr<Shader3D>>,
    pipelines: Res<ShareAssetMgr<Pipeline3D>>,
    imagetextures: Res<ShareAssetMgr<ImageTexture>>,
    shadermetas: Res<ShareAssetMgr<ShaderEffectMeta>>,
    passes: (
        Query<&PassBindGroupScene>,
        Query<&PassBindGroupModel>,
        Query<&PassBindGroupTextureSamplers>,
        Query<&PassBindGroups>,
        Query<&PassShader>,
        Query<&PassPipeline>,
        Query<&PassDraw>,
        Query<&PassBindEffectTextures>,
    ),
    mut stateglobal: ResMut<StateResource>,
    renderers: Query<&Renderer>,
) {
    // if stateglobal.debug == false { return };

    stateglobal.count_gltf              = asset_gltf.len();
    stateglobal.count_bindbuffer        = bindbuffers.asset_mgr().len();
    stateglobal.mem_bindbuffer          = bindbuffers.asset_mgr().size();
    stateglobal.count_bindgroup         = asset_mgr_bindgroup.0.len();
    stateglobal.count_pipeline          = pipelines.len();
    stateglobal.count_geometrybuffer    = vertexbuffers.total_buffer_count();
    stateglobal.size_geometrybuffer     = vertexbuffers.total_buffer_size();
    stateglobal.count_shader            = shaders.len();
    stateglobal.mem_shader              = shaders.size();
    stateglobal.count_imgtexture        = imagetextures.len();
    stateglobal.mem_imgtexture          = imagetextures.size();
    stateglobal.count_shadermeta        = shadermetas.len();
    stateglobal.mem_shadermeta          = shadermetas.size();

    let mut count;

    count = 0;
    passes.0.iter().for_each(|item| {
        if item.is_some() { count += 1; }
    });
    stateglobal.count_passset0 = count;

    count = 0;
    passes.1.iter().for_each(|item| {
        if item.is_some() { count += 1; }
    });
    stateglobal.count_passset1 = count;

    count = 0;
    passes.2.iter().for_each(|item| {
        if item.is_some() { count += 1; }
    });
    stateglobal.count_passset2 = count;

    count = 0;
    passes.3.iter().for_each(|item| {
        if item.is_some() { count += 1; }
    });
    stateglobal.count_passbindgroups = count;

    count = 0;
    passes.4.iter().for_each(|item| {
        if item.is_some() { count += 1; }
    });
    stateglobal.count_passshader = count;

    count = 0;
    passes.5.iter().for_each(|item| {
        if item.is_some() { count += 1; }
    });
    stateglobal.count_passpipeline = count;

    count = 0;
    passes.6.iter().for_each(|item| {
        if item.is_some() { count += 1; }
    });
    stateglobal.count_passdraw = count;

    count = 0;
    passes.7.iter().for_each(|item| {
        if item.0.is_some() { count += 1; }
    });
    stateglobal.count_passtexs = count;

    // count = 0;
    // passes.8.iter().for_each(|item| {
    //     if item.0 != empty.id() { count += 1; }
    // });
    // stateglobal.count_passmat = count;

    // *performance = Performance::default();
    // *particlesysperformance = ParticleSystemPerformance::default();

    let mut count_vertex = 0;
    renderers.iter().for_each(|renderer| {
        count_vertex += renderer.vertexs;
    });
    stateglobal.count_vertex = count_vertex as u32;
}

pub struct PluginStateGlobal;
impl Plugin for PluginStateGlobal {
    fn build(&self, app: &mut App) {
        app.world.insert_single_res(Performance::default());
        app.world.insert_single_res(StateResource::default());
        app.add_system(
            Update,
            // (
                // sys_state_scene,
                sys_state_resource
            // ).chain().run_if(should_run).in_set(ERunStageChap::StateCheck)
        );

        // let device = app.world.get_resource::<PiRenderDevice>().unwrap().0.clone();
        // let queue = app.world.get_resource::<PiRenderQueue>().unwrap().0.clone();

        // let maxcount = 
        // let mut allocator = app.world.get_resource_mut::<VertexBufferAllocator3D>().unwrap();
        // let buffer = StateGeometryBuffer::new(maxcount as u32, &mut allocator, &device, &queue);
    }
}


#[derive(Resource)]
pub struct StateGeometryBuffer{
    pub vertices: Vec<f32>,
    pub count: u32,
    pub maxcount: u32,
    buffer: (Arc<NotUpdatableBufferRange>, u32, u32),
    pub key: KeyVertexBuffer,
}
impl StateGeometryBuffer {
    pub const MAX_COUNT: u32 = 1024 * 1024;
    pub const FLOAT_PER_VERTEX: u32 = (3 + 4);
    pub const SIZE_PER_VERTEX: u32 = Self::FLOAT_PER_VERTEX * 4;
    pub fn buffer_desc(&self) -> VertexBufferDesc {
        VertexBufferDesc::new(
            self.key.clone(),
            VertexBufferDescRange::default(),
            vec![
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Position),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Color4),
            ],
            false,
        )
    }
    pub fn buffer(&self) -> Arc<NotUpdatableBufferRange> {
        self.buffer.0.clone()
    }
    pub fn new(
        maxbytes: u32, 
        allocator: &mut VertexBufferAllocator,
        device: &RenderDevice,
        queue: &RenderQueue,
    ) -> Option<Self> {
        let maxcount = maxbytes / Self::SIZE_PER_VERTEX;

        let size = maxbytes;
        let mut data = Vec::with_capacity(size as usize);
        for _ in 0..size {
            data.push(0);
        }
        // log::error!("StateGeometryBuffer {}", data.len());
        if let Some(buffer) = allocator.create_not_updatable_buffer_pre(device, queue, &data, None) {
            Some(Self {
                vertices: vec![],
                count: 0,
                maxcount: maxcount,
                buffer: (buffer, 0, size),
                key: KeyVertexBuffer::from("@SingleStateBuffer#@#@"),
            })
        } else {
            None
        }
    }
    pub fn after_collect(
        &mut self,
        queue: &RenderQueue,
    ) {
        if 0 < self.vertices.len()  {
            let buffer = self.buffer.0.buffer();
            queue.write_buffer(buffer, 0, bytemuck::cast_slice(&self.vertices));
            self.vertices.clear();
        }
    }
}

pub struct StateUIShader;
impl StateUIShader {
    pub const KEY: &'static str = "StateUIShader";
    pub fn res() -> ShaderEffectMeta {
        let mut nodemat = NodeMaterialBuilder::new();
        nodemat.values.uint_list.push(UniformPropertyUint(Atom::from("debug_normal"), 0, false));

        nodemat.vs = String::from("
        gl_Position = vec4(A_POSITION.xy, 0.5, 0.0);
        v_color = A_COLOR;
        ");
        nodemat.fs = String::from("
        gl_FragColor = v_color;
        ");

        nodemat.varyings = Varyings(
            vec![
                Varying { 
                    format: Atom::from("vec4"),
                    name: Atom::from("v_color"),
                },
            ]
        );

        nodemat.meta()
    }
}

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
    command.iter().for_each(|_v| { entitycount += 1; });

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
        "PSCount: {:?}, PSPerformance: {:?}, sys_emitmatrix: {:?}, sys_direction: {:?}, sys_update_buffer: {:?}, sys_update_buffer_trail: {:?}, sys_emission: {:?}, sys_emitter: {:?}, sys_force_over_life_time: {:?}, sys_gravity: {:?}",
        psperformance.particles, performance.particlesystem, psperformance.sys_emitmatrix, psperformance.sys_direction, psperformance.sys_update_buffer, psperformance.sys_update_buffer_trail
        , psperformance.sys_emission, psperformance.sys_emitter, psperformance.sys_force_over_life_time, psperformance.sys_gravity
    );
}

pub struct PluginBundleDefault;
impl Plugin for PluginBundleDefault{
    fn build(&self, app: &mut App) {
        // todo!()
        app.add_plugins(PluginRunstage)
        .add_plugins(PluginRunstage)
        .add_plugins(PluginGlobalAnimation)
        .add_plugins(PluginRenderBindGroup)
        .add_plugins(PluginScene)
        .add_plugins(PluginSceneAnimation)
        .add_plugins(PluginFlags)
        .add_plugins(PluginAnimeNodeEnable::new())
 
        .add_plugins(PluginTypeAnimatorableFloat::new())
        .add_plugins(PluginTypeAnimatorableVec2::new())
        .add_plugins(PluginTypeAnimatorableVec3::new())
        .add_plugins(PluginTypeAnimatorableVec4::new())
        .add_plugins(PluginTypeAnimatorableUint::new())
        .add_plugins(PluginTypeAnimatorableInt::new())

        .add_plugins(PluginGroupTransformNode)
        .add_plugins(PluginCamera)
        .add_plugins(PluginAnimeCameraFOV::new())
        .add_plugins(PluginAnimeCameraSize::new())
        .add_plugins(PluginMesh)
        .add_plugins(PluginAnimeRenderIndiceRange::new())
        .add_plugins(PluginGeometry)
        .add_plugins(PluginLighting)
        .add_plugins(PluginLayerMask)
        .add_plugins(PluginViewerBase)
        .add_plugins(PluginCulling)
        .add_plugins(PluginGroupMaterial)
        .add_plugins(PluginRenderer)
        .add_plugins(PluginPassObject)
        .add_plugins(PluginSkeleton)
        .add_plugins(PluginDefaultMaterial)
        .add_plugins(PluginDispose)
        .add_plugins(PluginStateGlobal)
        ;
    }
}
// impl Plugin for PluginBundleDefault {
//     fn build(self) -> PluginGroupBuilder {
//         let mut group = PluginGroupBuilder::start::<Self>();
        
//         group = group.add(PluginRunstage);
//         group = group.add(PluginGlobalAnimation);
//         group = group.add(PluginRenderBindGroup);
//         group = group.add(PluginScene);
//         group = group.add(PluginSceneAnimation);
//         group = group.add(PluginFlags);
//         group = group.add(PluginAnimeNodeEnable::new());
//         group = group.add(PluginTypeAnimatorableFloat::new());
//         group = group.add(PluginTypeAnimatorableVec2::new());
//         group = group.add(PluginTypeAnimatorableVec3::new());
//         group = group.add(PluginTypeAnimatorableVec4::new());
//         group = group.add(PluginTypeAnimatorableUint::new());
//         group = group.add(PluginTypeAnimatorableInt::new());
//         group = PluginGroupTransformNode::add(group);
//         group = group.add(PluginCamera)
//             .add(PluginAnimeCameraFOV::new())
//             .add(PluginAnimeCameraSize::new())
//             .add(PluginMesh)
//             // .add(PluginAnimeBoneOffset::new())
//             .add(PluginAnimeRenderIndiceRange::new())
//             .add(PluginGeometry)
//             .add(PluginLighting)
//             .add(PluginLayerMask)
//             .add(PluginViewerBase)
//             .add(PluginCulling);
//         group = PluginGroupMaterial::add(group);
//         group = group.add(PluginRenderer)
//             .add(PluginPassObject)
//             .add(PluginSkeleton)
//             .add(PluginDefaultMaterial)
//             .add(PluginDispose)
//             .add(PluginStateGlobal)
//             ;

//         group
//     }
    // fn init(
    //     &mut self,
    //     engine: &mut pi_scene_shell::engine_shell::EnginShell,
    //     stages: &mut pi_scene_shell::run_stage::RunStage,
    // ) -> Result<(), pi_scene_shell::plugin::ErrorPlugin> {
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
// }

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