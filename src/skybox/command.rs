use pi_ecs::{prelude::{ResMut, Query, EntityDelete}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::Number;

use crate::{object::{ObjectID, GameObject}, resources::RenderDynUniformBuffer, materials::material::MaterialID};

use super::material::{SkyboxMaterialPropertype, SingleSkyboxMaterialBindDynInfoSet};

pub enum SkyboxMaterialCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    Clear(),
    EmissiveColor(ObjectID, Number, Number, Number),
    EmissiveIntensity(ObjectID, Number),
}
#[derive(Default)]
pub struct SingeSkyboxMaterialCommandList {
    pub list: Vec<SkyboxMaterialCommand>,
}

pub struct SysSkyboxMaterialCommand;

#[setup]
impl SysSkyboxMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingeSkyboxMaterialCommandList>,
        mut materials: Query<GameObject, Write<SkyboxMaterialPropertype>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut matrecord: ResMut<SingleSkyboxMaterialBindDynInfoSet>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                SkyboxMaterialCommand::Create(entity) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            println!("DefaultMaterialCommand Create");
                            mat.insert_no_notify(SkyboxMaterialPropertype::new(&mut dynbuffer));
                            matrecord.add(MaterialID(entity));
                        },
                        None => {
                            
                        },
                    }
                },
                SkyboxMaterialCommand::Destroy(entity) => {
                    entity_delete.despawn(entity);
                },
                SkyboxMaterialCommand::Clear() => {
                    matrecord.list().drain(..).for_each(|entity| {
                        entity_delete.despawn(entity.0);
                    });
                },
                SkyboxMaterialCommand::EmissiveColor(entity, r, g, b) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            match mat.get_mut() {
                                Some(mat) => {
                                    mat.emissive_color = (r, g, b);
                                },
                                None => todo!(),
                            }
                        },
                        None => {
                            
                        },
                    }
                },
                SkyboxMaterialCommand::EmissiveIntensity(entity, intensity) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            match mat.get_mut() {
                                Some(mat) => {
                                    mat.emissive_intensity = intensity;
                                },
                                None => todo!(),
                            }
                        },
                        None => {
                            
                        },
                    }
                },
            }
        });
    }
}