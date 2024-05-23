mod base;
mod command;
mod command_sys;
mod float;
mod int;
mod mat4;
mod uint;
mod vec2;
mod vec3;
mod vec4;

use std::marker::PhantomData;


use crate::prelude::FrameDataPrepare;

use pi_world::{prelude::App, schedule::Update, world::Entity};
use pi_world::prelude::Plugin;
pub use base::*;
pub use command::*;
pub use command_sys::*;
pub use float::*;
pub use int::*;
use pi_assets::asset::GarbageEmpty;
use pi_bevy_asset::{AssetMgrConfigs, ShareAssetMgr};
use pi_bevy_render_plugin::should_run;
use pi_curves::curve::frame::KeyFrameDataTypeAllocator;
use pi_hash::XHashMap;
pub use uint::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;

use crate::{prelude::ERunStageChap, run_stage::should_run_with_animation};

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EAnimatorableType {
    // Mat4,
    Vec4,
    Vec3,
    Vec2,
    Float,
    Uint,
    Int,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EStageAnimation {
    Create,
    _CreateApply,
    Command,
    Start,
    Pause,
    Dispose,
    Running,
}

pub struct PluginGlobalAnimation;
impl Plugin for PluginGlobalAnimation {
    fn build(&self, app: &mut App) {
        // app.world.insert_single_res(ActionListAnimeGroupAttach::default());
        app.world
            .insert_single_res(ActionListAnimeGroupStartReset::default());
        app.world
            .insert_single_res(ActionListAnimatorableFloat::default());
        app.world
            .insert_single_res(ActionListAnimatorableVec2::default());
        app.world
            .insert_single_res(ActionListAnimatorableVec3::default());
        app.world
            .insert_single_res(ActionListAnimatorableVec4::default());
        app.world
            .insert_single_res(ActionListAnimatorableUint::default());
        app.world
            .insert_single_res(ActionListAnimatorableSint::default());
        app.world
            .insert_single_res(ActionListAddAnimationListen::default());
        app.world
            .insert_single_res(ActionListAddAnimationFrameEvent::default());
        app.world
            .insert_single_res(ActionListAnimeGroupCreate::default());
        app.world
            .insert_single_res(ActionListAnimeGroupDispose::default());
        app.world
            .insert_single_res(ActionListAnimationGroupAction::default());
        app.world
            .insert_single_res(ActionListAddTargetAnime::default());
        app.world
            .insert_single_res(ActionListAnimationWeight::default());

        // app.configure_set(Update, EStageAnimation::Create);
        // app.configure_set(
        //     Update,
        //     EStageAnimation::_CreateApply.after(EStageAnimation::Create),
        // );
        // app.configure_set(
        //     Update,
        //     EStageAnimation::Command.after(EStageAnimation::_CreateApply),
        // );
        // app.configure_set(
        //     Update,
        //     EStageAnimation::Running
        //         .in_set(FrameDataPrepare)
        //         .after(EStageAnimation::Command)
        //         .before(ERunStageChap::Anime),
        // );
        // app.configure_set(
        //     Update,
        //     EStageAnimation::Dispose
        //         .after(EStageAnimation::Running)
        //         .after(ERunStageChap::Dispose),
        // );
        // app.add_system(Update, apply_deferred.in_set(EStageAnimation::_CreateApply));

        app.add_system(
            Update,
            sys_create_animation_group
        );
        app.add_system(
            Update,
            sys_create_animatorable_entity
        );

        app.add_system(
            Update,
            sys_act_reset_while_animationgroup_start
        );
        app.add_system(Update, sys_act_animation_group_action);
        app.add_system(Update,sys_act_dispose_animation_group);
        app.add_system(Update,sys_animation_removed_data_clear);
        app.add_system(Update,sys_reset_anime_performance);

        let globalaboput = GlobalAnimeAbout {
            ty_alloc: KeyFrameDataTypeAllocator::default(),
            runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap::<Entity>::default(),
            dispose_animationgroups: vec![],
            group_records: XHashMap::default(),
        };
        app.world.insert_single_res(globalaboput);
        app.world.insert_single_res(GlobalAnimeEvents::default());
    }
}

pub struct PluginTypeAnime<D: TAnimatableComp, R: TAnimatableCompRecord<D> + 'static>(PhantomData<(D, R)>);
impl<D: TAnimatableComp, R: TAnimatableCompRecord<D>> PluginTypeAnime<D, R> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }
}
impl<D: TAnimatableComp + 'static, R: TAnimatableCompRecord<D>> Plugin for PluginTypeAnime<D, R> {
    fn build(&self, app: &mut App) {
        let ty = app
            .world
            .get_single_res_mut::<GlobalAnimeAbout>()
            .unwrap()
            .ty_alloc
            .alloc()
            .expect("");
        // log::warn!("AnimeType {:?}", ty);

        let cfg = app
            .world
            .get_single_res_mut::<AssetMgrConfigs>()
            .unwrap()
            .query::<D>();
        // 创建 动画曲线 资产表
        app.world
            .insert_single_res(ShareAssetMgr::<TypeFrameCurve<D>>::new(
                GarbageEmpty(),
                cfg.flag,
                cfg.max,
                cfg.timeout,
            ));

        let mut runtime_info_map = &mut app
            .world
            .get_single_res_mut::<GlobalAnimeAbout>()
            .unwrap()
            .runtimeinfos;

        let type_ctx = TypeAnimeContext::<D>::new(ty, &mut runtime_info_map);
        app.world.insert_single_res(type_ctx);

        app.add_system(Update,sys_apply_removed_data::<D>);
        app.add_system(Update,sys_animation_removed_data_clear);
        app.add_system(Update,sys_calc_reset_animatablecomp::<D, R>);
        app.add_system(Update, sys_calc_type_anime::<D>);
    }
}

pub type PluginTypeAnimatorableFloat = PluginTypeAnime<AnimatorableFloat, RecordAnimatorableFloat>;
pub type PluginTypeAnimatorableVec2 = PluginTypeAnime<AnimatorableVec2, RecordAnimatorableVec2>;
pub type PluginTypeAnimatorableVec3 = PluginTypeAnime<AnimatorableVec3, RecordAnimatorableVec3>;
pub type PluginTypeAnimatorableVec4 = PluginTypeAnime<AnimatorableVec4, RecordAnimatorableVec4>;
pub type PluginTypeAnimatorableUint = PluginTypeAnime<AnimatorableUint, RecordAnimatorableUint>;
pub type PluginTypeAnimatorableInt = PluginTypeAnime<AnimatorableSint, RecordAnimatorableInt>;
