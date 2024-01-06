#![feature(box_into_inner)]



use base::DemoScene;
use pi_curves::{curve::frame_curve::FrameCurve, easing::EEasingMode, amount::AnimationAmountCalc};
use pi_engine_shell::{prelude::*, frame_time::SingleFrameTimeCommand};

use pi_scene_context::prelude::{TypeAnimeAssetMgrs, TypeAnimeContexts};
use pi_scene_context::prelude::*;
use pi_scene_math::*;
use pi_mesh_builder::cube::*;

#[path = "../base.rs"]
mod base;
#[path = "../copy.rs"]
mod copy;

fn setup(
    mut commands: Commands,
    mut actions: pi_3d::ActionSets,
    mut animegroupres: ResourceAnimationGroup,
    mut fps: ResMut<SingleFrameTimeCommand>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    anime_assets: TypeAnimeAssetMgrs,
    mut anime_contexts: TypeAnimeContexts,
    mut assets: (ResMut<CustomRenderTargets>, Res<PiRenderDevice>, Res<ShareAssetMgr<SamplerRes>>, Res<PiSafeAtlasAllocator>,),
) {
    let tes_size = 20;
    fps.frame_ms = 30;
    

    let demopass = DemoScene::new(&mut commands, &mut actions, &mut animegroupres, 
        &mut assets.0, &assets.1, &assets.2, &assets.3,
        4., 0.7, (0., 0., -10.), false
    );
    let (scene, camera01) = (demopass.scene, demopass.camera);

    let (copyrenderer, copyrendercamera) = copy::PluginImageCopy::toscreen(&mut commands, &mut actions, scene, demopass.transparent_renderer,demopass.transparent_target);
    actions.renderer.connect.push(OpsRendererConnect::ops(demopass.transparent_renderer, copyrenderer, false));

    // actions.camera.size.push(OpsCameraOrthSize::ops(camera01, tes_size as f32));
    // actions.camera.target.push(OpsCameraTarget::ops(camera01, 0., -1., 4.));

    let root = commands.spawn_empty().id(); actions.transform.tree.push(OpsTransformNodeParent::ops(root, scene));
    actions.transform.create.push(OpsTransformNode::ops(scene, root));
    // actions.transform.localpos.push(OpsTransformNodeLocalPosition::ops(root, 0., 0., 0.));
    // actions.transform.tree.push(OpsTransformNodeParent::ops(camera01, root));


    let vertices = CubeBuilder::attrs_meta();
    let indices = Some(CubeBuilder::indices_meta());
    let state = base::instance_attr(false, false, false);
    let source = base::DemoScene::mesh(&mut commands, scene, scene, &mut actions,  vertices, indices, state);

    let idmat = defaultmat.0;
    actions.material.usemat.push(OpsMaterialUse::ops(source, idmat, DemoScene::PASS_OPAQUE));
    
    // let key_group = pi_atom::Atom::from("key_group");
    let id_group = commands.spawn_empty().id();
    // animegroupres.scene_ctxs.create_group(scene).unwrap();
    // animegroupres.global.record_group(source, id_group);
    actions.anime.create.push(OpsAnimationGroupCreation::ops(scene, id_group));
    // actions.anime.attach.push(OpsAnimationGroupAttach::ops(scene, source, id_group));

    {
        let key_curve0 = pi_atom::Atom::from("test"); 
        let key_curve0 = key_curve0.asset_u64();
        let curve = quaternion_curve();
        
        let asset_curve = if let Some(curve) = anime_assets.quaternion.get(&key_curve0) {
            curve
        } else {
            match anime_assets.quaternion.insert(key_curve0, TypeFrameCurve(curve)) {
                Ok(value) => {
                    value
                },
                Err(_) => {
                    return;
                },
            }
        };

        let animation = anime_contexts.quaternion.ctx.create_animation(0, AssetTypeFrameCurve::from(asset_curve) );
        actions.anime.add_target_anime.push(OpsAddTargetAnimation::ops(id_group.clone(), source, animation));
    }

    let mut param = AnimationGroupParam::default(); param.speed = 0.2;
    actions.anime.action.push(OpsAnimationGroupAction::Start(id_group, param, 0., pi_animation::base::EFillMode::NONE));
    // engine.start_animation_group(source, &key_group, 1.0, ELoopMode::OppositePly(None), 0., 1., 60, AnimationAmountCalc::default());
}

