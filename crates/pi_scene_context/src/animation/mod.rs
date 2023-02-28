use std::{marker::PhantomData, fmt::Debug};

use pi_animation::{type_animation_context::{TypeAnimationContext}};
use pi_assets::{asset::{GarbageEmpty}, mgr::AssetMgr};

use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator}};
use pi_ecs::prelude::{Component, Setup};
use pi_engine_shell::{object::{ObjectID}, run_stage::{ERunStageChap}, plugin::Plugin};

use self::{base::{GlobalAnimeAbout, TypeFrameCurve, TypeAnimeContext, AssetTypeFrameCurve}, system::{SysSceneAnime, SysTypeAnime, SysTypeAnimeDispose}, command::{SingleControlCommands, SingleModifyCommands, SysAnimeControlCommand, SysAnimeModifyCommand}};

pub mod base;
pub mod command;
pub mod system;
pub mod interface;
pub mod listen;


pub struct PluginTypeAnime<D: FrameDataValue + Component + Debug>(bool, usize, usize, PhantomData<D>);
impl<D: FrameDataValue + Component + Debug> PluginTypeAnime<D> {
    pub fn new(ref_garbage: bool, capacity: usize, timeout: usize) -> Self {
        Self(ref_garbage, capacity, timeout, PhantomData::default())
    }
}
impl<D: FrameDataValue + Component + Debug> Plugin for PluginTypeAnime<D> {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        let ty = if let Some(globalabout) = world.get_resource_mut::<GlobalAnimeAbout>() {
            globalabout.ty_alloc.alloc().expect("")
        } else {
            let mut globalaboput = GlobalAnimeAbout {
                ty_alloc: KeyFrameDataTypeAllocator::default(),
                runtimeinfos: pi_animation::runtime_info::RuntimeInfoMap::<ObjectID>::default(),
                dispose_animations: vec![],
                dispose_animationgroups: vec![],
            };
            let ty = globalaboput.ty_alloc.alloc().expect("");
            world.insert_resource(globalaboput);

            world.insert_resource(SingleControlCommands::default());
            world.insert_resource(SingleModifyCommands::default());

            SysAnimeControlCommand::setup(world, stages.query_stage::<SysAnimeControlCommand>(ERunStageChap::Initial));
            SysAnimeModifyCommand::setup(world, stages.query_stage::<SysAnimeModifyCommand>(ERunStageChap::Initial));

            SysSceneAnime::setup(world, stages.query_stage::<SysSceneAnime>(ERunStageChap::Anime));

            ty
        };
        
        // 创建 动画曲线 资产表
        world.insert_resource(AssetMgr::<TypeFrameCurve<D>>::new(GarbageEmpty(), self.0, self.1, self.2));

        let mut runtime_info_map = &mut world.get_resource_mut::<GlobalAnimeAbout>().unwrap().runtimeinfos;

        let type_ctx = TypeAnimeContext::<D> {
            ctx: TypeAnimationContext::<D, AssetTypeFrameCurve<D>>::new(ty, &mut runtime_info_map),
        };

        world.insert_resource(type_ctx);

        SysTypeAnimeDispose::<D>::setup(world, stages.query_stage::<SysTypeAnimeDispose::<D>>(ERunStageChap::Initial));
        SysTypeAnime::<D>::setup(world, stages.query_stage::<SysTypeAnime::<D>>(ERunStageChap::Anime));
        
        Ok(())
    }
}