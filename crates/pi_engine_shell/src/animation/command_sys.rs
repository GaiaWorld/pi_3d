

use bevy::{
    app::{ prelude::*, PluginGroupBuilder }, core::prelude::*, ecs::prelude::*, hierarchy::prelude::*, input::{prelude::*, InputPlugin},
    log::prelude::*, math::prelude::*, reflect::prelude::*, time::prelude::*,
    utils::prelude::*, window::{prelude::*},
    ecs::system::{CommandQueue, EntityCommands, SystemState, SystemParam}, prelude::{Deref, DerefMut},
    a11y::*,
    // winit::*,
};
use pi_animation::animation::AnimationInfo;
use pi_atom::Atom;
use pi_bevy_asset::ShareAssetMgr;
use pi_curves::curve::{frame::FrameDataValue, frame_curve::FrameCurve};
use pi_slotmap::DefaultKey;
use core::fmt::Debug;


use super::base::*;
use super::command::*;

pub fn sys_anime_group_create(
    mut cmds: ResMut<ActionListAnimeGroupCreate>,
    mut obj: Query<&mut AnimationGroups>,
) {
    // let mut list = 
    cmds.drain().drain(..).for_each(|OpsAnimationGroupCreation(id_obj, key_group, id_group)| {
        if let Ok(mut groups) = obj.get_mut(id_obj) {
            if groups.map.contains_key(&key_group) == false {
                groups.map.insert(key_group, id_group);
            }
        } else {
            cmds.push(OpsAnimationGroupCreation(id_obj, key_group, id_group))
        }
    });
}

pub fn sys_anime_pause(
    mut cmds: ResMut<ActionListAnimeGroupPause>,
    obj: Query<(&SceneID, &AnimationGroups)>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupPause(id_obj, key_group)| {
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
                if let Some(id_group) = groups.map.get(&key_group) {
                    ctx.0.pause(id_group.clone());
                }
            }
        } else {
            cmds.push(OpsAnimationGroupPause(id_obj, key_group))
        }
    });
}


pub fn sys_anime_start(
    mut cmds: ResMut<ActionListAnimeGroupStart>,
    obj: Query<(&SceneID, & AnimationGroups)>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
) {
    cmds.drain().drain(..).for_each(|OpsAnimationGroupStart(id_obj, key_group, param)| {
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
                if let Some(id_group) = groups.map.get(&key_group) {
                    ctx.0.start_with_progress(id_group.clone(), param.speed, param.loop_mode, param.from, param.to, param.fps, param.amountcalc);
                }
            }
        } else {
            cmds.push(OpsAnimationGroupStart(id_obj, key_group, param))
        }
    });
}

pub fn sys_anime_add_target_anime(
    mut cmds: ResMut<ActionListAddTargetAnime>,
    obj: Query<(&SceneID, & AnimationGroups)>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
) {
    // log::warn!("AddTargetAnime");
    cmds.drain().drain(..).for_each(|OpsAddTargetAnimation(id_obj, id_target, key_group, animation)| {
        // log::warn!("AddTargetAnime 0");
        if let Ok((id_scene, groups)) = obj.get(id_obj) {
            // log::warn!("AddTargetAnime 1");
            if let Some(id_group) = groups.map.get(&key_group) {
                // log::warn!("AddTargetAnime 2");
                if let Some(ctx) = scenectxs.get_mut(&id_scene.0) {
                    // log::warn!("AddTargetAnime Ok");
                    ctx.0.add_target_animation_notype(animation, id_group.clone(), id_target);
                }
            }
        } else {
            cmds.push(OpsAddTargetAnimation(id_obj, id_target, key_group, animation))
        }
    });
}

