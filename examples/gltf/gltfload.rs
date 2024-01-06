#![feature(box_into_inner)]

use pi_atom::Atom;
use pi_engine_shell::prelude::*;

#[path = "../base.rs"]
mod base;

fn setup(
    mut commands: Commands,
    loader: Res<pi_gltf2_load::GLTFResLoader>,
) {
    let id = commands.spawn_empty().id();
    loader.create_load(id, pi_gltf2_load::KeyGLTF { base_url: Atom::from("E:/Rust/PI/pi_3d/assets/gltf/AnMiaoYi_YeYueZouQinQu_Cast_ff/AnMiaoYi_YeYueZouQinQu_Cast_ff.gltf"), dyn_desc: Atom::from("")  });
}

fn sys_load_check(
    mut loader: ResMut<pi_gltf2_load::GLTFResLoader>,
) {
    let mut item = loader.fails.pop();
    while let Some(param) = item {
        log::warn!("Failed: {:?}, Error: {:?}", param, loader.get_fail_reason(param));
        item = loader.fails.pop();
    }
    let mut item = loader.success.pop();
    while let Some(param) = item {
        if let Some(gltf) = loader.get_success(param) {
            gltf.position.iter().for_each(|curve| {
                log::warn!("position: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
            });
            gltf.euler.iter().for_each(|curve| {
                log::warn!("euler: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
            });
            gltf.scaling.iter().for_each(|curve| {
                log::warn!("scaling: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
            });
            gltf.quaternion.iter().for_each(|curve| {
                log::warn!("quaternion: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
            });
            gltf.float.iter().for_each(|curve| {
                log::warn!("float: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
            });
            gltf.vec2s.iter().for_each(|curve| {
                log::warn!("vec2s: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
            });
            gltf.vec3s.iter().for_each(|curve| {
                log::warn!("vec3s: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
            });
            gltf.vec4s.iter().for_each(|curve| {
                log::warn!("vec4s: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
            });
        }
        log::warn!("Successed: {:?}, {:?}", param, loader.get_success(param).is_some());
        // log::error!("Successed: {:?}", param.1.errors.len());
        item = loader.success.pop();
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