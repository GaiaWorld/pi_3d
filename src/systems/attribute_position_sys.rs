use pi_ecs::prelude::Query;
use pi_ecs_macros::setup;

use crate::{object::GameObject, shaders::buildin_attributes::BuildinAttributePosition, flags::CullingFlag};


pub struct BuildinAttributePositionTick;
#[setup]
impl BuildinAttributePositionTick {
    #[system]
    pub fn tick(
        query_positions: Query<GameObject, (&BuildinAttributePosition, &CullingFlag)>,
    ) {
        
    }
}