pub struct ActionAnime;
impl ActionAnime {
    pub fn as_anime_group_target(
        commands: &mut EntityCommands,
    ) {
        commands.insert(AnimationGroups::default());
    }
    pub fn create_animation_group(
        app: &mut App,
        id_scene: Entity,
        id_obj: Entity,
        key_group: &Atom,
        id_group: DefaultKey,
    ) {
        if let Some(id_group) = app.world.get_resource_mut::<SceneAnimationContextMap>().unwrap().create_group(id_scene) {
            app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().record_group(id_obj, key_group, id_group);
            let mut cmds = app.world.get_resource_mut::<ActionListAnimeGroupCreate>().unwrap();
            cmds.push(OpsAnimationGroupCreation(id_obj, key_group.clone(), id_group));
        }

    }
    pub fn pause_animation_group(
        app: &mut App,
        id_obj: Entity,
        key_group: &Atom,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListAnimeGroupPause>().unwrap();
        cmds.push(OpsAnimationGroupPause(id_obj, key_group.clone()));
    }
    pub fn start_animation_group_percent(
        app: &mut App,
        id_obj: Entity,
        key_group: Atom,
        param: AnimationGroupParam,
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListAnimeGroupStart>().unwrap();
        cmds.push(OpsAnimationGroupStart(id_obj, key_group.clone(), param));
    }
    pub fn add_target_animation(
        app: &mut App,
        group_where: Entity,
        id_target: Entity,
        key_group: Atom,
        animation: AnimationInfo
    ) {
        let mut cmds = app.world.get_resource_mut::<ActionListAddTargetAnime>().unwrap();
        cmds.push(OpsAddTargetAnimation(group_where, id_target, key_group, animation));
    }

    pub fn create_animation<D: FrameDataValue + Component + Debug>(
        app: &mut App,
        curve: AssetTypeFrameCurve<D>,
    ) -> AnimationInfo {
        let mut type_ctx = app.world.get_resource_mut::<TypeAnimeContext<D>>().unwrap();
        type_ctx.ctx.create_animation(0, curve)
    }
    
    pub fn check_anim_curve<D: FrameDataValue + Component + Debug>(
        app: &mut App,
        key: &Atom,
    ) -> Option<AssetTypeFrameCurve<D>> {
        if let Some(value) = app.world.get_resource::<ShareAssetMgr<TypeFrameCurve<D>>>().unwrap().get(key) {
            Some(
                AssetTypeFrameCurve::<D>::from(value)
            )
        } else {
            None
        }
    }

    pub fn creat_anim_curve<D: FrameDataValue + Component + Debug>(
        app: &mut App,
        key: &Atom,
        curve: FrameCurve<D>,
    ) -> Result<AssetTypeFrameCurve<D>, TypeFrameCurve<D>> {
        match app.world.get_resource_mut::<ShareAssetMgr<TypeFrameCurve<D>>>().unwrap().insert(key.clone(), TypeFrameCurve(curve)) {
            Ok(value) => {
                Ok(AssetTypeFrameCurve::<D>::from(value))
            },
            Err(value) => Err(value),
        }

    }

}


pub fn sys_calc_type_anime<D: FrameDataValue + Component + Debug>(
    type_ctx: Res<TypeAnimeContext<D>>,
    runinfos: Res<GlobalAnimeAbout>,
    mut items: Query<&mut D>,
) {
    let time0 = pi_time::Instant::now();

    let ty = type_ctx.ctx.ty();
    // log::warn!("Anime Run ");
    let curves = type_ctx.ctx.curves();
    if let Some(list) = runinfos.runtimeinfos.list.get(ty) {
        // log::warn!("Anime Run 1");
            // log::warn!("Anime Run {:?}", list);
        for info in list {
            // log::warn!("Anime Run 1.5  {:?}", info.target);
            if let Ok(mut item) = items.get_mut(info.target) {
                // log::warn!("Anime Run 2");
                if let Some(Some(curve)) = curves.get(info.curve_id) {
                    // println!(">>>>>>>>>>>>>>>>>{}", info.amount_in_second);
                    let value = curve.as_ref().interple(info.amount_in_second);
                    // log::warn!("Anime Result {:?}", value);
                    // commands.entity(info.target).insert(value);
                    *item = value;
                }
            }
        }
    } else {
        log::trace!("Not Found Anime Type: {}", ty);
    }

    let time1 = pi_time::Instant::now();
    log::debug!("sys_calc_type_anime : {:?}", time1 - time0);
}

pub(crate) fn sys_apply_removed_data<D: FrameDataValue + Component + Debug>(
    mut type_ctx: ResMut<TypeAnimeContext<D>>,
    scenes: Res<SceneAnimationContextMap>,
) {
    scenes.apply_removed_animations(&mut type_ctx.ctx);
}
