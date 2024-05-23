use pi_scene_shell::prelude::*;
use crate::{
    object::{ObjectID},
};

#[derive(Clone, Copy, Component, Default)]
pub struct RendererID(pub ObjectID);