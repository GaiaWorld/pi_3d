use pi_engine_shell::{prelude::*, run_stage::PluginRunstage};
use default_render::PluginDefaultMaterial;
use pi_scene_context::{prelude::*, scene::PluginScene, animation::PluginSceneAnimation, transforms::PluginGroupTransformNode, cameras::PluginCamera, meshes::PluginMesh, geometry::PluginGeometry, light::{PluginLighting, base::Light}, layer_mask::PluginLayerMask, materials::PluginGroupMaterial, renderers::PluginRenderer, skeleton::PluginSkeleton};

pub struct Limit(pub wgpu::Limits);
// impl TMemoryAllocatorLimit for Limit {
//     fn max_size(&self) -> u64 {
//         500 * 1024 * 1024
//     }
// }

pub fn sys_scene_time_from_frame(
    mut scenes: Query<&mut SceneTime>,
    frame: Res<SingleFrameTimeCommand>,
) {
    scenes.iter_mut().for_each(|mut comp| {
        let time = comp.last_time_ms + frame.frame_ms;
        comp.reset(time);
    });
}

pub struct PluginSceneTimeFromPluginFrame;
impl Plugin for PluginSceneTimeFromPluginFrame {
    fn build(&self, app: &mut App) {
        app.add_system(
            sys_scene_time_from_frame.in_set(ERunStageChap::Command)
        );
    }
}

pub fn sys_nodeinfo(
    materials: Query<&MaterialRefs>,
    geometries: Query<&GeometryDesc>,
    transformnodes: Query<&TransformNode>,
    meshes: Query<&Mesh>,
    instancemeshes: Query<&InstanceMesh>,
    cameras: Query<&Camera>,
    renderers: Query<&Renderer>,
    lights: Query<&Light>,
    passes: Query<&ModelPass>,
    skeletons: Query<&Skeleton>,
    bones: Query<&BoneParent>,
    pipeline_center: Res<AssetDataCenterPipeline3D>,
    asset_mgr_bindgroup: Res<ShareAssetMgr<BindGroup>>,
    asset_mgr_bindgroup_layout: Res<ShareAssetMgr<BindGroupLayout>>,
) {
    let count_material = materials.iter().count();
    let count_geometry = geometries.iter().count();
    let count_transform = transformnodes.iter().count();
    let count_mesh = meshes.iter().count();
    let count_instance = instancemeshes.iter().count();
    let count_camera = cameras.iter().count();
    let count_renderer = renderers.iter().count();
    let count_light = lights.iter().count();
    let count_pass = passes.iter().count();
    let count_skeleton = skeletons.iter().count();
    let count_bone = bones.iter().count();

    let count_pipeline = pipeline_center.0.asset_mgr().len();
    let count_bindgroup = asset_mgr_bindgroup.0.len();
    let count_bindgrouplayout = asset_mgr_bindgroup_layout.0.len();

    log::warn!(
        "Materials: {:?}, Geometry: {:?}, Transform: {:?}, Mesh: {:?}, InstanceMesh: {:?}, Camera: {:?}, Renderer: {:?}, Light: {:?}, Pass: {:?}, Skeleton: {:?}, Bone: {:?}, Pipeline: {:?}, BindGroup: {:?}, BindGroupLayout: {:?}",
        count_material,
        count_geometry,
        count_transform,
        count_mesh,
        count_instance,
        count_camera,
        count_renderer,
        count_light,
        count_pass,
        count_skeleton,
        count_bone,
        count_pipeline,
        count_bindgroup,
        count_bindgrouplayout,
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