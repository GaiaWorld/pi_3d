use pi_scene_math::coordiante_system;
use pi_scene_shell::prelude::*;

#[derive(Deref, DerefMut, Component)]
pub struct SceneCoordinateSytem3D(pub coordiante_system::CoordinateSytem3);
impl Default for SceneCoordinateSytem3D {
    fn default() -> Self {
        Self(coordiante_system::CoordinateSytem3::default())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ESceneCoordinateMode {
    Left,
    Right,
}