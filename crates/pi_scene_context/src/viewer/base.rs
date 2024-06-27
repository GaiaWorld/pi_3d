use std::sync::Arc;

use pi_scene_shell::prelude::*;

use crate::transforms::prelude::*;


#[derive(Clone, Component, Default)]
pub struct ForceIncludeModelList(pub XHashSet<Entity>);

#[derive(Clone, Component, Default)]
pub struct FlagForceIncludeModelList;

#[derive(Clone, Component, Default)]
pub struct ModelList(pub XHashSet<Entity>);

#[derive(Clone, Component, Default)]
pub struct FlagModelList(pub bool);

#[derive(Component, Default)]
pub struct ModelListAdd(pub XHashSet<Entity>);

#[derive(Component, Default)]
pub struct FlagModelListAdd(pub bool);

#[derive(Component, Default)]
pub struct ModelListDel(pub XHashSet<Entity>);

#[derive(Component, Default)]
pub struct FlagModelListDel(pub bool);

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
impl Default for ViewerRenderTargetFormatOption {
    fn default() -> Self {
        Self { color: wgpu::TextureFormat::Rgba8Unorm, depth_stencil: wgpu::TextureFormat::Depth24PlusStencil8 }
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
    pub fn ray(&self, x: f32, y: f32) -> (Vector3, Vector3) {
        let invtransform = if let Some(invtransform) = self.0.try_inverse() {
            invtransform
        } else {
            Matrix::identity()
        };

        let near_screen_source = Vector3::new(x * 2. - 1., -(y * 2. - 1.), -1.0);
        let far_screen_source = Vector3::new(x * 2. - 1., -(y * 2. - 1.), 1.0);
        let mut near = Vector3::zeros();
        let mut far = Vector3::zeros();
        let vv = invtransform.fixed_view::<4, 1>(0, 3);
        CoordinateSytem3::transform_coordinates(&near_screen_source, &invtransform, &mut near);
        let num = near.x * vv.x + near.y * vv.y + near.z * vv.z + vv.w;
        near.scale_mut(1.0 / num);
        CoordinateSytem3::transform_coordinates(&far_screen_source, &invtransform, &mut far);
        let num = far.x * vv.x + far.y * vv.y + far.z * vv.z + vv.w;
        far.scale_mut(1.0 / num);

        let origin = near;
        let direction = far - origin;
        (origin, direction)
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

#[derive(Clone, Component)]
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

#[derive(Clone, Component, Default)]
pub struct BindViewer(pub Option<Arc<ShaderBindViewer>>);
impl BindViewer {
    pub fn new(allocator: &mut BindBufferAllocator) -> Option<Self> {
        if let Some(data) = ShaderBindViewer::new(allocator) {
            Some(Self ( Some(Arc::new(data)) ))
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