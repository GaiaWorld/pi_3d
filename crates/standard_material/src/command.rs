
use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, EntityDelete}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::Number;

use pi_scene_context::{object::{ObjectID, GameObject}, materials::material::MaterialID, };

use super::{standard_material::{StandardMaterialPropertype, SingleStandardMaterialBindDynInfoSet}};


pub enum DefaultMaterialCommand {
    Create(ObjectID),
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
        mut materials: Query<GameObject, Write<StandardMaterialPropertype>>,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut matrecord: ResMut<SingleStandardMaterialBindDynInfoSet>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                DefaultMaterialCommand::Create(entity) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            //  log::debug!("DefaultMaterialCommand Create");
                            mat.write(StandardMaterialPropertype::new(&mut dynbuffer));
                            matrecord.add(MaterialID(entity));
                            mat.notify_modify();
                        },
                        None => {
                            
                        },
                    }
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