use pi_render::rhi::{dyn_uniform_buffer::{BindOffset, Bind}};
use pi_scene_math::{Vector3, Number, coordiante_system::CoordinateSytem3, camera::{TPerspectiveCameraTool, TOrthographicCameraTool}};

use crate::{shaders::{FragmentUniformBind}, viewer::{TViewerProjectMatrix, ViewerProjectionMatrix}};

pub struct CameraRenderData {
    pub bind_offset: BindOffset,
}
impl CameraRenderData {
    pub const PI_MATRIX_V: usize = 16;
    pub const PI_MATRIX_P: usize = 16;
    pub const PI_MATRIX_VP: usize = 16;
    pub const PI_CAMERA_POSITION: usize = 4;
    pub const PI_ORTHCAMERA_DIRECT: usize = 4;

    pub const PI_MATRIX_V_OFFSIZE: usize = 0 * 4;
    pub const PI_MATRIX_P_OFFSIZE: usize = Self::PI_MATRIX_V_OFFSIZE + Self::PI_MATRIX_V * 4;
    pub const PI_MATRIX_VP_OFFSIZE: usize = Self::PI_MATRIX_P_OFFSIZE + Self::PI_MATRIX_VP * 4;
    pub const PI_CAMERA_POSITION_OFFSIZE: usize = Self::PI_MATRIX_VP_OFFSIZE + Self::PI_MATRIX_P * 4;
    pub const PI_ORTHCAMERA_DIRECT_OFFSIZE: usize = Self::PI_CAMERA_POSITION_OFFSIZE + Self::PI_CAMERA_POSITION * 4;

    pub fn new(
        dynbuffer: &mut render_resource::uniform_buffer::RenderDynUniformBuffer,
    ) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
        }
    }
}
impl FragmentUniformBind for CameraRenderData {
    const ID: u32 = 0;
    const SIZE: usize = Self::PI_ORTHCAMERA_DIRECT_OFFSIZE + Self::PI_ORTHCAMERA_DIRECT * 4;
}
impl Bind for CameraRenderData {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Camera;

#[derive(Debug, Clone)]
pub struct CameraViewport {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}
impl Default for CameraViewport {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
            w: 1.,
            h: 1.
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum EFixedMode {
    VerticalFixed,
    HorizontalFixed,
}

#[derive(Debug, Clone, Copy)]
pub enum EFreeCameraMode {
    Perspective,
    Orthograhic,
}

#[derive(Debug, Clone, Copy)]
pub struct CameraNearFar(pub Number, pub Number);

#[derive(Debug, Clone, Copy)]
pub struct CameraOrthograhicParam(pub Number, pub Number, pub Number, pub Number);

#[derive(Debug, Clone, Copy)]
pub struct CameraPerspectiveParam(pub Number);

#[derive(Debug, Clone)]
pub struct CameraParam {
    pub up: Vector3,
    pub minz: f32,
    pub maxz: f32,
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub fov: f32,
    pub fixed_mode: EFixedMode,
    pub mode: EFreeCameraMode,
    pub orth_size: Number,
    pub dirty: bool,
}
impl Default for CameraParam {
    fn default() -> Self {
        Self {
            up: Vector3::new(0., 1., 0.), minz: 1., maxz: 1000.,
            orth_size: 4.,
            left: -4.,
            top: 4.,
            right: 4.,
            bottom: -4.,
            fov: Number::to_radians(60.0),
            fixed_mode: EFixedMode::VerticalFixed,
            mode: EFreeCameraMode::Orthograhic,
            dirty: true,
        }
    }
}
impl TViewerProjectMatrix for CameraParam {
    fn project_matrix(&self, aspect: Number) -> ViewerProjectionMatrix {
        match self.mode {
            EFreeCameraMode::Perspective => {
                let m = match self.fixed_mode {
                    EFixedMode::VerticalFixed => CoordinateSytem3::perspective_lh(self.fov, aspect, self.minz, self.maxz, true),
                    EFixedMode::HorizontalFixed => CoordinateSytem3::perspective_lh(self.fov, aspect, self.minz, self.maxz, false),
                };
                ViewerProjectionMatrix(m)
            },
            EFreeCameraMode::Orthograhic => {
                let value = self.orth_size;
                let m = match self.fixed_mode {
                    EFixedMode::VerticalFixed => {
                        let left = -value * aspect;
                        let right = value * aspect;
                        let top = value;
                        let bottom = -value;
                        CoordinateSytem3::orthographic_lh(left, right, bottom, top, self.minz, self.maxz)
                    },
                    EFixedMode::HorizontalFixed => {
                        let left = -value;
                        let right = value;
                        let top = value / aspect;
                        let bottom = -value / aspect;
                        CoordinateSytem3::orthographic_lh(left, right, bottom, top, self.minz, self.maxz)
                    },
                };
                ViewerProjectionMatrix(m)
            },
        }
    }
}
