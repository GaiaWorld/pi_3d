use std::marker::PhantomData;

use pi_ecs::prelude::{Component, Setup};
use pi_engine_shell::{run_stage::{TSystemStageInfo, ERunStageChap}, object::ObjectID};
use pi_render::rhi::{dyn_uniform_buffer::{Uniform}};
use pi_scene_math::{Vector3, Matrix, coordiante_system::CoordinateSytem3};
use render_shader::shader_bind::ShaderBindSceneAboutCamera;

use crate::{bytes_write_to_memory, transforms::transform_node::{WorldMatrix, GlobalTransform, LocalPosition}};

use self::{command::{SingleViewerCommands, SysViewerCommand}, sys::{SysViewerViewMatrixByViewCalc, SysViewerViewMatrixUpdateByLocalPos, SysViewerViewMatrixUpdateByParentModify, SysViewerProjectionCalc, SysViewerUpdated}};

pub mod command;
pub mod sys;

pub struct ViewerID(pub ObjectID);

#[derive(Debug, Clone)]
pub struct ViewerViewMatrix(pub Matrix);
impl Default for ViewerViewMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl Uniform for ViewerViewMatrix {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.transpose().as_slice()), index as usize + ShaderBindSceneAboutCamera::OFFSET_VIEW_MATRIX as usize, buffer);
    }
}

#[derive(Debug, Clone)]
pub struct ViewerProjectionMatrix(pub Matrix);
impl Default for ViewerProjectionMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl Uniform for ViewerProjectionMatrix {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.transpose().as_slice()), index as usize + ShaderBindSceneAboutCamera::OFFSET_PROJECT_MATRIX as usize, buffer);
    }
}
#[derive(Debug, Clone)]
pub struct ViewerTransformMatrix(pub Matrix);
impl Default for ViewerTransformMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl Uniform for ViewerTransformMatrix {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        //  log::debug!(">>>>>>>>> {:?}", self.0.as_slice());
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutCamera::OFFSET_VIEW_PROJECT_MATRIX as usize, buffer);
    }
}
#[derive(Debug, Clone)]
pub struct ViewerGlobalPosition(pub Vector3);
impl Default for ViewerGlobalPosition {
    fn default() -> Self {
        Self(Vector3::new(0., 0., -1.))
    }
}
impl Uniform for ViewerGlobalPosition {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutCamera::OFFSET_CAMERA_POSITION as usize, buffer);
    }
}
#[derive(Debug, Clone)]
pub struct ViewerDirection(pub Vector3);
impl Default for ViewerDirection {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 1.))
    }
}
impl Uniform for ViewerDirection {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutCamera::OFFSET_CAMERA_DIRECTION as usize, buffer);
    }
}

pub trait TViewerViewMatrix {
    fn view_matrix(&self, coordsys: &CoordinateSytem3, local_pos: &LocalPosition, parent: Option<&GlobalTransform>) -> (ViewerViewMatrix, ViewerGlobalPosition);
}

pub trait TViewerProjectMatrix {
    fn project_matrix(&self, ratio: f32) -> ViewerProjectionMatrix;
}

#[derive(Debug)]
pub struct PluginViewer<T: TViewerViewMatrix + Component, S: TSystemStageInfo + 'static, T2: TViewerProjectMatrix + Component, S2: TSystemStageInfo + 'static>(PhantomData<(T, S, T2, S2)>);
impl<T: TViewerViewMatrix + Component, S: TSystemStageInfo + 'static, T2: TViewerProjectMatrix + Component, S2: TSystemStageInfo + 'static> pi_engine_shell::plugin::Plugin for PluginViewer<T, S, T2, S2> {
    fn init(
        &mut self,
        engine: &mut pi_engine_shell::engine_shell::EnginShell,
        stages: &mut pi_engine_shell::run_stage::RunStage,
    ) -> Result<(), pi_engine_shell::plugin::ErrorPlugin> {
        let world = engine.world_mut();

        if world.get_resource::<SingleViewerCommands>().is_none() {
            world.insert_resource(SingleViewerCommands::default());
            SysViewerCommand::setup(world, stages.query_stage::<SysViewerCommand>(ERunStageChap::Command));
        }

        SysViewerViewMatrixByViewCalc::<T, S>::setup(world, stages.query_stage::<SysViewerViewMatrixByViewCalc::<T, S>>(ERunStageChap::Command));
        SysViewerViewMatrixUpdateByLocalPos::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByLocalPos::<T>>(ERunStageChap::Command));
        SysViewerViewMatrixUpdateByParentModify::<T>::setup(world, stages.query_stage::<SysViewerViewMatrixUpdateByParentModify::<T>>(ERunStageChap::Command));
        SysViewerProjectionCalc::<T2, S2>::setup(world, stages.query_stage::<SysViewerProjectionCalc::<T2, S2>>(ERunStageChap::Command));
        SysViewerUpdated::<T, S, T2, S2>::setup(world, stages.query_stage::<SysViewerUpdated::<T, S, T2, S2>>(ERunStageChap::Command));

        Ok(())
    }
}
impl<T: TViewerViewMatrix + Component, S: TSystemStageInfo, T2: TViewerProjectMatrix + Component, S2: TSystemStageInfo> Default for PluginViewer<T, S, T2, S2> {
    fn default() -> Self {
        Self(PhantomData)
    }
}
