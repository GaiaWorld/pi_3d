use pi_ecs::{
    prelude::{EntityDelete, Query, ResMut},
    query::Write,
};
use pi_ecs_macros::setup;
use pi_scene_math::Number;

use pi_scene_context::{
    materials::material::MaterialID,
    object::{GameObject, ObjectID},
    resources::RenderDynUniformBuffer,
};

use super::material::{CloudMaterialPropertype, SingleCloudMaterialBindDynInfoSet};

pub enum CloudMaterialCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    Clear(),
    EmissiveSkyColor(ObjectID, (Number, Number, Number, Number)),
    EmissiveCloudColor(ObjectID, (Number, Number, Number, Number)),
    EmissiveAmplitude(ObjectID, Number),
    EmissiveOctaves(ObjectID, Number),
}
#[derive(Default)]
pub struct SingeCloudMaterialCommandList {
    pub list: Vec<CloudMaterialCommand>,
}

pub struct SysCloudMaterialCommand;

#[setup]
impl SysCloudMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingeCloudMaterialCommandList>,
        mut materials: Query<GameObject, Write<CloudMaterialPropertype>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut matrecord: ResMut<SingleCloudMaterialBindDynInfoSet>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| match cmd {
            CloudMaterialCommand::Create(entity) => match materials.get_mut(entity) {
                Some(mut mat) => {
                    println!("DefaultMaterialCommand Create");
                    mat.insert_no_notify(CloudMaterialPropertype::new(&mut dynbuffer));
                    matrecord.add(MaterialID(entity));
                }
                None => {}
            },
            CloudMaterialCommand::Destroy(entity) => {
                entity_delete.despawn(entity);
            }
            CloudMaterialCommand::Clear() => {
                matrecord.list().drain(..).for_each(|entity| {
                    entity_delete.despawn(entity.0);
                });
            }
            CloudMaterialCommand::EmissiveSkyColor(entity, color) => {
                match materials.get_mut(entity) {
                    Some(mut mat) => match mat.get_mut() {
                        Some(mat) => {
                            mat.sky_color = color;
                        }
                        None => todo!(),
                    },
                    None => {}
                }
            }
            CloudMaterialCommand::EmissiveCloudColor(entity, color) => {
                match materials.get_mut(entity) {
                    Some(mut mat) => match mat.get_mut() {
                        Some(mat) => {
                            mat.cloud_color = color;
                        }
                        None => todo!(),
                    },
                    None => {}
                }
            }
            CloudMaterialCommand::EmissiveAmplitude(entity, amplitude) => {
                match materials.get_mut(entity) {
                    Some(mut mat) => match mat.get_mut() {
                        Some(mat) => {
                            mat.amplitude = amplitude;
                        }
                        None => todo!(),
                    },
                    None => {}
                }
            }
            CloudMaterialCommand::EmissiveOctaves(entity, octaves) => {
                match materials.get_mut(entity) {
                    Some(mut mat) => match mat.get_mut() {
                        Some(mat) => {
                            mat.num_octaves = octaves;
                        }
                        None => todo!(),
                    },
                    None => {}
                }
            }
        });
    }
}
