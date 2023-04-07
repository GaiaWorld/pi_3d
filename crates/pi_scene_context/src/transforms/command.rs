
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector3, Quaternion, Rotation3};

use super::{transform_node::{LocalPosition, LocalRotation, LocalRotationQuaternion, LocalEulerAngles, LocalScaling, GlobalTransform}, tree_left_right::{TreeLeftRoot, TreeRightRoot}};

pub enum TreeCommand {
    Append(ObjectID, ObjectID),
    Remove(ObjectID),
}

pub fn tree_append(
    tree: &mut EntityTreeMut,
    cmd: TreeCommand,
) {
    match cmd {
        TreeCommand::Append(child, parent) => {
            tree.insert_child(child, parent, usize::MAX);
        },
        TreeCommand::Remove(child) => {
            tree.remove(child);
        },
    }
}

// pub struct SingleTreeCommandList {
//     pub list: Vec<TreeCommand>,
// }
// pub struct SysTreeCommand;
// impl TSystemStageInfo for SysTreeCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysTransformNodeCreateCommand::key()
//         ]
//     }
// }
// #[setup]
// impl SysTreeCommand {
//     #[system]
//     pub fn cmd(
//         mut cmds: ResMut<SingleTreeCommandList>,
//         entitys: Query<GameObject, ObjectID>,
//         mut scenes: Query<GameObject, (&mut TreeLeftRoot, &mut TreeRightRoot)>,
//         mut tree: EntityTreeMut<GameObject>,
//     ) {
//         cmds.list.drain(..).for_each(|cmd| {
//             match cmd {
//                 TreeCommand::Append(child, parent) => {
//                     if entitys.get(child).is_some() {
//                         tree.insert_child(child, parent, usize::MAX);
//                     }
//                 },
//                 TreeCommand::Remove(child) => {
//                     tree.remove(child);
//                 },
//             }
//         });
//     }
// }

pub fn create_transform_node(
    commands: &mut Commands,
    entity: Entity,
) {
    commands.entity(entity).insert(LocalPosition(Vector3::new(0., 0., 0.)))
        .insert(LocalScaling(Vector3::new(1., 1., 1.)))
        .insert(LocalRotation(Rotation3::identity()))
        .insert(LocalRotationQuaternion(Quaternion::identity()))
        .insert(LocalEulerAngles(Vector3::new(0., 0., 0.)))
        .insert(GlobalTransform::default());
}

pub fn modify_transform_node(
    commands: &mut Commands,
    cmd: ETransformNodeModifyCommand,
) {
    match cmd {
        ETransformNodeModifyCommand::ModifyPosition(entity, value) => {
            commands.entity(entity).insert(LocalPosition(value));
        },
        ETransformNodeModifyCommand::ModifyRotation(entity, value) => {
            
            // let rotation = Rotation3::from_euler_angles(value.y, value.x, value.z);
            // let quaternion = Quaternion::from_rotation_matrix(&rotation); 
            // qua_cmd.insert(node, LocalRotationQuaternion(quaternion));
            // rot_cmd.insert(node, LocalRotation(rotation));
            commands.entity(entity).insert(LocalEulerAngles(value));     
        },
        ETransformNodeModifyCommand::ModifyScaling(entity, value) => {
            commands.entity(entity).insert(LocalScaling(value));    
        },
        ETransformNodeModifyCommand::ModifyRotationQuaternion(entity, value) => {
            // let rotation = value.to_rotation_matrix();
            // let mut euler = Vector3::new(0., 0., 0.);
            // let (z, x, y) = rotation.euler_angles();
            // euler.copy_from_slice(&[x, y, z]);

            commands.entity(entity).insert(LocalRotationQuaternion(value));
            // rot_cmd.insert(node, LocalRotation(rotation));
            // eul_cmd.insert(node, LocalEulerAngles(euler));    
        },
        ETransformNodeModifyCommand::ModifyTarget(_, _) => todo!(),
    }
}

// pub enum ETransformNodeCreateCommand {
//     Create(ObjectID),
// }
// pub struct SingleTransformNodeCreateCommandList {
//     pub list: Vec<ETransformNodeCreateCommand>,
// }

// pub struct SysTransformNodeCreateCommand;
// impl TSystemStageInfo for SysTransformNodeCreateCommand {

