use pi_ecs::{prelude::{ResMut, Query, EntityDelete}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::{Vector3, Quaternion};

use crate::{object::{ObjectID, GameObject}};

use super::{transform_node::{LocalTransform, GlobalTransform}, dirty::DirtyLocalTransform};


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
        mut transforms: Query<GameObject, (Write<LocalTransform>, Write<GlobalTransform>, Write<DirtyLocalTransform>)>,
        mut delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TransformNodeCommand::Create(node) => {
                    match transforms.get_mut(node) {
                        Some(mut transform) => {
                            transform.0.insert_no_notify(LocalTransform::default());
                            transform.1.insert_no_notify(GlobalTransform::default());
                            transform.2.insert_no_notify(DirtyLocalTransform);
                            
                            // println!("DirtyLocalTransform >>>>>>>>>> ");
                        },
                        None => {},
                    }
                },
                TransformNodeCommand::Destroy(node) => {
                    delete.despawn(node);
                },
                TransformNodeCommand::ModifyPosition(node, value) => {
                    match transforms.get_mut(node) {
                        Some(mut transform) => {
                            match transform.0.get_mut() {
                                Some(transform) => {
                                    transform.position = value;
                                },
                                None => todo!(),
                            }
                            match transform.2.get_mut() {
                                Some(_) => {
                                },
                                None => {
                                    transform.2.insert_no_notify(DirtyLocalTransform);
                                    // println!("DirtyLocalTransform >>>>>>>>>> ");
                                },
                            }
                        },
                        None => {},
                    }
                },
                TransformNodeCommand::ModifyRotation(node, value) => {
                    match transforms.get_mut(node) {
                        Some(mut transform) => {
                            match transform.0.get_mut() {
                                Some(transform) => {
                                    transform.euler = value;
                                },
                                None => todo!(),
                            }
                            match transform.2.get_mut() {
                                Some(_) => {
                                },
                                None => {
                                    transform.2.insert_no_notify(DirtyLocalTransform);
                                    // println!("DirtyLocalTransform >>>>>>>>>> ");
                                },
                            }
                        },
                        None => {},
                    }      
                },
                TransformNodeCommand::ModifyScaling(node, value) => {
                    match transforms.get_mut(node) {
                        Some(mut transform) => {
                            match transform.0.get_mut() {
                                Some(transform) => {
                                    transform.scaling = value;
                                },
                                None => todo!(),
                            }
                            match transform.2.get_mut() {
                                Some(_) => {
                                },
                                None => {
                                    transform.2.insert_no_notify(DirtyLocalTransform);
                                    // println!("DirtyLocalTransform >>>>>>>>>> ");
                                },
                            }
                        },
                        None => {},
                    }      
                },
                TransformNodeCommand::ModifyRotationQuaternion(node, value) => {
                    match transforms.get_mut(node) {
                        Some(mut transform) => {
                            match transform.0.get_mut() {
                                Some(transform) => {
                                    transform.quaternion = value;
                                },
                                None => todo!(),
                            }
                            match transform.2.get_mut() {
                                Some(_) => {
                                },
                                None => {
                                    transform.2.insert_no_notify(DirtyLocalTransform);
                                    // println!("DirtyLocalTransform >>>>>>>>>> ");
                                },
                            }
                        },
                        None => {},
                    }      
                },
                TransformNodeCommand::ModifyTarget(_, _) => todo!(),
            }
        });
    }
}