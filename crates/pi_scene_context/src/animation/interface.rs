use std::{sync::Arc, marker::PhantomData};

use pi_animation::{type_animation_context::{TypeAnimationContext, AnimationContextAmount}, animation_result_pool::TypeAnimationResultPoolDefault, animation_group_manager::AnimationGroupManagerDefault, animation::AnimationInfo, target_animation::TargetAnimation, animation_group::AnimationGroupID, amount::AnimationAmountCalc, loop_mode::ELoopMode, animation_listener::{OnStart, OnFrameEvent, OnLoop, OnEnd}};
use pi_assets::{asset::{Handle, GarbageEmpty}, mgr::AssetMgr};
use pi_atom::Atom;
use pi_curves::curve::{frame::{FrameDataValue, KeyFrameDataTypeAllocator, KeyFrameCurveValue}, frame_curve::FrameCurve, FramePerSecond};
use pi_ecs::prelude::{Query, ResMut, Component, Commands, Setup};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::{TSystemStageInfo, ERunStageChap}, plugin::Plugin, setup};
use pi_share::Share;

use crate::scene::scene_time::SceneTime;

use super::{base::{AssetTypeFrameCurve, TypeFrameCurve, TypeAnimeContext}, command::{SingleControlCommands, EControlCommand, SingleModifyCommands, EModifyCommand}};

pub trait InterfaceAnimeAsset {
    fn check_anim_curve<D: FrameDataValue + Component>(
        &self,
        key_curve: &Atom,
    ) -> Option<AssetTypeFrameCurve<D>>;

    fn creat_anim_curve<D: FrameDataValue + Component>(
        &self,
        key_curve: &Atom,
        curve: FrameCurve<D>,
    ) -> AssetTypeFrameCurve<D>;

    fn create_animation<D: FrameDataValue + Component>(
        &self,
        curve: AssetTypeFrameCurve<D>,
    ) -> AnimationInfo;
}

pub trait InterfaceAnimationGroup {
    fn create_animation_group(
        &self,
        id_obj: ObjectID,
        key_animegroup: &Atom,
    ) -> &Self;

    fn create_target_animation(
        &self,
        id_obj: ObjectID,
        id_target: ObjectID,
        key_animegroup: &Atom,
        animation: AnimationInfo,
    ) -> &Self;

    fn start_animation_group(
        &self,
        id_obj: ObjectID,
        key_animegroup: &Atom,
        speed: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        from: KeyFrameCurveValue,
        to: KeyFrameCurveValue,
        frame_per_second: FramePerSecond,
        amountcalc: AnimationAmountCalc,
    ) -> &Self;
    
    fn pause_animation_group(
        &self,
        id_obj: ObjectID,
        key_animegroup: &Atom,
    ) -> &Self;
    
    fn listen_animation_group_start(
        &self,
        id_obj: ObjectID,
        key: &Atom,
        call: OnStart,
    ) -> &Self;
    
    fn listen_animation_group_frame(
        &self,
        id_obj: ObjectID,
        key: &Atom,
        call: OnFrameEvent<Atom>,
    ) -> &Self;
    
    fn listen_animation_group_loop(
        &self,
        id_obj: ObjectID,
        key: &Atom,
        call: OnLoop,
    ) -> &Self;
    
    fn listen_animation_group_end(
        &self,
        id_obj: ObjectID,
        key: &Atom,
        call: OnEnd,
    ) -> &Self;
}

impl InterfaceAnimeAsset for crate::engine::Engine {
    fn check_anim_curve<D: FrameDataValue + Component>(
        &self,
        key: &Atom,
    ) -> Option<AssetTypeFrameCurve<D>> {
        let world = self.world();

        if let Some(value) = world.get_resource::<Share<AssetMgr<TypeFrameCurve<D>>>>().unwrap().get(key) {
            Some(
                AssetTypeFrameCurve::<D>::from(value)
            )
        } else {
            None
        }
    }

    fn creat_anim_curve<D: FrameDataValue + Component>(
        &self,
        key: &Atom,
        curve: FrameCurve<D>,
    ) -> AssetTypeFrameCurve<D> {
        let world = self.world();

        let value = world.get_resource_mut::<Share<AssetMgr<TypeFrameCurve<D>>>>().unwrap().insert(key.clone(), TypeFrameCurve(curve)).unwrap();

        AssetTypeFrameCurve::<D>::from(value)
    }

    fn create_animation<D: FrameDataValue + Component>(
        &self,
        curve: AssetTypeFrameCurve<D>,
    ) -> AnimationInfo {
        let world = self.world();

        let type_ctx = world.get_resource_mut::<TypeAnimeContext<D>>().unwrap();

        type_ctx.ctx.create_animation(0, curve)
    }
}

impl InterfaceAnimationGroup for crate::engine::Engine {
    fn create_animation_group(
        &self,
        id_obj: ObjectID,
        key_animegroup: &Atom,
    ) -> &Self {
        let world = self.world();

        let cmds = world.get_resource_mut::<SingleControlCommands>().unwrap();

        cmds.0.push(EControlCommand::CreateAnimationGroup(id_obj, key_animegroup.clone()));

        self
    }

    fn create_target_animation(
        &self,
        id_obj: ObjectID,
        id_target: ObjectID,
        key_animegroup: &Atom,
        animation: AnimationInfo,
    ) -> &Self {
        let world = self.world();

        let cmds = world.get_resource_mut::<SingleModifyCommands>().unwrap();

        cmds.0.push(EModifyCommand::AddTargetAnimation(id_obj, id_target, key_animegroup.clone(), animation));

        self
    }

    fn start_animation_group(
        &self,
        id_obj: ObjectID,
        key_animegroup: &Atom,
        speed: KeyFrameCurveValue,
        loop_mode: ELoopMode,
        from: KeyFrameCurveValue,
        to: KeyFrameCurveValue,
        frame_per_second: FramePerSecond,
        amountcalc: AnimationAmountCalc,
    ) -> &Self {
        let world = self.world();

        let cmds = world.get_resource_mut::<SingleModifyCommands>().unwrap();

        cmds.0.push(EModifyCommand::StartAnimationGroupPercent(id_obj, key_animegroup.clone(), speed, loop_mode, from, to, frame_per_second, amountcalc));

        self
    }
    
    fn pause_animation_group(
        &self,
        id_obj: ObjectID,
        key_animegroup: &Atom,
    ) -> &Self {
        let world = self.world();

        let cmds = world.get_resource_mut::<SingleModifyCommands>().unwrap();

        cmds.0.push(EModifyCommand::PauseAnimationGroup(id_obj, key_animegroup.clone()));

        self
    }

    fn listen_animation_group_start(
        &self,
        id_obj: ObjectID,
        key: &Atom,
        call: OnStart,
    ) -> &Self {
        todo!()
    }

    fn listen_animation_group_frame(
        &self,
        id_obj: ObjectID,
        key: &Atom,
        call: OnFrameEvent<Atom>,
    ) -> &Self {
        todo!()
    }

    fn listen_animation_group_loop(
        &self,
        id_obj: ObjectID,
        key: &Atom,
        call: OnLoop,
    ) -> &Self {
        todo!()
    }

    fn listen_animation_group_end(
        &self,
        id_obj: ObjectID,
        key: &Atom,
        call: OnEnd,
    ) -> &Self {
        todo!()
    }

}