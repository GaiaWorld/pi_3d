#![feature(box_into_inner)]


use pi_3d::{PluginBundleDefault};
use pi_animation::{loop_mode::ELoopMode, amount::AnimationAmountCalc};
use pi_atom::Atom;
use pi_bevy_ecs_extend::system_param::layer_dirty::ComponentEvent;
use pi_bevy_render_plugin::PiRenderPlugin;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::{prelude::*, frame_time::PluginFrameTime};
use pi_node_materials::{prelude::*, PluginNodeMaterial, NodeMaterialBlocks};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, ball::*, quad::PluginQuadBuilder};
use unlit_material::{PluginUnlitMaterial, command::*, shader::UnlitShader, effects::main_opacity::MainOpacityShader};

use std::sync::Arc;
use pi_async_rt::prelude::AsyncRuntime;
use pi_hal::{init_load_cb, runtime::MULTI_MEDIA_RUNTIME, on_load};


fn setup(
    mut commands: Commands,
    loader: Res<pi_gltf2_load::GLTFResLoader>,
) {
    let id = commands.spawn_empty().id();
    loader.wait.push((id, pi_gltf2_load::KeyGLTF { base_url: Atom::from("E:/Rust/PI/pi_3d/assets/gltf/m_mine_20101_1/m_mine_20101_1.gltf"), dyn_desc: Atom::from("")  }))
}

fn sys_load_check(
    mut loader: ResMut<pi_gltf2_load::GLTFResLoader>,
) {
    let mut item = loader.fails.pop();
    while let Some(param) = item {
        log::error!("Failed: {:?}, Error: {:?}", param, loader.get_fail_reason(param));
        item = loader.fails.pop();
    }
    let mut item = loader.success.pop();
    while let Some(param) = item {
        log::error!("Successed: {:?}, {:?}", param, loader.get_success(param).is_some());
        // log::error!("Successed: {:?}", param.1.errors.len());
        item = loader.success.pop();
    }
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
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugin(PluginTest);
    
    app.add_startup_system(setup);
    app.add_system(sys_load_check);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    app.run()

}