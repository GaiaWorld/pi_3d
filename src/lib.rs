
use std::sync::Arc;
use pi_mesh_builder::cube::CubeBuilder;
use pi_scene_shell::{prelude::*, run_stage::PluginRunstage};
use pi_gltf2_load::{GLTFResLoader, ResGLTFRecords, GLTF};
use pi_node_materials::prelude::*;
use pi_trail_renderer::*;
use pi_particle_system::prelude::*;
use pi_particle_system::prelude::{ParticleSystemPerformance, ActionSetParticleSystem, ResourceParticleSystem};
use pi_scene_context::{
    animation::PluginSceneAnimation, cameras::PluginCamera, cullings::PluginCulling, geometry::{instance::{instanced_buffer::*, types::ModelInstanceAttributes}, PluginGeometry}, layer_mask::PluginLayerMask, light::PluginLighting, materials::PluginGroupMaterial, meshes::PluginMesh, prelude::*, renderers::PluginRenderer, scene::PluginScene, shadow::PluginShadowGenerator, skeleton::PluginSkeleton, transforms::{transform_node_sys::{TmpTransformWorldCalc0, TmpTransformWorldCalc1}, PluginGroupTransformNode}, viewer::PluginViewerBase
};
use pi_trail_renderer::{ActionSetTrailRenderer, ResTrailBuffer};

#[cfg(not(target_arch = "wasm32"))]
pub mod spector;

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
    pub capcity_inscommon: u32,
    pub capcity_combindata: u32,
    pub capcity_transformcalc: u32,
    pub capcity_gltfloader: u32,
    pub capcity_commands: u32,
    // pub scenes: XHashMap<Entity, StateScene>,
}

pub fn sys_state_resource(
    asset_gltf: Res<ShareAssetMgr<GLTF>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    bindbuffers: Res<ResBindBufferAllocator>,
    vertexbuffers: Res<VertexBufferAllocator3D>,
    shaders: Res<ShareAssetMgr<Shader3D>>,
    pipelines: Res<ShareAssetMgr<Pipeline3D>>,
    imagetextures: Res<ShareAssetMgr<ResImageTexture>>,
    shadermetas: Res<ShareAssetMgr<ShaderEffectMeta>>,
    passes: (
        // Query<&PassBindGroupScene>,
        // Query<&PassBindGroupModel>,
        // Query<&PassBindGroupTextureSamplers>,
        Query<&PassBindGroups>,
        Query<&PassShader>,
        Query<&PassPipeline>,
        Query<&PassDraw>,
        // Query<&PassBindEffectTextures>,
    ),
    mut stateglobal: ResMut<StateResource>,
    res: (
        Res<CombineBuffer>,
        Res<CombineDataCommon>,
        Res<TmpTransformWorldCalc0>,
        Res<TmpTransformWorldCalc1>,
        Res<ResImageTextureLoader>,
        Res<ImageTextureViewLoader2>,
    ),
    renderers: Query<&Renderer>,
    performance: Res<Performance>,
    instancedatas: Query<&ModelInstanceAttributes>,
    instancesource: Query<&InstancedMeshTransparentSortCollection>,
    combinebuffer: Res<CombineBuffer>,
    materials: Query<&BindEffect>,
) {
    // if performance.debug == false { return };
    let mut instancedatalen = 0;
    // instancedatas.iter().for_each(|item| {
    //     instancedatalen += item.bytes().len();
    // });
    instancesource.iter().for_each(|item| {
        instancedatalen += item.data.len();
    });
    instancedatalen += combinebuffer.data.size();

    let mut materialdata = 0;
    materials.iter().for_each(|item| {
        if let Some(item) = &item.0 {
            materialdata += item.data().len();
        }
    });

    stateglobal.count_gltf              = asset_gltf.len();
    stateglobal.count_bindbuffer        = bindbuffers.asset_mgr().len();
    stateglobal.mem_bindbuffer          = bindbuffers.memsize() + materialdata;
    stateglobal.count_bindgroup         = asset_mgr_bindgroup.0.len();
    stateglobal.count_pipeline          = pipelines.len();
    stateglobal.count_geometrybuffer    = vertexbuffers.total_buffer_count();
    stateglobal.size_geometrybuffer     = vertexbuffers.total_buffer_size() + instancedatalen as u64;
    stateglobal.count_shader            = shaders.len();
    stateglobal.mem_shader              = shaders.size();
    stateglobal.count_imgtexture        = imagetextures.len();
    stateglobal.mem_imgtexture          = imagetextures.size() + res.4.size();
    stateglobal.count_shadermeta        = shadermetas.len();
    stateglobal.mem_shadermeta          = shadermetas.size();
    stateglobal.capcity_inscommon       = res.0.size() as u32;
    stateglobal.capcity_combindata      = res.1.size() as u32;
    stateglobal.capcity_transformcalc   = res.2.memsize() as u32 + res.3.memsize() as u32;

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

    let mut count_vertex = 0;
    renderers.iter().for_each(|renderer| {
        count_vertex += renderer.vertexs;
    });
    stateglobal.count_vertex = count_vertex as u32;
}

pub struct PluginStateGlobal;
impl Plugin for PluginStateGlobal {
    fn build(&self, app: &mut App) {
        app.insert_resource(Performance::default());
        app.insert_resource(StateResource::default());

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update,
            sys_state_resource.in_set(ERunStageChap::StateCheck)
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(StageD3Final, sys_state_resource.in_set(ERunStageChap::StateCheck))
        ;

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
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Position, wgpu::VertexFormat::Float32x3),
                EVertexAttribute::Buildin(EBuildinVertexAtribute::Color4, wgpu::VertexFormat::Float32x4),
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
    pub fn res(engineopt: &EngineCustomPlugins) -> ShaderEffectMeta {
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

        nodemat.meta(engineopt)
    }
}

