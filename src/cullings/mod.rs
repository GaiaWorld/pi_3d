use parry3d::shape::ConvexPolyhedron;
use pi_ecs::{prelude::{ResMut, Query, Setup}, query::Write};
use pi_ecs_macros::setup;
use pi_scene_math::{
    frustum::FrustumPlanes, plane::Plane, vector::TMinimizeMaximize, Matrix, Vector3,
};
use pi_slotmap::{DefaultKey, KeyData, Key};
use crate::{object::{ObjectID, GameObject}};

use self::{bounding_box::BoundingBox, bounding_sphere::BoundingSphere, bounding::BoundingInfo, sys::SysCameraCulling};

pub mod bounding_box;
pub mod bounding_sphere;
pub mod sys;
pub mod bounding;
pub mod oct_tree;
pub mod quad_tree;


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
    Bounding(ObjectID, Vector3, Vector3),
}
#[derive(Debug, Default)]
pub struct SingleCullingCommandList {
    pub list: Vec<CullingCommand>,
}

pub struct SysCullingCommand;
#[setup]
impl SysCullingCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingleCullingCommandList>,
        mut objects: Query<GameObject, Write<BoundingInfo>>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                CullingCommand::Bounding(entity, min, max) => {
                    match objects.get_mut(entity) {
                        Some(mut item) => {
                            item.insert_no_notify(BoundingInfo::new(min, max));
                        },
                        None => todo!(),
                    }
                },
            }
        })
    }
}

pub struct PluginCulling;
impl crate::Plugin for PluginCulling {
    fn init(
        &mut self,
        world: &mut pi_ecs::world::World,
        engine: &mut crate::engine::Engine,
        stages: &mut crate::run_stage::RunStage,
    ) -> Result<(), crate::plugin::ErrorPlugin> {
        SysCullingCommand::setup(world, stages.command_stage());
        SysCameraCulling::setup(world, stages.after_world_matrix());

        world.insert_resource(SingleCullingCommandList::default());

        Ok(())
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

impl InterfaceBoundingInfo for crate::engine::Engine {
    fn set_bounding(
        &mut self,
        object: ObjectID,
        min: Vector3,
        max: Vector3,
    ) {
        let world = self.world();

        let commands = world.get_resource_mut::<SingleCullingCommandList>().unwrap();
        commands.list.push(CullingCommand::Bounding(object, min, max));
    }
}
