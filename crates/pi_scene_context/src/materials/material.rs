
use pi_ecs::{prelude::{Id, ResMut, Query}, query::Write};
use pi_ecs_macros::setup;

use crate::{object::{ObjectID, GameObject}, renderers::render_mode::ERenderMode};

pub trait TMaterial {
    fn render_mode(&self) -> ERenderMode;
}

///
/// 材质单独与 GameObject 关联
/// Mesh 使用

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MaterialID (pub ObjectID);

#[derive(Debug)]
pub enum MaterialIDCommand {
    Use(ObjectID, MaterialID),
    UnUse(ObjectID, MaterialID),
}

#[derive(Debug, Default)]
pub struct SingleMaterialIDCommandList {
    pub list: Vec<MaterialIDCommand>,
}

pub struct SysMaterialIDCommand;
#[setup]
impl SysMaterialIDCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleMaterialIDCommandList>,
        mut materials: Query<GameObject, Write<MaterialID>>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                MaterialIDCommand::Use(mat, id) => {
                    match materials.get_mut(mat) {
                        Some(mut mat) => {
                            mat.write(id);
                        },
                        None => todo!(),
                    }
                },
                MaterialIDCommand::UnUse(mat, id) => {
                    
                },
            }
        });
    }
}

pub trait InterfaceMaterial {
    fn use_material(
        & self,
        object: ObjectID,
        material: MaterialID,
    );
}

impl InterfaceMaterial for crate::engine::Engine {
    fn use_material(
        & self,
        object: ObjectID,
        material: MaterialID,
    ) {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleMaterialIDCommandList>().unwrap();
        commands.list.push(MaterialIDCommand::Use(object, material));
    }
}