pub fn sys_info_node(
    _scenes: Query<Entity, With<SceneTime>>,
    _states: Res<StateResource>,
) {
    // _scenes.iter().for_each(|entity| {
    //     if let Some(state) = _states.scenes.get(&entity) {
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

    pub fn info(v: EError) -> String {
        match v {
00001 => { String::from("ERROR_UNKOWN") }
00002 => { String::from("ERROR_VERTEX_BUFFER_CREATE_FAIL") }
00003 => { String::from("ERROR_BIND_BUFFER_CREATE_FAIL") }
00004 => { String::from("ERROR_BIND_GROUP_CREATE_FAIL") }
00005 => { String::from("ERROR_SHADER_CREATE_FAIL") }
00006 => { String::from("ERROR_PIPELINE_CREATE_FAIL") }
00007 => { String::from("ERROR_TEXTURE_CREATE_FAIL") }
00008 => { String::from("ERROR_TEXTURE_VIEW_CREATE_FAIL") }
00009 => { String::from("ERROR_SAMPER_CREATE_FAIL") }
00010 => { String::from("ERROR_BIND_VIEWER_CREATE_FAIL") }
00011 => { String::from("ERROR_BIND_EFFECT_CREATE_FAIL") }
00012 => { String::from("ERROR_MODIFY_ERROR_MATERIAL_TEXTURE") }
00013 => { String::from("ERROR_MATERIAL_SHADER_NOTFOUND") }
00014 => { String::from("ERROR_USE_MATERIAL_NULL_MAT") }
00015 => { String::from("ERROR_USE_MATERIAL_NULL_TARGET") }
00016 => { String::from("ERROR_TEXTURE_CACHE_FAIL") }
00017 => { String::from("ERROR_TEXTURE_CANT_LOAD_FROM_DATA") }
00018 => { String::from("ERROR_TEXTURE_LOAD_FAIL") }
00019 => { String::from("ERROR_TEXTURE_COMBINE_FAIL") }
00019 => { String::from("ERROR_TEXTURE_FROM_KTX_FAIL") }
00100 => { String::from("ERROR_ANIMATION_START_FAIL") }
00101 => { String::from("ERROR_ANIMATION_PAUSE_FAIL") }
00102 => { String::from("ERROR_ANIMATION_STOP_FAIL") }
00103 => { String::from("ERROR_ADD_TARGET_ANIMATION_FAIL") }
00100 => { String::from("ERROR_GRAPHIC_NONE_NGRAPHIC") }
00101 => { String::from("ERROR_GRAPHIC_NONE_NODE") }
00102 => { String::from("ERROR_GRAPHIC_EXTI_NODE") }
00103 => { String::from("ERROR_GRAPHIC_RUN_ERR") }
00104 => { String::from("ERROR_GRAPHIC_BUILD_ERR") }
00105 => { String::from("ERROR_GRAPHIC_INPUT_ERR") }
00106 => { String::from("ERROR_GRAPHIC_OUTPUT_ERR") }
00107 => { String::from("ERROR_GRAPHIC_CUSTOM_BUILD_ERR") }
00108 => { String::from("ERROR_GRAPHIC_CUSTOM_RUN_ERR") }
00109 => { String::from("ERROR_GRAPHIC_WRONG_NODE_TYPE") }
00110 => { String::from("ERROR_GRAPHIC_MISMATCH_PARAM") }
00111 => { String::from("ERROR_SUB_GRAPHIC_ERROR") }
00200 => { String::from("ERROR_GLTF_BIN_LOAD_FAIL") }
00201 => { String::from("ERROR_GLTF_BUFFER") }
00202 => { String::from("ERROR_GLTF_ACCESSOR") }
00203 => { String::from("ERROR_GLTF_IMAGE") }
00204 => { String::from("ERROR_GLTF_GLTF_LOAD") }
00205 => { String::from("ERROR_GLTF_GLTF_PARSE") }
00207 => { String::from("ERROR_GLTF_GLTF_CACHE") }
00208 => { String::from("ERROR_GLTF_VERTEX_BUFFER") }
00209 => { String::from("ERROR_GLTF_ANIMATION") }
10001 => { String::from("ERROR_ENTITY_NONE") }
10002 => { String::from("ERROR_ENTITY_DISPOSED") }
20001 => { String::from("ERROR_SCENE_NONE") }
20002 => { String::from("ERROR_SCENE_BIND_FAIL") }
20003 => { String::from("ERROR_ENVIRONMENT_INFO_PARSE") }
20004 => { String::from("ERROR_ENVIRONMENT_INFO_MAGICNUMBER") }
20100 => { String::from("ERROR_RENDERER_NOT_FOUND") }
50000 => { String::from("ERROR_PASS_BIND_SCENE_NONE") }
50001 => { String::from("ERROR_PASS_BIND_VIEWER_NONE") }
50002 => { String::from("ERROR_PASS_SET0_FAIL") }
50003 => { String::from("ERROR_PASS_BIND_MODEL_NONE") }
50004 => { String::from("ERROR_PASS_BIND_EFFECT_VALUE_NONE") }
50005 => { String::from("ERROR_PASS_BIND_LIGHTING_NONE") }
50006 => { String::from("ERROR_PASS_BIND_SKIN_NONE") }
50007 => { String::from("ERROR_PASS_SET1_FAIL") }
50008 => { String::from("ERROR_PASS_SET2_FAIL") }
50010 => { String::from("ERROR_PASS_BIND_SHADOW_NONE") }
50011 => { String::from("ERROR_PASS_BIND_BRDF_NONE") }
50012 => { String::from("ERROR_PASS_BIND_CAMERA_OPAQUE_NONE") }
50013 => { String::from("ERROR_PASS_BIND_CAMERA_DEPTH_NONE") }
50014 => { String::from("ERROR_PASS_BIND_ENV_NONE") }
50015 => { String::from("ERROR_PASS_SET3_FAIL") }
50016 => { String::from("ERROR_PASS_BIND_GROUPS_FAIL") }
50017 => { String::from("ERROR_PASS_SHADER_FAIL") }
50018 => { String::from("ERROR_PASS_PIPELINE_FAIL") }
50019 => { String::from("ERROR_PASS_DRAW_FAIL") }
50020 => { String::from("ERROR_PASS_BIND_VELOCITY_NONE") }
50021 => { String::from("ERROR_PASS_BIND_MODEL_INV_NONE") }
50022 => { String::from("ERROR_PASS_BIND_MORPH_NONE") }
50023 => { String::from("ERROR_PASS_BIND_SKININS_NONE") }
_ => { String::from("N") }
        }
    }
pub fn sys_info_draw(
    draws: Query<(
        // &PassBindGroupScene, &PassBindGroupModel, &PassBindEffectValue,
        // &PassBindEffectTextures,
        &PassShader, &PassBindGroups, &PassPipeline, &PassDraw
    )>,
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
    draws.iter().for_each(|(
        // bindgroup_scene, bindgroup_model,bindeffect,
        // bindtextures,
        shader, bindgroups, pipeline, draw)| {
        // if bindtextures.val().is_some() { count_textures += 1; }
        if bindgroups.val().is_some() { count_bindgroups += 1; }

        // if bindgroup_scene.is_some() { count_set0 += 1; }
        // if bindgroup_model.is_some() { count_set1 += 1; }
        // if bindeffect.0.is_some() { count_effect += 1; }

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
    mut actions: ActionSets,
    resource: ResourceSets,
    states: Res<StateResource>,
    psperformance: Res<ParticleSystemPerformance>,
    mut performance: ResMut<Performance>,
    errors: Res<ResErrorRecord>,
    cmd: Commands,
    command: Query<Entity>,
    actinstance: Res<ActionListInstanceMeshCreate>,
) {
    let mut entitycount = 0;
    command.iter().for_each(|_v| { entitycount += 1; });
    // log::warn!("Errors {:?}", errors.0.len());
    if performance.systems.len() > 0 {
        let mut str = String::from("");
        performance.systems.iter().for_each(|sys| {
            str += sys; str += "\n";
        });
        
        let temp = String::from("temp/");
        let root_dir = std::env::current_dir().unwrap();
        let file_name = temp.clone() + "systems.md";
        let _ = std::fs::write(root_dir.join(file_name), str);
    }
    // performance.systems.clear();
    performance.debug = true;
    
    let _ = actions.disposeref.drain();
    // log::warn!("DrawCall: {:?} WorldMatrix: {:?} DrawList {:?} Culling {:?} Uniform: {:?}", performance.drawcalls, performance.worldmatrix, performance.drawobjs, performance.culling, (performance.uniformupdate, performance.uniformbufferupdate));
    log::warn!(
        "WorldMem: {:?}, Materials: {:?}, BindBuffer: {:?}, VertexBufferSize: {:?}, Shaders: {:?}, Pipeline: {:?}, ImageTexture: {:?},",
        (entitycount, actions.memsize(), resource.memsize()), actinstance.memsize(), states.mem_bindbuffer, states.size_geometrybuffer, states.mem_shader, states.count_pipeline, states.mem_imgtexture
    );
    // log::warn!(
    //     "PSCount: {:?}, PSPerformance: {:?}, sys_emitmatrix: {:?}, sys_direction: {:?}, sys_update_buffer: {:?}, sys_update_buffer_trail: {:?}, sys_emission: {:?}, sys_emitter: {:?}, sys_force_over_life_time: {:?}, sys_prewarm: {:?}",
    //     psperformance.particles, performance.particlesystem, psperformance.sys_emitmatrix, psperformance.sys_direction, psperformance.sys_update_buffer, psperformance.sys_update_buffer_trail
    //     , psperformance.sys_emission, psperformance.sys_emitmatrix, psperformance.sys_over_life_time, psperformance.sys_prewarm
    // );
}

pub fn sys_info_error(
    mut states: ResMut<ResErrorRecord>,
) {
    states.1 = true;
    if (states.0.len() > 0) {
        log::error!("Errors: ");
    }
    states.0.drain(..).for_each(|err| {
        log::error!("Error: {:?}", info(err));
    });
}

pub struct PluginBundleDefault;
impl PluginBundleDefault {
    pub fn add(mut app: &mut App) -> &mut App {
        app
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
        .add_plugins(PluginTypeAnimatorableInt::new());

        app = PluginGroupTransformNode::add(app);
        app = app.add_plugins(PluginCamera)
            .add_plugins(PluginAnimeCameraFOV::new())
            .add_plugins(PluginAnimeCameraSize::new())
            .add_plugins(PluginMesh)
            .add_plugins(PluginSprite)
            // .add(PluginAnimeBoneOffset::new())
            .add_plugins(PluginAnimeRenderIndiceRange::new())
            .add_plugins(PluginGeometry)
            .add_plugins(PluginLighting)
            .add_plugins(PluginLayerMask)
            .add_plugins(PluginViewerBase)
            .add_plugins(PluginCulling);

        app = PluginGroupMaterial::add(app);
        app = app.add_plugins(PluginRenderer)
            .add_plugins(PluginPassObject)
            .add_plugins(PluginSkeleton)
            .add_plugins(PluginDefaultMaterial)
            .add_plugins(PluginDispose)
            .add_plugins(PluginStateGlobal)
            ;

        let entity = app.world.get_resource_mut::<SingleIDBaseDefaultMaterial>().unwrap().0;
        app.world.get_resource_mut::<ActionListMaterialCreate>().unwrap().push(OpsMaterialCreate::ops_with_matarray(entity, DefaultShader::KEY));
        app.world.get_resource_mut::<ActionListUniformVal>().unwrap().push(OpsUniformVal::ops(entity, EUniformVal::Float(Atom::from(BlockOpacity::KEY_ALPHA), 0.33)));
        app.world.get_resource_mut::<ActionListUniformVal>().unwrap().push(OpsUniformVal::ops(entity, EUniformVal::Vec3(Atom::from(BlockMainTexture::KEY_COLOR), 0.89, 0., 0.5)));

        app
    }
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
}

#[derive(SystemParam)]
pub struct ActionSetAnimation<'w> {
    pub anime_instance: ResMut<'w, ActionListTargetAnimationAttribute>,
    pub anime_sint: ResMut<'w, ActionListAnimatorableSint>,
    pub anime_float: ResMut<'w, ActionListAnimatorableFloat>,
    pub anime_uint: ResMut<'w, ActionListAnimatorableUint>,
    pub anime_vec2: ResMut<'w, ActionListAnimatorableVec2>,
    pub anime_vec3: ResMut<'w, ActionListAnimatorableVec3>,
    pub anime_vec4: ResMut<'w, ActionListAnimatorableVec4>,
}
impl<'w> MemSize for ActionSetAnimation<'w> {
    fn memsize(&self) -> usize {
        self.anime_instance.memsize()
        + self.anime_sint.memsize()
        + self.anime_float.memsize()
        + self.anime_uint.memsize()
        + self.anime_vec2.memsize()
        + self.anime_vec3.memsize()
        + self.anime_vec4.memsize()
    }
}

pub trait TActionSet {
    fn scene_create(&mut self) -> &mut ActionListSceneCreate;
    fn scene_options(&mut self) -> &mut ActionListSceneOption;
    fn scene_dispose(&mut self) -> &mut ActionListSceneDispose;
    fn scene_boundingbox(&mut self) -> &mut ActionListBoundingBoxDisplay;
    fn scene_collider(&mut self) -> &mut ActionListCollider;
    fn obj_dispose(&mut self) -> &mut ActionListDispose;
    fn transform_create(&mut self) -> &mut ActionListTransformNodeCreate;
    fn transform_localsrt(&mut self) -> &mut ActionListTransformNodeLocal;
    fn transform_localrotq(&mut self) -> &mut ActionListTransformNodeLocalRotationQuaternion;
    fn transform_tree(&mut self) -> &mut ActionListTransformNodeParent;
    fn transform_enable(&mut self) -> &mut ActionListNodeEnable;
    fn camera_create(&mut self) -> &mut ActionListCameraCreate;
    fn camera_param(&mut self) -> &mut ActionListCameraModify;
    fn camera_target(&mut self) -> &mut ActionListCameraTarget;
    fn camera_forceinclude(&mut self) -> &mut ActionListViewerForceInclude;
    fn mesh_create(&mut self) -> &mut ActionListMeshCreate;
    fn mesh_render_state(&mut self) -> &mut ActionListRenderState;
    fn mesh_pose(&mut self) -> &mut ActionListAbstractMeshPose;
    fn mesh_state(&mut self) -> &mut ActionListMeshStateModify;
    fn mesh_valuestate(&mut self) -> &mut ActionListAbstructMeshValueStateModify;
    fn mesh_bounding(&mut self) -> &mut ActionListMeshBounding;
    fn forcelighting(&mut self) -> &mut ActionListMeshForceLighting;
    fn skin_create(&mut self) -> &mut ActionListSkinCreate;
    fn skin_use(&mut self) -> &mut ActionListSkinUse;
    fn skin_bonecreate(&mut self) -> &mut ActionListBoneCreate;
    fn skin_bonepose(&mut self) -> &mut ActionListBonePose;
    fn mesh_layermask(&mut self) -> &mut ActionListLayerMask;
    fn instance_create(&mut self) -> &mut ActionListInstanceMeshCreate;
    fn instance_attr(&mut self) -> &mut ActionListInstanceAttr;
    fn instance_targetanime(&mut self) -> &mut ActionListTargetAnimationAttribute;
    fn geometry_create(&mut self) -> &mut ActionListGeometryCreate;
    fn material_usemat(&mut self) -> &mut ActionListMaterialUse;
    fn material_create(&mut self) -> &mut ActionListMaterialCreate;
    fn material_val(&mut self) -> &mut ActionListUniformVal;
    fn material_valb(&mut self) -> &mut ActionListUniformValB;
    fn light_create(&mut self) -> &mut ActionListLightCreate;
    fn light_param(&mut self) -> &mut ActionListLightParam;
    fn shadow_param(&mut self) -> &mut ActionListShadowGeneratorParam;
    fn shadow_create(&mut self) -> &mut ActionListShadowGenerator;
    fn renderer_subgraph(&mut self) -> &mut ActionListSubGraphCreate;
    fn renderer_create(&mut self) -> &mut ActionListRendererCreate;
    fn renderer_connect(&mut self) -> &mut ActionListRendererConnect;
    fn renderer_modify(&mut self) -> &mut ActionListRendererModify;
    fn renderer_target(&mut self) -> &mut ActionListRendererTarget;
    fn anime_create(&mut self) -> &mut ActionListAnimeGroupCreate;
    fn anime_action(&mut self) -> &mut ActionListAnimationGroupAction;
    fn anime_dispose(&mut self) -> &mut ActionListAnimeGroupDispose;
    fn anime_reset_while_start(&mut self) -> &mut ActionListAnimeGroupStartReset;
    fn anime_property_targetanime(&mut self) -> &mut ActionListPropertyTargetAnimation;
    fn anime_goto(&mut self) -> &mut ActionListAnimationGroupGoto;
    fn anime_float(&mut self) -> &mut ActionListAnimatorableFloat;
    fn anime_sint(&mut self) -> &mut ActionListAnimatorableSint;
    fn anime_uint(&mut self) -> &mut ActionListAnimatorableUint;
    fn anime_vec2(&mut self) -> &mut ActionListAnimatorableVec2;
    fn anime_vec3(&mut self) -> &mut ActionListAnimatorableVec3;
    fn anime_vec4(&mut self) -> &mut ActionListAnimatorableVec4;
    fn trail_create(&mut self) -> &mut ActionListTrail;
    fn trail_age(&mut self) -> &mut ActionListTrailAge;
    fn parsys_calculator(&mut self) -> &mut ActionListCPUParticleCalculator;
    fn parsys_create(&mut self) -> &mut ActionListCPUParticleSystem;
    fn parsys_state(&mut self) -> &mut ActionListCPUParticleSystemState;
    fn parsys_trailmaterial(&mut self) -> &mut ActionListCPUParticleSystemTrailMaterial;
    fn sprite_create(&mut self) -> &mut ActionListSpriteCreate;
    fn sprite_modify(&mut self) -> &mut ActionListSpriteModify;
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
    // pub abstructmesh: ActionSetAbstructMesh<'w>,
    pub instance: ActionSetInstanceMesh<'w>,
    pub geometry: ActionSetGeometry<'w>,
    pub material: ActionSetMaterial<'w>,
    pub anime: ActionSetAnimationGroup<'w>,
    pub animation: ActionSetAnimation<'w>,
    pub renderer: ActionSetRenderer<'w>,
    pub trail: ActionSetTrailRenderer<'w>,
    pub parsys: ActionSetParticleSystem<'w>,
    pub property_targetanimation: ResMut<'w, ActionListPropertyTargetAnimation>,
    pub spritecreate: ResMut<'w, ActionListSpriteCreate>,
    pub spritemodify: ResMut<'w, ActionListSpriteModify>,
    pub disposeref: ResMut<'w, ActionListDisposeReadyForRef>,
}
impl<'w> MemSize for ActionSets<'w> {
    fn memsize(&self) -> usize {
        self.scene.memsize()
        + self.scene_dispose.memsize()
        + self.obj_dispose.memsize()
        + self.camera.memsize()
        + self.light.memsize()
        + self.shadow.memsize()
        + self.transform.memsize()
        + self.mesh.memsize()
        + self.skin.memsize()
        + self.instance.memsize()
        + self.geometry.memsize()
        + self.material.memsize()
        + self.anime.memsize()
        + self.animation.memsize()
        + self.renderer.memsize()
        + self.trail.memsize()
        + self.parsys.memsize()
        + self.property_targetanimation.memsize()
        + self.spritecreate.memsize()
        + self.spritemodify.memsize()
        + self.disposeref.memsize()
    }
}
impl<'w> ActionSets<'w> {
    pub fn record(&self, result: &mut [f64], mut offset: usize) -> usize {
        offset += 0; result[offset] = self.scene.memsize() as f64;
        offset += 1; result[offset] = self.scene_dispose.memsize() as f64;
        offset += 1; result[offset] = self.obj_dispose.memsize() as f64;
        offset += 1; result[offset] = self.camera.memsize() as f64;
        offset += 1; result[offset] = self.light.memsize() as f64;
        offset += 1; result[offset] = self.shadow.memsize() as f64;
        offset += 1; result[offset] = self.transform.memsize() as f64;
        offset += 1; result[offset] = self.mesh.memsize() as f64;
        offset += 1; result[offset] = self.skin.memsize() as f64;
        offset += 1; result[offset] = self.instance.memsize() as f64;
        offset += 1; result[offset] = self.geometry.memsize() as f64;
        offset += 1; result[offset] = self.material.memsize() as f64;
        offset += 1; result[offset] = self.anime.memsize() as f64;
        offset += 1; result[offset] = self.animation.memsize() as f64;
        offset += 1; result[offset] = self.renderer.memsize() as f64;
        offset += 1; result[offset] = self.trail.memsize() as f64;
        offset += 1; result[offset] = self.parsys.memsize() as f64;
        offset += 1; result[offset] = self.property_targetanimation.memsize() as f64;
        offset += 1; result[offset] = self.spritecreate.memsize() as f64;
        offset += 1; result[offset] = self.spritemodify.memsize() as f64;
        offset += 1; result[offset] = self.disposeref.memsize() as f64;
        offset + 1
    }
}

impl<'w> TActionSet for ActionSets<'w> {
    fn scene_create(&mut self) -> &mut ActionListSceneCreate {
        &mut self.scene.create
    }

    fn scene_options(&mut self) -> &mut ActionListSceneOption {
        &mut self.scene.options
    }

    fn scene_dispose(&mut self) -> &mut ActionListSceneDispose {
        &mut self.scene_dispose
    }

    fn scene_boundingbox(&mut self) -> &mut ActionListBoundingBoxDisplay {
        &mut self.scene.boundingboxdisplay
    }

    fn scene_collider(&mut self) -> &mut ActionListCollider {
        &mut self.scene.collider
    }

    fn obj_dispose(&mut self) -> &mut ActionListDispose {
        &mut self.obj_dispose
    }

    fn transform_create(&mut self) -> &mut ActionListTransformNodeCreate {
        &mut self.transform.create
    }

    fn transform_localsrt(&mut self) -> &mut ActionListTransformNodeLocal {
        &mut self.transform.localsrt
    }

    fn transform_localrotq(&mut self) -> &mut ActionListTransformNodeLocalRotationQuaternion {
        &mut self.transform.localrotq
    }

    fn transform_tree(&mut self) -> &mut ActionListTransformNodeParent {
        &mut self.transform.tree
    }

    fn transform_enable(&mut self) -> &mut ActionListNodeEnable {
        &mut self.transform.enable
    }

    fn camera_create(&mut self) -> &mut ActionListCameraCreate {
        &mut self.camera.create
    }

    fn camera_param(&mut self) -> &mut ActionListCameraModify {
        &mut self.camera.param
    }

    fn camera_target(&mut self) -> &mut ActionListCameraTarget {
        &mut self.camera.target
    }

    fn camera_forceinclude(&mut self) -> &mut ActionListViewerForceInclude {
        &mut self.camera.forceinclude
    }

    fn mesh_create(&mut self) -> &mut ActionListMeshCreate {
        &mut self.mesh.create
    }

    fn mesh_render_state(&mut self) -> &mut ActionListRenderState {
        &mut self.mesh.render_state
    }

    fn mesh_pose(&mut self) -> &mut ActionListAbstractMeshPose {
        &mut self.mesh.pose
    }

    fn mesh_state(&mut self) -> &mut ActionListMeshStateModify {
        &mut self.mesh.state
    }

    fn mesh_valuestate(&mut self) -> &mut ActionListAbstructMeshValueStateModify {
        &mut self.mesh.value_state
    }

    fn mesh_bounding(&mut self) -> &mut ActionListMeshBounding {
        &mut self.mesh.bounding
    }

    fn mesh_layermask(&mut self) -> &mut ActionListLayerMask {
        &mut self.mesh.layermask
    }

    fn forcelighting(&mut self) -> &mut ActionListMeshForceLighting {
        &mut self.mesh.forcelighting
    }

    fn skin_create(&mut self) -> &mut ActionListSkinCreate {
        &mut self.skin.skin_create
    }

    fn skin_use(&mut self) -> &mut ActionListSkinUse {
        &mut self.skin.skin_use
    }

    fn skin_bonecreate(&mut self) -> &mut ActionListBoneCreate {
        &mut self.skin.bone_create
    }

    fn skin_bonepose(&mut self) -> &mut ActionListBonePose {
        &mut self.skin.bone_pose
    }

    fn instance_create(&mut self) -> &mut ActionListInstanceMeshCreate {
        &mut self.instance.create
    }

    fn instance_attr(&mut self) -> &mut ActionListInstanceAttr {
        &mut self.instance.attr
    }

    fn instance_targetanime(&mut self) -> &mut ActionListTargetAnimationAttribute {
        &mut self.animation.anime_instance
    }

    fn geometry_create(&mut self) -> &mut ActionListGeometryCreate {
        &mut self.geometry.create
    }

    fn material_usemat(&mut self) -> &mut ActionListMaterialUse {
        &mut self.material.usemat
    }

    fn material_create(&mut self) -> &mut ActionListMaterialCreate {
        &mut self.material.create
    }

    fn material_val(&mut self) -> &mut ActionListUniformVal {
        &mut self.material.val
    }

    fn material_valb(&mut self) -> &mut ActionListUniformValB {
        &mut self.material.valb
    }

    fn light_create(&mut self) -> &mut ActionListLightCreate {
        &mut self.light.create
    }

    fn light_param(&mut self) -> &mut ActionListLightParam {
        &mut self.light.param
    }

    fn shadow_param(&mut self) -> &mut ActionListShadowGeneratorParam {
        &mut self.shadow.param
    }

    fn shadow_create(&mut self) -> &mut ActionListShadowGenerator {
        &mut self.shadow.create
    }

    fn renderer_subgraph(&mut self) -> &mut ActionListSubGraphCreate {
        &mut self.renderer.subgraph
    }

    fn renderer_create(&mut self) -> &mut ActionListRendererCreate {
        &mut self.renderer.create
    }

    fn renderer_connect(&mut self) -> &mut ActionListRendererConnect {
        &mut self.renderer.connect
    }

    fn renderer_modify(&mut self) -> &mut ActionListRendererModify {
        &mut self.renderer.modify
    }

    fn renderer_target(&mut self) -> &mut ActionListRendererTarget {
        &mut self.renderer.target
    }

    fn anime_create(&mut self) -> &mut ActionListAnimeGroupCreate {
        &mut self.anime.create
    }

    fn anime_action(&mut self) -> &mut ActionListAnimationGroupAction {
        &mut self.anime.action
    }

    fn anime_dispose(&mut self) -> &mut ActionListAnimeGroupDispose {
        &mut self.anime.dispose
    }

    fn anime_reset_while_start(&mut self) -> &mut ActionListAnimeGroupStartReset {
        &mut self.anime.reset_while_start
    }

    fn anime_property_targetanime(&mut self) -> &mut ActionListPropertyTargetAnimation {
        &mut self.property_targetanimation
    }

    fn anime_goto(&mut self) -> &mut ActionListAnimationGroupGoto {
        &mut self.anime.goto
    }

    fn anime_float(&mut self) -> &mut ActionListAnimatorableFloat {
        &mut self.animation.anime_float
    }

    fn anime_sint(&mut self) -> &mut ActionListAnimatorableSint {
        &mut self.animation.anime_sint
    }

    fn anime_uint(&mut self) -> &mut ActionListAnimatorableUint {
        &mut self.animation.anime_uint
    }

    fn anime_vec2(&mut self) -> &mut ActionListAnimatorableVec2 {
        &mut self.animation.anime_vec2
    }

    fn anime_vec3(&mut self) -> &mut ActionListAnimatorableVec3 {
        &mut self.animation.anime_vec3
    }

    fn anime_vec4(&mut self) -> &mut ActionListAnimatorableVec4 {
        &mut self.animation.anime_vec4
    }

    fn trail_create(&mut self) -> &mut ActionListTrail {
        &mut self.trail.create
    }

    fn trail_age(&mut self) -> &mut ActionListTrailAge {
        &mut self.trail.age
    }

    fn parsys_calculator(&mut self) -> &mut ActionListCPUParticleCalculator {
        &mut self.parsys.calculator
    }

    fn parsys_create(&mut self) -> &mut ActionListCPUParticleSystem {
        &mut self.parsys.create
    }

    fn parsys_state(&mut self) -> &mut ActionListCPUParticleSystemState {
        &mut self.parsys.state
    }

    fn parsys_trailmaterial(&mut self) -> &mut ActionListCPUParticleSystemTrailMaterial {
        &mut self.parsys.trailmaterial
    }

    fn sprite_create(&mut self) -> &mut ActionListSpriteCreate {
        &mut self.spritecreate
    }

    fn sprite_modify(&mut self) -> &mut ActionListSpriteModify {
        &mut self.spritemodify
    }
}

#[derive(SystemParam)]
pub struct ResourceSets<'w> {
    pub default_mat: Res<'w, SingleIDBaseDefaultMaterial>,
    pub node_material_blocks: ResMut<'w, NodeMaterialBlocks>,
    pub imgtex_loader: ResMut<'w, ResImageTextureLoader>,
    pub imgtex_loader_state: ResMut<'w, ResStateTextureLoader>,
    pub imgtex_asset: Res<'w, ShareAssetMgr<ImageTextureFrame>>,
    pub imgtexview_asset: Res<'w, ShareAssetMgr<ImageTextureViewFrame>>,
    pub gltf2_asset: Res<'w, ShareAssetMgr<GLTF>>,
    pub gltf2_records: ResMut<'w, ResGLTFRecords>,
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
    pub error_record: ResMut<'w, ResErrorRecord>,
    pub textureatlas: ResMut<'w, ResSpriteFrames>,
    pub enginopt: Res<'w, EngineCustomPlugins>,
    pub combinebuffer: Res<'w, CombineBuffer>,
    pub commondata: Res<'w, CombineDataCommon>,
    pub matrix0: Res<'w, TmpTransformWorldCalc0>,
    pub matrix1: Res<'w, TmpTransformWorldCalc1>,
    pub texloader2: Res<'w, ImageTextureViewLoader2>,
    pub vballocator: Res<'w, VertexBufferAllocator3D>,
    pub texcombine: ResMut<'w, ResTextureCombineAtlas2DMgr>,
}
impl<'w> MemSize for ResourceSets<'w> {
    fn memsize(&self) -> usize {
        8
        + self.node_material_blocks.memsize()
        + self.imgtex_loader.memsize()
        + self.imgtex_loader_state.memsize()
        + self.imgtex_asset.size()
        + self.imgtexview_asset.size()
        + self.gltf2_asset.size()
        + self.anime_assets.memsize()
        + self.anime_contexts.memsize()
        + self.render_targets.memsize()
        + self.asset_samp.size()
        + self.asset_atlas.0.size()
        + self.scene_lighting_limit.memsize()
        + self.model_lighting_limit.memsize()
        + self.scene_shadow_limit.memsize()
        + self.vb_mgr.size()
        + self.vb_wait.size()
        + self.shader_metas.size()
        + self.anime_global.memsize()
        + self.anime_events.memsize()
        + self.trailbuffer.memsize()
        + self.particlesys.memsize()
        + self.error_record.memsize()
        + self.textureatlas.memsize()
        + self.enginopt.memsize()
        + self.combinebuffer.memsize()
        + self.commondata.memsize()
        + self.matrix0.memsize()
        + self.matrix1.memsize()
        + self.texloader2.memsize() 
        + self.vballocator.total_buffer_size() as usize
    }
}
impl<'w> ResourceSets<'w> {
    pub fn record(&self, result: &mut [f64], mut offset: usize) -> usize {
        offset += 0; result[offset] = self.node_material_blocks.memsize() as f64;
        offset += 1; result[offset] = self.imgtex_loader.memsize() as f64;
        offset += 1; result[offset] = self.imgtex_loader_state.memsize() as f64;
        offset += 1; result[offset] = self.imgtex_asset.size() as f64;
        offset += 1; result[offset] = self.imgtexview_asset.size() as f64;
        offset += 1; result[offset] = self.gltf2_asset.size() as f64;
        offset += 1; result[offset] = self.anime_assets.memsize() as f64;
        offset += 1; result[offset] = self.anime_contexts.memsize() as f64;
        offset += 1; result[offset] = self.render_targets.memsize() as f64;
        offset += 1; result[offset] = self.asset_samp.size() as f64;
        offset += 1; result[offset] = self.asset_atlas.0.size() as f64;
        offset += 1; result[offset] = self.scene_lighting_limit.memsize() as f64;
        offset += 1; result[offset] = self.model_lighting_limit.memsize() as f64;
        offset += 1; result[offset] = self.scene_shadow_limit.memsize() as f64;
        offset += 1; result[offset] = self.vb_mgr.size() as f64;
        offset += 1; result[offset] = self.vb_wait.size() as f64;
        offset += 1; result[offset] = self.shader_metas.size() as f64;
        offset += 1; result[offset] = self.anime_global.memsize() as f64;
        offset += 1; result[offset] = self.anime_events.memsize() as f64;
        offset += 1; result[offset] = self.trailbuffer.memsize() as f64;
        offset += 1; result[offset] = self.particlesys.memsize() as f64;
        offset += 1; result[offset] = self.error_record.memsize() as f64;
        offset += 1; result[offset] = self.textureatlas.memsize() as f64;
        offset += 1; result[offset] = self.enginopt.memsize() as f64;
        offset += 1; result[offset] = self.combinebuffer.memsize() as f64;
        offset += 1; result[offset] = self.commondata.memsize() as f64;
        offset += 1; result[offset] = self.matrix0.memsize() as f64;
        offset += 1; result[offset] = self.matrix1.memsize() as f64;
        offset += 1; result[offset] = self.texloader2.memsize() as f64;
        offset += 1; result[offset] = self.vballocator.total_buffer_size() as f64;
        offset + 1
    }
}

impl TActionSet for World {
    fn scene_create(&mut self) -> &mut pi_scene_context::prelude::ActionListSceneCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListSceneCreate>().unwrap()
    }

    fn scene_options(&mut self) -> &mut pi_scene_context::prelude::ActionListSceneOption {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListSceneOption>().unwrap()
    }

    fn scene_dispose(&mut self) -> &mut pi_scene_shell::prelude::ActionListSceneDispose {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListSceneDispose>().unwrap()
    }

    fn scene_boundingbox(&mut self) -> &mut pi_scene_context::prelude::ActionListBoundingBoxDisplay {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListBoundingBoxDisplay>().unwrap()
    }

    fn scene_collider(&mut self) -> &mut pi_scene_context::prelude::ActionListCollider {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListCollider>().unwrap()
    }

    fn obj_dispose(&mut self) -> &mut pi_scene_context::prelude::ActionListDispose {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListDispose>().unwrap()
    }

    fn transform_create(&mut self) -> &mut pi_scene_context::prelude::ActionListTransformNodeCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListTransformNodeCreate>().unwrap()
    }

    fn transform_localsrt(&mut self) -> &mut pi_scene_context::prelude::ActionListTransformNodeLocal {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListTransformNodeLocal>().unwrap()
    }

    fn transform_localrotq(&mut self) -> &mut pi_scene_context::prelude::ActionListTransformNodeLocalRotationQuaternion {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListTransformNodeLocalRotationQuaternion>().unwrap()
    }

    fn transform_tree(&mut self) -> &mut pi_scene_context::prelude::ActionListTransformNodeParent {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListTransformNodeParent>().unwrap()
    }

    fn transform_enable(&mut self) -> &mut pi_scene_context::prelude::ActionListNodeEnable {
        &mut *self.get_resource_mut::<>().unwrap()
    }

    fn camera_create(&mut self) -> &mut pi_scene_context::prelude::ActionListCameraCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListCameraCreate>().unwrap()
    }

    fn camera_param(&mut self) -> &mut pi_scene_context::prelude::ActionListCameraModify {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListCameraModify>().unwrap()
    }

    fn camera_target(&mut self) -> &mut pi_scene_context::prelude::ActionListCameraTarget {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListCameraTarget>().unwrap()
    }

    fn camera_forceinclude(&mut self) -> &mut pi_scene_context::prelude::ActionListViewerForceInclude {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListViewerForceInclude>().unwrap()
    }

    fn mesh_create(&mut self) -> &mut pi_scene_context::prelude::ActionListMeshCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListMeshCreate>().unwrap()
    }

    fn mesh_render_state(&mut self) -> &mut pi_scene_context::prelude::ActionListRenderState {
        &mut *self.get_resource_mut::<>().unwrap()
    }

    fn mesh_pose(&mut self) -> &mut pi_scene_context::prelude::ActionListAbstractMeshPose {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListAbstractMeshPose>().unwrap()
    }

    fn mesh_state(&mut self) -> &mut pi_scene_context::prelude::ActionListMeshStateModify {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListMeshStateModify>().unwrap()
    }

    fn mesh_valuestate(&mut self) -> &mut pi_scene_context::prelude::ActionListAbstructMeshValueStateModify {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListAbstructMeshValueStateModify>().unwrap()
    }

    fn mesh_bounding(&mut self) -> &mut pi_scene_context::prelude::ActionListMeshBounding {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListMeshBounding>().unwrap()
    }

    fn forcelighting(&mut self) -> &mut pi_scene_context::prelude::ActionListMeshForceLighting {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListMeshForceLighting>().unwrap()
    }

    fn skin_create(&mut self) -> &mut pi_scene_context::prelude::ActionListSkinCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListSkinCreate>().unwrap()
    }

    fn skin_use(&mut self) -> &mut pi_scene_context::prelude::ActionListSkinUse {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListSkinUse>().unwrap()
    }

    fn skin_bonecreate(&mut self) -> &mut pi_scene_context::prelude::ActionListBoneCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListBoneCreate>().unwrap()
    }

    fn skin_bonepose(&mut self) -> &mut pi_scene_context::prelude::ActionListBonePose {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListBonePose>().unwrap()
    }

    fn mesh_layermask(&mut self) -> &mut pi_scene_context::prelude::ActionListLayerMask {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListLayerMask>().unwrap()
    }

    fn instance_create(&mut self) -> &mut pi_scene_context::prelude::ActionListInstanceMeshCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListInstanceMeshCreate>().unwrap()
    }

    fn instance_attr(&mut self) -> &mut pi_scene_context::prelude::ActionListInstanceAttr {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListInstanceAttr>().unwrap()
    }

    fn instance_targetanime(&mut self) -> &mut pi_scene_context::prelude::ActionListTargetAnimationAttribute {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListTargetAnimationAttribute>().unwrap()
    }

    fn geometry_create(&mut self) -> &mut pi_scene_context::prelude::ActionListGeometryCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListGeometryCreate>().unwrap()
    }

    fn material_usemat(&mut self) -> &mut pi_scene_context::prelude::ActionListMaterialUse {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListMaterialUse>().unwrap()
    }

    fn material_create(&mut self) -> &mut pi_scene_context::prelude::ActionListMaterialCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListMaterialCreate>().unwrap()
    }

    fn material_val(&mut self) -> &mut pi_scene_context::prelude::ActionListUniformVal {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListUniformVal>().unwrap()
    }

    fn material_valb(&mut self) -> &mut pi_scene_context::prelude::ActionListUniformValB {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListUniformValB>().unwrap()
    }

    fn light_create(&mut self) -> &mut pi_scene_context::prelude::ActionListLightCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListLightCreate>().unwrap()
    }

    fn light_param(&mut self) -> &mut pi_scene_context::prelude::ActionListLightParam {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListLightParam>().unwrap()
    }

    fn shadow_param(&mut self) -> &mut pi_scene_context::prelude::ActionListShadowGeneratorParam {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListShadowGeneratorParam>().unwrap()
    }

    fn shadow_create(&mut self) -> &mut pi_scene_context::prelude::ActionListShadowGenerator {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListShadowGenerator>().unwrap()
    }

    fn renderer_subgraph(&mut self) -> &mut pi_scene_context::prelude::ActionListSubGraphCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListSubGraphCreate>().unwrap()
    }

    fn renderer_create(&mut self) -> &mut pi_scene_context::prelude::ActionListRendererCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListRendererCreate>().unwrap()
    }

    fn renderer_connect(&mut self) -> &mut pi_scene_context::prelude::ActionListRendererConnect {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListRendererConnect>().unwrap()
    }

    fn renderer_modify(&mut self) -> &mut pi_scene_context::prelude::ActionListRendererModify {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListRendererModify>().unwrap()
    }

    fn renderer_target(&mut self) -> &mut pi_scene_context::prelude::ActionListRendererTarget {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListRendererTarget>().unwrap()
    }

    fn anime_create(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimeGroupCreate {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimeGroupCreate>().unwrap()
    }

    fn anime_action(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimationGroupAction {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimationGroupAction>().unwrap()
    }

    fn anime_dispose(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimeGroupDispose {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimeGroupDispose>().unwrap()
    }

    fn anime_reset_while_start(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimeGroupStartReset {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimeGroupStartReset>().unwrap()
    }

    fn anime_property_targetanime(&mut self) -> &mut pi_scene_context::prelude::ActionListPropertyTargetAnimation {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListPropertyTargetAnimation>().unwrap()
    }

    fn anime_goto(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimationGroupGoto {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimationGroupGoto>().unwrap()
    }

    fn anime_float(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimatorableFloat {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimatorableFloat>().unwrap()
    }

    fn anime_sint(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimatorableSint {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimatorableSint>().unwrap()
    }

    fn anime_uint(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimatorableUint {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimatorableUint>().unwrap()
    }

    fn anime_vec2(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimatorableVec2 {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimatorableVec2>().unwrap()
    }

    fn anime_vec3(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimatorableVec3 {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimatorableVec3>().unwrap()
    }

    fn anime_vec4(&mut self) -> &mut pi_scene_shell::prelude::ActionListAnimatorableVec4 {
        &mut *self.get_resource_mut::<pi_scene_shell::prelude::ActionListAnimatorableVec4>().unwrap()
    }

    fn trail_create(&mut self) -> &mut pi_trail_renderer::ActionListTrail {
        &mut *self.get_resource_mut::<pi_trail_renderer::ActionListTrail>().unwrap()
    }

    fn trail_age(&mut self) -> &mut pi_trail_renderer::ActionListTrailAge {
        &mut *self.get_resource_mut::<pi_trail_renderer::ActionListTrailAge>().unwrap()
    }

    fn parsys_calculator(&mut self) -> &mut pi_particle_system::prelude::ActionListCPUParticleCalculator {
        &mut *self.get_resource_mut::<pi_particle_system::prelude::ActionListCPUParticleCalculator>().unwrap()
    }

    fn parsys_create(&mut self) -> &mut pi_particle_system::prelude::ActionListCPUParticleSystem {
        &mut *self.get_resource_mut::<pi_particle_system::prelude::ActionListCPUParticleSystem>().unwrap()
    }

    fn parsys_state(&mut self) -> &mut pi_particle_system::prelude::ActionListCPUParticleSystemState {
        &mut *self.get_resource_mut::<pi_particle_system::prelude::ActionListCPUParticleSystemState>().unwrap()
    }

    fn parsys_trailmaterial(&mut self) -> &mut pi_particle_system::prelude::ActionListCPUParticleSystemTrailMaterial {
        &mut *self.get_resource_mut::<pi_particle_system::prelude::ActionListCPUParticleSystemTrailMaterial>().unwrap()
    }

    fn sprite_create(&mut self) -> &mut pi_scene_context::prelude::ActionListSpriteCreate {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListSpriteCreate>().unwrap()
    }

    fn sprite_modify(&mut self) -> &mut pi_scene_context::prelude::ActionListSpriteModify {
        &mut *self.get_resource_mut::<pi_scene_context::prelude::ActionListSpriteModify>().unwrap()
    }
}



#[derive(Default, Resource)]
pub struct DisplayBoxs(pub XHashMap<Entity, Entity>);

pub fn _request_transform(world: &mut World, entity: Entity) -> String {
    let mut style = serde_json::from_str::<serde_json::Value>("{}").unwrap();
    if let Ok(r) =  world.get_component::<LocalPosition>(entity) {
            style["position"] = format!("[{:?},{:?},{:?}]", r.0.x, r.0.y, r.0.z).into();
    };
    
    if let Ok(r) =  world.get_component::<LocalScaling>(entity) {
            style["scaling"] = format!("[{:?},{:?},{:?}]", r.0.x, r.0.y, r.0.z).into();
    };
    
    if let Ok(r) =  world.get_component::<LocalEulerAngles>(entity) {
            style["rotation"] = format!("[{:?},{:?},{:?}]", r.0.x, r.0.y, r.0.z).into();
    };

    if let Ok(r) = world.get_component::<Enable>(entity) {
            style["enable"] = r.bool().into();
    };

    if let Ok(r) = world.get_component::<GlobalEnable>(entity) {
        style["isEnabled"] = r.0.into();
    };
    if let Ok(r) = world.get_component::<Collider>(entity) {
        style["ColliderMin"] = r.minimum.to_string().into();
        style["ColliderMax"] = r.maximum.to_string().into();
        style["ColliderTreshold"] = r.intersection_treshold.into();
    };

    if let Ok(r) = world.get_component::<RenderQueueSortParam>(entity) {
        style["RenderGroup"] = r.group.to_string().into();
        style["RenderIndex"] = r.index.to_string().into();
    }

    if let Ok(gm) = world.get_component::<GlobalMatrix>(entity) {
        let vec = gm.xyz();
        style["globalPosition"] = format!("[{:?},{:?},{:?}]", vec.0, vec.1, vec.2).into();
        let gm = gm.matrix().clone();
        if let Ok(mut transform) = world.get_component_mut::<AbsoluteTransform>(entity) {
            let mut tmpscl = Vector3::zeros();
            let mut tmprot = Rotation3::identity();
            let vec = transform.rotation_quaternion(&gm, &mut tmpscl, &mut tmprot);
            style["globalRotation"] = format!("[{:?},{:?},{:?},{:?}]", vec.i, vec.j, vec.k, vec.w).into();
            let vec = transform.scaling(&gm, &mut tmpscl, &mut tmprot);
            style["globalScaling"] = format!("[{:?},{:?},{:?}]", vec.x, vec.y, vec.z).into();
        }
    }

    format!("{{\"cmd\": \"transform\", \"payload\": {} }}", style.to_string())
}
pub fn _request_meshpass(world: &mut World, entity: Entity) -> String {
    let mut style = serde_json::from_str::<serde_json::Value>("{}").unwrap();
    match world.get_component::<PassIDs>(entity) {
        Ok(r) => {
            let mut passinfos = serde_json::from_str::<serde_json::Value>("{}").unwrap();
            let mut idx = 0;
            r.clone().0.iter().for_each(|pass| {
                let mut passinfo = serde_json::from_str::<serde_json::Value>("{}").unwrap();
                let entity = *pass;
                if let Ok(val) = world.get_component::<PassDraw>(entity) {
                    passinfo["Draw"] = val.0.into();
                }
                if let Ok(val) = world.get_component::<PassBindGroups>(entity) {
                    passinfo["BindGroups"] = val.val().is_some().into();
                }
                if let Ok(val) = world.get_component::<PassPipeline>(entity) {
                    passinfo["Pipeline"] = val.val().is_some().into();
                }
                if let Ok(val) = world.get_component::<PassShader>(entity) {
                    if let Some(val) = &val.0 {
                        passinfo["Shader"] = val.key().key_meta.as_str().into();
                    } else {
                        passinfo["Shader"] = "None".into();
                    }
                }
                
                if let Ok(r) = world.get_component::<RenderState>(entity) {
                    style["Blend"] = r.blend.to_string().into();
                    style["DepthWrite"] = r.depth.depth_write.into();
                    style["DepthTest"] = format!("{:?}", r.depth.compare).into();
                    style["DepthBias"] = format!("{:?}", r.depth.bias).into();
                    style["Cull"] = format!("{:?}", r.primitive.cull).into();
                    style["FrontFace"] = format!("{:?}", r.primitive.frontface).into();
                    style["Polygon"] = format!("{:?}", r.primitive.polygon).into();
                    style["Topology"] = format!("{:?}", r.primitive.topology).into();
                    style["Stencil"] = format!("{:?}", r.stencil).into();
                }
                passinfos[idx.to_string()] = passinfo.into();
                idx += 1;
            });
            style["PassInfos"] = passinfos.into();
        },
        Err(_) => {},
    };

    format!("{{\"cmd\": \"meshpasses\", \"payload\": {} }}", style.to_string())
}
pub fn _request_camera(world: &mut World, entity: Entity) -> String {

    let mut style = serde_json::from_str::<serde_json::Value>("{}").unwrap();
    if let Ok(val) = world.get_component::<CameraParam>(entity) {
        let mut info = serde_json::from_str::<serde_json::Value>("{}").unwrap();
        info["Fov"]     = val.fov.0.into();
        info["Near"]    = val.nearfar.0.into();
        info["Far"]     = val.nearfar.1.into();
        info["Size"]    = val.orth.0.into();

        style["Camera"] = info.into();
    };

    format!("{{\"cmd\": \"camera\", \"payload\": {} }}", style.to_string())
}

pub fn _request_select3d(world: &mut World, entity: Entity) {
    if let Ok(sceneid) = world.get_component::<SceneID>(entity) {
        let sceneid = sceneid.0.clone();
        if let Ok(gm) = world.get_component::<GlobalMatrix>(entity) {
            let pose = gm.matrix.clone();

            let boxs = world.get_resource::<DisplayBoxs>().unwrap();
            if boxs.0.get(&sceneid).is_none() {
                let (vertices, indices) = (CubeBuilder::attrs_meta(), CubeBuilder::indices_meta());
                let id_geo: Entity = world.spawn_empty_id();
                let mesh = world.spawn_empty_id(); 
                let state: MeshInstanceState = MeshInstanceState::default();
                world.get_resource_mut::<ActionListTransformNodeParent>().unwrap().push(OpsTransformNodeParent::ops(mesh, sceneid));
                world.get_resource_mut::<ActionListMeshCreate>().unwrap().push(OpsMeshCreation::ops(sceneid, mesh, state));
                world.get_resource_mut::<ActionListGeometryCreate>().unwrap().push(OpsGeomeryCreate::ops(mesh, id_geo, vertices, indices));

                // actions.mesh.depth_compare.push(OpsDepthCompare::ops(mesh, CompareFunction::LessEqual));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_01, EDepthState::Compare(CompareFunction::Always)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_02, EDepthState::Compare(CompareFunction::Always)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_03, EDepthState::Compare(CompareFunction::Always)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_04, EDepthState::Compare(CompareFunction::Always)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_05, EDepthState::Compare(CompareFunction::Always)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_06, EDepthState::Compare(CompareFunction::Always)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_07, EDepthState::Compare(CompareFunction::Always)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_08, EDepthState::Compare(CompareFunction::Always)));

                let defaultmat = world.get_resource::<SingleIDBaseDefaultMaterial>().unwrap().0;
                world.get_resource_mut::<ActionListMaterialUse>().unwrap().push(OpsMaterialUse::ops(mesh, defaultmat, PassTag::PASS_TAG_06));
                world.get_resource_mut::<ActionListMeshStateModify>().unwrap().push(OpsMeshStateModify::ops(mesh, EMeshStateModify::BoundingCullingMode(ECullingStrategy::None)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::depth_state(mesh, PassTag::PASS_TAG_06, EDepthState::Write(false)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::primitive_state(mesh, PassTag::PASS_TAG_06, EPrimitiveState::CPolygonMode(PolygonMode::Line)));
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::render_queue(mesh, i32::MAX, i32::MAX));
                let  mut blend = ModelBlend::one_one();
                blend.combine();
                world.get_resource_mut::<ActionListRenderState>().unwrap().push(OpsRenderState::blend(mesh, PassTag::PASS_TAG_06, blend));

                let boxs = world.get_resource_mut::<DisplayBoxs>().unwrap();
                boxs.0.insert(sceneid, mesh);
            }
            let mesh = world.get_resource_mut::<DisplayBoxs>().unwrap().0.get(&sceneid).unwrap().clone();
            world.get_resource_mut::<ActionListAbstractMeshPose>().unwrap().push(OpsAbstractMeshPose::ops(mesh, pose));
            world.get_resource_mut::<ActionListNodeEnable>().unwrap().push(OpsNodeEnable::ops(mesh, true));
            return;
        }

    };
}