use pi_ecs::{prelude::{ResMut, Query}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::{Vector3, Number, Perspective3, Orthographic3, Matrix};

use crate::object::{ObjectID, GameObject};


pub struct FreeCameraParam {
}

impl Default for FreeCameraParam {
    fn default() -> Self {
        Self {
        }
    }
}

impl FreeCameraParam {
    const P0: Vector3 = Vector3::new(-1., -1., 0.);
    const P1: Vector3 = Vector3::new(-1., -1., 0.);
    const P2: Vector3 = Vector3::new(-1., -1., 0.);
    const P3: Vector3 = Vector3::new(-1., -1., 0.);
    const P4: Vector3 = Vector3::new(-1., -1., 0.);
    const P5: Vector3 = Vector3::new(-1., -1., 0.);
    const P6: Vector3 = Vector3::new(-1., -1., 0.);
    const P7: Vector3 = Vector3::new(-1., -1., 0.);

}
