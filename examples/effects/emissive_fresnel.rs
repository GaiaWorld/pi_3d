#![feature(box_into_inner)]


use base::DemoScene;
use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_mesh_builder::ball::*;
use pi_node_materials::prelude::*;
use pi_scene_context::prelude::*;
use unlit_material::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;


fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut renderercmds: ActionSetRenderer,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    ActionMaterial::regist_material_meta(
        &matcmds.metas,
        KeyShaderMeta::from(EmissiveFresnelShader::KEY),
        EmissiveFresnelShader::meta(),
    );

    let tes_size = 5;
    fps.frame_ms = 4;
    let demopass = DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut renderercmds, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -10.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut matcmds, &mut meshcmds, &mut geometrycmd, &mut cameracmds, &mut transformcmds, &mut renderercmds, scene, demopass.transparent_renderer,demopass.transparent_target);
    renderercmds.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let vertices = BallBuilder::attrs_meta();
    let indices = Some(BallBuilder::indices_meta());
    let state = MeshInstanceState { state: 0, ..Default::default() };
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut meshcmds, &mut geometrycmd, &mut transformcmds, vertices, indices, state);

    let mut blend = ModelBlend::default(); blend.combine();
    meshcmds.blend.push(OpsRenderBlend::ops(source, blend));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    matcmds.create.push(OpsMaterialCreate::ops(
        idmat,
        EmissiveFresnelShader::KEY,
    ));
    matcmds.vec4.push(OpsUniformVec4::ops(
        idmat,
        Atom::from(BlockEmissiveTexture::KEY_INFO),
        1.,
        0.,
        0.,
        1.,
    ));
    matcmds.vec2.push(OpsUniformVec2::ops(
        idmat,
        Atom::from(BlockEmissiveFresnel::KEY_PARAM),
        0.2,
        4.,
    ));
}

fn sys_setup_ball(
    mut data_map: ResMut<VertexBufferDataMap3D>,
) {
    let param = BallParam {
        sectors: 20,
        stacks: 20,
    };

    let (positions, normals, indices, uvs) = generate_sphere(&param);
    let id = "BallPos#20#20";
    ActionVertexBuffer::create(
        &mut data_map,
        KeyVertexBuffer::from(id),
        bytemuck::cast_slice(&positions)
            .iter()
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    );
    let id = "BallNor#20#20";
    ActionVertexBuffer::create(
        &mut data_map,
        KeyVertexBuffer::from(id),
        bytemuck::cast_slice(&normals)
            .iter()
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    );
    let id = "BallUV#20#20";
    ActionVertexBuffer::create(
        &mut data_map,
        KeyVertexBuffer::from(id),
        bytemuck::cast_slice(&uvs)
            .iter()
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    );
    let id = "BallInd#20#20";
    ActionVertexBuffer::create_indices(
        &mut data_map,
        KeyVertexBuffer::from(id),
        bytemuck::cast_slice(&indices)
            .iter()
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    );
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
    // bevy_mod_debugdump::print_main_schedule(&mut app);

    // app.run()
    loop { app.update(); }
}
