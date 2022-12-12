use std::time::Instant;

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

use super::material::{WaterMaterialPropertype, SingleWaterMaterialBindDynInfoSet};

pub enum WaterMaterialCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    Clear(),
    EmissiveSeaBase(ObjectID, (Number, Number, Number, Number)),
    EmissiveSeaWaterColor(ObjectID, (Number, Number, Number, Number))
}
#[derive(Default)]
pub struct SingeWaterMaterialCommandList {
    pub list: Vec<WaterMaterialCommand>,
}

pub struct SysWaterMaterialCommand;

#[setup]
impl SysWaterMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingeWaterMaterialCommandList>,
        mut materials: Query<GameObject, Write<WaterMaterialPropertype>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut matrecord: ResMut<SingleWaterMaterialBindDynInfoSet>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| match cmd {
            WaterMaterialCommand::Create(entity) => match materials.get_mut(entity) {
                Some(mut mat) => {
                    println!("DefaultMaterialCommand Create");
                    mat.insert_no_notify(WaterMaterialPropertype::new(&mut dynbuffer));
                    matrecord.add(MaterialID(entity));
                }
                None => {}
            },
            WaterMaterialCommand::Destroy(entity) => {
                entity_delete.despawn(entity);
            }
            WaterMaterialCommand::Clear() => {
                matrecord.list().drain(..).for_each(|entity| {
                    entity_delete.despawn(entity.0);
                });
            }
            WaterMaterialCommand::EmissiveSeaBase(entity, color) => {
                match materials.get_mut(entity) {
                    Some(mut mat) => match mat.get_mut() {
                        Some(mat) => {
                            mat.sea_base = color;
                        }
                        None => todo!(),
                    },
                    None => {}
                }
            }
            WaterMaterialCommand::EmissiveSeaWaterColor(entity, color) => {
                match materials.get_mut(entity) {
                    Some(mut mat) => match mat.get_mut() {
                        Some(mat) => {
                            mat.sea_water_color = color;
                        }
                        None => todo!(),
                    },
                    None => {}
                }
            }
        });
    }
}
