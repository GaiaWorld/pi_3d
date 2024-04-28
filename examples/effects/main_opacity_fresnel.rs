#![feature(box_into_inner)]

use base::DemoScene;
use pi_atom::Atom;
use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};
use pi_scene_context::prelude::*;
use pi_mesh_builder::ball::*;
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
) {
    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(MainOpacityFresnelShader::KEY), MainOpacityFresnelShader::create(&nodematblocks));

    let tes_size = 5;
    fps.frame_ms = 4;
    let demopass = base::DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -10.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let vertices = BallBuilder::attrs_meta();
    let indices = Some(BallBuilder::indices_meta());
    let state = MeshInstanceState::default();
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.blend.push(OpsRenderBlend::ops(source, DemoScene::PASS_OPAQUE, blend));

    let idmat = commands.spawn_empty().id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.create.push(OpsMaterialCreate::ops(idmat, MainOpacityFresnelShader::KEY));
    actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/fractal.png"),
    }));
    actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockOpacityTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/icon_city.png"),
    }));
    actions.material.vec4.push(
        OpsUniformVec4::ops(
            idmat, 
            Atom::from(BlockEmissiveTexture::KEY_INFO), 
            1., 1., 1., 1.
        )
    );
    actions.material.vec2.push(
        OpsUniformVec2::ops(
            idmat, 
            Atom::from(BlockOpacityTextureUVOffsetSpeed::KEY_PARAM), 
            1., 1.
        )
    );
    actions.material.vec4.push(
        OpsUniformVec4::ops(
            idmat, 
            Atom::from(BlockEmissiveFresnel::KEY_RIGHT), 
            0.2, 0., 0., 1.
        )
    );
    actions.material.vec2.push(
        OpsUniformVec2::ops(
            idmat, 
            Atom::from(BlockEmissiveFresnel::KEY_PARAM), 
            0.2, 4.
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
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Startup, sys_setup_ball);
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    
    
    // app.run()
    loop { app.update(); }

}