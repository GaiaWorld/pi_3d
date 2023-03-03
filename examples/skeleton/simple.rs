#![feature(box_into_inner)]


use default_render::interface::InterfaceDefaultMaterial;
use pi_3d::PluginBundleDefault;
use pi_atom::Atom;
use pi_engine_shell::{engine_shell::AppShell, frame_time::InterfaceFrameTime, assets::local_load::PluginLocalLoad, run_stage::{ERunStageChap, TSystemStageInfo}};
use pi_render::{rhi::{options::RenderOptions, device::RenderDevice, RenderQueue}, render_3d::shader::{uniform_texture::UniformTextureWithSamplerParam, skin_code::ESkinBonesPerVertex}, renderer::{sampler::KeySampler, texture::KeyTexture, attributes::{VertexAttribute, EVertexDataKind}, vertex_buffer_desc::VertexBufferDesc}};
use pi_scene_context::{plugin::Plugin, object::ObjectID,
    transforms::{command::{SingleTransformNodeModifyCommandList, ETransformNodeModifyCommand}, interface::InterfaceTransformNode},
    scene::{interface::InterfaceScene},
    cameras::interface::InterfaceCamera,
    meshes::{interface::InterfaceMesh},
    layer_mask::{interface::InterfaceLayerMask, LayerMask},
    materials::{interface::{InterfaceMaterial}},
    geometry::{TInterfaceGeomtery, indices::InterfaceBufferIndices},
    skeleton::{PluginSkeleton, interface::TInterfaceSkeleton}, 
    renderers::render_primitive::{InterfaceRenderPrimitive, ECullMode},
    pass::EPassTag
};
use pi_ecs::prelude::{ResMut, Setup};
use pi_ecs_macros::setup;
use pi_scene_math::{Vector3, Vector4};
use unlit_material::{interface::InterfaceUnlitMaterial, PluginUnlitMaterial};
use pi_mesh_builder::cube::{InterfaceCube, CubeBuilder, PluginCubeBuilder};

#[derive(Debug, Default)]
pub struct SingleTestData {
    pub time: f32,
    pub transforms: Vec<(ObjectID, f32, f32, f32)>,
}

pub struct SysTest;
impl TSystemStageInfo for SysTest {}
#[setup]
impl SysTest {
    #[system]
    pub fn sys(
        mut list: ResMut<SingleTestData>,
        mut transform_commands: ResMut<SingleTransformNodeModifyCommandList>,
    ) {
        list.time += 16.0;
        let time = list.time;
        list.transforms.iter_mut().for_each(|mut item| {
            let x0 = time % 4000.0 / 4000.0;
            let x = x0 * 3.1415926 * 2.;
            // let y0 = time % 4000.0 / 4000.0;
            // let y = y0 * 3.1415926 * 2.;
            // let z0 = time % 4000.0 / 4000.0;
            // let z = z0 * 3.1415926 * 2.;
            // transform_commands.list.push(TransformNodeCommand::ModifyPosition(item.0, Vector3::new(x.cos() * 3., 0., 0.)));
            // transform_commands.list.push(TransformNodeCommand::ModifyScaling(item.0, Vector3::new(x.cos() + 0.5, x.sin() + 0.5, x + 0.5)));
            transform_commands.list.push(ETransformNodeModifyCommand::ModifyPosition(item.0, Vector3::new(item.1 * x.cos(), item.2 * x.cos(), item.3 * x.cos())));
        });
    }
}

