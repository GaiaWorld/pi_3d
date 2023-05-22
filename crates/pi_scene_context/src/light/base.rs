
use pi_engine_shell::prelude::*;
use pi_scene_math::{Vector3, Isometry3, coordiante_system::CoordinateSytem3, vector::{TToolMatrix, TToolVector3}};

use crate::{
    viewer::prelude::*,
    transforms::prelude::*,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Component)]
pub enum Light {
    Directional,
    Point,
    Spot,
    Hemispheric,
}

#[derive(Component)]
pub struct DirectLight;

#[derive(Component)]
pub struct SpotLight;

#[derive(Component)]
pub struct HemisphericLight;


#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Component)]
pub enum LightingMode {
    Lambert,
    PBR,
}

#[derive(Component)]
pub struct LightDirection(pub Vector3);
impl Default for LightDirection {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 1.))
    }
}
impl TViewerViewMatrix for LightDirection {
    fn view_matrix(&self, coordsys: &pi_scene_math::coordiante_system::CoordinateSytem3, local_pos: &LocalPosition, parent: Option<&mut GlobalTransform>) -> (ViewerViewMatrix, ViewerGlobalPosition) {

        match parent {
            Some(parent) => {
                let transformation = &parent.matrix;
                let mut eye = Vector3::zeros();
                CoordinateSytem3::transform_coordinates(&local_pos.0, transformation, &mut eye);
                // log::warn!("local_pos: {:?}", local_pos);
                // log::warn!("eye: {:?}", eye);

                let mut target = local_pos.0 + self.0;
                if self.0.normalize().dot(&Vector3::new(0., 1., 0.)).abs() == 1. {
                    target += Vector3::new(0., 0., 0.001);
                }
                CoordinateSytem3::transform_coordinates(&target.clone(), transformation, &mut target);
                // log::warn!("target: {:?}", target);


                let mut up = Vector3::zeros();
                CoordinateSytem3::transform_coordinates(&Vector3::new(0., 1., 0.), transformation, &mut up);
                // log::warn!("up: {:?}", up);

                let mut iso = Isometry3::identity();
                coordsys.lookat(&eye, &target, &up, &mut iso);
                // iso.translation.clone_from(&Translation3::new(eye.x, eye.y, eye.z));

                (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
            },
            None => {
                let mut iso = Isometry3::identity();
                let eye = local_pos.0.clone();
                let mut target = local_pos.0 + self.0;
                let up = Vector3::new(0., 1., 0.);
                if self.0.normalize().dot(&up).abs() == 1. {
                    target += Vector3::new(0., 0., 0.001);
                }

                coordsys.lookat(&eye, &target, &Vector3::new(0., 1., 0.), &mut iso);

                // iso.translation.clone_from(&Translation3::new(local_pos.0.x, local_pos.0.y, local_pos.0.z));
                
                log::warn!("iso: {:?}", iso);
                (ViewerViewMatrix(iso.to_matrix()), ViewerGlobalPosition(eye))
            },
        }
    }
}

