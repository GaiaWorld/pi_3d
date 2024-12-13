#![feature(box_into_inner)]


use base::DemoScene;
use ktx::KtxInfo;
use pi_animation::{loop_mode::ELoopMode, animation_group::AnimationGroupID};
use pi_atom::Atom;
use pi_curves::curve::frame_curve::FrameCurve;
use pi_hal::runtime::MULTI_MEDIA_RUNTIME;
use pi_render::rhi::device;
use pi_scene_shell::prelude::*;
use pi_node_materials::{prelude::*, NodeMaterialBlocks};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;
use unlit_material::*;

use core::num;
use std::mem::replace;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut matmetas: ResMut<ShareAssetMgr<ShaderEffectMeta>>,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    nodematblocks: Res<NodeMaterialBlocks>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
    mut list: ResMut<ActionListTestData>,
    demooption: Res<base::DemoOption>,
    image_assets_mgr: Res<ShareAssetMgr<ImageTextureFrame>>,
    queue: Res<PiRenderQueue>,
    mut testtex: ResMut<ResDemoTex>,
    engineopt: Res<EngineCustomPlugins>,
) {
    let (demopass, scene, camera01, copyrenderer, copyrendercamera) = if let (Some(demo), Some(copyrenderer), Some(copyrendercamera)) = (&demooption.demo, &demooption.copyrenderer, &demooption.copyrendercamera) {
        (demo, demo.scene, demo.camera, *copyrenderer, *copyrendercamera)
    } else { return; };

    ActionMaterial::regist_material_meta(&matmetas, KeyShaderMeta::from(OpacityClipShader::KEY), OpacityClipShader::create(&nodematblocks, &engineopt));

    let tes_size = 2;
    fps.frame_ms = 50;
    
    let tt = include_bytes!("../../assets/fight_res/01.s3tc.ktx");
    let ktx = ktx::Ktx::new(tt.as_slice());
    let w = ktx.pixel_width();
    let h = ktx.pixel_height();
    log::warn!("{:?}", (w, h));
    let mut url: EKeyTexture = EKeyTexture::from("assets/images/fractal.png");
    let key = Atom::from("TESTTEX");
    let url = {
        let width = 1024;
        let height = 1024;
        let dimension = wgpu::TextureViewDimension::D2;
        let format = wgpu::TextureFormat::Bc3RgbaUnorm;

        let texkey = KeyImageTextureFrame { url: key.clone(), file: false, compressed: true, cancombine: false
        };

        let device = &assets.1;
        let queue = &queue;
        let texture = ImageTextureFrame::create_data_texture(
            &device, &queue, &key, width, height, format,
            dimension, true, 1, None, None, 0
        );
        match image_assets_mgr.insert(texkey.clone(), ImageTextureFrame::new(texture)) {
            Ok(data) => { testtex.tex = Some(data) },
            Err(_) => {},
        };

        EKeyTexture::ImageFrame(KeyImageTextureViewFrame::new(texkey, TextureViewDesc::default() ))
    };

    actions.camera.param.push(OpsCameraModify::ops( camera01, ECameraModify::OrthSize( tes_size as f32 )));

    let root = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(root, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, root));

    let node = commands.spawn_empty_id(); actions.transform.tree.push(OpsTransformNodeParent::ops(node, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, node));

    let vertices = CubeBuilder::attrs_meta();
    let indices = Some(CubeBuilder::indices_meta());
    let state = base::instance_attr(false, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);
    let mut blend = ModelBlend::default(); blend.combine();
    actions.mesh.render_state.push(OpsRenderState::blend(source, DemoScene::PASS_TRANSPARENT, blend));

    actions.transform.tree.push(OpsTransformNodeParent::ops(source, node));
    actions.transform.tree.push(OpsTransformNodeParent::ops(node, root));

    let idmat = commands.spawn_empty_id();
    actions.material.create.push(OpsMaterialCreate::ops(idmat, OpacityClipShader::KEY));
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_TRANSPARENT));
    actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
        slotname: Atom::from(BlockMainTexture::KEY_TEX),
        sample: KeySampler::linear_repeat(),
        url: url,
        ..Default::default()
    }));
    // actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
    //     slotname: Atom::from(BlockOpacityTexture::KEY_TEX),
    //     filter: true,
    //     sample: KeySampler::linear_repeat(),
    //     url: EKeyTexture::from("assets/images/eff_ui_ll_085.png"),
    // }));
    actions.material.val.push(OpsUniformVal::float(
            idmat, 
            Atom::from(BlockCutoff::KEY_VALUE), 
            0.5
        )
    );
    actions.material.val.push(OpsUniformVal::vec4(
            idmat, 
            Atom::from(BlockEmissiveTexture::KEY_INFO), 
            1., 1., 1., 1.
        )
    );
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty_id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    // {
    //     let key_curve0 = pi_atom::Atom::from("cutoff");
    //     let key_curve0 =key_curve0.asset_u64();
    //     let mut curve = FrameCurve::<Cutoff>::curve_frame_values(10000);
    //     curve.curve_frame_values_frame(0, Cutoff(0.));
    //     curve.curve_frame_values_frame(10000, Cutoff(0.5));
        
    //     let asset_curve = if let Some(curve) = anime_assets.alphacutoff.get(&key_curve0) {
    //         curve
    //     } else {
    //         match anime_assets.alphacutoff.insert(key_curve0, TypeFrameCurve(curve)) {
    //             Ok(value) => {
    //                 value
    //             },
    //             Err(_) => {
    //                 return;
    //             },
    //         }
    //     };
    
    //     let animation = anime_contexts.alphacutoff.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
    //     actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group, idmat, animation));
    // }
    {
        let key_curve0 = pi_atom::Atom::from("Pos");
        let key_curve0 = key_curve0.asset_u64();
        let mut curve = FrameCurve::<LocalPosition>::curve_frame_values(10000);
        curve.curve_frame_values_frame(0, LocalPosition(Vector3::new(0., 0., 0.)));
        curve.curve_frame_values_frame(10000, LocalPosition(Vector3::new(2., 0., 0.)));
        
        let asset_curve = if let Some(curve) = anime_assets.position.get(&key_curve0) {
            curve
        } else {
            match anime_assets.position.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => {
                    value
                },
                Err(_e) => {
                    return;
                },
            }
        };
    
        let animation = anime_contexts.position.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.action.push(OpsAnimationGroupAction::addtarget(id_group, root, animation));
    }
    let mut parma = AnimationGroupParam::default();
    parma.loop_mode = ELoopMode::Not;
    parma.speed = 1.;
    // actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, parma, 0., pi_animation::base::EFillMode::NONE));

    // animegroupres.global.add_frame_event_listen(id_group);
    // animegroupres.global.add_frame_event(id_group, 0.5, 100);
    actions.anime.action.push(OpsAnimationGroupAction::listen_start(id_group));
    actions.anime.action.push(OpsAnimationGroupAction::listen_end(id_group));

    list.material = Some(idmat);
}

