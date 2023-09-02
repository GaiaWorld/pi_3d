use std::sync::Arc;

use pi_gltf2_load::GLTF;
use pi_hash::XHashMap;
use pi_node_materials::prelude::*;
use pi_particle_system::prelude::*;
use pi_scene_context::{prelude::*, light::base::Light};
use pi_trail_renderer::TrailBase;

#[derive(Default)]
pub struct StateScene {
    pub count_animationgroup: usize,
    pub count_transform: usize,
    pub count_mesh: usize,
    pub count_mesh_ok: usize,
    pub count_geometry: usize,
    pub count_material: usize,
    pub count_instance: usize,
    pub count_skeleton: usize,
    pub count_camera: usize,
    pub count_light: usize,
    pub count_pointlight: usize,
    pub count_drawobj: usize,
    pub count_vertex: usize,
    pub count_particlesys: usize,
    pub count_trail: usize,
    // pub entity: Entity,
}

pub fn sys_state_scene(
    geometrys: Query<(&MeshID, &RenderGeometryComp)>,
    transformnodes: Query<Entity, With<TransformNode>>,
    meshes: Query<(Entity, Option<&BindModel>), With<Mesh>>,
    instancemeshes: Query<Entity, With<InstanceMesh>>,
    cameras: Query<Entity, With<Camera>>,
    lights: Query<Entity, With<Light>>,
    skeletons: Query<Entity, With<Skeleton>>,
    particlesys: Query<Entity, With<ParticleSystemTime>>,
    trails: Query<Entity, With<TrailBase>>,
    renderers: Query<(&ViewerID, &Renderer)>,
    idscenes: Query<&SceneID>,
    scenes: Query<Entity, With<SceneTime>>,
    scenectxs: Res<SceneAnimationContextMap>,
    mut stateglobal: ResMut<StateGlobal>,
) {
    scenes.iter().for_each(|entity| {
        stateglobal.scenes.insert(entity, StateScene::default());
    });

    if stateglobal.debug == false { return };

    geometrys.iter().for_each(|(idmesh, rendergeo)| {
        if rendergeo.is_some() {
            if let Ok(idscene) = idscenes.get(idmesh.0) {
                if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                    state.count_geometry += 1;
                }
            }
        }
    });
    transformnodes.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_transform += 1;
            }
        }
    });
    meshes.iter().for_each(|(entity, bind)| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_mesh += 1;
                if bind.is_some() {
                    state.count_mesh_ok += 1;
                }
            }
        }
    });
    instancemeshes.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_instance += 1;
            }
        }
    });
    cameras.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_camera += 1;
            }
        }
    });
    lights.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_light += 1;
            }
        }
    });
    skeletons.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_skeleton += 1;
            }
        }
    });
    particlesys.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_particlesys += 1;
            }
        }
    });
    trails.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_trail += 1;
            }
        }
    });
    renderers.iter().for_each(|(idviewer, renderer)| {
        if let Ok(idscene) = idscenes.get(idviewer.0) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_drawobj += renderer.draws.list.len();
                renderer.draws.list.iter().for_each(|item| {
                    let vertex = if let Some(indices) = &item.indices {
                        indices.value_range().end - indices.value_range().start
                    } else { item.vertex.end - item.vertex.start };
                    state.count_vertex += (vertex * (item.instances.end - item.instances.start)) as usize;
                });
            }
        }
    });
    scenectxs.iter().for_each(|(scene, ctx)| {
        if let Some(state) = stateglobal.scenes.get_mut(scene) {
            state.count_animationgroup += ctx.0.group_mgr.groups.len();
        }
    });
}

#[derive(Resource, Default)]
pub struct StateGlobal {
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
    pub scenes: XHashMap<Entity, StateScene>,
}

pub fn sys_state_global(
    asset_gltf: Res<ShareAssetMgr<GLTF>>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    bindbuffers: Res<ResBindBufferAllocator>,
    vertexbuffers: Res<VertexBufferAllocator3D>,
    shaders: Res<AssetDataCenterShader3D>,
    pipelines: Res<AssetDataCenterPipeline3D>,
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
        Query<&MaterialID>,
    ),
    mut stateglobal: ResMut<StateGlobal>,
    mut performance: ResMut<Performance>,
    particlesysperformance: Res<ParticleSystemPerformance>,
    empty: Res<SingleEmptyEntity>,
) {
    if stateglobal.debug == false { return };

    stateglobal.count_gltf = asset_gltf.len();
    stateglobal.count_bindbuffer = bindbuffers.asset_mgr().len();
    stateglobal.mem_bindbuffer = bindbuffers.asset_mgr().size();
    stateglobal.count_bindgroup = asset_mgr_bindgroup.0.len();
    stateglobal.count_pipeline = pipelines.asset_mgr().len();
    stateglobal.count_geometrybuffer = vertexbuffers.total_buffer_count();
    stateglobal.size_geometrybuffer = vertexbuffers.total_buffer_size();
    stateglobal.count_shader = shaders.asset_mgr().len();
    stateglobal.mem_shader = shaders.asset_mgr().size();
    stateglobal.count_imgtexture = imagetextures.len();
    stateglobal.mem_imgtexture = imagetextures.size();
    stateglobal.count_shadermeta = shadermetas.len();
    stateglobal.mem_shadermeta = shadermetas.size();

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

    count = 0;
    passes.8.iter().for_each(|item| {
        if item.0 != empty.id() { count += 1; }
    });
    stateglobal.count_passmat = count;

    // *performance = Performance::default();
    performance.particlesystem = particlesysperformance.total();
    // *particlesysperformance = ParticleSystemPerformance::default();
}

pub struct PluginStateGlobal;
impl Plugin for PluginStateGlobal {
    fn build(&self, app: &mut App) {
        app.insert_resource(Performance::default());
        app.insert_resource(StateGlobal::default());
        app.add_systems(
            Update,
            (
                sys_state_scene,
                sys_state_global
            ).chain().run_if(should_run).in_set(ERunStageChap::StateCheck)
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
        VertexBufferDesc {
            key: self.key.clone(),
            range: None,
            attrs: vec![
                VertexAttribute { kind: EVertexDataKind::Position, format: wgpu::VertexFormat::Float32x3 },
                VertexAttribute { kind: EVertexDataKind::Color4, format: wgpu::VertexFormat::Float32x4 },
            ],
            step_mode: wgpu::VertexStepMode::Vertex,
            instance: false,
        }
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
        nodemat.values.uint_list.push(UniformPropertyUint(Atom::from("debug_normal"), 0));

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