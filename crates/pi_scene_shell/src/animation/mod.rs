mod base;
mod command;
mod command_sys;
mod float;
mod vec2;
mod vec3;
mod vec4;
mod uint;
mod int;
mod mat4;

use crate::{ecs::*, object::{sys_dispose, sys_dispose_can}, prelude::runif_3d, run_stage::StageD3};
// use bevy_app::{App, Plugin, Update};
// use bevy_ecs::{schedule::{SystemSet, IntoSystemSetConfig, apply_deferred, IntoSystemConfigs}, entity::Entity};

use std::marker::PhantomData;

use crate::prelude::FrameDataPrepare;

pub use base::*;
pub use command::*;
pub use command_sys::*;
pub use float::*;
pub use uint::*;
pub use int::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
use pi_assets::asset::GarbageEmpty;
use pi_bevy_asset::{ShareAssetMgr, AssetMgrConfigs};
use pi_curves::curve::frame::KeyFrameDataTypeAllocator;
use pi_hash::XHashMap;

use crate::prelude::ERunStageChap;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
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
        // app.insert_resource(ActionListAnimeGroupAttach::default());
        app.insert_resource(ActionListAnimeGroupStartReset::default());
        app.insert_resource(ActionListAnimatorableFloat::default());
        app.insert_resource(ActionListAnimatorableVec2::default());
        app.insert_resource(ActionListAnimatorableVec3::default());
        app.insert_resource(ActionListAnimatorableVec4::default());
        app.insert_resource(ActionListAnimatorableUint::default());
        app.insert_resource(ActionListAnimatorableSint::default());
        app.insert_resource(ActionListAnimeGroupCreate::default());
        app.insert_resource(ActionListAnimeGroupDispose::default());
        app.insert_resource(ActionListAnimationGroupAction::default());

        app.configure_set(StageD3, EStageAnimation::Create       .in_set(ERunStageChap::Modify));
        app.configure_set(StageD3, EStageAnimation::_CreateApply .in_set(ERunStageChap::Modify).after(EStageAnimation::Create));
        app.configure_set(StageD3, EStageAnimation::Command      .in_set(ERunStageChap::Modify).after(EStageAnimation::_CreateApply));
        app.configure_set(StageD3, EStageAnimation::Running      .in_set(ERunStageChap::Modify).in_set(FrameDataPrepare).after(EStageAnimation::Command));
        app.configure_set(StageD3, EStageAnimation::Dispose      .in_set(ERunStageChap::Dispose));
        
#[cfg(feature="use_bevy")]
{
    app.add_systems(StageD3, apply_deferred.in_set(EStageAnimation::_CreateApply));
    app.add_systems(
        Update,
        (
            sys_create_animation_group,
            sys_create_animatorable_entity
        ).in_set(EStageAnimation::Create),
        (
            sys_act_reset_while_animationgroup_start    , // .run_if(should_run),
            sys_act_animation_group_action              , // .run_if(should_run),
            sys_act_dispose_animation_group             , // .run_if(should_run),
        ).chain().in_set(EStageAnimation::Command),
        (
            sys_animation_removed_data_clear,
            sys_reset_anime_performance
        ).in_set(EStageAnimation::Dispose)
    );
}
#[cfg(not(feature = "use_bevy"))]
{
    app
        .add_systems(StageD3, sys_create_animation_group                              .in_set(EStageAnimation::Create))
        .add_systems(StageD3, sys_create_animatorable_entity                          .in_set(EStageAnimation::Create))
        .add_systems(StageD3, sys_act_reset_while_animationgroup_start                                                            .in_set(EStageAnimation::Command))
        .add_systems(StageD3, sys_act_animation_group_action          .after(sys_act_reset_while_animationgroup_start)    .in_set(EStageAnimation::Command))
        .add_systems(StageD3, sys_act_dispose_animation_group         .after(sys_act_animation_group_action)              .in_set(EStageAnimation::Command))
        .add_systems(StageD3, sys_animation_removed_data_clear                                                            .in_set(EStageAnimation::Running))
        .add_systems(StageD3, sys_reset_anime_performance             .after(sys_animation_removed_data_clear)    .in_set(EStageAnimation::Running))
        ;
}

        let globalaboput = GlobalAnimeAbout {
            ty_alloc: KeyFrameDataTypeAllocator::default(),
            runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap::<Entity>::default(),
            dispose_animationgroups: vec![],
            group_records: XHashMap::default(),
        };
        app.insert_resource(globalaboput);
        app.insert_resource(GlobalAnimeEvents::default());
    }
}

pub struct PluginTypeAnime<D: TAnimatableComp>(PhantomData<D>);
impl<D: TAnimatableComp> PluginTypeAnime<D> {
    pub fn new() -> Self {
        Self(PhantomData::default())
    }
}
impl<D: TAnimatableComp> Plugin for PluginTypeAnime<D> {

    fn build(&self, app: &mut App) {
        let ty = app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().ty_alloc.alloc().expect("");
        // log::warn!("AnimeType {:?}", ty);

        app.insert_resource(AnimeTargetRecordValues::<D>::default());

        let cfg = app.world.get_resource_mut::<AssetMgrConfigs>().unwrap().query::<D>();
        // 创建 动画曲线 资产表
        app.world.insert_resource(ShareAssetMgr::<TypeFrameCurve<D>>::new(GarbageEmpty(), cfg.flag, cfg.max, cfg.timeout));

        let mut runtime_info_map = &mut app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().runtimeinfos;

        let type_ctx = TypeAnimeContext::<D>::new(ty, &mut runtime_info_map);
        app.insert_resource(type_ctx);

#[cfg(feature="use_bevy")]
{
    app.add_systems(StageD3, 
        (
            sys_apply_removed_data::<D>     // .run_if(should_run)
            .before(sys_animation_removed_data_clear)
        ).in_set(EStageAnimation::Dispose)
    );
    app.add_systems(
        Update,
        (
            sys_calc_type_anime::<D>                , // .run_if(should_run_with_animation)
        ).chain().in_set(EStageAnimation::Running)
    );
}
#[cfg(not(feature = "use_bevy"))]
{
    
    app
        .add_systems(StageD3, sys_apply_removed_data::<D>     .before(sys_animation_removed_data_clear)    .in_set(EStageAnimation::Running))
        .add_systems(StageD3, sys_calc_type_anime::<D>       .before(sys_apply_removed_data::<D>).in_set(EStageAnimation::Running))
        .add_systems(StageD3, sys_remove_anime_target_record::<D>       .before(sys_dispose).after(sys_dispose_can).in_set(ERunStageChap::_Dispose))
        ;
}
    }
}

pub type PluginTypeAnimatorableFloat = PluginTypeAnime<AnimatorableFloat>;
pub type PluginTypeAnimatorableVec2 = PluginTypeAnime<AnimatorableVec2>;
pub type PluginTypeAnimatorableVec3 = PluginTypeAnime<AnimatorableVec3>;
pub type PluginTypeAnimatorableVec4 = PluginTypeAnime<AnimatorableVec4>;
pub type PluginTypeAnimatorableUint = PluginTypeAnime<AnimatorableUint>;
pub type PluginTypeAnimatorableInt  = PluginTypeAnime<AnimatorableSint>;