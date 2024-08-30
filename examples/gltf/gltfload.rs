#![feature(box_into_inner)]

use pi_atom::Atom;
use pi_scene_shell::prelude::*;

#[path = "../base.rs"]
mod base;

fn setup(
    mut commands: Commands,
    loader: Res<pi_gltf2_load::GLTFResLoader>,
) {
    let id = commands.spawn_empty_id();
    loader.create_load(id, Atom::from("assets/gltf/AnMiaoYi_YeYueZouQinQu_Cast_ff/AnMiaoYi_YeYueZouQinQu_Cast_ff.gltf") );
}

fn sys_load_check(
    mut loader: ResMut<pi_gltf2_load::GLTFResLoader>,
) {
    loader.failed.drain().for_each(|(query, error)| {
        log::warn!("Failed: {:?}, Error: {:?}", query, error);
    });
    loader.successed.drain().for_each(|(query, gltf)| {
        gltf.position.iter().for_each(|curve| {
            // log::warn!("position: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
        });
        gltf.euler.iter().for_each(|curve| {
            // log::warn!("euler: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
        });
        gltf.scaling.iter().for_each(|curve| {
            // log::warn!("scaling: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
        });
        gltf.quaternion.iter().for_each(|curve| {
            // log::warn!("quaternion: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
        });
        gltf.float.iter().for_each(|curve| {
            // log::warn!("float: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
        });
        gltf.vec2s.iter().for_each(|curve| {
            // log::warn!("vec2s: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
        });
        gltf.vec3s.iter().for_each(|curve| {
            // log::warn!("vec3s: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
        });
        gltf.vec4s.iter().for_each(|curve| {
            // log::warn!("vec4s: {:?}", (&curve.0.frames, &curve.0.values, &curve.0.cubic_spline_values));
        });

        log::warn!("Successed: {:?}, {:?}", query, gltf.path);
        // log::error!("Successed: {:?}", param.1.errors.len());
    });
}


pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}


pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins_with_gltf();

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
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    app.add_systems(Update, sys_load_check);
    
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}