
use pi_engine_shell::prelude::*;
use pi_scene_math::{coordiante_system::CoordinateSytem3, camera::{TOrthographicCameraTool, TPerspectiveCameraTool}, Isometry3, Vector3, vector::{TToolMatrix, TToolVector3}};

use crate::{
    viewer::prelude::*,
    transforms::prelude::*,
};

#[derive(Component)]
pub struct DirectionalShadowDirection(pub Vector3);
impl Default for DirectionalShadowDirection {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 1.))
    }
}
impl TViewerViewMatrix for DirectionalShadowDirection {
    fn view_matrix(&self, coordsys: &pi_scene_math::coordiante_system::CoordinateSytem3, local_pos: &LocalPosition, parent: Option<&GlobalTransform>) -> (ViewerViewMatrix, ViewerGlobalPosition) {
        let mut position = local_pos.0.clone();
        let target = position + self.0;
        let initial_focal_distance = (target - position).metric_distance(&Vector3::zeros());
        if (position.z - target.z).abs() < 0.0001 {
            position.z += 0.0001;
        }

        let mut refrence_point = Vector3::new(0., 0., 1.); refrence_point.scale_mut(initial_focal_distance);
        let mut cam_matrix = Isometry3::identity();
        coordsys.lookat(&position, &target, &CoordinateSytem3::up(), &mut cam_matrix);
        cam_matrix.inverse_mut();

        let rx = cam_matrix.rotation.to_rotation_matrix();
        let _transformed_reference_point = rx * refrence_point;
        
        let mut target = position + _transformed_reference_point;
        let mut up = rx * Vector3::new(0.0, 1.0, 0.0);
        
            match parent {
                Some(parent) => {
                    let transformation = &parent.matrix;
                    let mut eye = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(&position, transformation, &mut eye);
                    // log::warn!("local_pos: {:?}", local_pos);
                    // log::warn!("eye: {:?}", eye);

                    // let mut target = local_pos.0 + target;
                    // if self.target.normalize().dot(&self.up).abs() == 1. {
                    //     target += Vector3::new(0., 0., 0.001);
                    // }
                    CoordinateSytem3::transform_coordinates(&target.clone(), transformation, &mut target);
                    // log::warn!("target: {:?}", target);


                    // let mut up = Vector3::zeros();
                    CoordinateSytem3::transform_coordinates(&up.clone(), transformation, &mut up);
                    // log::warn!("up: {:?}", up);

                    let mut iso = Isometry3::identity();
                    coordsys.lookat(&eye, &target, &up, &mut iso);
                    // iso.translation.clone_from(&Translation3::new(eye.x, eye.y, eye.z));

                    (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
                },
                None => {
                    let mut iso = Isometry3::identity();
                    let eye = position.clone();
                    // let mut target = local_pos.0 + self.target;
                    // if self.target.normalize().dot(&self.up).abs() == 1. {
                    //     target += Vector3::new(0., 0., 0.001);
                    // }
                    coordsys.lookat(&eye, &target, &up, &mut iso);

                    // iso.translation.clone_from(&Translation3::new(local_pos.0.x, local_pos.0.y, local_pos.0.z));
                    
                    (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
                },
            }
    }
}


#[derive(Component)]
pub struct DirectionalShadowProjection {
    pub minz: f32,
    pub maxz: f32,
    pub frustum_size: f32,
}
impl Default for DirectionalShadowProjection {
    fn default() -> Self {
        Self { minz: 0.0, maxz: 10., frustum_size: 40. }
    }
}
impl TViewerProjectMatrix for DirectionalShadowProjection {
    fn project_matrix(&self, _ratio: f32) -> ViewerProjectionMatrix {
        let value = self.frustum_size;
        let m = CoordinateSytem3::orthographic_lh(-value, value, -value, value, self.minz, self.maxz);
        ViewerProjectionMatrix(m)
    }
}

#[derive(Component)]
pub struct SpotShadowProjection {
    pub minz: f32,
    pub maxz: f32,
    pub fov: f32,
}
impl Default for SpotShadowProjection {
    fn default() -> Self {
        Self { minz: 0.0, maxz: 20., fov: 0.5 }
    }
}
impl TViewerProjectMatrix for SpotShadowProjection {
    fn project_matrix(&self, _ratio: f32) -> ViewerProjectionMatrix {
        let m = CoordinateSytem3::perspective_lh(self.fov, 1., self.minz, self.maxz, false);
        ViewerProjectionMatrix(m)
    }
}