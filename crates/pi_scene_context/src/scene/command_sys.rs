use pi_engine_shell::prelude::*;

use crate::{
    transforms::{prelude::*, command_sys::ActionTransformNode},
    prelude::SceneMainCameraID,
};

use super::{prelude::*};

pub fn sys_act_scene_create(
    mut cmds: ResMut<ActionListSceneCreate>,
    mut commands: Commands,
    mut dynbuffer: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneCreation(entity, cfg)| {
        ActionScene::init(&mut commands, entity, cfg);
        ActionTransformNode::init_for_tree(&mut commands.entity(entity));
        if let Some(bindeffect) = BindSceneEffect::new( &mut dynbuffer) {
            commands.entity(entity).insert(bindeffect);
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
            .insert(SceneFog::new())
            .insert(AmbientLight::new())
            .insert(TreeLeftRoot::new(id_left))
            .insert(TreeRightRoot::new(id_right))
            .insert(AnimationGroups::default())
            .insert(SceneMainCameraID(None))
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
            .insert(SceneFog::new())
            .insert(AmbientLight::new())
            .insert(TreeLeftRoot::new(id_left))
            .insert(TreeRightRoot::new(id_right))
            .insert(AnimationGroups::default())
            .insert(SceneMainCameraID(None))
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
