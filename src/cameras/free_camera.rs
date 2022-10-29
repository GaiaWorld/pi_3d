use pi_ecs::{prelude::{ResMut, Query}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::{Vector3, Number, Perspective3, Orthographic3, Matrix};

use crate::object::{ObjectID, GameObject};

use super::{camera::{CameraParam, CameraRenderData}, target_camera::TargetCameraParam};

#[derive(Debug, Clone, Copy)]
pub enum EFixedMode {
    VerticalFixed,
    HorizontalFixed,
}

#[derive(Debug, Clone, Copy)]
pub enum EFreeCameraMode {
    Perspective,
    Orthograhic,
}

pub struct FreeCameraParam {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub fov: f32,
    pub fixed_mode: EFixedMode,
    pub mode: EFreeCameraMode,
    pub orth_size: Number,
}

impl Default for FreeCameraParam {
    fn default() -> Self {
        Self { 
            orth_size: 5.,
            left: -4.,
            top: 4.,
            right: 4.,
            bottom: -4.,
            fov: Number::to_radians(60.0),
            fixed_mode: EFixedMode::VerticalFixed,
            mode: EFreeCameraMode::Orthograhic,
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

    ///
    /// * `aspect` width / height pixels ratio
    pub fn project_matrix(&self, camera: &CameraParam, c_p_m: &mut CameraRenderData, aspect: Number) {
        match self.mode {
            EFreeCameraMode::Perspective => {
                let fovy = match self.fixed_mode {
                    EFixedMode::VerticalFixed => self.fov,
                    EFixedMode::HorizontalFixed => self.fov / aspect,
                };
                let p = Perspective3::new(aspect, fovy, camera.minz, camera.maxz);
                c_p_m.project_matrix.copy_from(p.as_matrix());
            },
            EFreeCameraMode::Orthograhic => {
                let value = self.orth_size;
                
                let mut left = -value;
                let mut right = value;
                let mut top = value;
                let mut bottom = -value;
                match self.fixed_mode {
                    EFixedMode::VerticalFixed => {
                        left = -value * aspect;
                        right = value * aspect;
                        top = value;
                        bottom = -value;
                    },
                    EFixedMode::HorizontalFixed => {
                        left = -value;
                        right = value;
                        top = value / aspect;
                        bottom = -value / aspect;
                    },
                }
                let p = Orthographic3::new(0., right, 0., top, camera.minz, camera.maxz);
                c_p_m.project_matrix.copy_from(p.as_matrix());
            },
        };
    }
}

#[derive(Debug)]
pub enum FreeCameraCommand {
    Create(ObjectID),
    ModifyMode(ObjectID, EFreeCameraMode),
    ModifyFov(ObjectID, Number),
    ModifyFixedMode(ObjectID, EFixedMode),
    ModifyOrthSize(ObjectID, Number),
}

#[derive(Debug, Default)]
pub struct SingleFreeCameraCommandList {
    pub list: Vec<FreeCameraCommand>,
}

pub struct SysFreeCameraCommand;
#[setup]
impl SysFreeCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleFreeCameraCommandList>,
        mut cameras: Query<GameObject, Write<FreeCameraParam>>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                FreeCameraCommand::Create(entity) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            camera.insert_no_notify(FreeCameraParam::default());
                        },
                        None => todo!(),
                    }
                },
                FreeCameraCommand::ModifyMode(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(camera) => {
                            // camera.get_or_default().mode = value;
                        },
                        None => todo!(),
                    }
                },
                FreeCameraCommand::ModifyFov(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            // camera.get_or_default().fov = value;
                        },
                        None => todo!(),
                    }
                },
                FreeCameraCommand::ModifyFixedMode(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            // camera.get_or_default().fixed_mode = value;
                        },
                        None => todo!(),
                    }
                },
                FreeCameraCommand::ModifyOrthSize(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            // camera.get_or_default().orth_size = value;
                        },
                        None => todo!(),
                    }
                },
            }
        });

    }
}

pub struct FreeCameraProjectionCalc;
#[setup]
impl FreeCameraProjectionCalc {
    #[system]
    pub fn calc(
        mut query_cameras: Query<GameObject, (&CameraParam, &FreeCameraParam, &mut CameraRenderData)>,
    ) {
        println!("Projection Matrix Calc:");
        query_cameras.iter_mut().for_each(|(camera, free_camera, mut c_p_m)| {
            free_camera.project_matrix(camera, &mut c_p_m, 1.0);
            println!("{}", c_p_m.project_matrix);
        });
    }
}