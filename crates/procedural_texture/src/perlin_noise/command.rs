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

use super::material::{PerlinNoiseMaterialPropertype, SinglePerlinNoiseMaterialBindDynInfoSet};

pub enum PerlinNoiseMaterialCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    Clear(),
    EmissiveSize(ObjectID, Number),
}
#[derive(Default)]
pub struct SingePerlinNoiseMaterialCommandList {
    pub list: Vec<PerlinNoiseMaterialCommand>,
}

pub struct SysPerlinNoiseMaterialCommand;

#[setup]
impl SysPerlinNoiseMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingePerlinNoiseMaterialCommandList>,
        mut materials: Query<GameObject, Write<PerlinNoiseMaterialPropertype>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut matrecord: ResMut<SinglePerlinNoiseMaterialBindDynInfoSet>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| match cmd {
            PerlinNoiseMaterialCommand::Create(entity) => match materials.get_mut(entity) {
                Some(mut mat) => {
                    println!("DefaultMaterialCommand Create");
                    mat.insert_no_notify(PerlinNoiseMaterialPropertype::new(&mut dynbuffer));
                    matrecord.add(MaterialID(entity));
                }
                None => {}
            },
            PerlinNoiseMaterialCommand::Destroy(entity) => {
                entity_delete.despawn(entity);
            }
            PerlinNoiseMaterialCommand::Clear() => {
                matrecord.list().drain(..).for_each(|entity| {
                    entity_delete.despawn(entity.0);
                });
            }
            PerlinNoiseMaterialCommand::EmissiveSize(entity, size) => {
                match materials.get_mut(entity) {
                    Some(mut mat) => match mat.get_mut() {
                        Some(mat) => {
                            mat.size = size;
                        }
                        None => todo!(),
                    },
                    None => {}
                }
            }
        });
    }
}
