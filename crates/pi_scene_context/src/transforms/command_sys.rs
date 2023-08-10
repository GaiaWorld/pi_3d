use pi_engine_shell::prelude::*;
use pi_scene_math::*;

use crate::flags::*;
use crate::object::ActionEntity;
use crate::scene::command_sys::ActionScene;

use super::command::*;
use super::transform_node::*;

pub fn sys_act_transform_node_create(
    mut cmds: ResMut<ActionListTransformNodeCreate>,
    mut tree: ResMut<ActionListTransformNodeParent>,
    mut commands: Commands,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNode(scene, entity, name)| {
        let mut transformnode = if let Some(cmd) = commands.get_entity(entity) {
            cmd
        } else {
            return;
        };
        ActionTransformNode::init(&mut transformnode, &mut tree, scene, name);
        ActionAnime::as_anime_group_target(&mut transformnode);
        transformnode.insert(TransformNode);
    });
}

pub fn sys_act_transform_parent(
    mut cmds: ResMut<ActionListTransformNodeParent>,
    // mut commands: Commands,
    mut tree: EntityTreeMut,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeParent(entity, val, count)| {
        if tree.get_down(val).is_some() && tree.get_up(entity).is_some() {
            // log::warn!("transform_parent Child {:?} Parent {:?}", entity, val);
            // log::warn!("Tree {:?}, Parent: {:?}", entity, val);
            ActionTransformNode::tree_modify(&mut tree, entity, val);
        } else {
            if count < 2 {
                cmds.push(OpsTransformNodeParent(entity, val, count + 1));
            }
        }
    });
}

pub fn sys_act_local_rotation(
    mut cmds: ResMut<ActionListTransformNodeLocalRotationQuaternion>,
    mut nodes: Query<(&mut LocalRotationQuaternion, &mut RecordLocalRotationQuaternion)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalRotationQuaternion(entity, x, y, z, w, count)| {
        if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
            let data = LocalRotationQuaternion::create(x, y, z, w);
            record.0 = data.clone();
            *node = data;
        } else {
            if count < 2 {
                cmds.push(OpsTransformNodeLocalRotationQuaternion(entity, x, y, z, w, count + 1));
            }
        }
    });
}

pub fn sys_act_local_position(
    mut cmds: ResMut<ActionListTransformNodeLocalPosition>,
    mut nodes: Query<(&mut LocalPosition, &mut RecordLocalPosition)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalPosition(entity, val, count)| {
        if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
            // log::warn!("transform_Position {:?} Val: {:?}", entity, val);
            record.0 = LocalPosition(val);
            *node = LocalPosition(val);
        } else {
            if count < 2 {
                cmds.push(OpsTransformNodeLocalPosition(entity, val, count + 1));
            }
        }
    });
}

pub fn sys_act_local_euler(
    mut cmds: ResMut<ActionListTransformNodeLocalEuler>,
    mut nodes: Query<(&mut LocalEulerAngles, &mut RecordLocalEulerAngles)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalEuler(entity, val, count)| {
        if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
            record.0 = LocalEulerAngles(val);
            *node = LocalEulerAngles(val);
        } else {
            if count < 2 {
                cmds.push(OpsTransformNodeLocalEuler(entity, val, count + 1));
            }
        }
    });
}

pub fn sys_act_local_scaling(
    mut cmds: ResMut<ActionListTransformNodeLocalScaling>,
    mut nodes: Query<(&mut LocalScaling, &mut RecordLocalScaling)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocalScaling(entity, val, count)| {
        if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
            record.0 = LocalScaling(val);
            *node = LocalScaling(val);
        } else {
            if count < 2 {
                cmds.push(OpsTransformNodeLocalScaling(entity, val, count + 1));
            }
        }
    });
}

pub struct ActionTransformNode;
impl ActionTransformNode {
    pub fn init(
        transformnode: &mut EntityCommands,
        tree: &mut ActionListTransformNodeParent,
        scene: Entity,
        name: String,
    ) {
        ActionEntity::init(transformnode);
        ActionScene::add_to_scene(transformnode, tree, scene);
        ActionTransformNode::init_for_tree(transformnode);
        ActionTransformNode::as_transform_node(transformnode, name);
    }
    fn as_transform_node(
        commands: &mut EntityCommands,
        name: String,
    ) {
        commands
            .insert(UniqueName(Atom::from(name)))
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
            .insert(WorldMatrix::new(Matrix::identity()))
            .insert(WorldMatrixInv::new(Matrix::identity()))
            .insert(GlobalTransform::default())
            .insert(FlagAnimationStartResetComp)
            .insert(CullingFlag(true))
            ;
    }

    pub(crate) fn init_for_tree(
        commands: &mut EntityCommands,
    ) {
        commands.insert(Down::default())
            .insert(Up::default())
            .insert(Layer::default())
            .insert(Enable::default())
            .insert(RecordEnable::default())
            .insert(GlobalEnable(true))
            ;
    }

    pub(crate) fn tree_modify(
        tree: &mut EntityTreeMut,
        child: Entity,
        parent: Entity,
    ) {
        // log::warn!("InsertChild");
        // log::warn!("Tree Remove {:?}", child);
        tree.remove(child);
        // log::warn!("Tree insert_child {:?} Parent: {:?}", child, parent);
        tree.insert_child(child, parent, 0);
    }
}