#[derive(Resource)]
pub struct ResDemoTex {
    tex: Option<Handle<ImageTextureFrame>>,
    counter: u32,
}

pub fn sys_sub_texture(
    mut tex: ResMut<ResDemoTex>,
    queue: Res<PiRenderQueue>,
    mut cmds: ResMut<TextureCombineCmds>,
    assets: Res<ShareAssetMgr<ImageTextureFrame>>,
) {
    tex.counter += 1;
    let (path, xoffset, yoffset, width, height) = if tex.counter == 60 {
        ("assets/fight_res/01.s3tc.ktx", 0, 0, 256, 256)
    } else if tex.counter == 120 {
        ("assets/fight_res/02.s3tc.ktx", 256, 0, 128, 64)
    } else if tex.counter == 180 {
        ("assets/fight_res/03.s3tc.ktx", 0, 256, 128, 128)
    } else if tex.counter == 240 {
        ("assets/fight_res/04.s3tc.ktx", 256, 256, 512, 512)
    } else { return };

    let info = [
        ("assets/fight_res/01.s3tc.ktx", 0, 0, 256, 256),
        ("assets/fight_res/02.s3tc.ktx", 256, 0, 128, 64),
        ("assets/fight_res/03.s3tc.ktx", 0, 256, 128, 128),
        ("assets/fight_res/04.s3tc.ktx", 256, 256, 512, 512)
    ];
    // let info = [
    //     ("assets/fight_res/01.png", 0, 0, 256, 256),
    //     ("assets/fight_res/02.png", 256, 0, 128, 64),
    //     ("assets/fight_res/03.png", 0, 256, 128, 128),
    //     ("assets/fight_res/04.png", 256, 256, 512, 512)
    // ];


    let path = Atom::from(path);
    let queue = queue.clone();
    if tex.tex.is_some() {
        let requestid = 0;
        let dimension = wgpu::TextureViewDimension::D2;
        let format = wgpu::TextureFormat::Bc3RgbaUnorm;
        let key = Atom::from("TESTTEX");
        let texkey = KeyImageTextureFrame { url: key.clone(), file: false, compressed: true, 
            cancombine: false
        };
        let mut atlas = XHashMap::default();
        let mut idx = 0;
        for (path, x, y, w, h) in info {
            atlas.insert(Atom::from(path), (requestid, idx, true, x, y, w, h));
            idx += 1;
        }
        cmds.request(requestid, texkey, atlas, &assets);
        tex.tex = None;
    }

    cmds.successed().for_each(|requestid| {
        log::warn!("Success: {}", requestid);
    });
    cmds.failed().for_each(|requestid| {
        log::warn!("Failed: {}", requestid);
    });
}

