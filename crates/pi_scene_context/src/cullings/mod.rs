

use pi_engine_shell::prelude::*;
use pi_scene_math::{
    Vector3,
};

use self::{bounding_box::BoundingBox, bounding_sphere::BoundingSphere, bounding::BoundingInfo};

pub mod bounding_box;
pub mod bounding_sphere;
pub mod sys;
pub mod bounding;
// pub mod oct_tree;
// pub mod quad_tree;


#[derive(Debug, Clone)]
pub struct IsCulled;

/// 检测级别
/// *
pub enum ECullingStrategy {
    /// 检测 包围球中心 在不在 视锥, 检测 包围球 在不在 视锥
    Optimistic,
    /// 检测 包围球中心 在不在 视锥, 检测 包围球 在不在 视锥, 检测 包围盒 在不在 视锥
    STANDARD,
}

pub trait TIntersect {
    fn intersects_point(&self, p: &Vector3) -> bool;
    fn intersects_box(&self, b: &BoundingBox) -> bool;
    fn intersects_sphere(&self, s: &BoundingSphere) -> bool;
    fn intersects_min_max(&self, min: &Vector3, max: &Vector3) -> bool;
}

#[derive(Debug)]
pub enum CullingCommand {
    Bounding(Vector3, Vector3),
}

pub struct ActionCulling;
impl ActionCulling {
    pub fn modify(
        entitycmd: &mut EntityCommands,
        action: CullingCommand,
    ) {
        match action {
            CullingCommand::Bounding(min, max) => {
                entitycmd.insert(BoundingInfo::new(min, max));
            }
        }
    }
}


pub struct PluginCulling;
impl Plugin for PluginCulling {
    // fn init(
    //     &mut self,
    //     engine: &mut crate::engine::Engine,
    //     stages: &mut crate::run_stage::RunStage,
    // ) -> Result<(), crate::plugin::ErrorPlugin> {
    //     let world = engine.world_mut();

    //     SysCullingCommand::setup(world, stages.query_stage::<SysCullingCommand>(ERunStageChap::Initial));
    //     SysCameraCulling::setup(world, stages.query_stage::<SysCameraCulling>(ERunStageChap::Command));

    //     world.insert_resource(SingleCullingCommandList::default());

    //     Ok(())
    // }

    fn build(&self, app: &mut App) {
        // todo!()
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