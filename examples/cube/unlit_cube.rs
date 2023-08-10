#![feature(box_into_inner)]


use pi_3d::PluginBundleDefault;
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime};
use pi_node_materials::{prelude::*, NodeMaterialBlocks, PluginNodeMaterial};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, ball::*, quad::{PluginQuadBuilder, QuadBuilder}};
use unlit_material::{PluginUnlitMaterial, command::*, shader::UnlitShader, effects::{main_opacity::MainOpacityShader, main_opacity_fresnel::MainOpacityFresnelShader, two_opacity_mix::TwoOpacityMixShader, stripes_virtual::StripesVirtualShader, distortion_uv::DistortionUVShader}};

use std::sync::Arc;
use pi_async_rt::rt::AsyncRuntime;
use pi_hal::{init_load_cb, runtime::MULTI_MEDIA_RUNTIME, on_load};

// #[derive(Debug, Default)]
// pub struct SingleTestData {
//     pub transforms: Vec<(ObjectID, f32, f32, f32)>,
// }

// pub struct SysTest;
// impl TSystemStageInfo for SysTest {}
// #[setup]
// impl SysTest {
//     #[system]
//     pub fn sys(
//         mut list: ResMut<SingleTestData>,
//         mut transform_commands: ResMut<SingleTransformNodeModifyCommandList>,
//     ) {
//         list.transforms.iter_mut().for_each(|mut item| {
//             item.1 = item.1 + 16.0;
//             item.2 = item.2 + 16.0;
//             item.3 = item.3 + 16.0;
//             let x0 = item.1 % 4000.0 / 4000.0;
//             let x = x0 * 3.1415926 * 2.;
//             let y0 = item.2 % 4000.0 / 4000.0;
//             let y = y0 * 3.1415926 * 2.;
//             let z0 = item.3 % 4000.0 / 4000.0;
//             let z = z0 * 3.1415926 * 2.;
//             // transform_commands.list.push(TransformNodeCommand::ModifyPosition(item.0, Vector3::new(x.cos() * 3., 0., 0.)));
//             // transform_commands.list.push(TransformNodeCommand::ModifyScaling(item.0, Vector3::new(x.cos() + 0.5, x.sin() + 0.5, x + 0.5)));
//             transform_commands.list.push(ETransformNodeModifyCommand::ModifyRotation(item.0, Vector3::new(x, y, z)));
//         });
//     }
// }

// #[derive(Debug)]
// pub struct PluginTest;
// impl Plugin for PluginTest {
//     fn init(
//         &mut self,
//         engine: &mut pi_scene_context::engine::Engine,
//         stages: &mut pi_scene_context::run_stage::RunStage,
//     ) -> Result<(), pi_scene_context::plugin::ErrorPlugin> {
//         PluginLocalLoad.init(engine, stages);
//         PluginBundleDefault.init(engine, stages);
//         // PluginMaterialTextures.init(engine, stages);
//         // PluginMainTexture.init(engine, stages);
//         PluginUnlitMaterial.init(engine, stages);
//         PluginCubeBuilder.init(engine, stages);

//         let world = engine.world_mut();

//         SysTest::setup(world, stages.query_stage::<SysTest>(ERunStageChap::Command));

//         let testdata = SingleTestData::default();
//         world.insert_resource(testdata);

//         Ok(())
//     }
// }

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
    let tes_size = 20;
    fps.frame_ms = 4;
    final_render.cleardepth = 0.0;

    let scene = commands.spawn_empty().id();
    scenecmds.create.push(OpsSceneCreation::ops(scene, ScenePassRenderCfg::default()));

    let camera01 = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(camera01, scene));
    cameracmds.create.push(OpsCameraCreation::ops(scene, camera01, String::from("TestCamera"), true));
    transformcmds.localpos.push(OpsTransformNodeLocalPosition::ops(camera01, 0., 0., -10.));
    cameracmds.active.push(OpsCameraActive::ops(camera01, true));
    cameracmds.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));
    
    let desc = RendererGraphicDesc {
        pre: Some(final_render.clear_entity),
        curr: String::from("TestCamera"),
        next: Some(final_render.render_entity),
        passorders: PassTagOrders::new(vec![EPassTag::Opaque, EPassTag::Water, EPassTag::Sky, EPassTag::Transparent])
    };
    let id_renderer = commands.spawn_empty().id(); renderercmds.create.push(OpsRendererCreate::ops(id_renderer, desc.curr.clone()));
    renderercmds.connect.push(OpsRendererConnect::ops(final_render.clear_entity, id_renderer));
    renderercmds.connect.push(OpsRendererConnect::ops(id_renderer, final_render.render_entity));
    cameracmds.render.push(OpsCameraRendererInit::ops(camera01, id_renderer, desc.curr, desc.passorders, ColorFormat::Rgba8Unorm, DepthStencilFormat::None));


    let source = commands.spawn_empty().id(); transformcmds.tree.push(OpsTransformNodeParent::ops(source, scene));
    meshcmds.create.push(OpsMeshCreation::ops(scene, source, String::from("TestCube")));
    let mut blend = ModelBlend::default(); blend.combine();
    meshcmds.blend.push(OpsRenderBlend::ops(source, blend));
    
    let id_geo = commands.spawn_empty().id();
    geometrycmd.create.push(OpsGeomeryCreate::ops(source, id_geo, CubeBuilder::attrs_meta(), Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    matcmds.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY, EPassTag::Transparent));
    matcmds.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        filter: true,
        sample: KeySampler::linear_repeat(),
        url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/Q69L5MmgSNC2xbBiAwZcDw.png"),
    }));
    
    matcmds.usemat.push(OpsMaterialUse::ops(source, idmat));
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
    
    app.add_startup_system(setup);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()
}