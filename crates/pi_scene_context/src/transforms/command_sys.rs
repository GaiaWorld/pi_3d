use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_scene_math::*;

use crate::flags::UniqueName;
use crate::scene::command_sys::ActionScene;

use super::command::*;
use super::transform_node::*;

pub fn sys_act_transform_node_create(
    mut cmds: ResMut<ActionListTransformNodeCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNode(scene, entity, name)| {
        let mut transformnode = commands.entity(entity);
        ActionScene::add_to_scene(&mut transformnode, &mut tree, scene);
        ActionTransformNode::init_for_tree(&mut transformnode);
        ActionTransformNode::as_transform_node(&mut transformnode, name);
        ActionAnime::as_anime_group_target(&mut transformnode);
    });
}

pub fn sys_act_transform_parent(
    mut cmds: ResMut<ActionListTransformNodeParent>,
    // mut commands: Commands,
    mut tree: EntityTreeMut,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeParent(entity, val)| {
        if tree.get_down(val).is_some() && tree.get_up(entity).is_some() {
            log::warn!("Tree {:?}, Parent: {:?}", entity, val);
            ActionTransformNode::tree_modify(&mut tree, entity, val);
        } else {
            log::warn!("pUSH {:?}, Parent: {:?}", entity, val);
            cmds.push(OpsTransformNodeParent(entity, val));
        }
    });
}

pub fn sys_act_local_position(
    mut cmds: ResMut<ActionListTransformNodeLocalPosition>,
    mut nodes: Query<&mut LocalPosition>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalPosition(entity, val)| {
        if let Ok(mut node) = nodes.get_mut(entity) {
            *node = LocalPosition(val);
        } else {
            cmds.push(OpsTransformNodeLocalPosition(entity, val));
        }
    });
}

pub fn sys_act_local_euler(
    mut cmds: ResMut<ActionListTransformNodeLocalEuler>,
    mut nodes: Query<&mut LocalEulerAngles>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalEuler(entity, val)| {
        if let Ok(mut node) = nodes.get_mut(entity) {
            *node = LocalEulerAngles(val);
        } else {
            cmds.push(OpsTransformNodeLocalEuler(entity, val));
        }
    });
}

pub fn sys_act_local_scaling(
    mut cmds: ResMut<ActionListTransformNodeLocalScaling>,
    mut nodes: Query<&mut LocalScaling>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalScaling(entity, val)| {
        if let Ok(mut node) = nodes.get_mut(entity) {
            *node = LocalScaling(val);
        } else {
            cmds.push(OpsTransformNodeLocalScaling(entity, val));
        }
    });
}

pub struct ActionTransformNode;
impl ActionTransformNode {
    pub(crate) fn as_transform_node(
        commands: &mut EntityCommands,
        name: String,
    ) {
        commands
            .insert(UniqueName(Atom::from(name)))
            .insert(LocalPosition(Vector3::new(0., 0., 0.)))
            .insert(LocalScaling(Vector3::new(1., 1., 1.)))
            .insert(LocalRotation(Rotation3::identity()))
            .insert(LocalRotationQuaternion(Quaternion::identity()))
            .insert(LocalEulerAngles(Vector3::new(0., 0., 0.)))
            .insert(LocalMatrix::new(Matrix::identity()))
            .insert(WorldMatrix::new(Matrix::identity()))
            .insert(GlobalTransform::default())
            ;
    }

    pub(crate) fn init_for_tree(
        commands: &mut EntityCommands,
    ) {
        commands.insert(Down::default())
            .insert(Up::default())
            .insert(Layer::default());
    }

    pub(crate) fn tree_modify(
        tree: &mut EntityTreeMut,
        child: Entity,
        parent: Entity,
    ) {
        // log::warn!("InsertChild");
        tree.remove(child);
        tree.insert_child(child, parent, 0);
    }
}