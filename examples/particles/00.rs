

use base::DemoScene;
use pi_scene_shell::prelude::*;
use pi_scene_context::{prelude::*, viewer::prelude::{ViewerGlobalPosition, ViewerViewMatrix}};
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    let tes_size = 10;
    fps.frame_ms = 4;

    let demopass = base::DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        tes_size as f32, 0.7, (0., 0., -10.), true
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let source = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(source, scene));
    actions.mesh.create.push(OpsMeshCreation::ops(scene, source, base::particelsystem_mesh_state()));
    
    let id_geo = commands.spawn_empty().id();
    let attrs = CubeBuilder::attrs_meta();
    actions.geometry.create.push(OpsGeomeryCreate::ops(source, id_geo, attrs, Some(CubeBuilder::indices_meta())));

    let idmat = commands.spawn_empty().id();
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
    actions.material.create.push(OpsMaterialCreate::ops(idmat, UnlitShader::KEY));
    // actions.material.texture.push(OpsUniformTexture::ops(idmat, UniformTextureWithSamplerParam {
    //     slotname: Atom::from("_MainTex"),
    //     filter: true,
    //     sample: KeySampler::default(),
    //     url: EKeyTexture::from("E:/Rust/PI/pi_3d/assets/images/bubbles.png"),
    // }));
    
    commands.entity(source).insert((Particle,));
}

#[derive(Component)]
pub struct Particle;

fn sys_demo_particle(
    particles: Query<(&SceneID, &GeometryID), With<Particle>>,
    scenes: Query<(&SceneTime, &SceneMainCameraID)>,
    cameras: Query<(&ViewerGlobalPosition, &ViewerViewMatrix)>,
    mut actions: pi_3d::ActionSets,
) {
    particles.iter().for_each(|(idscene, idgeo)| {
        if let Ok((scenetime, maincamera)) = scenes.get(idscene.0) {

            let (_camerapos, _camerarotationmatrix) = if let Some(maincamera) = maincamera.0 {
                if let Ok((viewpos, viewmat)) = cameras.get(maincamera) {
                    (viewpos.0.clone(), viewmat.get_rotation_matrix())
                } else {
                    (Vector3::new(0., 0., -1.), Matrix::identity())
                }
            } else {
                (Vector3::new(0., 0., -1.), Matrix::identity())
            };

            // if let Ok((_, _)) = geometrys.get_mut(idgeo.0) {
            //     let mut buffermatrix = vec![];
            //     let mut buffercolor = vec![];
            //     let mut bufferuv = vec![];
            
            //     for z in 0..20 {
            //         let ringcount = (z + 1) * 10;
            //         let tt = if z % 2 == 0 {
            //             scenetime.time_ms as f32 * 0.002
            //         } else {
            //             scenetime.time_ms as f32 * 0.002 * -1.
            //         };
            //         for x in 0..ringcount {
            //             let t: f32 = (tt + x as f32 * (1. / ringcount as f32)) * 3.1415926 * 2.;
            //             let mut wm = Matrix::identity();
            //             wm.append_translation_mut(
            //                 &Vector3::new(
            //                     f32::cos(t) * 2. * ( z as f32 + 1.0),
            //                     f32::sin(t) * 2. * ( z as f32 + 1.0),
            //                     0.,
            //                 )
            //             );
            //             buffermatrix.push(wm);

            //             buffercolor.push(Vector4::new(
            //                 f32::cos(tt + x as f32) * 0.5 + 0.5,
            //                 f32::sin(tt) * 0.5 + 0.5,
            //                 f32::sin(tt + z as f32) * 0.5 + 0.5,
            //                 f32::cos(tt) * 0.5 + 0.5,
            //             ));
            //             bufferuv.push(Vector4::new(1., 1., 0., 0.));
            //         }
            //     }

            //     let mut colordata : Vec<u8> = vec![];
            //     buffercolor.iter().for_each(|v| {
            //         bytemuck::cast_slice(v.as_slice()).iter().for_each(|v| {
            //             colordata.push(*v);
            //         })
            //     });
                
            //     let mut wmdata: Vec<u8> = vec![];
            //     buffermatrix.iter().for_each(|v| {
            //         bytemuck::cast_slice(v.as_slice()).iter().for_each(|v| {
            //             wmdata.push(*v);
            //         })
            //     });
            //     let mut uvdata: Vec<u8> = vec![];
            //     bufferuv.iter().for_each(|v| {
            //         bytemuck::cast_slice(v.as_slice()).iter().for_each(|v| {
            //             uvdata.push(*v);
            //         })
            //     });

            //     actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(node, x, y, z));
            //     actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(node, x, y, z));
            //     actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(node, x, y, z));

            //     // instance_buffer_update::<InstanceBufferWorldMatrix>(wmdata, idgeo.0, &mut wm, &mut geoloader, &mut vb_data_map, &mut slots, &mut allocator, &asset_mgr, &device, &queue);
            //     // instance_buffer_update::<InstanceBufferColor>(colordata, idgeo.0, &mut colors, &mut geoloader, &mut vb_data_map, &mut slots, &mut allocator, &asset_mgr, &device, &queue);
            // }
        }
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
    
    app.add_plugins(PluginTest);
    app.world.get_resource_mut::<StateRecordCfg>().unwrap().write_state = false;
    
    app.add_systems(Update, 
        sys_demo_particle.in_set(StageModel::RenderMatrix)
    );

        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    
    // app.run()
    loop { app.update(); }

}