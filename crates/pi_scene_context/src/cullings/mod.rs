

use pi_engine_shell::prelude::*;
use pi_scene_math::Vector3;

use crate::{transforms::transform_node_sys::sys_world_matrix_calc, prelude::{StageTransform, StageModel}, viewer::prelude::sys_abstructmesh_culling_flag_reset};

use self::{bounding_box::BoundingBox, bounding_sphere::BoundingSphere, sys::{sys_update_culling_by_worldmatrix, sys_update_culling_by_cullinginfo}, command::{ActionListMeshBounding, ActionListMeshBoundingCullingMode}, command_sys::{sys_act_mesh_bounding, sys_act_mesh_bounding_culling}};

mod bounding_box;
mod bounding_sphere;
mod sys;
mod bounding;
mod oct_tree;
mod quad_tree;
mod base;
mod command;
mod command_sys;
mod ray_test;

pub mod prelude;



#[derive(Debug, Clone, Component)]
pub struct IsCulled;

pub trait TIntersect {
    fn intersects_point(&self, p: &Vector3) -> bool;
    fn intersects_box(&self, b: &BoundingBox) -> bool;
    fn intersects_sphere(&self, s: &BoundingSphere) -> bool;
    fn intersects_min_max(&self, min: &Vector3, max: &Vector3) -> bool;
}

pub struct PluginCulling;
impl Plugin for PluginCulling {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListMeshBounding::default());
        app.insert_resource(ActionListMeshBoundingCullingMode::default());

        app.add_systems(Update, (
            sys_act_mesh_bounding,
            sys_act_mesh_bounding_culling
        ).in_set(StageModel::AbstructMeshCommand));

        app.add_systems(
            Update,
            (
                sys_update_culling_by_worldmatrix,
                sys_update_culling_by_cullinginfo,
                sys_abstructmesh_culling_flag_reset,
            ).chain().after(sys_world_matrix_calc).in_set(StageTransform::TransformCalcMatrix)
        );
    }
}

pub trait InterfaceBoundingInfo {
    fn set_bounding(
        &mut self,
        object: ObjectID,
        min: Vector3,
        max: Vector3,
    );
}

// impl InterfaceBoundingInfo for crate::engine::Engine {
//     fn set_bounding(
//         &mut self,
//         object: ObjectID,
//         min: Vector3,
//         max: Vector3,
//     ) {
//         let world = self.world();

//         let commands = world.get_resource_mut::<SingleCullingCommandList>().unwrap();
//         commands.list.push(CullingCommand::Bounding(object, min, max));
//     }
// }

// pub trait InterfaceBoundingInfo<T>
//     where T: TEngine
// {
//     fn set_bounding(
//         engine: &T,
//         object: ObjectID,
//         min: Vector3,
//         max: Vector3,
//     ) -> &T {
//         let world = engine.world();

//         let commands = world.get_resource_mut::<SingleCullingCommandList>().unwrap();
//         commands.list.push(CullingCommand::Bounding(object, min, max));
        
//         engine
//     }
// }