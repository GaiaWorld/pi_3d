use pi_ecs::{prelude::{ResMut, Query, EntityDelete, Commands}};
use pi_ecs_macros::setup;
use pi_ecs_utils::prelude::EntityTreeMut;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{Vector3, Quaternion, Rotation3};

use crate::{object::{ObjectID, GameObject}, };

use super::{transform_node::{LocalPosition, LocalRotation, LocalRotationQuaternion, LocalEulerAngles, LocalScaling, GlobalTransform}};

pub enum TreeCommand {
    Append(ObjectID, ObjectID),
    Remove(ObjectID),
}

pub struct SingleTreeCommandList {
    pub list: Vec<TreeCommand>,
}
pub struct SysTreeCommand;
impl TSystemStageInfo for SysTreeCommand {
    
}
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
impl TSystemStageInfo for SysTransformNodeCommand {

}
#[setup]
impl SysTransformNodeCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleTransformNodeCommandList>,
        mut delete: EntityDelete<GameObject>,
        mut gtr_cmd: Commands<GameObject, GlobalTransform>,
        mut pos_cmd: Commands<GameObject, LocalPosition>,
        mut scl_cmd: Commands<GameObject, LocalScaling>,
        mut rot_cmd: Commands<GameObject, LocalRotation>,
        mut qua_cmd: Commands<GameObject, LocalRotationQuaternion>,
        mut eul_cmd: Commands<GameObject, LocalEulerAngles>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                TransformNodeCommand::Create(node) => {
                    pos_cmd.insert(node, LocalPosition(Vector3::new(0., 0., 0.)));
                    scl_cmd.insert(node, LocalScaling(Vector3::new(1., 1., 1.)));
                    rot_cmd.insert(node, LocalRotation(Rotation3::identity()));
                    qua_cmd.insert(node, LocalRotationQuaternion(Quaternion::identity()));
                    eul_cmd.insert(node, LocalEulerAngles(Vector3::new(0., 0., 0.)));
                    gtr_cmd.insert(node, GlobalTransform::default());
                },
                TransformNodeCommand::Destroy(node) => {
                    delete.despawn(node);
                },
                TransformNodeCommand::ModifyPosition(node, value) => {
                    pos_cmd.insert(node, LocalPosition(value));
                },
                TransformNodeCommand::ModifyRotation(node, value) => {
                    
                    let rotation = Rotation3::from_euler_angles(value.y, value.x, value.z);
                    let quaternion = Quaternion::from_rotation_matrix(&rotation); 
                    qua_cmd.insert(node, LocalRotationQuaternion(quaternion));
                    rot_cmd.insert(node, LocalRotation(rotation));
                    eul_cmd.insert(node, LocalEulerAngles(value));     
                },
                TransformNodeCommand::ModifyScaling(node, value) => {
                    scl_cmd.insert(node, LocalScaling(value));    
                },
                TransformNodeCommand::ModifyRotationQuaternion(node, value) => {
                    let rotation = value.to_rotation_matrix();
                    let mut euler = Vector3::new(0., 0., 0.);
                    let (z, x, y) = rotation.euler_angles();
                    euler.copy_from_slice(&[x, y, z]);

                    qua_cmd.insert(node, LocalRotationQuaternion(value));
                    rot_cmd.insert(node, LocalRotation(rotation));
                    eul_cmd.insert(node, LocalEulerAngles(euler));    
                },
                TransformNodeCommand::ModifyTarget(_, _) => todo!(),
            }
        });
    }
}