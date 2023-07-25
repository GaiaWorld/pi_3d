mod base;
mod command;
mod command_sys;
use core::fmt::Debug;
use std::marker::PhantomData;

use bevy::prelude::{App, Plugin, IntoSystemConfigs, Entity, IntoSystemConfig, Component, Resource};

pub use base::*;
pub use command::*;
pub use command_sys::*;
use pi_animation::type_animation_context::TypeAnimationContext;
use pi_assets::asset::GarbageEmpty;
use pi_bevy_asset::{ShareAssetMgr, AssetCapacity, AssetMgrConfigs};
use pi_bevy_render_plugin::should_run;
use pi_curves::curve::frame::{KeyFrameDataTypeAllocator, FrameDataValue};
use pi_hash::XHashMap;

use crate::{prelude::ERunStageChap, engine_shell::asset_capacity};

pub struct PluginGlobalAnimation;
impl Plugin for PluginGlobalAnimation {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListAnimeGroupAttach::default());
        app.insert_resource(ActionListAnimeGroupStartReset::default());
        // app.insert_resource(ActionListAnimeGroupCreate::default());
        // app.insert_resource(ActionListAnimeGroupPause::default());
        // app.insert_resource(ActionListAnimeGroupStart::default());
        // app.insert_resource(ActionListAddTargetAnime::default());

        app.add_systems(
            (
                sys_anime_group_attach.run_if(should_run),
                sys_calc_reset_while_animationgroup_start.run_if(should_run),
                // sys_anime_group_create.run_if(should_run),
                // sys_anime_add_target_anime.run_if(should_run),
                // sys_anime_start.run_if(should_run),
                // sys_anime_pause.run_if(should_run),
            ).chain().in_set(ERunStageChap::Command)
        );
        
        app.add_system(
            sys_animation_removed_data_clear.run_if(should_run).in_set(ERunStageChap::Initial)
        );

        let globalaboput = GlobalAnimeAbout {
            ty_alloc: KeyFrameDataTypeAllocator::default(),
            runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap::<Entity>::default(),
            dispose_animationgroups: vec![],
            group_records: XHashMap::default(),
        };
        app.insert_resource(globalaboput);
        app.insert_resource(SceneAnimationContextMap::default());
        app.insert_resource(GlobalAnimeEvents::default());
    }
}

pub struct PluginTypeAnime<D: TAnimatableComp, R: TAnimatableCompRecord<D>>(PhantomData<(D, R)>);
impl<D: TAnimatableComp, R: TAnimatableCompRecord<D>> PluginTypeAnime<D, R> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }
}
impl<D: TAnimatableComp, R: TAnimatableCompRecord<D>> Plugin for PluginTypeAnime<D, R> {

    fn build(&self, app: &mut App) {
        let ty = app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().ty_alloc.alloc().expect("");
        // log::warn!("AnimeType {:?}", ty);

        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<D>();
        // 创建 动画曲线 资产表
        app.world.insert_resource(ShareAssetMgr::<TypeFrameCurve<D>>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));

        let mut runtime_info_map = &mut app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().runtimeinfos;

        let type_ctx = TypeAnimeContext::<D>::new(ty, &mut runtime_info_map);
        app.insert_resource(type_ctx);

        app.add_system(
            sys_apply_removed_data::<D>.run_if(should_run).before(sys_animation_removed_data_clear)
        );
        app.add_systems(
            (
                sys_calc_reset_animatablecomp::<D, R>.run_if(should_run),
                sys_calc_type_anime::<D>.run_if(should_run)
            ).chain().in_set(ERunStageChap::Anime)
        );
        
        // app.add_system(sys_calc_type_anime::<D>.in_set(ERunStageChap::Anime));

        // SysTypeAnimeDispose::<D>::setup(world, stages.query_stage::<SysTypeAnimeDispose::<D>>(ERunStageChap::Initial));
        // SysTypeAnime::<D>::setup(world, stages.query_stage::<SysTypeAnime::<D>>(ERunStageChap::Anime));
    }
}
