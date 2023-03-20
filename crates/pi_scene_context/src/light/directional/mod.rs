use pi_scene_math::{coordiante_system::CoordinateSytem3, camera::TOrthographicCameraTool};

use crate::viewer::{TViewerProjectMatrix, ViewerProjectionMatrix};

pub mod system;

pub struct DirectionalShadowProjection {
    minz: f32,
    maxz: f32,
    frustum_size: f32,
}
impl TViewerProjectMatrix for DirectionalShadowProjection {
    fn project_matrix(&self, ratio: f32) -> crate::viewer::ViewerProjectionMatrix {
        let value = self.frustum_size;
        let m = CoordinateSytem3::orthographic_lh(-value, value, -value, value, self.minz, self.maxz);
        ViewerProjectionMatrix(m)
    }
}