pub struct S3TC {
    data: Vec<u8>,
    width: u32,
    height: u32,
    size_per_pixel: u32,
    format: wgpu::TextureFormat,
}
impl S3TC {
    pub fn new(width: u32, height: u32, format: wgpu::TextureFormat) -> Self {
        let mut vec = vec![];
        let len = width * height;
        match format {
            wgpu::TextureFormat::Rgba8Unorm => {
                let mut data = Vec::with_capacity((len * 4) as usize);
                for _ in 0..len {
                    data.push(128);
                    data.push(0);
                    data.push(0);
                    data.push(255);
                }
                let size_per_pixel = 4;
                Self {
                    data: data,
                    width,
                    height,
                    size_per_pixel,
                    format
                }
            },
            wgpu::TextureFormat::Bc1RgbaUnorm => todo!(),
            wgpu::TextureFormat::Bc1RgbaUnormSrgb => todo!(),
            wgpu::TextureFormat::Bc2RgbaUnorm => todo!(),
            wgpu::TextureFormat::Bc2RgbaUnormSrgb => todo!(),
            wgpu::TextureFormat::Bc3RgbaUnorm => {
                let val: u16 = (0 << 11) + (0 << 5) + (0 << 0);
                let vv = [val,val,val,val];
                let tmp = bytemuck::cast_slice(&vv);
                let mut val = vec![];
                val.extend_from_slice(tmp);
                val.push(0);
                val.push(0);
                val.push(0);
                val.push(0);
                val.push(0);
                val.push(0);
                val.push(0);
                val.push(0);
                let len = len / 4 / 4;
                for _ in 0..len {
                    vec.extend_from_slice(&val);
                }
                let size_per_pixel = 16;
                Self {
                    data: vec,
                    width: width / 4,
                    height: height / 4,
                    size_per_pixel,
                    format
                }
            },
            wgpu::TextureFormat::Bc3RgbaUnormSrgb => todo!(),
            wgpu::TextureFormat::Astc { block, channel } => {
                match block {
                    wgpu::AstcBlock::B4x4 => todo!(),
                    wgpu::AstcBlock::B5x4 => todo!(),
                    wgpu::AstcBlock::B5x5 => todo!(),
                    wgpu::AstcBlock::B6x5 => todo!(),
                    wgpu::AstcBlock::B6x6 => todo!(),
                    wgpu::AstcBlock::B8x5 => todo!(),
                    wgpu::AstcBlock::B8x6 => todo!(),
                    wgpu::AstcBlock::B8x8 => todo!(),
                    wgpu::AstcBlock::B10x5 => todo!(),
                    wgpu::AstcBlock::B10x6 => todo!(),
                    wgpu::AstcBlock::B10x8 => todo!(),
                    wgpu::AstcBlock::B10x10 => todo!(),
                    wgpu::AstcBlock::B12x10 => todo!(),
                    wgpu::AstcBlock::B12x12 => todo!(),
                }
            },
            _ => todo!()
        }
    }

}

pub fn sys_anime_event(
    mut events: ResMut<GlobalAnimeEvents>,
    mut test: ResMut<ActionListTestData>,
    mut actions: pi_3d::ActionSets,
) {
    let mut list: Vec<(Entity, Entity, u8, u32)> = replace(&mut events, vec![]);
    list.drain(..).for_each(|item| {
        log::warn!("Event {:?}", item);
        if let Some(idmat) = test.material {
            if item.2 == 2 {
                // actions.material.valb.push(OpsUniformValB::texture(idmat, UniformTextureWithSamplerParam {
                //     slotname: Atom::from(BlockMainTexture::KEY_TEX),
                //     filter: true,
                //     sample: KeySampler::linear_repeat(),
                //     url: EKeyTexture::from("assets/images/eff_ui_ll_085.png"),
                // }));
            // } else {
            //     test.change = true;
            }
        }
    });
}

#[derive(Resource)]
pub struct ActionListTestData {
    material: Option<Entity>,
    change: bool,
}

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData { material: None, change: false  });
    }
}

pub fn main() {
    let (mut app, window, event_loop) = base::test_plugins();
    
    app.insert_resource(crate::base::DemoOption {
        orthographic_camera: true,
        camera_size: 5.,
        camera_position: (0., 0., -10.),
        ..Default::default()
    });
    app.add_startup_system(Update, base::setup_demoinit);

    app.add_plugins(PluginTest);
    
    // app.add_systems(Update, pi_3d::sys_info_node);
    // app.add_systems(Update, pi_3d::sys_info_resource);
    // app.add_systems(Update, pi_3d::sys_info_draw);
        #[cfg(feature = "use_bevy")]
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    #[cfg(not(feature = "use_bevy"))]
    app.add_startup_system(Update, setup.after(base::setup_default_mat));
    
    app.add_systems(Update, sys_anime_event.in_set(ERunStageChap::Modify));
    app.add_systems(Update, sys_sub_texture.in_set(ERunStageChap::Modify));

    app.world.insert_resource(ResDemoTex { tex: None, counter: 0 });
    
    // app.run()
    crate::base::run_loop(app, window, event_loop)

}