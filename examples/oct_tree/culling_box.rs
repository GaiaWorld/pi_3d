
use pi_scene_shell::prelude::*;
use pi_scene_context::prelude::*;
use pi_winit::event::{Event, WindowEvent};
use pi_world::editor::EntityEditor;

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
    let  (mut app, window, event_loop)  = base::test_plugins_with_gltf();
    app.add_plugins(
        pi_pbr::PluginPBR
    );
    app.add_plugins(
        pbr_material::PluginPBRMaterial
    );

    app.add_system(Update, pi_3d::sys_info_node);
    app.add_system(Update, pi_3d::sys_info_resource);
    app.add_system(Update, pi_3d::sys_info_draw);
    app.world.get_single_res_mut::<StateRecordCfg>().unwrap().write_state = false;

    app.add_startup_system(Update, pbr::setup.after(base::setup_default_mat));
    app.add_startup_system(Update, base::active_lighting_shadow);
    app.add_system(Update, display_boundingbox);
    
    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.set_exit();
                }
                
                _ => (),
            },
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            Event::RedrawRequested(_window_id) => {
                app.run();
            }
            
            _ => (),
        }
    });

}