use pi_hash::XHashMap;
use pi_particle_system::prelude::ParticleSystemTime;
use pi_scene_context::{prelude::*, light::base::Light};
use pi_trail_renderer::TrailBase;

#[derive(Default)]
pub struct StateScene {
    pub count_animationgroup: usize,
    pub count_transform: usize,
    pub count_mesh: usize,
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
}

pub fn sys_state_scene(
    materials: Query<Entity, With<MaterialRefs>>,
    transformnodes: Query<Entity, With<TransformNode>>,
    meshes: Query<Entity, With<Mesh>>,
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

    materials.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_material += 1;
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
    meshes.iter().for_each(|entity| {
        if let Ok(idscene) = idscenes.get(entity) {
            if let Some(state) = stateglobal.scenes.get_mut(&idscene.0) {
                state.count_mesh += 1;
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
    pub count_bindgroup: usize,
    pub count_pipeline: usize,
    pub count_shader: usize,
    pub count_bindbuffer: usize,
    pub count_geometrybuffer: usize,
    pub size_geometrybuffer: u64,
    pub scenes: XHashMap<Entity, StateScene>,
}

pub fn sys_state_global(
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
    bindbuffers: Res<ResBindBufferAllocator>,
    vertexbuffers: Res<VertexBufferAllocator3D>,
    shaders: Res<AssetDataCenterShader3D>,
    pipelines: Res<AssetDataCenterPipeline3D>,
    imagetextures: Res<ShareAssetMgr<ImageTexture>>,
    mut stateglobal: ResMut<StateGlobal>,
) {
    if stateglobal.debug == false { return };

    stateglobal.count_bindbuffer = bindbuffers.asset_mgr().len();
    stateglobal.count_bindgroup = asset_mgr_bindgroup.0.len();
    stateglobal.count_pipeline = pipelines.asset_mgr().len();
    stateglobal.count_geometrybuffer = vertexbuffers.total_buffer_count();
    stateglobal.size_geometrybuffer = vertexbuffers.total_buffer_size();
    stateglobal.count_shader = shaders.asset_mgr().len();
    stateglobal.count_imgtexture = imagetextures.len();
}

pub struct PluginStateGlobal;
impl Plugin for PluginStateGlobal {
    fn build(&self, app: &mut App) {
        app.insert_resource(StateGlobal::default());
        app.add_system(sys_state_scene.run_if(should_run).in_set(ERunStageChap::StateCheck));  
        app.add_system(sys_state_global.run_if(should_run).in_set(ERunStageChap::StateCheck));   
    }
}