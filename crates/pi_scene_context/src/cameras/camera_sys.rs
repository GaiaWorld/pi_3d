use pi_ecs::{prelude::{Query, Commands}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
use pi_scene_math::{coordiante_system::CoordinateSytem3, Rotation3};

use crate::{transforms::{transform_node::{LocalPosition, LocalRotation}}, object::{GameObject, ObjectID}, cameras::{target_camera::TargetCameraParam}};


pub struct TargetCameraEffectLocalRotation;
impl TSystemStageInfo for TargetCameraEffectLocalRotation {
}
#[setup]
impl TargetCameraEffectLocalRotation {
    #[system]
    pub fn calc(
        query_cameras: Query<GameObject, (ObjectID, &TargetCameraParam, &LocalPosition)>,
        mut rot_cmd: Commands<GameObject, LocalRotation>,
    ) {
        //  log::debug!("Target Camera Control Calc:");
        let coordsys = CoordinateSytem3::left();
        query_cameras.iter().for_each(|(obj, target_camera, lposition)| {
            let mut rotation = Rotation3::identity();
            target_camera.calc_rotation(&coordsys, &lposition.0, &mut rotation);
            rot_cmd.insert(obj, LocalRotation(rotation));
        });
    }
}