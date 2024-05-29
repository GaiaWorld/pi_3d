// use std::f32::consts::E;

use pi_scene_shell::add_component;
use pi_scene_shell::prelude::pi_world::editor::EntityEditor;
use pi_scene_shell::prelude::*;

use crate::flags::*;
use crate::object::ActionEntity;
use crate::scene::command_sys::ActionScene;

use super::command::*;
use super::transform_node::*;

pub fn sys_create_transform_node(
    mut cmds: ResMut<ActionListTransformNodeCreate>,
    mut editor: EntityEditor,
   
) {
    cmds.drain().drain(..).for_each(|OpsTransformNode(scene, entity)| {
        if !editor.contains_entity(entity) {
            return;
        }

        let _ = ActionTransformNode::init(entity, &mut editor, scene);
        add_component(&mut editor, entity, TransformNode).unwrap();
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
pub type ActionTransformNodeBundle = (
    TransformNodeDirty,
    LocalPosition,
    LocalScaling,
    LocalRotationQuaternion,
    LocalEulerAngles,
    RecordLocalPosition,
    RecordLocalScaling,
    RecordLocalRotationQuaternion,
    RecordLocalEulerAngles,
    LocalRotation,
    LocalMatrix,
    GlobalMatrix,
    AbsoluteTransform,
    FlagAnimationStartResetComp,
    CullingFlag
);

pub struct ActionTransformNode;
impl ActionTransformNode {
    pub fn init(
        entity: Entity, 
        editor: &mut EntityEditor,
        scene: Entity,
    ) {
        ActionEntity::init(entity, editor);
        ActionScene::add_to_scene(entity, editor, scene);
        ActionTransformNode::init_for_tree(entity, editor);
        ActionTransformNode::as_transform_node(entity, editor);
    }
    fn as_transform_node(
        entity: Entity,
        editor: &mut EntityEditor,
    ) {
        let components = [
            editor.init_component::<TransformNodeDirty>(),
            editor.init_component::<LocalPosition>(),
            editor.init_component::<LocalScaling>(),
            editor.init_component::<LocalRotationQuaternion>(),
            editor.init_component::<LocalEulerAngles>(),
            editor.init_component::<RecordLocalPosition>(),
            editor.init_component::<RecordLocalScaling>(),
            editor.init_component::<RecordLocalRotationQuaternion>(),
            editor.init_component::<RecordLocalEulerAngles>(),
            editor.init_component::<LocalRotation>(),
            editor.init_component::<LocalMatrix>(),
            editor.init_component::<GlobalMatrix>(),
            editor.init_component::<AbsoluteTransform>(),
            editor.init_component::<FlagAnimationStartResetComp>(),
            editor.init_component::<CullingFlag>(),
        ];
        editor.add_components(entity, &components).unwrap();

        *editor.get_component_unchecked_mut_by_id(entity, components[0]) =TransformNodeDirty;
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) =LocalPosition::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) =LocalScaling::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) =LocalRotationQuaternion::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) =LocalEulerAngles::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[5]) =RecordLocalPosition::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[6]) =RecordLocalScaling::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[7]) =RecordLocalRotationQuaternion::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[8]) =RecordLocalEulerAngles::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[9]) =LocalRotation(Rotation3::identity());
        *editor.get_component_unchecked_mut_by_id(entity, components[10]) =LocalMatrix::new(Matrix::identity());
        *editor.get_component_unchecked_mut_by_id(entity, components[11]) =GlobalMatrix::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[12]) =AbsoluteTransform::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[13]) =FlagAnimationStartResetComp;
        *editor.get_component_unchecked_mut_by_id(entity, components[14]) =CullingFlag(true);

    }

    pub(crate) fn init_for_tree(
        entity: Entity,
        editor: &mut EntityEditor,
    ) {
        // log::debug!("init_for_tree====={:?}", commands.id());
        let components = [
            editor.init_component::<Down>(),
            editor.init_component::<Up>(),
            editor.init_component::<Layer>(),
            editor.init_component::<Enable>(),
            editor.init_component::<RecordEnable>(),
            editor.init_component::<GlobalEnable>(),
        ];
        editor.add_components(entity, &components).unwrap();

        *editor.get_component_unchecked_mut_by_id(entity, components[0]) =Down::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[1]) = Up::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[2]) = Layer::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[3]) = Enable::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[4]) = RecordEnable::default();
        *editor.get_component_unchecked_mut_by_id(entity, components[5]) = GlobalEnable(true);

        // commands
        //     .alter(entity, (Down::default(), Up::default(), Layer::default(),
        //     // .insert(NodeChilds::default())
        //     // .insert(NodeParent(None))
        //     Enable::default(), RecordEnable::default(), GlobalEnable(true))
        // );
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