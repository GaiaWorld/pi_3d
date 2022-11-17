use pi_ecs::prelude::Query;
use pi_ecs_macros::setup;

use crate::{object::GameObject, };

use super::MainCameraRenderer;

pub struct DrawSortTick;
#[setup]
impl DrawSortTick {
    #[system]
    pub fn tick(
        mut main_camera_renderers: Query<GameObject, &mut MainCameraRenderer>,
    ) {
        // println!("Draw Sort Tick");
        // main_camera_renderers.iter_mut().for_each(|mut item| {
        //     item.opaque_draws.draws.sort();
        //     item.skybox_draws.draws.sort();
        //     item.transparent_draws.draws.sort();
        // });
    }
}