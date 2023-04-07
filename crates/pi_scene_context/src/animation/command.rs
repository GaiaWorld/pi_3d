use std::{mem::replace};

use bevy::prelude::Resource;
use pi_animation::{animation::AnimationInfo, loop_mode::ELoopMode, amount::AnimationAmountCalc, animation_listener::{OnStart, OnFrameEvent, OnLoop, OnEnd}};

use pi_atom::Atom;
use pi_curves::curve::{frame::{KeyFrameCurveValue}, FramePerSecond, FrameIndex};
use pi_engine_shell::prelude::*;

use crate::{flags::SceneID};

use super::base::{AnimationGroups, SceneAnimationContext, GlobalAnimeAbout};


#[derive(Debug)]
pub(crate) enum EControlCommand {
    CreateAnimationGroup(ObjectID, Atom),
}
#[derive(Debug, Default, Resource)]
pub(crate) struct SingleControlCommands(pub(crate) Vec<EControlCommand>);

pub(crate) enum EModifyCommand {
    PauseAnimationGroup(ObjectID, Atom),
    StartAnimationGroupPercent(ObjectID, Atom, KeyFrameCurveValue, ELoopMode, KeyFrameCurveValue, KeyFrameCurveValue, FramePerSecond, AnimationAmountCalc),
    AddTargetAnimation(ObjectID, ObjectID, Atom, AnimationInfo),
}
#[derive(Default, Resource)]
pub(crate) struct SingleModifyCommands(pub(crate) Vec<EModifyCommand>);


// pub struct SysAnimeControlCommand;
// impl TSystemStageInfo for SysAnimeControlCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysSceneCreateCommand::key()
//         ]
//     }
// }
// #[setup]
// impl SysAnimeControlCommand {
//     #[system]
    pub(crate) fn sys_anime_control_cmds(
        mut cmds: ResMut<SingleControlCommands>,
        mut sce: Query<&mut SceneAnimationContext>,
        mut obj: Query<(&SceneID, &mut AnimationGroups)>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EControlCommand::CreateAnimationGroup(id_obj, key_group) => {
                    if let Some((id_scene, mut groups)) = obj.get_mut(id_obj) {
                        if groups.map.contains_key(&key_group) == false {
                            if let Some(mut ctx) = sce.get_mut(id_scene.0) {
                                let id_group = ctx.0.create_animation_group();
                                groups.map.insert(key_group, id_group);
                            }
                        }
                    }
                },
            }
        })
    }
// }

// pub struct SysAnimeModifyCommand;
// impl TSystemStageInfo for SysAnimeModifyCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysAnimeControlCommand::key()
//         ]
//     }
// }
// #[setup]
// impl SysAnimeModifyCommand {
//     #[system]
    pub(crate) fn sys_anime_modify_cmds(
        mut cmds: ResMut<SingleModifyCommands>,
        mut sce: Query<&mut SceneAnimationContext>,
        mut obj: Query<(&SceneID, &mut AnimationGroups)>,
        mut globalinfo: ResMut<GlobalAnimeAbout>,
    ) {
        let mut list = replace(&mut cmds.0, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                EModifyCommand::PauseAnimationGroup(id_obj, key_group) => {
                    if let Some((id_scene, mut groups)) = obj.get_mut(id_obj) {
                        if let Some(mut ctx) = sce.get_mut(id_scene.0) {
                            if let Some(id_group) = groups.map.get(&key_group) {
                                ctx.0.pause(id_group.clone());
                            }
                        }
                    }
                },
                EModifyCommand::StartAnimationGroupPercent(id_obj, key_group, speed, loop_mode, from, to, frames_per_second, amountcalc) => {
                    if let Some((id_scene, mut groups)) = obj.get_mut(id_obj) {
                        if let Some(mut ctx) = sce.get_mut(id_scene.0) {
                            if let Some(id_group) = groups.map.get(&key_group) {
                                ctx.0.start_with_progress(id_group.clone(), speed, loop_mode, from, to, frames_per_second, amountcalc);
                            }
                        }
                    }
                },
                EModifyCommand::AddTargetAnimation(id_obj, id_target, key_group, animation) => {
                    if let Some((id_scene, mut groups)) = obj.get_mut(id_obj) {
                        if let Some(id_group) = groups.map.get(&key_group) {
                            if let Some(mut ctx) = sce.get_mut(id_scene.0) {
                                ctx.0.add_target_animation(animation, id_group.clone(), id_target);
                            }
                        }
                    }
                },
            }
        })
    }
// }

enum EEventCommand {
    AddAnimationGroupFrameEvent(ObjectID, Atom, FrameIndex, Atom),
    ListenAnimationGroupStart(ObjectID, Atom, OnStart),
    ListenAnimationGroupFrame(ObjectID, Atom, OnFrameEvent<Atom>),
    ListenAnimationGroupLoop(ObjectID, Atom, OnLoop),
    ListenAnimationGroupEnd(ObjectID, Atom, OnEnd),
}