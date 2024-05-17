// use std::f32::consts::E;

use pi_scene_shell::prelude::*;

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
        transformnode.insert((
            TransformNode,
            ActionTransformNode::init(scene),
        ));
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
        }
    });
}

pub fn sys_act_local(
    mut cmds: ResMut<ActionListTransformNodeLocal>,
    mut nodes: Query<(&mut LocalPosition, &mut RecordLocalPosition)>,
    mut nodes_euler: Query<(&mut LocalEulerAngles, &mut RecordLocalEulerAngles)>,
    mut nodes_scaling: Query<(&mut LocalScaling, &mut RecordLocalScaling)>,
) {
    cmds.drain().drain(..).for_each(|OpsTransformNodeLocal(entity, val)| {
        match val {
            ETransformSRT::Euler(x, y, z) => {
                if let Ok((mut node, mut record)) = nodes_euler.get_mut(entity) {
                    let val = Vector3::new(x, y, z);
                    record.0 = LocalEulerAngles(val);
                    *node = LocalEulerAngles(val);
                }
            },
            ETransformSRT::Translation(x, y, z) => {
                if let Ok((mut node, mut record)) = nodes.get_mut(entity) {
                    let val = Vector3::new(x, y, z);
                    record.0 = LocalPosition(val);
                    *node = LocalPosition(val);
                }
            },
            ETransformSRT::Scaling(x, y, z) => {
                if let Ok((mut node, mut record)) = nodes_scaling.get_mut(entity) {
                    let val = Vector3::new(x, y, z);
                    record.0 = LocalScaling(val);
                    *node = LocalScaling(val);
                }
            },
        }
    });
}

pub type TreeNodeBundle = (Down, Up, Layer, Enable, RecordEnable, GlobalEnable);

pub type TransformNodeBundle = (
    (DisposeReady, DisposeCan),
    SceneID,
    TreeNodeBundle,
    (
        TransformNodeDirty, LocalPosition, LocalScaling, LocalRotationQuaternion, LocalEulerAngles,
        RecordLocalPosition, RecordLocalScaling, RecordLocalRotationQuaternion, RecordLocalEulerAngles,
        LocalRotation, LocalMatrix, GlobalMatrix, AbsoluteTransform, FlagAnimationStartResetComp, CullingFlag
    )
);

pub struct ActionTransformNode;
impl ActionTransformNode {
    pub fn init(scene: Entity) -> TransformNodeBundle {
        (
            ActionEntity::init(),
            SceneID(scene),
            ActionTransformNode::init_for_tree(),
            ActionTransformNode::as_transform_node(),
        )
    }
    fn as_transform_node() -> (
        TransformNodeDirty, LocalPosition, LocalScaling, LocalRotationQuaternion, LocalEulerAngles,
        RecordLocalPosition, RecordLocalScaling, RecordLocalRotationQuaternion, RecordLocalEulerAngles,
        LocalRotation, LocalMatrix, GlobalMatrix, AbsoluteTransform, FlagAnimationStartResetComp, CullingFlag
    ) {
        (
            TransformNodeDirty,
            LocalPosition::default(),
            LocalScaling::default(),
            LocalRotationQuaternion::default(),
            LocalEulerAngles::default(),
            RecordLocalPosition::default(),
            RecordLocalScaling::default(),
            RecordLocalRotationQuaternion::default(),
            RecordLocalEulerAngles::default(),
            LocalRotation(Rotation3::identity()),
            LocalMatrix::new(Matrix::identity()),
            GlobalMatrix::default(),
            AbsoluteTransform::default(),
            FlagAnimationStartResetComp,
            CullingFlag(true),
        )
    }

    pub(crate) fn init_for_tree() -> TreeNodeBundle {
        // log::debug!("init_for_tree====={:?}", commands.id());
        (
            Down::default(),
            Up::default(),
            Layer::default(),
            Enable::default(),
            RecordEnable::default(),
            GlobalEnable(false),
            // .insert(NodeChilds::default())
            // .insert(NodeParent(None))
        )
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