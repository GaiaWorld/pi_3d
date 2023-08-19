use pi_3d_state::{PluginStateGlobal, StateGlobal};
use pi_engine_shell::{prelude::*, run_stage::PluginRunstage};
use default_render::PluginDefaultMaterial;
use pi_scene_context::{prelude::*, scene::PluginScene, animation::PluginSceneAnimation, transforms::PluginGroupTransformNode, cameras::PluginCamera, meshes::PluginMesh, geometry::PluginGeometry, light::{PluginLighting, base::Light}, layer_mask::PluginLayerMask, materials::PluginGroupMaterial, renderers::PluginRenderer, skeleton::PluginSkeleton};

pub struct Limit(pub wgpu::Limits);
// impl TMemoryAllocatorLimit for Limit {
//     fn max_size(&self) -> u64 {
//         500 * 1024 * 1024
//     }
// }


pub fn sys_info_node(
    scenes: Query<Entity, With<SceneTime>>,
    states: Res<StateGlobal>,
) {
    scenes.iter().for_each(|entity| {
        if let Some(state) = states.scenes.get(&entity) {
            log::warn!(
                "Scene: {:?}, Draw: {:?}, Vertex: {:?}, Materials: {:?}, Transform: {:?}, Mesh: {:?}, InstanceMesh: {:?}, Camera: {:?}, Light: {:?}, Skeleton: {:?}, ParticleSys: {:?}, Trail: {:?}, AnimeGroup: {:?}",
                entity,
                state.count_drawobj,
                state.count_vertex,
                state.count_material,
                state.count_transform,
                state.count_mesh,
                state.count_instance,
                state.count_camera,
                state.count_light,
                state.count_skeleton,
                state.count_particlesys,
                state.count_trail,
                state.count_animationgroup,
            );
        }
    });
}


pub fn sys_info_draw(
    draws: Query<(&PassBindGroupScene, &PassBindGroupModel, &PassBindEffectValue, &PassShader, &PassBindGroups, &PassPipeline, &PassDraw)>,
    geometries: Query<&RenderGeometryComp>,
    meshes: Query<&RenderGeometryEable>,
    viewers: Query<&ModelListAfterCulling>,
) {
    let mut count_set0 = 0;
    let mut count_set1 = 0;
    let mut count_effect = 0;
    let mut count_bindgroups = 0;
    let mut count_shader = 0;
    let mut count_pipeline = 0;
    let mut count_draw = 0;
    draws.iter().for_each(|(bindgroup_scene, bindgroup_model, bindeffect, shader, bindgroups, pipeline, draw)| {
        if bindgroup_scene.is_some() { count_set0 += 1; }
        if bindgroup_model.is_some() { count_set1 += 1; }
        if bindeffect.0.is_some() { count_effect += 1; }
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
    viewers.iter().for_each(|item| {
        viewer_cullings.push(item.0.len());
    });

    log::warn!(
        "ReadyGeo: {:?}-{:?}, Cullings: {:?}, Set0: {:?}, Set1: {:?}, Eff: {:?}, BindGroups: {:?}, Shader: {:?}, Pipeline: {:?}, Draw: {:?}",
        count_ready_geo, count_ready_geo_mesh, viewer_cullings, count_set0, count_set1, count_effect, count_bindgroups, count_shader, count_pipeline, count_draw
    );
}

pub fn sys_info_resource(
    states: Res<StateGlobal>,
) {
    log::warn!(
        "BindBuffer: {:?}, VertexBuffer: {:?}, VertexBufferSize: {:?}, Shaders: {:?}, Pipeline: {:?}, ImageTexture: {:?},",
        states.count_bindbuffer, states.count_geometrybuffer, states.size_geometrybuffer, states.count_shader, states.count_pipeline, states.count_imgtexture
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
        group = PluginGroupTransformNode::add(group);
        group = group.add(PluginCamera)
            .add(PluginAnimeCameraFOV::new())
            .add(PluginAnimeCameraSize::new())
            .add(PluginMesh)
            .add(PluginAnimeBoneOffset::new())
            .add(PluginAnimeRenderIndiceRange::new())
            .add(PluginGeometry)
            .add(PluginLighting)
            .add(PluginLayerMask);
        group = PluginGroupMaterial::add(group);
        group = group.add(PluginRenderer)
            .add(PluginSkeleton)
            .add(PluginDefaultMaterial)
            .add(PluginDispose)
            .add(PluginStateGlobal)
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