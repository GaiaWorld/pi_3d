use pi_scene_math::{coordiante_system::CoordinateSytem3, camera::TOrthographicCameraTool};
use pi_engine_shell::prelude::*;

use crate::viewer::prelude::*;

pub mod system;


#[derive(Component)]
pub struct DirectionalShadowProjection {
    minz: f32,
    maxz: f32,
    frustum_size: f32,
}
impl TViewerProjectMatrix for DirectionalShadowProjection {
    fn project_matrix(&self, _ratio: f32) -> ViewerProjectionMatrix {
        let value = self.frustum_size;
        let m = CoordinateSytem3::orthographic_lh(-value, value, -value, value, self.minz, self.maxz);
        ViewerProjectionMatrix(m)
    }
}