// }
// #[setup]
// impl SysTransformNodeCreateCommand {
//     #[system]
//     pub fn cmd(
//         mut cmds: ResMut<SingleTransformNodeCreateCommandList>,
//         mut gtr_cmd: Commands<GameObject, GlobalTransform>,
//         mut pos_cmd: Commands<GameObject, LocalPosition>,
//         mut scl_cmd: Commands<GameObject, LocalScaling>,
//         mut rot_cmd: Commands<GameObject, LocalRotation>,
//         mut qua_cmd: Commands<GameObject, LocalRotationQuaternion>,
//         mut eul_cmd: Commands<GameObject, LocalEulerAngles>,
//     ) {
//         cmds.list.drain(..).for_each(|cmd| {
//             match cmd {
//                 ETransformNodeCreateCommand::Create(node) => {
//                     pos_cmd.insert(node, LocalPosition(Vector3::new(0., 0., 0.)));
//                     scl_cmd.insert(node, LocalScaling(Vector3::new(1., 1., 1.)));
//                     rot_cmd.insert(node, LocalRotation(Rotation3::identity()));
//                     qua_cmd.insert(node, LocalRotationQuaternion(Quaternion::identity()));
//                     eul_cmd.insert(node, LocalEulerAngles(Vector3::new(0., 0., 0.)));
//                     gtr_cmd.insert(node, GlobalTransform::default());
//                 },
//             }
//         });
//     }
// }

pub enum ETransformNodeModifyCommand {
    ModifyPosition(ObjectID, Vector3),
    ModifyRotation(ObjectID, Vector3),
    ModifyScaling(ObjectID, Vector3),
    ModifyRotationQuaternion(ObjectID, Quaternion),
    ModifyTarget(ObjectID, Vector3),
}
// pub struct SingleTransformNodeModifyCommandList {
//     pub list: Vec<ETransformNodeModifyCommand>,
// }

// pub struct SysTransformNodeModifyCommand;
// impl TSystemStageInfo for SysTransformNodeModifyCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysTransformNodeCreateCommand::key()
//         ]
//     }
// }
// #[setup]
// impl SysTransformNodeModifyCommand {
//     #[system]
//     pub fn cmd(
//         mut cmds: ResMut<SingleTransformNodeModifyCommandList>,
//         mut gtr_cmd: Commands<GameObject, GlobalTransform>,
//         mut pos_cmd: Commands<GameObject, LocalPosition>,
//         mut scl_cmd: Commands<GameObject, LocalScaling>,
//         mut rot_cmd: Commands<GameObject, LocalRotation>,
//         mut qua_cmd: Commands<GameObject, LocalRotationQuaternion>,
//         mut eul_cmd: Commands<GameObject, LocalEulerAngles>,
//     ) {
//         cmds.list.drain(..).for_each(|cmd| {
//             match cmd {
//                 ETransformNodeModifyCommand::ModifyPosition(node, value) => {
//                     pos_cmd.insert(node, LocalPosition(value));
//                 },
//                 ETransformNodeModifyCommand::ModifyRotation(node, value) => {
                    
//                     // let rotation = Rotation3::from_euler_angles(value.y, value.x, value.z);
//                     // let quaternion = Quaternion::from_rotation_matrix(&rotation); 
//                     // qua_cmd.insert(node, LocalRotationQuaternion(quaternion));
//                     // rot_cmd.insert(node, LocalRotation(rotation));
//                     eul_cmd.insert(node, LocalEulerAngles(value));     
//                 },
//                 ETransformNodeModifyCommand::ModifyScaling(node, value) => {
//                     scl_cmd.insert(node, LocalScaling(value));    
//                 },
//                 ETransformNodeModifyCommand::ModifyRotationQuaternion(node, value) => {
//                     // let rotation = value.to_rotation_matrix();
//                     // let mut euler = Vector3::new(0., 0., 0.);
//                     // let (z, x, y) = rotation.euler_angles();
//                     // euler.copy_from_slice(&[x, y, z]);

//                     qua_cmd.insert(node, LocalRotationQuaternion(value));
//                     // rot_cmd.insert(node, LocalRotation(rotation));
//                     // eul_cmd.insert(node, LocalEulerAngles(euler));    
//                 },
//                 ETransformNodeModifyCommand::ModifyTarget(_, _) => todo!(),
//             }
//         });
//     }
// }