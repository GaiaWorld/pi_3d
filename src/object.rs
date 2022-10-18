use pi_ecs::prelude::Id;

#[derive(Debug, Clone, Copy, Default)]
pub struct GameObject;

pub type ObjectID = Id<GameObject>;