#[derive(Debug)]
pub struct PluginTest;
impl Plugin for PluginTest {
    fn init(
        &mut self,
        engine: &mut pi_scene_context::engine::Engine,
        stages: &mut pi_scene_context::run_stage::RunStage,
    ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {

        PluginLocalLoad.init(engine, stages);
        PluginBundleDefault.init(engine, stages);
        PluginUnlitMaterial.init(engine, stages);
        PluginCubeBuilder.init(engine, stages);
        PluginSkeleton.init(engine, stages);

        let world = engine.world_mut();

        SysTest::setup(world, stages.query_stage::<SysTest>(ERunStageChap::Command));

        let testdata = SingleTestData::default();
        world.insert_resource(testdata);

        Ok(())
    }
}

impl PluginTest {
    pub fn setup(
        engine: &pi_engine_shell::engine_shell::EnginShell,
    ) {

        let tes_size = 5;
        let testdata = engine.world().get_resource_mut::<SingleTestData>().unwrap();

        engine.frame_time(4);

        // Test Code
        let scene01 = engine.create_scene();
        let camera01 = engine.create_free_camera(scene01);
        engine.active_camera(camera01, true);
        engine.layer_mask(camera01, LayerMask::default());
        engine.transform_position(camera01, Vector3::new(0., 0., -10.));
        engine.free_camera_orth_size(camera01, tes_size as f32);

        // let matid = engine.create_default_material();
        // engine.emissive_intensity(entity, intensity);
        let unlitmaterial = engine.create_unlit_material(EPassTag::Opaque);
        engine.set_texture(
            unlitmaterial, 
            UniformTextureWithSamplerParam {
                slotname: Atom::from("_MainTex"),
                filter: true,
                sample: KeySampler::default(),
                url: KeyTexture::from("E:/Rust/PI/pi_3d/assets/images/top.jpg"),
            },
            false
        );

        let source = engine.create_mesh(scene01);
        let mut attrs = CubeBuilder::attrs_meta();

        let bone0 = engine.create_transform_node(scene01);
        testdata.transforms.push((bone0, 0., 0., 0.));
        let bone1 = engine.create_transform_node(scene01);
        testdata.transforms.push((bone1, 1., 0., 0.));
        let bone2 = engine.create_transform_node(scene01);
        testdata.transforms.push((bone2, -1., 0., 0.));
        let bone3 = engine.create_transform_node(scene01);
        testdata.transforms.push((bone3, 0., 1., 0.));
        let bone4 = engine.create_transform_node(scene01);
        testdata.transforms.push((bone4, 0., -1., 0.));
        engine.transform_parent(bone1, bone0);
        engine.transform_parent(bone2, bone0);
        engine.transform_parent(bone3, bone0);
        engine.transform_parent(bone4, bone0);

        let device = engine.world().get_resource::<RenderDevice>().unwrap();
        let queue = engine.world().get_resource::<RenderQueue>().unwrap();
        let data: [u16; 48] = [
            0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0,
            1, 1, 1, 1, 1, 1, 1, 1, 
            2, 2, 2, 2, 2, 2, 2, 2, 
            3, 3, 3, 3, 3, 3, 3, 3, 
            4, 4, 4, 4, 4, 4, 4, 4
        ];
        // normals
        let jointkey = pi_atom::Atom::from("TestJoint");
        engine.create_vertex_buffer(jointkey.clone(), bytemuck::cast_slice(&data).iter().map(|v| *v).collect::<Vec<u8>>());

        let format = wgpu::VertexFormat::Uint16x2;
        let jointdesc = VertexBufferDesc::vertices(jointkey.clone(), None, vec![VertexAttribute { kind: EVertexDataKind::MatricesIndices1, format }]);
        attrs.push(jointdesc);

        engine.use_geometry(source, attrs);
        engine.use_indices(source, CubeBuilder::indices_meta());
        engine.use_material(source, unlitmaterial);
        engine.layer_mask(source, LayerMask::default());
        engine.transform_rotation_euler(source, Vector3::new(1. as f32 * 0.2, 1. as f32 * 0.2, 1. as f32 * 0.2));
        engine.cull_mode(source, ECullMode::Off);

        let skeleton = engine.create_skeleton_ubo(ESkinBonesPerVertex::One, bone0, vec![bone0, bone1, bone2, bone3, bone4]);
        engine.use_skeleton(source, skeleton);
    }
}

pub fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let mut shell = AppShell::new(
        RenderOptions {
            backends: wgpu::Backends::VULKAN,
            power_preference: wgpu::PowerPreference::HighPerformance,
            ..Default::default()
        }
    );
    shell.add_plugin(PluginTest);
    shell.ready();
    shell.setup(&PluginTest::setup);
    shell.run();
}