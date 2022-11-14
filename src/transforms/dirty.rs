
use pi_ecs::{prelude::Query, query::{Write, With}};
use pi_ecs_macros::setup;

use crate::object::GameObject;

#[derive(Debug, Default)]
pub struct DirtyLocalTransform;

#[derive(Debug, Default)]
pub struct DirtyGlobalTransform;

pub struct SysDirtyTransformNodeTick;
#[setup]
impl SysDirtyTransformNodeTick {
    #[system]
    pub fn tick(
        mut query_local: Query<GameObject, (Write<DirtyLocalTransform>), With<DirtyLocalTransform>>,
        mut query_global: Query<GameObject, (Write<DirtyGlobalTransform>), With<DirtyGlobalTransform>>,
    ) {
        query_local.iter_mut().for_each(|(mut local)| {
            local.remove();
            // println!("DirtyLocalTransform Remove >>>>>>>>>> ");
        });
        query_global.iter_mut().for_each(|(mut global)| {
            global.remove();
        });
    }
}
