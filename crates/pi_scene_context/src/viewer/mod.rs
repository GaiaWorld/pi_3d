use std::{marker::PhantomData, sync::Arc};

use pi_engine_shell::prelude::*;
use pi_hash::XHashMap;
use pi_render::{
    renderer::bind_buffer::{
        BindBufferRange,
        BindBufferAllocator,
    },
    render_3d::binds::scene::base::ShaderBindSceneAboutBase
};
use pi_scene_math::{Vector3, Matrix, coordiante_system::CoordinateSytem3};

use crate::{transforms::transform_node::{GlobalTransform, LocalPosition}};

use self::{
    sys::{SysViewerViewMatrixByViewCalc, SysViewerViewMatrixUpdateByParentModify, SysViewerProjectionCalc, SysViewerTransformUpdated, SysViewerUpdated},
};

pub mod command;
pub mod sys_culling;
pub mod sys;
pub mod interface;

#[derive(Debug, Default, Clone)]
pub struct ModelList(pub XHashMap<ObjectID, ObjectID>);
#[derive(Debug, Default, Clone)]
pub struct FlagModelList(pub bool);
#[derive(Debug, Default)]
pub struct ModelListAdd(pub XHashMap<ObjectID, ObjectID>);
pub struct FlagModelListAdd(pub bool);
#[derive(Debug, Default)]
pub struct ModelListDel(pub XHashMap<ObjectID, ObjectID>);
pub struct FlagModelListDel(pub bool);
#[derive(Debug, Default)]
pub struct ModelListAfterCulling(pub Vec<ObjectID>);

/// 视口ID - 可能是 相机、灯光
pub struct ViewerID(pub ObjectID);

/// 视口状态
pub struct ViewerActive(pub bool);

#[derive(Debug, Clone)]
pub struct ViewerViewMatrix(pub Matrix);
impl Default for ViewerViewMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl ViewerViewMatrix {
    fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindSceneAboutBase::OFFSET_VIEW_MATRIX as usize, bytemuck::cast_slice(self.0.transpose().as_slice()));
    }
}

#[derive(Debug, Clone)]
pub struct ViewerProjectionMatrix(pub Matrix);
impl Default for ViewerProjectionMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl ViewerProjectionMatrix {
    fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindSceneAboutBase::OFFSET_PROJECT_MATRIX as usize, bytemuck::cast_slice(self.0.transpose().as_slice()));
    }
}
// impl Uniform for ViewerProjectionMatrix {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.transpose().as_slice()), index as usize + ShaderBindSceneAboutBase::OFFSET_PROJECT_MATRIX as usize, buffer);
//     }
// }
#[derive(Debug, Clone)]
pub struct ViewerTransformMatrix(pub Matrix);
impl Default for ViewerTransformMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl ViewerTransformMatrix {
    fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindSceneAboutBase::OFFSET_VIEW_PROJECT_MATRIX as usize, bytemuck::cast_slice(self.0.as_slice()));
    }
}
// impl Uniform for ViewerTransformMatrix {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         //  log::debug!(">>>>>>>>> {:?}", self.0.as_slice());
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutBase::OFFSET_VIEW_PROJECT_MATRIX as usize, buffer);
//     }
// }
#[derive(Debug, Clone)]
pub struct ViewerGlobalPosition(pub Vector3);
impl Default for ViewerGlobalPosition {
    fn default() -> Self {
        Self(Vector3::new(0., 0., -1.))
    }
}
impl ViewerGlobalPosition {
    fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindSceneAboutBase::OFFSET_CAMERA_POSITION as usize, bytemuck::cast_slice(self.0.as_slice()));
    }
}
// impl Uniform for ViewerGlobalPosition {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutBase::OFFSET_CAMERA_POSITION as usize, buffer);
//     }
// }
#[derive(Debug, Clone)]
pub struct ViewerDirection(pub Vector3);
impl Default for ViewerDirection {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 1.))
    }
}
impl ViewerDirection {
    fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindSceneAboutBase::OFFSET_CAMERA_DIRECTION as usize, bytemuck::cast_slice(self.0.as_slice()));
    }
}
// impl Uniform for ViewerDirection {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutBase::OFFSET_CAMERA_DIRECTION as usize, buffer);
//     }
// }

#[derive(Debug, Clone)]
pub struct BindViewer(pub Arc<ShaderBindSceneAboutBase>);
impl BindViewer {
    pub fn new(allocator: &mut BindBufferAllocator) -> Option<Self> {
        if let Some(data) = ShaderBindSceneAboutBase::new(allocator) {
            Some(Self ( Arc::new(data) ))
        } else {
            None
        }
    }
}

pub trait TViewerViewMatrix {
    fn view_matrix(&self, coordsys: &CoordinateSytem3, local_pos: &LocalPosition, parent: Option<&mut GlobalTransform>) -> (ViewerViewMatrix, ViewerGlobalPosition);
}

