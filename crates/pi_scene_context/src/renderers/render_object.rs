use pi_scene_shell::prelude::*;
use crate::{
    object::{ObjectID},
};

#[derive(Debug, Clone, Copy, Component)]
pub struct RendererID(pub ObjectID);