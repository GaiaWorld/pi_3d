use pi_ecs::{prelude::Query, query::{Write, With}};
use pi_ecs_macros::setup;

use crate::object::GameObject;

#[derive(Debug)]
pub struct DirtyDefaultMaterialPropertype;

pub struct SysDirtyDefaultMaterialPropertype;
#[setup]
impl SysDirtyDefaultMaterialPropertype {
    #[system]
    pub fn tick(
        mut materials: Query<GameObject, Write<DirtyDefaultMaterialPropertype>, With<DirtyDefaultMaterialPropertype>>,
    ) {
        materials.iter_mut().for_each(|mut item| {
            item.remove();
        });
    }
}