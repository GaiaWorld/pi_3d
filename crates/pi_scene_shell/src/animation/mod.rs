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

use std::marker::PhantomData;

use bevy_app::{App, Plugin, Update};
use bevy_ecs::{schedule::{SystemSet, IntoSystemSetConfig, apply_deferred, IntoSystemConfigs}, entity::Entity};

pub use base::*;
pub use command::*;
pub use command_sys::*;
pub use float::*;
pub use mat4::*;
pub use uint::*;
pub use int::*;
pub use vec2::*;
pub use vec3::*;
pub use vec4::*;
use pi_assets::asset::GarbageEmpty;
use pi_bevy_asset::{ShareAssetMgr, AssetMgrConfigs};
use pi_bevy_render_plugin::should_run;
use pi_curves::curve::frame::KeyFrameDataTypeAllocator;
use pi_hash::XHashMap;

use crate::{prelude::ERunStageChap, run_stage::should_run_with_animation};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
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
        app.insert_resource(ActionListAddAnimationListen::default());
        app.insert_resource(ActionListAddAnimationFrameEvent::default());
        app.insert_resource(ActionListAnimeGroupCreate::default());
        app.insert_resource(ActionListAnimeGroupDispose::default());
        app.insert_resource(ActionListAnimationGroupAction::default());
        app.insert_resource(ActionListAddTargetAnime::default());
        app.insert_resource(ActionListAnimationWeight::default());

        app.configure_set(Update, EStageAnimation::Create);
        app.configure_set(Update, EStageAnimation::_CreateApply.after(EStageAnimation::Create));
        app.configure_set(Update, EStageAnimation::Command.after(EStageAnimation::_CreateApply));
        app.configure_set(Update, EStageAnimation::Running.after(EStageAnimation::Command).before(ERunStageChap::Anime));
        app.configure_set(Update, EStageAnimation::Dispose.after(EStageAnimation::Running).after(ERunStageChap::Dispose));
        app.add_systems(Update, apply_deferred.in_set(EStageAnimation::_CreateApply));
        
        app.add_systems(
			Update,
            (
                sys_create_animation_group,
                sys_create_animatorable_entity
            ).in_set(EStageAnimation::Create)
        );

        app.add_systems(
			Update,
            (
                sys_act_reset_while_animationgroup_start.run_if(should_run),
                // sys_anime_group_create.run_if(should_run),
                sys_act_add_target_animation,
                sys_act_add_animation_group_frame_event,
                sys_act_add_animation_group_listen,
                // sys_anime_start.run_if(should_run),
                // sys_anime_pause.run_if(should_run),
                sys_act_animation_group_action.run_if(should_run),
                sys_act_dispose_animation_group.run_if(should_run),
            ).chain().in_set(EStageAnimation::Command)
        );
        
        app.add_systems(Update, 
            (
                sys_animation_removed_data_clear,
                sys_reset_anime_performance
            ).run_if(should_run).in_set(EStageAnimation::Dispose)
        );

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

        app.add_systems(Update, 
            sys_apply_removed_data::<D>.run_if(should_run).before(sys_animation_removed_data_clear).in_set(EStageAnimation::Dispose)
        );
        app.add_systems(
			Update,
            (
                sys_calc_reset_animatablecomp::<D, R>.run_if(should_run),
                sys_calc_type_anime::<D>.run_if(should_run_with_animation)
            ).chain().in_set(EStageAnimation::Running)
        );
    }
}

pub type PluginTypeAnimatorableFloat = PluginTypeAnime<AnimatorableFloat, RecordAnimatorableFloat>;
pub type PluginTypeAnimatorableVec2 = PluginTypeAnime<AnimatorableVec2, RecordAnimatorableVec2>;
pub type PluginTypeAnimatorableVec3 = PluginTypeAnime<AnimatorableVec3, RecordAnimatorableVec3>;
pub type PluginTypeAnimatorableVec4 = PluginTypeAnime<AnimatorableVec4, RecordAnimatorableVec4>;
pub type PluginTypeAnimatorableUint = PluginTypeAnime<AnimatorableUint, RecordAnimatorableUint>;
pub type PluginTypeAnimatorableInt = PluginTypeAnime<AnimatorableSint, RecordAnimatorableInt>;