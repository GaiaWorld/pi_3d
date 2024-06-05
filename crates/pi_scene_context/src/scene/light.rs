use pi_scene_shell::prelude::*;

pub enum EMaxDirectLight {
    N004,
    N016,
    N064,
    N256,
    N512,
}

pub enum EMaxSpotLight {
    N004,
    N016,
    N064,
    N256,
    N512,
}

pub enum EMaxHemisphericLight {
    N004,
    N016,
    N064,
    N256,
    N512,
}

#[derive(Component, Default, Deref, DerefMut)]
pub struct DirectLights(pub Vec<Entity>);

#[derive(Component, Default, Deref, DerefMut)]
pub struct SpotLights(pub Vec<Entity>);

#[derive(Component, Default, Deref, DerefMut)]
pub struct HemisphericLights(pub Vec<Entity>);

