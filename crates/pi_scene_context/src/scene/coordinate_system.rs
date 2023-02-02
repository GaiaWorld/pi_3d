use pi_scene_math::coordiante_system;

pub type SceneCoordinateSytem = coordiante_system::CoordinateSytem3;

#[derive(Debug, Clone, Copy)]
pub enum ESceneCoordinateMode {
    Left,
    Right,
}