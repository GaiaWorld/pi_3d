use std::sync::Arc;

use pi_scene_shell::prelude::*;

use crate::transforms::prelude::*;


#[derive(Debug, Default, Clone, Component)]
pub struct ForceIncludeModelList(pub XHashSet<Entity>);

#[derive(Debug, Default, Clone, Component)]
pub struct FlagForceIncludeModelList;

#[derive(Debug, Default, Clone, Component)]
pub struct ModelList(pub XHashSet<Entity>);

#[derive(Debug, Default, Clone, Component)]
pub struct FlagModelList(pub bool);

#[derive(Debug, Default, Component)]
pub struct ModelListAdd(pub XHashSet<Entity>);

#[derive(Component)]
pub struct FlagModelListAdd(pub bool);

#[derive(Debug, Default, Component)]
pub struct ModelListDel(pub XHashSet<Entity>);

#[derive(Component)]
pub struct FlagModelListDel(pub bool);

#[derive(Debug, Default, Component)]
pub struct ModelListAfterCulling(pub Vec<Entity>);

/// 视口ID - 可能是 相机、灯光
#[derive(Component)]
pub struct ViewerID(pub Entity);

/// 视口状态
#[derive(Debug, Clone, Copy, Component)]
pub struct ViewerActive(pub bool);

// /// 视口尺寸
// #[derive(Debug, Clone, Copy, Component)]
// pub struct ViewerSize(pub u32, pub u32);
// impl Default for ViewerSize {
//     fn default() -> Self {
//         Self(Self::DEFAULT_WIDTH, Self::DEFAULT_HEIGHT)
//     }
// }
// impl ViewerSize {
//     pub const DEFAULT_WIDTH: u32 = 100;
//     pub const DEFAULT_HEIGHT: u32 = 100;
// }

/// 视口尺寸
#[derive(Debug, Clone, Copy, Component)]
pub struct ViewerAspect(pub f32);
impl Default for ViewerAspect {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Component)]
pub struct ViewerCullFilter {
    _test: Vec<Entity>
}
impl Default for ViewerCullFilter {
    fn default() -> Self {
        Self {
            _test: vec![]
        }
    }
}
impl ViewerCullFilter {
    pub fn add(&mut self, _entity: Entity) {

    }
    pub fn remove(&mut self, _entity: Entity) {

    }
}

#[derive(Component)]
pub struct ViewerRenderTargetFormatOption {
    pub color: wgpu::TextureFormat,
    pub depth_stencil: wgpu::TextureFormat,
}

#[derive(Debug, Clone, Component)]
pub struct ViewerViewMatrix(pub Matrix);
impl Default for ViewerViewMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl ViewerViewMatrix {
    pub fn get_rotation_matrix(&self) -> Matrix {
        let mut result = self.0.clone();
        let mut temp = result.fixed_view_mut::<3, 1>(0, 3);
        let vv = temp.as_mut_slice();
        vv[0] = 0.;vv[1] = 0.;vv[2] = 0.;
        result
    }
    pub fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindViewer::OFFSET_VIEW_MATRIX as usize, bytemuck::cast_slice(self.0.as_slice()));
        range.write_data(ShaderBindViewer::OFFSET_CAMERA_ROTATION as usize, bytemuck::cast_slice(self.get_rotation_matrix().as_slice()));
    }
}

#[derive(Debug, Clone, Component)]
pub struct ViewerProjectionMatrix(pub Matrix);
impl Default for ViewerProjectionMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl ViewerProjectionMatrix {
    pub fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindViewer::OFFSET_PROJECT_MATRIX as usize, bytemuck::cast_slice(self.0.as_slice()));
    }
}
// impl Uniform for ViewerProjectionMatrix {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.transpose().as_slice()), index as usize + ShaderBindSceneAboutBase::OFFSET_PROJECT_MATRIX as usize, buffer);
//     }
// }
#[derive(Debug, Clone, Component)]
pub struct ViewerTransformMatrix(pub Matrix);
impl Default for ViewerTransformMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl ViewerTransformMatrix {
    pub fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindViewer::OFFSET_VIEW_PROJECT_MATRIX as usize, bytemuck::cast_slice(self.0.as_slice()));
    }
}
// impl Uniform for ViewerTransformMatrix {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         //  log::debug!(">>>>>>>>> {:?}", self.0.as_slice());
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutBase::OFFSET_VIEW_PROJECT_MATRIX as usize, buffer);
//     }
// }
#[derive(Debug, Clone, Component)]
pub struct ViewerGlobalPosition(pub Vector3);
impl Default for ViewerGlobalPosition {
    fn default() -> Self {
        Self(Vector3::new(0., 0., -1.))
    }
}
impl ViewerGlobalPosition {
    pub fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindViewer::OFFSET_CAMERA_POSITION as usize, bytemuck::cast_slice(self.0.as_slice()));
    }
}
// impl Uniform for ViewerGlobalPosition {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutBase::OFFSET_CAMERA_POSITION as usize, buffer);
//     }
// }
#[derive(Debug, Clone, Component)]
pub struct ViewerDirection(pub Vector3);
impl Default for ViewerDirection {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 1.))
    }
}
impl ViewerDirection {
    pub fn update(&self, range: &BindBufferRange) {
        range.write_data(ShaderBindViewer::OFFSET_CAMERA_DIRECTION as usize, bytemuck::cast_slice(self.0.as_slice()));
    }
}
// impl Uniform for ViewerDirection {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + ShaderBindSceneAboutBase::OFFSET_CAMERA_DIRECTION as usize, buffer);
//     }
// }

#[derive(Debug, Clone, Component)]
pub enum ViewerDistanceCompute {
    Base,
    Direction,
}
impl Default for ViewerDistanceCompute {
    fn default() -> Self {
        Self::Base
    }
}
impl ViewerDistanceCompute {
    pub fn distance(&self, view: &Vector3, view_direction: &Vector3, target: &Vector3) -> Number {
        match self {
            ViewerDistanceCompute::Base => {
                let temp = target - view;
                temp.dot(&temp)
            },
            ViewerDistanceCompute::Direction => {
                let temp = target - view;
                view_direction.dot(&temp)
            },
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct BindViewer(pub Arc<ShaderBindViewer>);
impl BindViewer {
    pub fn new(allocator: &mut BindBufferAllocator) -> Option<Self> {
        if let Some(data) = ShaderBindViewer::new(allocator) {
            Some(Self ( Arc::new(data) ))
        } else {
            None
        }
    }
}

pub trait TViewerViewMatrix {
    fn view_matrix(&self, coordsys: &CoordinateSytem3, local_pos: &LocalPosition, parent: Option<(&GlobalMatrix, Isometry3)>) -> (ViewerViewMatrix, ViewerGlobalPosition);
}

pub trait TViewerProjectMatrix {
    fn project_matrix(&self, ratio: f32) -> ViewerProjectionMatrix;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageViewer {
    ForceInclude,
}

pub trait TCullingPerformance {
    fn culling_time(&mut self, ms: u32);
}