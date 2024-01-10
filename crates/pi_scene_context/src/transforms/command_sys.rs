// use std::f32::consts::E;

use pi_scene_shell::prelude::*;
use pi_scene_math::*;

use crate::flags::*;
use crate::object::ActionEntity;
use crate::scene::command_sys::ActionScene;

use super::command::*;
use super::transform_node::*;

pub fn sys_create_transform_node(
    mut cmds: ResMut<ActionListTransformNodeCreate>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNode(scene, entity)| {
        let mut transformnode = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            return;
        };
        ActionTransformNode::init(&mut transformnode, scene);
        transformnode.insert(TransformNode);
    });
}

pub fn sys_act_transform_parent(
    mut cmds: ResMut<ActionListTransformNodeParent>,
    // mut parents: Query<&mut NodeChilds>,
    // mut childrens: Query<(&SceneID, &mut NodeParent)>,
    nodes: Query<&DisposeReady, (With<Layer>, With<Down>, With<Up>)>,
    mut tree: EntityTreeMut,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeParent(entity, val)| {
        if let (Some(_down), Some(up)) = (tree.get_down(val), tree.get_up(entity)) {
            // log::warn!("transform_parent Child {:?} Parent {:?}", entity, val);
            // log::warn!("Tree {:?}, Parent: {:?}", entity, val);
            if let (Ok(state0), Ok(state1)) = (nodes.get(entity), nodes.get(val)) {
                if state0.0 == false && state1.0 == false {
                    if nodes.contains(up.parent()) {
                        tree.remove(entity);
                    }
                    // log::warn!("AAA insert_child=====child: {:?}, parent: {:?}",entity, val);
                    // log::warn!("Tree insert_child {:?} Parent: {:?}", child, parent);
                    tree.insert_child(entity, val, 0);
                    // ActionTransformNode::tree_modify(&mut tree, entity, val);
                }
            }
        }
    });
}

pub fn sys_act_local_rotation(
    mut cmds: ResMut<ActionListTransformNodeLocalRotationQuaternion>,
    mut nodes: Query<(&mut LocalRotationQuaternion, &mut RecordLocalRotationQuaternion)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalRotationQuaternion(entity, x, y, z, w)| {
        if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
            let data = LocalRotationQuaternion::create(x, y, z, w);
            // log::error!("act_local_rotation {:?}", (entity, &data));
            record.0 = data.clone();
            *node = data;
        // } else {
        //     if count < 4 {
        //         cmds.push(OpsTransformNodeLocalRotationQuaternion(entity, x, y, z, w, count + 1));
        //     }
        }
    });
}

pub fn sys_act_local_position(
    mut cmds: ResMut<ActionListTransformNodeLocalPosition>,
    mut nodes: Query<(&mut LocalPosition, &mut RecordLocalPosition)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalPosition(entity, val)| {
        if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
            // log::warn!("transform_Position {:?} Val: {:?}", entity, val);
            record.0 = LocalPosition(val);
            *node = LocalPosition(val);
        // } else {
        //     if count < 4 {
        //         cmds.push(OpsTransformNodeLocalPosition(entity, val, count + 1));
        //     }
        }
    });
}

pub fn sys_act_local_euler(
    mut cmds: ResMut<ActionListTransformNodeLocalEuler>,
    mut nodes: Query<(&mut LocalEulerAngles, &mut RecordLocalEulerAngles)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalEuler(entity, val)| {
        if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
            record.0 = LocalEulerAngles(val);
            *node = LocalEulerAngles(val);
            // log::error!("sys_act_local_euler {:?}", (entity, &val));
        // } else {
        //     if count < 4 {
        //         cmds.push(OpsTransformNodeLocalEuler(entity, val, count + 1));
        //     }
        }
    });
}

pub fn sys_act_local_scaling(
    mut cmds: ResMut<ActionListTransformNodeLocalScaling>,
    mut nodes: Query<(&mut LocalScaling, &mut RecordLocalScaling)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalScaling(entity, val)| {
        if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
            record.0 = LocalScaling(val);
            *node = LocalScaling(val);
        // } else {
        //     if count < 2 {
        //         cmds.push(OpsTransformNodeLocalScaling(entity, val, count + 1));
        //     }
        }
    });
}

pub struct ActionTransformNode;
impl ActionTransformNode {
    pub fn init(
        transformnode: &mut EntityCommands,
        scene: Entity,
    ) {
        ActionEntity::init(transformnode);
        ActionScene::add_to_scene(transformnode, scene);
        ActionTransformNode::init_for_tree(transformnode);
        ActionTransformNode::as_transform_node(transformnode);
    }
    fn as_transform_node(
        commands: &mut EntityCommands,
    ) {
        commands
            .insert(TransformNodeDirty)
            .insert(LocalPosition::default())
            .insert(LocalScaling::default())
            .insert(LocalRotationQuaternion::default())
            .insert(LocalEulerAngles::default())
            .insert(RecordLocalPosition::default())
            .insert(RecordLocalScaling::default())
            .insert(RecordLocalRotationQuaternion::default())
            .insert(RecordLocalEulerAngles::default())
            .insert(LocalRotation(Rotation3::identity()))
            .insert(LocalMatrix::new(Matrix::identity()))
            .insert(GlobalMatrix::default())
            .insert(AbsoluteTransform::default())
            .insert(FlagAnimationStartResetComp)
            .insert(CullingFlag(true))
            ;
    }

    pub(crate) fn init_for_tree(
        commands: &mut EntityCommands,
    ) {
        // log::debug!("init_for_tree====={:?}", commands.id());
        commands
            .insert(Down::default())
            .insert(Up::default())
            .insert(Layer::default())
            // .insert(NodeChilds::default())
            // .insert(NodeParent(None))
            .insert(Enable::default())
            .insert(RecordEnable::default())
            .insert(GlobalEnable(true))
            ;
    }

    pub(crate) fn _tree_modify(
        tree: &mut EntityTreeMut,
        child: Entity,
        parent: Entity,
    ) {
        // log::warn!("InsertChild");
        // log::warn!("Tree Remove {:?}", child);
        tree.remove(child);
        // log::warn!("insert_child=====child: {:?}, parent: {:?}",child, parent);
        // log::warn!("Tree insert_child {:?} Parent: {:?}", child, parent);
        tree.insert_child(child, parent, 0);
    }
}