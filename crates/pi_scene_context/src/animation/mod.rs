use std::{marker::PhantomData, fmt::Debug};

use pi_animation::{type_animation_context::{TypeAnimationContext}};
use pi_assets::{asset::{GarbageEmpty}, mgr::AssetMgr};
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator}};

use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;

use self::{
    base::*,
    system::*,
    command_sys::*,
    command::*,
};

pub mod base;
pub mod command;
pub mod command_sys;
pub mod system;
pub mod interface;
pub mod listen;
pub mod prelude;

pub struct PluginAnimation;
impl Plugin for PluginAnimation {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListAnimeGroupCreate::default());
        app.insert_resource(ActionListAnimeGroupPause::default());
        app.insert_resource(ActionListAnimeGroupStart::default());
        app.insert_resource(ActionListAddTargetAnime::default());

        app.add_systems(
            (
                sys_anime_group_create,
                sys_anime_add_target_anime,
                sys_anime_start,
                sys_anime_pause,
            ).chain().in_set(ERunStageChap::Command)
        );
    }
}

pub struct PluginTypeAnime<D: FrameDataValue + Component + Debug>(bool, usize, usize, PhantomData<D>);
impl<D: FrameDataValue + Component + Debug> PluginTypeAnime<D> {
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(ref_garbage, capacity, timeout, PhantomData::default())
    }
}
impl<D: FrameDataValue + Component + Debug> Plugin for PluginTypeAnime<D> {

    fn build(&self, app: &mut bevy::prelude::App) {
        
        let ty = if let Some(mut globalabout) = app.world.get_resource_mut::<GlobalAnimeAbout>() {
            globalabout.ty_alloc.alloc().expect("")
        } else {
            let mut globalaboput = GlobalAnimeAbout {
                ty_alloc: KeyFrameDataTypeAllocator::default(),
                runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap::<ObjectID>::default(),
                dispose_animationgroups: vec![],
                group_records: XHashMap::default(),
            };
            let ty = globalaboput.ty_alloc.alloc().expect("");
            app.world.insert_resource(globalaboput);
            app.world.insert_resource(SceneAnimationContextMap::default());
            app.world.insert_resource(GlobalAnimeEvents::default());

            app.add_system(sys_scene_anime_ctx.in_set(ERunStageChap::AnimeAmount));

            ty
        };
        
        // 创建 动画曲线 资产表
        app.world.insert_resource(ShareAssetMgr::<TypeFrameCurve<D>>::new(GarbageEmpty(), self.0, self.1, self.2));

        let mut runtime_info_map = &mut app.world.get_resource_mut::<GlobalAnimeAbout>().unwrap().runtimeinfos;

        let type_ctx = TypeAnimeContext::<D> {
            ctx: TypeAnimationContext::<D, AssetTypeFrameCurve<D>>::new(ty, &mut runtime_info_map),
        };

        app.world.insert_resource(type_ctx);

        app.add_system(
            sys_listen_type_anime_ctx::<D>.in_set(ERunStageChap::Command)
        );
        app.add_system(sys_calc_type_anime::<D>.in_set(ERunStageChap::Anime));

        // SysTypeAnimeDispose::<D>::setup(world, stages.query_stage::<SysTypeAnimeDispose::<D>>(ERunStageChap::Initial));
        // SysTypeAnime::<D>::setup(world, stages.query_stage::<SysTypeAnime::<D>>(ERunStageChap::Anime));
    }
}