pub trait TViewerProjectMatrix {
    fn project_matrix(&self, ratio: f32) -> ViewerProjectionMatrix;
}

///
/// * 渲染视口功能
/// * T - 获取 ViewerViewMatrix 的 Component
/// * S - 获取 ViewerViewMatrix 需要依赖的 System
/// * T2 - 获取 ViewerProjectionMatrix 的 Component
/// * S2 - 获取 ViewerProjectionMatrix 需要依赖的 System
#[derive(Debug)]
pub struct PluginViewer<
    T: TViewerViewMatrix + Component,
    S: TSystemStageInfo + 'static,
    T2: TViewerProjectMatrix + Component,
    S2: TSystemStageInfo + 'static,
>(PhantomData<(T, S, T2, S2)>);
impl<
    T: TViewerViewMatrix + Component,
    S: TSystemStageInfo + 'static,
    T2: TViewerProjectMatrix + Component,
    S2: TSystemStageInfo + 'static,
> Plugin for PluginViewer<T, S, T2, S2> {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        // if world.get_resource::<SingleRendererCommandList>().is_none() {
        //     world.insert_resource(SingleRendererCommandList::default());
        //     // 依赖的 ViewerRenderersInfo 初始化的 System 应该在 Initial 阶段
        //     log::warn!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> ");
        //     SysViewerRendererCommandTick::setup(world, stages.query_stage::<SysViewerRendererCommandTick>(ERunStageChap::Command));
        //     SysModelListUpdateByViewer::setup(world, stages.query_stage::<SysModelListUpdateByViewer>(ERunStageChap::Command));
        //     SysModelListUpdateByModel::setup(world, stages.query_stage::<SysModelListUpdateByModel>(ERunStageChap::Command));
        //     SysModelListAfterCullingTick::setup(world, stages.query_stage::<SysModelListAfterCullingTick>(ERunStageChap::Command));
        // } else {
        //     log::warn!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 2");
        // }

        SysViewerViewMatrixByViewCalc::<T, S>::setup(world, stages.query_stage::<SysViewerViewMatrixByViewCalc::<T, S>>(ERunStageChap::Command));
        // SysViewerViewMatrixUpdateByLocalPos::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByLocalPos::<T>>(ERunStageChap::Command));
        SysViewerViewMatrixUpdateByParentModify::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByParentModify::<T>>(ERunStageChap::Command));
        SysViewerProjectionCalc::<T2, S2>::setup(world, stages.query_stage::<SysViewerProjectionCalc::<T2, S2>>(ERunStageChap::Command));
        SysViewerTransformUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerTransformUpdated::<T, S, T2, S2>>(ERunStageChap::Command));
        SysViewerUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerUpdated::<T, S, T2, S2>>(ERunStageChap::Command));

        Ok(())
    }

    fn build(&self, app: &mut App) {
        let world = &mut app.world;

        // if world.get_resource::<SingleRendererCommandList>().is_none() {
        //     world.insert_resource(SingleRendererCommandList::default());
        //     // 依赖的 ViewerRenderersInfo 初始化的 System 应该在 Initial 阶段
        //     log::warn!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> ");
        //     SysViewerRendererCommandTick::setup(world, stages.query_stage::<SysViewerRendererCommandTick>(ERunStageChap::Command));
        //     SysModelListUpdateByViewer::setup(world, stages.query_stage::<SysModelListUpdateByViewer>(ERunStageChap::Command));
        //     SysModelListUpdateByModel::setup(world, stages.query_stage::<SysModelListUpdateByModel>(ERunStageChap::Command));
        //     SysModelListAfterCullingTick::setup(world, stages.query_stage::<SysModelListAfterCullingTick>(ERunStageChap::Command));
        // } else {
        //     log::warn!(">>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>> 2");
        // }

        SysViewerViewMatrixByViewCalc::<T, S>::setup(world, stages.query_stage::<SysViewerViewMatrixByViewCalc::<T, S>>(ERunStageChap::Command));
        // SysViewerViewMatrixUpdateByLocalPos::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByLocalPos::<T>>(ERunStageChap::Command));
        SysViewerViewMatrixUpdateByParentModify::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByParentModify::<T>>(ERunStageChap::Command));
        SysViewerProjectionCalc::<T2, S2>::setup(world, stages.query_stage::<SysViewerProjectionCalc::<T2, S2>>(ERunStageChap::Command));
        SysViewerTransformUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerTransformUpdated::<T, S, T2, S2>>(ERunStageChap::Command));
        SysViewerUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerUpdated::<T, S, T2, S2>>(ERunStageChap::Command));

    }
}
impl<
    T: TViewerViewMatrix + Component,
    S: TSystemStageInfo,
    T2: TViewerProjectMatrix + Component,
    S2: TSystemStageInfo,
> Default for PluginViewer<T, S, T2, S2> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
