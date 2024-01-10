use bevy::prelude::*;
use pi_scene_shell::prelude::ActionList;
use pi_scene_math::Vector3;

use crate::{
    transforms::prelude::GlobalMatrix,
    cullings::prelude::SceneBoundingPool,
    viewer::prelude::ViewerTransformMatrix,
};

pub struct RayTest(pub Entity, pub Entity, pub f32, pub f32);
pub type ActionListRayTest = ActionList<RayTest>;

#[derive(Resource, Default, Debug)]
pub struct RayTestID(Option<Entity>);

pub struct PluginRayTest;
impl Plugin for PluginRayTest {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListRayTest::default());
        app.insert_resource(RayTestID::default());
        app.add_systems(Update, sys_ray_test);
    }
}

pub fn sys_ray_test(
    mut rays: ResMut<ActionListRayTest>,
    mut res: ResMut<RayTestID>,
    transforms: Query<&GlobalMatrix>,
    scenes: Query<&ViewerTransformMatrix>,
    pool: Query<&SceneBoundingPool>,
) {
    // println!("=======111111");
    rays.drain()
        .drain(..)
        .for_each(|RayTest(scene_id, entity, x, y)| {
            // println!("=======22222");
            if let Ok(world_matrix) = transforms.get(entity) {
                let world_matrix = &world_matrix.matrix;
                // println!("=======33333");
                let x = (x - 0.5) * 2.;
                let y = (y - 0.5) * 2.;
                let org = Vector3::from(world_matrix.fixed_view::<3, 1>(0, 3));

                if let Ok(viewer) = scenes.get(entity) {
                    // println!("=======44444");
                    let pos = if let Some(inverse) = viewer.0.try_inverse() {
                        inverse.transform_vector(&Vector3::new(x, y, 0.))
                    } else {
                        viewer.0.transform_vector(&Vector3::new(x, y, 0.))
                    };
                    let dir = pos - org;
                    let mut result = None;
                    if let Ok(pool) = pool.get(scene_id) {
                        // println!("========55555");
                        pool.ray_test(org, dir, &mut result);
                        res.0 = result;
                    }
                }
            }
        });
}
