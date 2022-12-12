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

use crate::material::{SingleSkeletonsBindDynInfoSet, SkeletonsPropertype};

pub enum SkeletonsCommand {
    Create(ObjectID),
    Destroy(ObjectID),
}
#[derive(Default)]
pub struct SingeSkeletonsCommandList {
    pub list: Vec<SkeletonsCommand>,
}

pub struct SysSkeletonsCommand;

#[setup]
impl SysSkeletonsCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingeSkeletonsCommandList>,
        mut materials: Query<GameObject, Write<SkeletonsPropertype>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut matrecord: ResMut<SingleSkeletonsBindDynInfoSet>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| match cmd {
            SkeletonsCommand::Create(entity) => match materials.get_mut(entity) {
                Some(mut mat) => {
                    println!("DefaultMaterialCommand Create");
                    mat.insert_no_notify(SkeletonsPropertype::new(&mut dynbuffer));
                    matrecord.add(MaterialID(entity));
                }
                None => {}
            },
            SkeletonsCommand::Destroy(entity) => {
                entity_delete.despawn(entity);
            }
        });
    }
}
