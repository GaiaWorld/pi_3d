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

#[derive(Debug)]
pub enum FreeCameraCommand {
    Create(ObjectID),
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
            }
        });

    }
}
