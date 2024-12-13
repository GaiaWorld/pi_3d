use std::sync::Arc;

use pi_scene_shell::prelude::*;

use crate::{cullings::prelude::PiRay, transforms::prelude::*};


#[derive(Clone, Component, Default)]
pub struct ForceIncludeModelList(pub XHashSet<Entity>);

#[derive(Clone, Component, Default)]
pub struct FlagForceIncludeModelList;

#[derive(Clone, Component, Default)]
pub struct ModelList(pub XHashSet<Entity>);

#[derive(Clone, Component, Default)]
pub struct FlagModelList(pub bool);

#[derive(Component, Default)]
pub struct ModelListAfterCulling(pub Vec<Entity>);

/// 视口ID - 可能是 相机、灯光
#[derive(Component, Default)]
pub struct ViewerID(pub Entity);

/// 视口状态
#[derive(Clone, Copy, Component, Default)]
pub struct ViewerActive(pub bool);

/// 视口尺寸
#[derive(Clone, Copy, Component)]
pub struct ViewerAspect(pub f32);
impl Default for ViewerAspect {
    fn default() -> Self {
        Self(1.0)
    }
}

#[derive(Clone, Component)]
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

#[derive(Clone, Component)]
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
#[derive(Clone, Component)]
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
    pub fn ray(&self, projectx: f32, projecty: f32) -> PiRay {
        let mut invtransform = self.0.clone();
        CoordinateSytem3::try_inverse_mut(&mut invtransform);

        let x = projectx;
        let y = projecty;
        let mut origin = Vector3::zeros();
        let mut far = Vector3::zeros();

        CoordinateSytem3::transform_coordinates_floats(x, y, 0., &invtransform, &mut origin);
        CoordinateSytem3::transform_coordinates_floats(x, y, 1., &invtransform, &mut far);

        let mut direction = far - origin;
        direction.normalize_mut();

        PiRay {
            origin: (origin.x, origin.y, origin.z),
            far: (far.x, far.y, far.z),
            direction: (direction.x, direction.y, direction.z),
        }
    }
}
#[derive(Clone, Component)]
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

#[derive(Clone, Component)]
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

#[derive(Clone)]
pub enum EViewerDistanceCompute {
    Base,
    Direction,
}
#[derive(Clone, Component)]
pub struct ViewerDistanceCompute {
    pub call: fn(&(Number, Number, Number), &(Number, Number, Number), &(Number, Number, Number)) -> Number,
}
impl Default for ViewerDistanceCompute {
    fn default() -> Self {
        Self { call: Self::base }
    }
}
impl ViewerDistanceCompute {
    pub fn new(mode: EViewerDistanceCompute) -> Self {
        match mode {
            EViewerDistanceCompute::Base => Self { call: Self::base },
            EViewerDistanceCompute::Direction => Self { call: Self::direction },
        }
    }
    pub fn distance(&self, view: &(Number, Number, Number), view_direction: &(Number, Number, Number), target: &(Number, Number, Number)) -> Number {
        (self.call)(view, view_direction, target)
    }
    pub fn base(view: &(Number, Number, Number), view_direction: &(Number, Number, Number), target: &(Number, Number, Number)) -> Number {
        let x = target.0 - view.0;
        let y = target.1 - view.1;
        let z = target.2 - view.2;
        x * x + y * y + z * z
    }
    pub fn direction(view: &(Number, Number, Number), view_direction: &(Number, Number, Number), target: &(Number, Number, Number)) -> Number {
        let x = target.0 - view.0;
        let y = target.1 - view.1;
        let z = target.2 - view.2;
        view_direction.0 * x + view_direction.1 * y + view_direction.2 * z
    }
}

#[derive(Clone, Component, Default)]
pub struct BindViewer(pub Option<ShaderBindViewer>);
impl BindViewer {
    pub fn new(allocator: &mut BindBufferAllocator) -> Self {
        Self (ShaderBindViewer::new(allocator))
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
    TransformMatrixCalc,
    Culling,
}

pub trait TCullingPerformance {
    fn culling_time(&mut self, ms: u32);
}