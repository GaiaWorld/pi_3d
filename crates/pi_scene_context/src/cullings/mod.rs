

use pi_scene_shell::prelude::*;
use pi_scene_math::Vector3;

use crate::{prelude::StageModel, viewer::prelude::sys_abstructmesh_culling_flag_reset, scene::StageScene, materials::prelude::StageMaterial};

use self::{bounding_box::BoundingBox, bounding_sphere::BoundingSphere, sys::*, command::*, command_sys::*};

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


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageCulling {
    Command,
    CalcBounding,
}

#[derive(Clone, Component, Default)]
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
        app.insert_resource(ActionListBoundingBoxDisplay::default());

#[cfg(feature = "use_bevy")]
        app.configure_sets(Update, 
            (
                StageCulling::Command.after(StageScene::_Create).before(StageMaterial::Command),
                StageCulling::CalcBounding.after(StageModel::RenderMatrix),
            )
        );

#[cfg(feature = "use_bevy")]
        app.add_systems(
            Update, 
            (
                sys_act_mesh_bounding_culling_display.in_set(StageCulling::Command),
                sys_act_mesh_bounding.in_set(StageModel::AbstructMeshCommand),
                (
                    sys_update_culling_by_worldmatrix,
                    sys_update_culling_by_cullinginfo,
                    sys_abstructmesh_culling_flag_reset,
                ).chain().in_set(StageCulling::CalcBounding)
            )
        );

#[cfg(not(feature = "use_bevy"))]
        app
        .configure_set(Update, StageCulling::Command.after(StageScene::_Create).before(StageMaterial::Command))
        .configure_set(Update, StageCulling::CalcBounding.after(StageModel::RenderMatrix))
        ;

#[cfg(not(feature = "use_bevy"))]
        app
        .add_systems(Update, sys_act_mesh_bounding_culling_display   .in_set(StageCulling::Command))
        .add_systems(Update, sys_act_mesh_bounding                   .in_set(StageModel::AbstructMeshCommand))
        .add_systems(Update, sys_update_culling_by_worldmatrix                                                              .in_set(StageCulling::CalcBounding))
        .add_systems(Update, sys_update_culling_by_cullinginfo           .after(sys_update_culling_by_worldmatrix)  .in_set(StageCulling::CalcBounding))
        .add_systems(Update, sys_abstructmesh_culling_flag_reset         .after(sys_update_culling_by_cullinginfo)  .in_set(StageCulling::CalcBounding))
        ;
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