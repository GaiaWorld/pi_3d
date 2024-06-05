
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;
#[path = "../light.rs"]
mod light;
#[path = "../shadow.rs"]
mod shadow;
#[path = "../pbr_material.rs"]
mod pbr_material;

#[path = "../pbr/01.rs"]
mod pbr;

pub fn display_boundingbox(
    mut actions: pi_3d::ActionSets,
    scenes: Query<(Entity, &BoundingBoxDisplay)>,
) {
    scenes.iter().for_each(|(entity, state)| {
        if state.display == false {
            actions.scene.boundingboxdisplay.push(OpsBoundingBoxDisplay::ops(entity, true, base::DemoScene::PASS_TRANSPARENT));
        }
    });
}

pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins_with_gltf();
    app.add_plugins(
        pi_pbr::PluginPBR
    );
    app.add_plugins(
        pbr_material::PluginPBRMaterial
    );

    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Update, pi_3d::sys_info_draw);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;

    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, pbr::setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, pbr::setup.after(base::setup_default_mat));
    #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, base::active_lighting_shadow);
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, base::active_lighting_shadow);
    #[cfg(feature = "use_bevy")]
    app.add_systems(Update, display_boundingbox);
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, display_boundingbox);
    

    // app.run()
    loop { app.update(); }

}