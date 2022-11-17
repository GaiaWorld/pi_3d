
use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, EntityDelete}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::Number;

use pi_scene_context::{object::{ObjectID, GameObject}, materials::material::MaterialID, resources::RenderDynUniformBuffer};

use super::{default_material::{DefaultMaterialPropertype, SingleDefaultMaterialBindDynInfoSet}};


pub enum DefaultMaterialCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    Clear(),
    EmissiveColor(ObjectID, Number, Number, Number),
    EmissiveIntensity(ObjectID, Number),
}
#[derive(Default)]
pub struct SingeDefaultMaterialCommandList {
    pub list: Vec<DefaultMaterialCommand>,
}
pub struct SysDefaultMaterialCommand;
#[setup]
impl SysDefaultMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingeDefaultMaterialCommandList>,
        mut materials: Query<GameObject, Write<DefaultMaterialPropertype>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut matrecord: ResMut<SingleDefaultMaterialBindDynInfoSet>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                DefaultMaterialCommand::Create(entity) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            //  println!("DefaultMaterialCommand Create");
                            mat.write(DefaultMaterialPropertype::new(&mut dynbuffer));
                            matrecord.add(MaterialID(entity));
                            mat.notify_modify();
                        },
                        None => {
                            
                        },
                    }
                },
                DefaultMaterialCommand::Destroy(entity) => {
                    entity_delete.despawn(entity);
                },
                DefaultMaterialCommand::Clear() => {
                    matrecord.list().drain(..).for_each(|entity| {
                        entity_delete.despawn(entity.0);
                    });
                },
                DefaultMaterialCommand::EmissiveColor(entity, r, g, b) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            match mat.get_mut() {
                                Some(mat) => {
                                    mat.emissive_color = (r, g, b);
                                },
                                None => todo!(),
                            };
                            mat.notify_modify();
                        },
                        None => {
                            
                        },
                    }
                },
                DefaultMaterialCommand::EmissiveIntensity(entity, intensity) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            match mat.get_mut() {
                                Some(mat) => {
                                    mat.emissive_intensity = intensity;
                                },
                                None => todo!(),
                            };
                            mat.notify_modify();
                        },
                        None => {
                            
                        },
                    }
                },
            }
        });
    }
}