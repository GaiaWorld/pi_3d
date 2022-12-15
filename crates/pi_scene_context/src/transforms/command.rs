use pi_ecs::{prelude::{ResMut, Query, EntityDelete}, query::Write};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTreeMut;
use pi_scene_math::{Vector3, Quaternion, Rotation3, coordiante_system::CoordinateSytem3, vector::TToolRotation};

use crate::{object::{ObjectID, GameObject}, };

use super::{transform_node::{LocalPosition, LocalRotation, LocalRotationQuaternion, LocalEulerAngles, LocalScaling, LocalRoationWithQuaternion}};

pub enum TreeCommand {
    Append(ObjectID, ObjectID),
    Remove(ObjectID),
}

pub struct SingleTreeCommandList {
    pub list: Vec<TreeCommand>,
}
pub struct SysTreeCommand;
#[setup]
impl SysTreeCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleTreeCommandList>,
        entitys: Query<GameObject, ObjectID>,
        mut tree: EntityTreeMut<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TreeCommand::Append(child, parent) => {
                    if entitys.get(child).is_some() {
                        tree.insert_child(child, parent, usize::MAX);
                    }
                },
                TreeCommand::Remove(child) => {
                    tree.remove(child);
                },
            }
        });
    }
}


pub enum TransformNodeCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    ModifyPosition(ObjectID, Vector3),
    ModifyRotation(ObjectID, Vector3),
    ModifyScaling(ObjectID, Vector3),
    ModifyRotationQuaternion(ObjectID, Quaternion),
    ModifyTarget(ObjectID, Vector3),
}
pub struct SingleTransformNodeCommandList {
    pub list: Vec<TransformNodeCommand>,
}

pub struct SysTransformNodeCommand;
#[setup]
impl SysTransformNodeCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleTransformNodeCommandList>,
        mut transforms: Query<
            GameObject,
            (
                Write<LocalPosition>, Write<LocalScaling>,
                Write<LocalRotation>, Write<LocalRotationQuaternion>, Write<LocalEulerAngles>
            )
        >,
        mut delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TransformNodeCommand::Create(node) => {
                    match transforms.get_mut(node) {
                        Some((mut position, mut lscaling, mut lrotation, mut lquaternion, mut leuler)) => {
                            position.write(LocalPosition(Vector3::new(0., 0., 0.)));
                            lscaling.write(LocalScaling(Vector3::new(1., 1., 1.)));
                            lrotation.write(LocalRotation(Rotation3::identity()));
                            lquaternion.write(LocalRotationQuaternion(Quaternion::identity()));
                            leuler.write(LocalEulerAngles(Vector3::new(0., 0., 0.)));
                        },
                        None => {},
                    }
                },
                TransformNodeCommand::Destroy(node) => {
                    delete.despawn(node);
                },
                TransformNodeCommand::ModifyPosition(node, value) => {
                    match transforms.get_mut(node) {
                        Some((mut position, mut lscaling, mut lrotation, mut lquaternion, mut leuler)) => {
                            position.write(LocalPosition(value));
                        },
                        None => {},
                    }
                },
                TransformNodeCommand::ModifyRotation(node, value) => {
                    match transforms.get_mut(node) {
                        Some((mut position, mut lscaling, mut lrotation, mut lquaternion, mut leuler)) => {
                            
                            let rotation = Rotation3::from_euler_angles(value.y, value.x, value.z);
                            let quaternion = Quaternion::from_rotation_matrix(&rotation);
                            lquaternion.write(LocalRotationQuaternion(quaternion));
                            lrotation.write(LocalRotation(rotation));
                            leuler.write(LocalEulerAngles(value));
                        },
                        None => {},
                    }      
                },
                TransformNodeCommand::ModifyScaling(node, value) => {
                    match transforms.get_mut(node) {
                        Some((mut position, mut lscaling, mut lrotation, mut lquaternion, mut leuler)) => {
                            lscaling.write(LocalScaling(value));
                        },
                        None => {},
                    }      
                },
                TransformNodeCommand::ModifyRotationQuaternion(node, value) => {
                    match transforms.get_mut(node) {
                        Some((mut position, mut lscaling, mut lrotation, mut lquaternion, mut leuler)) => {
                            
                            let rotation = value.to_rotation_matrix();
                            let mut euler = Vector3::new(0., 0., 0.);
                            let (z, x, y) = rotation.euler_angles();
                            euler.copy_from_slice(&[x, y, z]);

                            lquaternion.write(LocalRotationQuaternion(value));
                            lrotation.write(LocalRotation(rotation));
                            leuler.write(LocalEulerAngles(euler));
                        },
                        None => {},
                    }      
                },
                TransformNodeCommand::ModifyTarget(_, _) => todo!(),
            }
        });
    }
}