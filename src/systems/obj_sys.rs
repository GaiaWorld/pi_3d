use pi_ecs_macros::{listen, setup};
use pi_ecs::{prelude::Query, query::With};
use pi_ecs_utils::prelude::EntityTree;
use pi_scene_math::Matrix;
use pi_slotmap_tree::Storage;

use crate::{transforms::transform_node::TransformNode, object::{GameObject, ObjectID}, scene::SceneParam, flags::SceneID01};


pub fn obj_transform_billboard_cacl(
    transform: &mut TransformNode,
) {

}