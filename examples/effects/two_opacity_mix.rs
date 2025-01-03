#![feature(box_into_inner)]

use base::DemoScene;
use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};
use pi_scene_context::prelude::*;
use pi_mesh_builder::{cube::*, ball::*};
use unlit_material::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut matmetas: ResMut<ShareAssetMgr<ShaderEffectMeta>>,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    nodematblocks: Res<NodeMaterialBlocks>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    demooption: Res<base::DemoOption>,
    limit: Res<DeviceLimits3D>,
    engineopt: Res<EngineCustomPlugins>,
) {
    log::warn!("Limit {:?}", limit.min_uniform_buffer_offset_alignment);
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(TwoOpacityMixShader::KEY), TwoOpacityMixShader::create(&nodematblocks, &engineopt));

    let tes_size = 5;
    fps.frame_ms = 4;

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let vertices = CubeBuilder::attrs_meta();
    let indices = CubeBuilder::indices_meta();
    let state = MeshInstanceState::default();
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.render_state.push(OpsRenderState::blend(source, DemoScene::PASS_TRANSPARENT, blend));

    let idmat = commands.spawn_empty_id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.create.push(OpsMaterialCreate::ops(idmat, TwoOpacityMixShader::KEY));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("assets/images/fractal.png"),
        ..Default::default()
    }));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockOpacityTexture::KEY_TEX),
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("assets/images/eff_ui_ll_085.png"),
        ..Default::default()
    }));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockOpacity2Texture::KEY_TEX),
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("assets/images/eff_uv_lf_002.png"),
        ..Default::default()
    }));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMixTexture::KEY_TEX),
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("assets/images/icon_city.png"),
        ..Default::default()
    }));
    actions.material.val.push(OpsUniformVal::vec4(
            idmat, 
            Atom::from(BlockEmissiveTexture::KEY_INFO), 
            1., 1., 1., 1.
        )
    );
    actions.material.val.push(OpsUniformVal::vec2(
            idmat, 
            Atom::from(BlockOpacityTextureUVOffsetSpeed::KEY_PARAM), 
            1., 1.
        )
    );
    actions.material.val.push(OpsUniformVal::vec2(
            idmat, 
            Atom::from(BlockOpacity2TextureUVOffsetSpeed::KEY_PARAM), 
            1., 1.
        )
    );
    actions.material.val.push(OpsUniformVal::float(
            idmat, 
            Atom::from(TwoOpacityMixShader::KEY_MIX_CONTROL), 
            1.0,
        )
    );
}

fn sys_setup_ball(
    mut data_map: ResMut<VertexBufferDataMap3D>,
) {
    let param = BallParam { sectors: 20, stacks: 20 };

    let (positions, normals, indices, uvs) = generate_sphere(&param);
    let id = "BallPos#20#20";
    ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(id), bytemuck::cast_slice(&positions).iter().map(|v| *v).collect::<Vec<u8>>());
    let id = "BallNor#20#20";
    ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(id), bytemuck::cast_slice(&normals).iter().map(|v| *v).collect::<Vec<u8>>());
    let id = "BallUV#20#20";
    ActionVertexBuffer::create(&mut data_map, KeyVertexBuffer::from(id), bytemuck::cast_slice(&uvs).iter().map(|v| *v).collect::<Vec<u8>>());
    let id = "BallInd#20#20";
    ActionVertexBuffer::create_indices(&mut data_map, KeyVertexBuffer::from(id), bytemuck::cast_slice(&indices).iter().map(|v| *v).collect::<Vec<u8>>());
}


pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}

pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins();

    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 5.,
        camera_fov: 0.7,
        camera_position: (0., 0., -10.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);
    
    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, sys_setup_ball);
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, sys_setup_ball);
        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}