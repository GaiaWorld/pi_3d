#![feature(box_into_inner)]

use pi_atom::Atom;
use pi_engine_shell::prelude::*;
use pi_node_materials::NodeMaterialBlocks;
use pi_scene_context::prelude::*;
use pi_mesh_builder::{cube::*, ball::*};
use unlit_material::*;


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
    mut final_render: ResMut<WindowRenderer>,
    nodematblocks: Res<NodeMaterialBlocks>,
    mut renderercmds: ActionSetRenderer,
) {
    ActionMaterial::regist_material_meta(&matcmds.metas,  KeyShaderMeta::from(StripesVirtualShader::KEY), StripesVirtualShader::create(&nodematblocks));

    let tes_size = 5;
    fps.frame_ms = 4;

    let (scene, camera01) = base::DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 0., -10.), true);
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));

    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    let instancestate = 0;
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, MeshInstanceState { state: instancestate, use_single_instancebuffer: false }));
    let mut blend = ModelBlend::default(); blend.combine();
    meshcmds.blend.push(OpsRenderBlend::ops(source, blend));
    
    let id_geo = commands.spawn_empty().id();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, 
        CubeBuilder::attrs_meta(),
        Some(CubeBuilder::indices_meta()),
    ));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    matcmds.create.push(OpsMaterialCreate::ops(idmat, StripesVirtualShader::KEY, EPassTag::Transparent));
    matcmds.float.push(
        OpsUniformFloat::ops(
            idmat, 
            Atom::from(StripesVirtualShader::KEY_SPEED), 
            1.,
        )
    );
    matcmds.float.push(
        OpsUniformFloat::ops(
            idmat, 
            Atom::from(StripesVirtualShader::KEY_STEP), 
            0.1,
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

#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Startup, sys_setup_ball);
    app.add_systems(Startup, setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    // app.run()
    loop { app.update(); }

}