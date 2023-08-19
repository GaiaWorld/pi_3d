#![feature(box_into_inner)]


use base::DemoScene;
use pi_3d::PluginBundleDefault;
use pi_animation::{amount::AnimationAmountCalc, loop_mode::ELoopMode};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{frame_time::PluginFrameTime, prelude::*};
use pi_mesh_builder::{ball::*, cube::*, quad::PluginQuadBuilder};
use pi_node_materials::{prelude::*, NodeMaterialBlocks, PluginNodeMaterial};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use unlit_material::{
    command::*,
    effects::{emissive_fresnel::EmissiveFresnelShader, main_opacity::MainOpacityShader},
    shader::UnlitShader,
    PluginUnlitMaterial,
};

use pi_async_rt::rt::AsyncRuntime;
use pi_hal::{init_load_cb, on_load, runtime::MULTI_MEDIA_RUNTIME};
use std::sync::Arc;


fn setup(
    mut commands: Commands,
    mut scenecmds: ActionSetScene,
    mut cameracmds: ActionSetCamera,
    mut transformcmds: ActionSetTransform,
    mut meshcmds: ActionSetMesh,
    mut instancemeshcmds: ActionSetInstanceMesh,
    mut geometrycmd: ActionSetGeometry,
    mut matcmds: ActionSetMaterial,
    mut animegroupcmd: ActionSetAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut final_render: ResMut<WindowRenderer>,
    nodematblocks: Res<NodeMaterialBlocks>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut renderercmds: ActionSetRenderer,
) {
    ActionMaterial::regist_material_meta(
        &matcmds.metas,
        &mut matcmds.metas_wait,
        KeyShaderMeta::from(EmissiveFresnelShader::KEY),
        EmissiveFresnelShader::meta(),
    );

    let tes_size = 5;
    fps.frame_ms = 4;
    let (scene, camera01) = DemoScene::new(&mut commands, &mut scenecmds, &mut cameracmds, &mut transformcmds, &mut animegroupcmd, &mut final_render, &mut renderercmds, tes_size as f32, 0.7, (0., 0., -10.), true);


    let source = commands.spawn_empty().id();
    transformcmds
        .tree
        .push(OpsTransformNodeParent::ops(source, scene));
    meshcmds.create.push(OpsMeshCreation::ops(
        scene,
        source,
        String::from("TestCube"),
    ));
    let mut blend = ModelBlend::default();
    blend.combine();
    meshcmds.blend.push(OpsRenderBlend::ops(source, blend));

    let id_geo = commands.spawn_empty().id();
    geometrycmd.create.push(OpsGeomeryCreate::ops(
        source,
        id_geo,
        vec![
            VertexBufferDesc::vertices(
                KeyVertexBuffer::from("BallPos#20#20"),
                None,
                vec![VertexAttribute {
                    kind: EVertexDataKind::Position,
                    format: wgpu::VertexFormat::Float32x3,
                }],
            ),
            VertexBufferDesc::vertices(
                KeyVertexBuffer::from("BallNor#20#20"),
                None,
                vec![VertexAttribute {
                    kind: EVertexDataKind::Normal,
                    format: wgpu::VertexFormat::Float32x3,
                }],
            ),
            VertexBufferDesc::vertices(
                KeyVertexBuffer::from("BallUV#20#20"),
                None,
                vec![VertexAttribute {
                    kind: EVertexDataKind::UV,
                    format: wgpu::VertexFormat::Float32x2,
                }],
            ),
        ],
        Some(IndicesBufferDesc {
            format: wgpu::IndexFormat::Uint16,
            buffer_range: None,
            buffer: KeyVertexBuffer::from("BallInd#20#20"),
        }),
    ));

    let idmat = commands.spawn_empty().id();
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
    matcmds.create.push(OpsMaterialCreate::ops(
        idmat,
        EmissiveFresnelShader::KEY,
        EPassTag::Transparent,
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
    asset_mgr: Res<ShareAssetMgr<EVertexBufferRange>>,
    mut data_map: ResMut<VertexBufferDataMap3D>,
) {
    let param = BallParam {
        sectors: 20,
        stacks: 20,
    };

    let (positions, normals, indices, uvs) = generate_sphere(&param);
    let id = ("BallPos#20#20");
    ActionVertexBuffer::create(
        &mut data_map,
        KeyVertexBuffer::from(id),
        bytemuck::cast_slice(&positions)
            .iter()
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    );
    let id = ("BallNor#20#20");
    ActionVertexBuffer::create(
        &mut data_map,
        KeyVertexBuffer::from(id),
        bytemuck::cast_slice(&normals)
            .iter()
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    );
    let id = ("BallUV#20#20");
    ActionVertexBuffer::create(
        &mut data_map,
        KeyVertexBuffer::from(id),
        bytemuck::cast_slice(&uvs)
            .iter()
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    );
    let id = ("BallInd#20#20");
    ActionVertexBuffer::create_indices(
        &mut data_map,
        KeyVertexBuffer::from(id),
        bytemuck::cast_slice(&indices)
            .iter()
            .map(|v| *v)
            .collect::<Vec<u8>>(),
    );
}

pub trait AddEvent {
    // 添加事件， 该实现每帧清理一次
    fn add_frame_event<T: Event>(&mut self) -> &mut Self;
}

impl AddEvent for App {
    fn add_frame_event<T: Event>(&mut self) -> &mut Self {
        if !self.world.contains_resource::<Events<T>>() {
            self.init_resource::<Events<T>>()
                .add_system(Events::<T>::update_system);
        }
        self
    }
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
        app.add_frame_event::<ComponentEvent<Changed<Layer>>>();
    }
}

#[path = "../base.rs"]
mod base;
pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugin(PluginTest);

    app.add_startup_system(sys_setup_ball);
    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);

    app.run()
}
