
use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode};
use pi_engine_shell::prelude::*;
use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_node_materials::prelude::BlockMainTexture;
use pi_scene_context::{prelude::*, light::PluginLighting};
use pi_scene_math::*;
use pi_mesh_builder::{cube::*, ball::BallBuilder};

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
    let mut app = base::test_plugins_with_gltf();
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

    app.add_systems(Startup, pbr::setup.after(base::setup_default_mat));
    app.add_systems(Startup, base::active_lighting_shadow);
    app.add_systems(Update, display_boundingbox);
    // bevy_mod_debugdump::print_main_schedule(&mut app);

    // app.run()
    loop { app.update(); }

}