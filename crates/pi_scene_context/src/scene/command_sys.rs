use pi_engine_shell::prelude::*;

use crate::{
    transforms::{prelude::*, command_sys::ActionTransformNode},
    prelude::{SceneMainCameraID, Enable, GlobalEnable},
};

use super::{prelude::*};

pub fn sys_act_scene_create(
    mut cmds: ResMut<ActionListSceneCreate>,
    mut commands: Commands,
    mut dynbuffer: ResMut<ResBindBufferAllocator>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneCreation(entity, cfg)| {
        ActionScene::init(&mut commands, entity, cfg);
        ActionTransformNode::init_for_tree(&mut commands.entity(entity));
        if let Some(bindeffect) = BindSceneEffect::new( &mut dynbuffer) {
            commands.entity(entity).insert(bindeffect);
        }
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
    pub fn create(
        app: &mut App,
        passcfg: ScenePassRenderCfg,
    ) -> Entity {
        
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let id_left = commands.spawn_empty().id();
        let id_right = commands.spawn_empty().id();

        let mut entitycmds = commands.spawn_empty();

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
            .insert(Enable(1.))
            .insert(GlobalEnable(true))
        ;

        let entity = entitycmds.id();

        entity
    }
    
    pub fn init(
        commands: &mut Commands,
        scene: Entity,
        passcfg: ScenePassRenderCfg,
    ) {
        let id_left = commands.spawn_empty().id();
        let id_right = commands.spawn_empty().id();

        let mut entitycmds = commands.entity(scene);

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
