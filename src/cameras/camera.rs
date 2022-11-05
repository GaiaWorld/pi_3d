use pi_ecs::{prelude::{ResMut, Query, EntityDelete}, query::Write};
use pi_ecs_macros::setup;
use pi_render::rhi::{dyn_uniform_buffer::{Uniform, BindOffset, Bind}};
use pi_scene_math::{Vector3, Matrix, Number, coordiante_system::CoordinateSytem3, camera::{TPerspectiveCameraTool, TOrthographicCameraTool}};

use crate::{bytes_write_to_memory, shaders::{FragmentUniformBind}, object::{ObjectID, GameObject}, resources::RenderDynUniformBuffer};

#[derive(Debug, Clone)]
pub struct CameraViewMatrix(pub Matrix);
impl Default for CameraViewMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl Uniform for CameraViewMatrix {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.transpose().as_slice()), index as usize + CameraRenderData::PI_MATRIX_V_OFFSIZE, buffer);
    }
}
#[derive(Debug, Clone)]
pub struct CameraProjectionMatrix(pub Matrix);
impl Default for CameraProjectionMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl Uniform for CameraProjectionMatrix {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.transpose().as_slice()), index as usize + CameraRenderData::PI_MATRIX_P_OFFSIZE, buffer);
    }
}
#[derive(Debug, Clone)]
pub struct CameraTransformMatrix(pub Matrix);
impl Default for CameraTransformMatrix {
    fn default() -> Self {
        Self(Matrix::default())
    }
}
impl Uniform for CameraTransformMatrix {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        println!(">>>>>>>>> {:?}", self.0.as_slice());
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + CameraRenderData::PI_MATRIX_VP_OFFSIZE, buffer);
    }
}
#[derive(Debug, Clone)]
pub struct CameraGlobalPosition(pub Vector3);
impl Default for CameraGlobalPosition {
    fn default() -> Self {
        Self(Vector3::new(0., 0., -1.))
    }
}
impl Uniform for CameraGlobalPosition {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + CameraRenderData::PI_CAMERA_POSITION_OFFSIZE, buffer);
    }
}
#[derive(Debug, Clone)]
pub struct CameraDirection(pub Vector3);
impl Default for CameraDirection {
    fn default() -> Self {
        Self(Vector3::new(0., 0., 1.))
    }
}
impl Uniform for CameraDirection {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + CameraRenderData::PI_ORTHCAMERA_DIRECT_OFFSIZE, buffer);
    }
}

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
        dynbuffer: &mut RenderDynUniformBuffer,
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
        }
    }
}
impl CameraParam {
    ///
    /// * `aspect` width / height pixels ratio
    pub fn project_matrix(&self, c_p_m: &mut CameraProjectionMatrix, aspect: Number) {
        match self.mode {
            EFreeCameraMode::Perspective => {
                c_p_m.0 = match self.fixed_mode {
                    EFixedMode::VerticalFixed => CoordinateSytem3::perspective_lh(self.fov, aspect, self.minz, self.maxz, true),
                    EFixedMode::HorizontalFixed => CoordinateSytem3::perspective_lh(self.fov, aspect, self.minz, self.maxz, false),
                };
            },
            EFreeCameraMode::Orthograhic => {
                let value = self.orth_size;
                c_p_m.0 = match self.fixed_mode {
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
            },
        };
    }
}

#[derive(Debug)]
pub enum CameraCommand {
    Create(ObjectID),
    Destroy(ObjectID),
    ModifyMode(ObjectID, EFreeCameraMode),
    ModifyFov(ObjectID, Number),
    ModifyFixedMode(ObjectID, EFixedMode),
    ModifyOrthSize(ObjectID, Number),
}


#[derive(Debug, Default)]
pub struct SingleCameraCommandList {
    pub list: Vec<CameraCommand>,
}

pub struct SysCameraCommand;
#[setup]
impl SysCameraCommand {
    #[system]
    pub fn cmds(
        mut cmds: ResMut<SingleCameraCommandList>,
        mut cameras: Query<GameObject, (Write<CameraParam>, Write<CameraViewMatrix>, Write<CameraProjectionMatrix>, Write<CameraTransformMatrix>, Write<CameraGlobalPosition>, Write<CameraDirection>)>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                CameraCommand::Create(entity) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            camera.0.insert_no_notify(CameraParam::default());
                            camera.1.insert_no_notify(CameraViewMatrix::default());
                            camera.2.insert_no_notify(CameraProjectionMatrix::default());
                            camera.3.insert_no_notify(CameraTransformMatrix::default());
                            camera.4.insert_no_notify(CameraGlobalPosition::default());
                            camera.5.insert_no_notify(CameraDirection::default());
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::Destroy(entity) => {
                    entity_delete.despawn(entity);
                },
                CameraCommand::ModifyMode(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(camera) => {
                            // camera.get_or_default().mode = value;
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::ModifyFov(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            // camera.get_or_default().fov = value;
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::ModifyFixedMode(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            // camera.get_or_default().fixed_mode = value;
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::ModifyOrthSize(entity, value) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            match camera.0.get_mut() {
                                Some(camera) => {
                                    camera.orth_size = value;
                                },
                                None => todo!(),
                            }
                        },
                        None => todo!(),
                    }
                },
            }
        });

    }
}