#![feature(box_into_inner)]
#![feature(test)]
extern crate test;

use std::sync::Arc;

use pi_atom::Atom;
use pi_gltf::Gltf;
use pi_gltf2_load::{relative_path, GLTFBase, GLTFTempLoaded, GLTF};
use pi_particle_system::prelude::{ActionSetParticleSystem, ResourceParticleSystem};
use pi_scene_context::prelude::TypeAnimeAssetMgrs;
use pi_scene_shell::prelude::*;

#[path = "../base.rs"]
mod base;

fn setup(
    mut commands: Commands,
    loader: Res<pi_gltf2_load::GLTFResLoader>,
) {
    let id = commands.spawn_empty_id();
    // loader.create_load(id, Atom::from("examples/gltf/eff_new_common_hit.gltf") );
    // loader.create_load(id, Atom::from("assets/gltf/AnMiaoYi_YeYueZouQinQu_Cast_ff/AnMiaoYi_YeYueZouQinQu_Cast_ff.gltf") );
    // loader.create_load(id, Atom::from("assets/gltf/eff_ui_leijie/eff_ui_leijie.gltf") );
    // loader.create_load(id, Atom::from("assets/gltf/Common_ZhongDu_Buff/Common_ZhongDu_Buff.gltf") );
    loader.create_load(id, Atom::from("assets/gltf/m_mine_20101_1/m_mine_20101_1.gltf") );
    loader.create_load(id, Atom::from("assets/gltf/m_mine_20201_3/m_mine_20201_3.gltf") );
}

fn sys_load_check(
    mut loader: ResMut<pi_gltf2_load::GLTFResLoader>,
    mut commands: Commands,
    anime_assets: TypeAnimeAssetMgrs,
    mut vballocator: ResMut<VertexBufferAllocator3D>,
    mut particlesys_cmds: ActionSetParticleSystem,
    mut particlesys_res: ResourceParticleSystem,
    vb_assets_mgr: Res<ShareAssetMgr<AssetVertexBuffer>>,
    assets_mgr: Res<ShareAssetMgr<GLTF>>,
    device: Res<PiRenderDevice>,
    queue: Res<PiRenderQueue>,
    mut _performance: ResMut<Performance>,
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

        log::warn!("Successed: {:?}, {:?}, {:?}",
            query, gltf.path, (gltf.position.len(), gltf.euler.len(), gltf.scaling.len(), gltf.quaternion.len(), gltf.float.len(), gltf.vec2s.len(), gltf.vec3s.len(), gltf.vec4s.len())
        );
        // log::error!("Successed: {:?}", param.1.errors.len());
    });
    // let key = "./eff_new_common_hit.gltf";
    // let data = include_bytes!("./eff_new_common_hit.gltf");
    // // let buffer0 = include_bytes!("./SWrNZM4WpMXD3Sx5ath2eo.anim.bin");
    // // let buffer1 = include_bytes!("./eff_new_common_hit.mesh.bin");
    // match pi_gltf::Gltf::from_slice(data) {
    //     Ok(gltf) => {
    //         // let mut buffers = vec![];
    //         let mut haserror = false;
    //         for buffer in gltf.buffers() {
    //             match buffer.source() {
    //                 pi_gltf::buffer::Source::Bin => {
    //                     haserror = true;
    //                 },
    //                 pi_gltf::buffer::Source::Uri(bufferpath) => {
    //                     if bufferpath.starts_with("data:") {
    //                         haserror = true;
    //                     } else {
    //                         // if bufferpath == "SWrNZM4WpMXD3Sx5ath2eo.anim.bin" {
    //                         //     buffers.push(Arc::new(buffer0.to_vec()));
    //                         // } else {
    //                         //     buffers.push(Arc::new(buffer1.to_vec()));

    //                         // }
    //                     }
    //                 },
    //             }
    //             if haserror { break; }
    //         };
    //         // if haserror == false {
    //         //     let mut size = 0;
    //         //     buffers.iter().for_each(|val| { size += val.len(); });
    //         //     let result = GLTFBase { gltf: Arc::new(gltf), size, buffers };
    //         //     let gltf = GLTFTempLoaded::analy(
    //         //         result, Atom::from(key.clone()), &mut commands, &vb_assets_mgr, &mut vballocator,
    //         //         &device, &queue, &anime_assets, &mut particlesys_cmds, &mut particlesys_res
    //         //     );
    //         // }

    //     },
    //     Err(_e) => {
            
    //     }
    // };
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
    app.add_systems(StageD3, sys_load_check);
    
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)
}

#[cfg(test)]
mod test_mod {
    use pi_gltf::Gltf;
    use pi_gltf2_load::relative_path;
    use test::Bencher;

    #[bench]
    fn performance(b: &mut Bencher) {
        let data = include_bytes!("./eff_new_common_hit.gltf");
        b.iter(move || {
            
            match Gltf::from_slice(data) {
                Ok(gltf) => {
                    let mut haserror = false;
                    for buffer in gltf.buffers() {
                        match buffer.source() {
                            pi_gltf::buffer::Source::Bin => {
                                haserror = true;
                            },
                            pi_gltf::buffer::Source::Uri(bufferpath) => {
                                if bufferpath.starts_with("data:") {
                                    haserror = true;
                                } else {
                                    let bufferpath = relative_path(bufferpath, "./AnMiaoYi_YeYueZouQinQu_Cast_ff.gltf");
                                    // match pi_hal::file::load_from_url(&Atom::from(bufferpath) ).await {
                                    //     Ok(bufferfile) => {
                                    //         buffers.push(bufferfile);
                                    //     },
                                    //     Err(_e) =>  {
                                    //         haserror = true;
                                    //         // log::warn!("load gltf bin fail: {:?}", desc.as_str());
                                    //         fail.push((key.clone(), ErrorRecord::ERROR_GLTF_BIN_LOAD_FAIL));
                                    //         // Err(std::io::Error::new(std::io::ErrorKind::NotFound, ""));
                                    //     },
                                    // };
                                }
                            },
                        }
                        if haserror { break; }
                    };
                },
                Err(_e) => {
                    
                }
            };
        });
    }
}