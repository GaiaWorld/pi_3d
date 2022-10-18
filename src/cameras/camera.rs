use pi_render::rhi::{dyn_uniform_buffer::{Uniform, DynUniformBuffer, BindOffset, Bind}, bind_group::BindGroup, device::RenderDevice, bind_group_layout::BindGroupLayout};
use pi_scene_math::{Vector3, Vector4, Matrix, frustum::FrustumPlanes, plane::Plane, Point3, Isometry3, Translation3, Orthographic3};

use crate::{materials::bytes_write_to_memory, shaders::{FragmentUniformBind}, environment::{fog::SceneFog, ambient_light::AmbientLight}, scene::SceneTime};

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
    pub const PI_CAMERA_POSITION: usize = 4;
    pub const PI_ORTHCAMERA_DIRECT: usize = 4;

    pub const PI_MATRIX_V_OFFSIZE: usize = 0 * 4;
    pub const PI_MATRIX_P_OFFSIZE: usize = Self::PI_MATRIX_V_OFFSIZE + Self::PI_MATRIX_V * 4;
    pub const PI_CAMERA_POSITION_OFFSIZE: usize = Self::PI_MATRIX_P_OFFSIZE + Self::PI_MATRIX_P * 4;
    pub const PI_ORTHCAMERA_DIRECT_OFFSIZE: usize = Self::PI_CAMERA_POSITION_OFFSIZE + Self::PI_CAMERA_POSITION * 4;

    pub fn new(
        dynbuffer: &mut DynUniformBuffer,
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
        bytes_write_to_memory(bytemuck::cast_slice(self.view_matrix.as_slice()), index as usize + Self::PI_MATRIX_V_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.project_matrix.as_slice()), index as usize + Self::PI_MATRIX_P_OFFSIZE, buffer);
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

pub struct MainCameraBindGroup {
    pub bind_group: Option<BindGroup>,
    pub set: u32,
}
impl MainCameraBindGroup {
    pub fn init(
        &mut self,
        device: &RenderDevice,
        dynbuffer: &mut DynUniformBuffer,
    ) {
        let camera_bind = dynbuffer.alloc_binding::<CameraRenderData>();
        let fog_bind = dynbuffer.alloc_binding::<SceneFog>();
        let time_bind = dynbuffer.alloc_binding::<SceneTime>();
        let ambient_light_bind = dynbuffer.alloc_binding::<AmbientLight>();

        let bind_group_0_layout = BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some("BuildinBindGroup"),
                    entries: &[
                        CameraRenderData::ENTRY,
                        SceneTime::ENTRY,
                        SceneFog::ENTRY,
                        AmbientLight::ENTRY,
                    ],
                }
            )
        );
        
        let bind_group_0 = BindGroup::from(
            device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    label: Some("BuildinBindGroup"),
                    layout: &bind_group_0_layout,
                    entries: &[
                        CameraRenderData::entry(&camera_bind, dynbuffer),
                        SceneTime::entry(&time_bind, dynbuffer),
                        SceneFog::entry(&fog_bind, dynbuffer),
                        AmbientLight::entry(&ambient_light_bind, dynbuffer),
                    ],
                }
            )
        );

        self.bind_group = Some(bind_group_0);
    }
}

// 初始方向为z轴正方向
pub struct Camera {
    pub up: Vector3,
    pub minz: f32,
    pub maxz: f32,
    /// 
    /// * Define the default inertia of the camera.
    /// * This helps giving a smooth feeling to the camera movement.
    pub inertia: f32,
    pub viewport: Vector4,
    pub view_matrix: Matrix,
    pub project_matrix: Matrix,
    pub transform_matrix: Matrix,
    pub global_position: Translation3,
}


impl Default for Camera {
    fn default() -> Self {
        Self {
            up: Vector3::new(0., 1., 0.),
            minz: 0.1,
            maxz: 1000.,
            inertia: 0.7,
            viewport: Vector4::new(0., 0., 1., 1.),
            view_matrix: Matrix::identity(),
            project_matrix: Matrix::identity(),
            transform_matrix: Matrix::identity(),
            global_position: Translation3::new(0., 0., 0.),
        }
    }
}