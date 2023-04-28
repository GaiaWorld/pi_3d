use pi_scene_math::coordiante_system;
use pi_engine_shell::prelude::*;

#[derive(Deref, DerefMut, Component)]
pub struct SceneCoordinateSytem3D(pub coordiante_system::CoordinateSytem3);
impl Default for SceneCoordinateSytem3D {
    fn default() -> Self {
        Self(coordiante_system::CoordinateSytem3::default())
    }
}

#[derive(Debug, Clone, Copy, Component)]
pub enum ESceneCoordinateMode {
    Left,
    Right,
}