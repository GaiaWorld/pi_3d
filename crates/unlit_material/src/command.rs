
use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, EntityDelete, Res}, query::Write, storage::Local};
use pi_ecs_macros::setup;
use pi_render::rhi::device::RenderDevice;
use pi_scene_math::Number;

use pi_scene_context::{object::{ObjectID, GameObject}, materials::{material::MaterialID, bind_group::{RenderBindGroupPool, RenderBindGroupKey}}, resources::RenderDynUniformBuffer};

use crate::{define::UnlitMaterialDefines, bind_group::{UnlitMaterialBindGroup, SingleUnlitBindGroupList}};

use super::{unlit_material::{UnlitMaterialPropertype, SingleUnlitMaterialBindDynInfoSet}};

// pub(crate) static mut RES_ID_COMMAND_LIST: Option<Local> = None;

pub enum UnlitMaterialCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    Clear(),
    BaseColor(ObjectID, Number, Number, Number),
    Opacity(ObjectID, Number),
}
#[derive(Default)]
pub struct SingleUnlitMaterialCommandList {
    pub list: Vec<UnlitMaterialCommand>,
}

pub struct SysUnlitMaterialCommand;
#[setup]
impl SysUnlitMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleUnlitMaterialCommandList>,
        mut materials: Query<GameObject, Write<UnlitMaterialPropertype>>,
        mut material_defines: Query<GameObject, Write<UnlitMaterialDefines>>,
        mut material_bindgroup_values: Query<GameObject, Write<UnlitMaterialBindGroup>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut matrecord: ResMut<SingleUnlitMaterialBindDynInfoSet>,
        mut entity_delete: EntityDelete<GameObject>,
        mut unlit_bindgroup: ResMut<SingleUnlitBindGroupList>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                UnlitMaterialCommand::Create(entity) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            //  println!("DefaultMaterialCommand Create");
                            mat.write(UnlitMaterialPropertype::new(&mut dynbuffer));
                            matrecord.add(MaterialID(entity));
                            mat.notify_modify();
                        },
                        None => {
                            
                        },
                    }
                    
                    match material_defines.get_mut(entity) {
                        Some(mut defines) => {
                            defines.write(UnlitMaterialDefines::default())
                        },
                        None => {

                        }
                    }
                    match material_bindgroup_values.get_mut(entity) {
                        Some(mut item) => {
                            match unlit_bindgroup.value {
                                None => {
                                    let group = RenderBindGroupKey::from(UnlitMaterialBindGroup::LABEL);
                                    bindgroups.creat(&device, group.clone(), UnlitMaterialBindGroup::layout_entries().as_slice(), UnlitMaterialBindGroup::SET);
                                    unlit_bindgroup.value = Some(group);
                                },
                                _ => {}
                            }
                            item.write(UnlitMaterialBindGroup(unlit_bindgroup.value.as_ref().unwrap().clone()));
                        },
                        None => {

                        }
                    }
                },
                UnlitMaterialCommand::Destroy(entity) => {
                    entity_delete.despawn(entity);
                },
                UnlitMaterialCommand::Clear() => {
                    matrecord.list().drain(..).for_each(|entity| {
                        entity_delete.despawn(entity.0);
                    });
                },
                UnlitMaterialCommand::BaseColor(entity, r, g, b) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            match mat.get_mut() {
                                Some(mat) => {
                                    mat.base_color = (r, g, b);
                                },
                                None => todo!(),
                            };
                            mat.notify_modify();
                        },
                        None => {
                            
                        },
                    }
                },
                UnlitMaterialCommand::Opacity(entity, intensity) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            match mat.get_mut() {
                                Some(mat) => {
                                    mat.opacity = intensity;
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

// pub enum EMainTextureCommand;