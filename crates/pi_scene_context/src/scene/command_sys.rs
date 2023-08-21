use pi_engine_shell::prelude::*;

use crate::{
    transforms::{prelude::*, command_sys::ActionTransformNode},
    prelude::{SceneMainCameraID, Enable, GlobalEnable}, object::ActionEntity,
};

use super::{prelude::*};

pub fn sys_create_scene(
    mut cmds: ResMut<ActionListSceneCreate>,
    mut commands: Commands,
    mut dynbuffer: ResMut<ResBindBufferAllocator>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneCreation(entity, cfg)| {

        let id_left = commands.spawn_empty().id();
        let id_right = commands.spawn_empty().id();

        if let Some(mut entitycmds) = commands.get_entity(entity) {
            ActionScene::init(&mut entitycmds, entity, cfg, id_left, id_right, &mut dynbuffer);
        } else {
            commands.entity(id_left).despawn();
            commands.entity(id_right).despawn();
            return;
        };

    });
}

pub fn sys_act_scene_time(
    mut cmds: ResMut<ActionListSceneTime>,
    mut scenes: Query<&mut SceneTime>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneTime(entity, val)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            comp.reset(val as u64);
        }
    });
}

pub fn sys_act_scene_ambientcolor(
    mut cmds: ResMut<ActionListSceneAmbientColor>,
    mut scenes: Query<&mut AmbientColor>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneAmbientColor(entity, r, g, b, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = AmbientColor(r, g, b);
        } else if count < 2 {
            cmds.push(OpsSceneAmbientColor(entity, r, g, b, count + 1))
        }
    });
}


pub fn sys_act_scene_ambientintensity(
    mut cmds: ResMut<ActionListSceneAmbientIntensity>,
    mut scenes: Query<&mut AmbientIntensity>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneAmbientIntensity(entity, val, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = AmbientIntensity(val);
        } else if count < 2 {
            cmds.push(OpsSceneAmbientIntensity(entity, val, count + 1))
        }
    });
}

pub fn sys_act_scene_fogcolor(
    mut cmds: ResMut<ActionListSceneFogColor>,
    mut scenes: Query<&mut SceneFogColor>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneFogColor(entity, r, g, b, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = SceneFogColor(r, g, b);
        } else if count < 2 {
            cmds.push(OpsSceneFogColor(entity, r, g, b, count + 1))
        }
    });
}


pub fn sys_act_scene_fogparam(
    mut cmds: ResMut<ActionListSceneFogParam>,
    mut scenes: Query<&mut SceneFogParam>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneFogParam(entity, val, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = SceneFogParam(val);
        } else if count < 2 {
            cmds.push(OpsSceneFogParam(entity, val, count + 1))
        }
    });
}

pub fn sys_act_scene_animation_enable(
    mut cmds: ResMut<ActionListSceneAnimationEnable>,
    mut scenes: Query<&mut SceneAnimationEnable>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneAnimationEnable(entity, val, count)| {
        if let Ok(mut comp) = scenes.get_mut(entity) {
            *comp = SceneAnimationEnable(val);
        } else if count < 2 {
            cmds.push(OpsSceneAnimationEnable(entity, val, count + 1))
        }
    });
}

pub struct ActionScene;
impl ActionScene {
    pub fn init(
        entitycmds: &mut EntityCommands,
        scene: Entity,
        passcfg: ScenePassRenderCfg,
        id_left: Entity,
        id_right: Entity,
        dynbuffer: &mut BindBufferAllocator,
    ) {
        ActionTransformNode::init_for_tree(entitycmds);
        ActionEntity::init(entitycmds);

        entitycmds
            .insert(passcfg)
            .insert(SceneCoordinateSytem3D::default())
            .insert(SceneTime::new())
            .insert(SceneFogColor(1., 1., 1.))
            .insert(SceneFogParam(FogParam::None))
            .insert(AmbientColor(1., 1., 1.))
            .insert(AmbientIntensity(1.))
            .insert(TreeLeftRoot::new(id_left))
            .insert(TreeRightRoot::new(id_right))
            // .insert(AnimationGroups::default())
            .insert(SceneMainCameraID(None))
            .insert(SceneAnimationEnable::default())
            ;
        
        if let Some(bindeffect) = BindSceneEffect::new(dynbuffer) {
            entitycmds.insert(bindeffect);
        }
    }

    pub(crate) fn add_to_scene(
        commands: &mut EntityCommands,
        tree: &mut ActionListTransformNodeParent,
        scene: Entity,
    ) {
        // tree.push(OpsTransformNodeParent::ops(commands.id(), scene));
        commands
            .insert(SceneID(scene));
    }
}
