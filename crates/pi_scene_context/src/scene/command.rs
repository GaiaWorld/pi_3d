
use pi_engine_shell::prelude::*;


use crate::{
    flags::SceneID,
    scene::environment::fog::SceneFog,
    animation::base::{SceneAnimationContext, AnimationGroups},
    transforms::{tree_left_right::{TreeLeftRoot, TreeRightRoot}, command::ActionTransformNode}, prelude::{ActionListTransformNodeParent, OpsTransformNodeParent},
};

use super::{
    coordinate_system::SceneCoordinateSytem3D,
    environment::{ambient_light::AmbientLight, BindSceneEffect, scene_time::SceneTime, }
};

pub type ActionListSceneCreate = ActionList<Entity>;
pub fn sys_act_scene_create(
    mut cmds: ResMut<ActionListSceneCreate>,
    mut commands: Commands,
    mut dynbuffer: ResMut<ResBindBufferAllocator>,
    device: Res<PiRenderDevice>,
) {
    cmds.drain().drain(..).for_each(|entity| {
        ActionScene::init(&mut commands, entity);
        ActionTransformNode::init_for_tree(&mut commands.entity(entity));
        if let Some(bindeffect) = BindSceneEffect::new(&device, &mut dynbuffer) {
            commands.entity(entity).insert(bindeffect);
        }
    });
}

pub struct ActionScene;
impl ActionScene {
    pub fn create(
        app: &mut App,
    ) -> Entity {
        
        let mut queue = CommandQueue::default();
        let mut commands = Commands::new(&mut queue, &app.world);

        let id_left = commands.spawn_empty().id();
        let id_right = commands.spawn_empty().id();

        let mut entitycmds = commands.spawn_empty();

        entitycmds
            .insert(SceneCoordinateSytem3D::default())
            .insert(SceneTime::new())
            .insert(SceneFog::new())
            .insert(AmbientLight::new())
            .insert(SceneAnimationContext::new())
            .insert(TreeLeftRoot::new(id_left))
            .insert(TreeRightRoot::new(id_right))
            .insert(AnimationGroups::default());

        let entity = entitycmds.id();

        entity
    }
    
    pub fn init(
        commands: &mut Commands,
        scene: Entity,
    ) {
        let id_left = commands.spawn_empty().id();
        let id_right = commands.spawn_empty().id();

        let mut entitycmds = commands.entity(scene);

        entitycmds
            .insert(SceneCoordinateSytem3D::default())
            .insert(SceneTime::new())
            .insert(SceneFog::new())
            .insert(AmbientLight::new())
            .insert(SceneAnimationContext::new())
            .insert(TreeLeftRoot::new(id_left))
            .insert(TreeRightRoot::new(id_right))
            .insert(AnimationGroups::default());
    }
    
    pub(crate) fn add_to_scene(
        commands: &mut EntityCommands,
        tree: &mut ActionListTransformNodeParent,
        scene: Entity,
    ) {
        tree.push(OpsTransformNodeParent::ops(commands.id(), scene));
        commands
            .insert(SceneID(scene));
    }
}