fn quaternion_curve() -> FrameCurve<LocalRotationQuaternion> {
    let fps = 60.;
    let mut curve = FrameCurve::<LocalRotationQuaternion>::curve_cubic_spline(fps as FrameIndex);

    let data: [(f32, [f32; 4], [f32; 4], [f32; 4]); 21] = [
        (0., [0., 0., 0., -1.], [0., -4.6930341720581055, 0., 0.36934909224510193], [0., -4.6930341720581055, 0., 0.36934909224510193]),
        (0.03333333507180214,[0., -0.15643447637557983, 0., -0.9876883625984192],[0., -4.635254859924316, 0., 0.7341518998146057],[0., -4.635254859924316, 0., 0.7341518998146057]),
        (0.06666667014360428,[0., -0.30901700258255005, 0., -0.9510565400123596],[0., -4.463340759277344, 0., 1.4502273797988892],[0., -4.463340759277344, 0., 1.4502273797988892]),
        (0.10000000894069672,[0., -0.45399054884910583, 0., -0.8910065293312073],[0., -4.181523323059082, 0., 2.1305930614471436],[0., -4.181523323059082, 0., 2.1305930614471436]),
        (0.13333334028720856,[0., -0.5877852439880371, 0., -0.80901700258255],[0., -3.796743392944336, 0., 2.7584967613220215],[0., -3.796743392944336, 0., 2.7584967613220215]),
        (0.1666666716337204,[0., -0.7071067690849304, 0., -0.7071067690849304],[0., -3.318476676940918, 0., 3.3184757232666016],[0., -3.318476676940918, 0., 3.3184757232666016]),
        (0.20000000298023224,[0., -0.80901700258255, 0., -0.5877853035926819],[0., -2.7584967613220215, 0., 3.7967441082000732],[0., -2.7584967613220215, 0., 3.7967441082000732]),
        (0.23333333432674408,[0., -0.8910065293312073, 0., -0.45399051904678345],[0., -2.1305928230285645, 0., 4.181524276733398],[0., -2.1305928230285645, 0., 4.181524276733398]),
        (0.2666666805744171,[0., -0.9510565400123596, 0., -0.30901697278022766],[0., -1.4502272605895996, 0., 4.463339805603027],[0., -1.4502272605895996, 0., 4.463339805603027]),
        (0.30000001192092896,[0., -0.9876883625984192, 0., -0.15643449127674103],[0., -0.7341519594192505, 0., 4.635255813598633],[0., -0.7341519594192505, 0., 4.635255813598633]),
        (0.3333333432674408,[0., -1.00000000000000000, 0., 4.371138828673793e-8],[0., 0., 0., 4.6930341720581055],[0., 0., 0., 4.6930341720581055]),
        (0.36666667461395264,[0., -0.9876883625984192, 0., 0.15643444657325745],[0., 0.7341519594192505, 0., 4.63525390625],[0., 0.7341519594192505, 0., 4.63525390625]),
        (0.4000000059604645,[0., -0.9510565400123596, 0., 0.3090169429779053],[0., 1.4502267837524414, 0., 4.463339805603027],[0., 1.4502267837524414, 0., 4.463339805603027]),
        (0.4333333373069763,[0., -0.891006588935852, 0., 0.4539903998374939],[0., 2.1305932998657227, 0., 4.181524276733398],[0., 2.1305932998657227, 0., 4.181524276733398]),
        (0.46666666865348816,[0., -0.80901700258255, 0., 0.5877851843833923],[0., 2.758497476577759, 0., 3.796745777130127],[0., 2.758497476577759, 0., 3.796745777130127]),
        (0.5000000000000000,[0., -0.7071067690849304, 0., 0.7071067690849304],[0., 3.3184757232666016, 0., 3.318477153778076],[0., 3.3184757232666016, 0., 3.318477153778076]),
        (0.5333333611488342,[0., -0.5877851843833923, 0., 0.8090170621871948],[0., 3.7967429161071777, 0., 2.7584948539733887],[0., 3.7967429161071777, 0., 2.7584948539733887]),
        (0.5666667222976685,[0., -0.4539903700351715, 0., 0.891006588935852],[0., 4.181522369384766, 0., 2.1305911540985107],[0., 4.181522369384766, 0., 2.1305911540985107]),
        (0.6000000834465027,[0., -0.30901679396629333, 0., 0.9510565996170044],[0., 4.463342189788818, 0., 1.4502263069152832],[0., 4.463342189788818, 0., 1.4502263069152832]),
        (0.6333334445953369,[0., -0.15643396973609924, 0., 0.987688422203064],[0., 4.635257720947266, 0., 0.7341510057449341],[0., 4.635257720947266, 0., 0.7341510057449341]),
        (0.6666666865348816,[0., 8.742277657347586e-8, 0., 1.000000000000000],[0., 4.693034648895264, 0., 0.36934834718704224],[0., 4.693034648895264, 0., 0.36934834718704224]),
    ];
    data.iter().for_each(|item| {
        curve.curve_cubic_splice_frame(
            (fps * item.0) as u16,
            LocalRotationQuaternion::create(item.1[0], item.1[1], item.1[2], item.1[3]),
            LocalRotationQuaternion::create(item.2[0], item.2[1], item.2[2], item.2[3]),
            LocalRotationQuaternion::create(item.3[0], item.3[1], item.3[2], item.3[3])
        );
    });
    // let result = curve.interple(33.5995255 / 60., &pi_curves::amount::AnimationAmountCalc::from_easing(EEasingMode::None));
    // log::warn!("{:?}", result);
    curve
}


pub type ActionListTestData = ActionList<(ObjectID, f32, f32, f32)>;

pub struct PluginTest;
impl Plugin for PluginTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListTestData::default());
    }
}

pub fn main() {
    let mut app = base::test_plugins();
    
    app.add_plugins(PluginTest);
    
    app.add_systems(Update, pi_3d::sys_info_node);
    app.add_systems(Update, pi_3d::sys_info_resource);
    app.add_systems(Startup, setup.after(base::setup_default_mat));
    // bevy_mod_debugdump::print_main_schedule(&mut app);
    
    // app.run()
    loop { app.update(); }

}