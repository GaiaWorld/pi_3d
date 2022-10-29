use pi_ecs::{prelude::{ResMut, Query, EntityDelete}, query::Write};
use pi_ecs_macros::setup;
use pi_render::rhi::{dyn_uniform_buffer::{Uniform, BindOffset, Bind}};
use pi_scene_math::{Vector3, Matrix};

use crate::{bytes_write_to_memory, shaders::{FragmentUniformBind}, object::{ObjectID, GameObject}, resources::RenderDynUniformBuffer};

// #[derive(Debug, Clone)]
// pub struct ViewMatrix(pub Matrix);
// impl Default for ViewMatrix {
//     fn default() -> Self {
//         Self(Matrix::default())
//     }
// }
// impl Uniform for ViewMatrix {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + CameraRenderData::PI_MATRIX_V_OFFSIZE, buffer);
//     }
// }
// #[derive(Debug, Clone)]
// pub struct ProjectionMatrix(pub Matrix);
// impl Default for ProjectionMatrix {
//     fn default() -> Self {
//         Self(Matrix::default())
//     }
// }
// impl Uniform for ProjectionMatrix {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + CameraRenderData::PI_MATRIX_P_OFFSIZE, buffer);
//     }
// }
// #[derive(Debug, Clone)]
// pub struct TransformMatrix(pub Matrix);
// impl Default for TransformMatrix {
//     fn default() -> Self {
//         Self(Matrix::default())
//     }
// }
// #[derive(Debug, Clone)]
// pub struct CameraGlobalPosition(pub Vector3);
// impl Default for CameraGlobalPosition {
//     fn default() -> Self {
//         Self(Vector3::new(0., 0., 0.))
//     }
// }
// impl Uniform for CameraGlobalPosition {
//     fn write_into(&self, index: u32, buffer: &mut [u8]) {
//         bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + CameraRenderData::PI_CAMERA_POSITION_OFFSIZE, buffer);
//     }
// }

pub struct CameraRenderData {
    pub view_matrix: Matrix,
    pub project_matrix: Matrix,
    pub transform_matrix: Matrix,
    pub global_position: Vector3,
    pub camera_direction: Vector3,
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
            view_matrix: Matrix::identity(),
            project_matrix: Matrix::identity(),
            transform_matrix: Matrix::identity(),
            global_position: Vector3::new(0., 0., -1.),
            camera_direction: Vector3::new(0., 0., 1.),
            bind_offset: dynbuffer.alloc_binding::<Self>(),
        }
    }
}
impl Uniform for CameraRenderData {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.view_matrix.transpose().as_slice()), index as usize + Self::PI_MATRIX_V_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.project_matrix.transpose().as_slice()), index as usize + Self::PI_MATRIX_P_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.transform_matrix.transpose().as_slice()), index as usize + Self::PI_MATRIX_VP_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.global_position.as_slice()), index as usize + Self::PI_CAMERA_POSITION_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.camera_direction.as_slice()), index as usize + Self::PI_ORTHCAMERA_DIRECT, buffer);
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

#[derive(Debug, Clone)]
pub struct CameraParam {
    pub up: Vector3,
    pub minz: f32,
    pub maxz: f32,
}
impl Default for CameraParam {
    fn default() -> Self {
        Self { up: Vector3::new(0., 1., 0.), minz: 0.01, maxz: 1000. }
    }
}

#[derive(Debug)]
pub enum CameraCommand {
    Create(ObjectID),
    Destroy(ObjectID),
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
        mut cameras: Query<GameObject, Write<CameraParam>>,
        mut entity_delete: EntityDelete<GameObject>,
    ) {
        cmds.list.drain(..).for_each(|cmd| {
            match cmd {
                CameraCommand::Create(entity) => {
                    match cameras.get_mut(entity) {
                        Some(mut camera) => {
                            camera.insert_no_notify(CameraParam::default());
                        },
                        None => todo!(),
                    }
                },
                CameraCommand::Destroy(entity) => {
                    entity_delete.despawn(entity);
                },
            }
        });

    }
}