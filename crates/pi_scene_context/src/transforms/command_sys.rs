// use std::f32::consts::E;

use pi_scene_shell::prelude::*;

use crate::flags::*;
use crate::object::ActionEntity;

use super::command::*;
use super::transform_node::*;

pub fn sys_create_transform_node(
    mut cmds: ResMut<ActionListTransformNodeCreate>,
    // mut commands: Commands,
    mut alter: Alter<(), (), (EntityTag, TransformNode, TransformNodeBundle), ()>,
) {
    cmds.drain().for_each(|OpsTransformNode(scene, entity)| {
        // let mut transformnode = if let Some(cmd) = commands.get_entity(entity) {
        //     cmd
        // } else {
        //     return;
        // };
        let bundle = (
            EntityTag(TAG_TRANSFORM),
            TransformNode,
            ActionTransformNode::init(scene),
        );
        // commands.entity(entity).insert(bundle);
        let _ = alter.alter(entity, bundle);
    });
}

pub fn sys_act_local_rotation(
    mut cmds: ResMut<ActionListTransformNodeLocalRotationQuaternion>,
    mut nodes: Query<&mut LocalRotationQuaternion>,
    mut record: ResMut<AnimeTargetRecordValues<LocalRotationQuaternion>>,
) {
    cmds.drain().for_each(|OpsTransformNodeLocalRotationQuaternion(entity, x, y, z, w)| {
        if let Ok(mut node) = nodes.get_mut(entity) {
            let data = LocalRotationQuaternion::create(x, y, z, w);
            // log::error!("act_local_rotation {:?}", (entity, &data));
            record.insert(entity, data.clone());
            *node = data;
        }
    });
}
pub fn sys_act_local(
    mut treecmds: ResMut<ActionListTransformNodeParent>,
    // mut parents: Query<&mut NodeChilds>,
    // mut childrens: Query<(&SceneID, &mut NodeParent)>,
    treenodes: Query<&DisposeReady, (With<Layer>, With<Down>, With<Up>)>,
    mut flags: Query<&mut TransformNodeParent>,
    mut tree: EntityTreeMut,

    mut cmds: ResMut<ActionListTransformNodeLocal>,
    mut nodes: Query<&mut LocalPosition>,
    mut nodes_euler: Query<&mut LocalEulerAngles>,
    mut nodes_scaling: Query<&mut LocalScaling>,
    mut recordpos: ResMut<AnimeTargetRecordValues<LocalPosition>>,
    mut recordeul: ResMut<AnimeTargetRecordValues<LocalEulerAngles>>,
    mut recordscl: ResMut<AnimeTargetRecordValues<LocalScaling>>,
) {
    treecmds.drain().for_each(|OpsTransformNodeParent(entity, val)| {
        if let Ok(mut flag) = flags.get_mut(entity) {
            *flag = TransformNodeParent;
        }
        if let (Some(_down), Some(up)) = (tree.get_down(val), tree.get_up(entity)) {
            // log::warn!("transform_parent Child {:?} Parent {:?}", entity, val);
            // log::warn!("Tree {:?}, Parent: {:?}", entity, val);
            if let (Ok(state0), Ok(state1)) = (treenodes.get(entity), treenodes.get(val)) {
                if state0.0 == false && state1.0 == false {
                    if treenodes.contains(up.parent()) {
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

    // log::error!("cmds {:?}", cmds.capacity());
    cmds.drain().for_each(|OpsTransformNodeLocal(entity, val)| {
        match val {
            ETransformSRT::Euler(x, y, z) => {
                if let Ok(mut node) = nodes_euler.get_mut(entity) {
                    let val = Vector3::new(x, y, z);
                    recordeul.insert(entity, LocalEulerAngles(val));
                    *node = LocalEulerAngles(val);
                }
            },
            ETransformSRT::Translation(x, y, z) => {
                if let Ok(mut node) = nodes.get_mut(entity) {
                    let val = Vector3::new(x, y, z);
                    recordpos.insert(entity, LocalPosition(val));
                    *node = LocalPosition(val);
                }
            },
            ETransformSRT::Scaling(x, y, z) => {
                if let Ok(mut node) = nodes_scaling.get_mut(entity) {
                    let val = Vector3::new(x, y, z);
                    recordscl.insert(entity, LocalScaling(val));
                    *node = LocalScaling(val);
                }
            },
        }
    });
}

pub type BundleTreeNode = (Down, Up, Layer, Enable, GlobalEnable, TransformNodeParent);
pub type BundleTransform = (
    TransformNodeDirty, LocalPosition, LocalScaling, LocalRotationQuaternion, LocalEulerAngles,
    LocalRotation, FlagLocalMatrix, LocalMatrix, GlobalMatrix, AbsoluteTransform, FlagAnimationStartResetComp,
);

pub type TransformNodeBundle = (
    BundleEntity,
    SceneID,
    BundleTreeNode,
    BundleTransform,
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
    fn as_transform_node() -> BundleTransform {
        (
            TransformNodeDirty(true),
            LocalPosition::default(),
            LocalScaling::default(),
            LocalRotationQuaternion::default(),
            LocalEulerAngles::default(),
            LocalRotation(Rotation3::identity()),
            FlagLocalMatrix,
            LocalMatrix::new(Matrix::identity()),
            GlobalMatrix::default(),
            AbsoluteTransform::default(),
            FlagAnimationStartResetComp,
        )
    }

    pub(crate) fn init_for_tree() -> BundleTreeNode {
        // log::debug!("init_for_tree====={:?}", commands.id());
        (
            Down::default(),
            Up::default(),
            Layer::default(),
            Enable::default(),
            GlobalEnable(false),
            TransformNodeParent,
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