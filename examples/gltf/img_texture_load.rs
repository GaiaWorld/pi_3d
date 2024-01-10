#![feature(box_into_inner)]

use pi_atom::Atom;
use pi_scene_shell::prelude::*;

#[path = "../base.rs"]
mod base;

fn setup(
    mut loader: ResMut<ImageTextureLoader>,
) {
    loader.create_load(KeyImageTexture { url: Atom::from("E:/Rust/PI/pi_3d/assets/images/eff_ui_ll_0805.png"), file: true, srgb: true, ..Default::default() });
}

fn sys_load_check(
    mut loader: ResMut<ImageTextureLoader>,
) {
    let mut item = loader.fails.pop();
    while let Some(param) = item {
        log::debug!("Failed: {:?}, Error: {:?}", param, loader.query_failed_reason(param));
        item = loader.fails.pop();
    }
    let mut item = loader.success_load.pop();
    while let Some(param) = item {
        log::debug!("Successed: {:?}, {:?}", param, loader.query_success(param).is_some());
        // log::error!("Successed: {:?}", param.1.errors.len());
        item = loader.success_load.pop();
    }
}

pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}



pub fn main() {
    let mut app = base::test_plugins_with_gltf();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    app.add_systems(Update, sys_load_check);
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    // app.run()
    loop { app.update(); }

}