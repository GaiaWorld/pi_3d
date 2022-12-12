use pi_ecs::{prelude::{ResMut, Query}, query::{With, Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::object::GameObject;
use pi_render::rhi::dyn_uniform_buffer::{BindOffset, Bind, Uniform};
use pi_scene_math::Matrix;

use crate::{shaders::FragmentUniformBind, resources::RenderDynUniformBuffer, transforms::{transform_node::GlobalTransform, dirty::DirtyGlobalTransform}, bytes_write_to_memory};

// pub struct BuildinTimeBind {
//     pub bind_offset: BindOffset,
// }
// impl BuildinTimeBind {
//     pub const TIME: usize = 4;
//     pub const DELTA_TIME: usize = 4;

//     pub const TIME_OFFSIZE: usize = 0 * 4;
//     pub const DELTA_TIME_OFFSIZE: usize = Self::TIME_OFFSIZE + Self::TIME * 4;

// }
// impl FragmentUniformBind for BuildinTimeBind {
//     const ID: u32 = 1;
//     const SIZE: usize = Self::DELTA_TIME_OFFSIZE + Self::DELTA_TIME * 4;
// }
// impl Bind for BuildinTimeBind {
//     fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
//         pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
//     }
//     fn min_size() -> usize {
//         Self::SIZE
//     }
// }

// pub struct BuildinFogBind {
//     pub bind_offset: BindOffset,
// }
// impl BuildinFogBind {
//     pub const FOG_PARAM: usize = 4;
//     pub const FOG_COLOR: usize = 4;

//     pub const FOG_PARAM_OFFSIZE: usize = 0 * 4;
//     pub const FOG_COLOR_OFFSIZE: usize = Self::FOG_PARAM_OFFSIZE + Self::FOG_PARAM_OFFSIZE * 4;
// }
// impl FragmentUniformBind for BuildinFogBind {
//     const ID: u32 = 2;
//     const SIZE: usize = Self::FOG_COLOR_OFFSIZE + Self::FOG_COLOR * 4;
// }
// impl Bind for BuildinFogBind {
//     fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
//         pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
//     }
//     fn min_size() -> usize {
//         Self::SIZE
//     }
// }

// pub struct BuildinAmbientLightBind {
//     pub bind_offset: BindOffset,
// }
// impl BuildinAmbientLightBind {
//     pub const AMBIENT_LIGHT: usize = 4;
//     pub const AMBIENT_LIGHT_OFFSIZE: usize = 0 * 4;
// }
// impl FragmentUniformBind for BuildinAmbientLightBind {
//     const ID: u32 = 3;
//     const SIZE: usize = Self::AMBIENT_LIGHT_OFFSIZE + Self::AMBIENT_LIGHT * 4;
// }
// impl Bind for BuildinAmbientLightBind {
//     fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
//         pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
//     }
//     fn min_size() -> usize {
//         Self::SIZE
//     }
// }

/// Model Uniform Bind
pub struct BuildinModelBind {
    pub bind_offset: BindOffset,
    pub matrix: Matrix,
    pub matrix_inv: Matrix,
}
impl BuildinModelBind {
    pub const OBJECT_TO_WORLD: usize = 16;
    pub const WORLD_TO_OBJECT: usize = 16;

    pub const OBJECT_TO_WORLD_OFFSIZE: usize = 0 * 4;
    pub const WORLD_TO_OBJECT_OFFSIZE: usize = Self::OBJECT_TO_WORLD_OFFSIZE + Self::WORLD_TO_OBJECT * 4;

    pub fn new(
        dynbuffer: &mut RenderDynUniformBuffer,
    ) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
            matrix: Matrix::identity(),
            matrix_inv: Matrix::identity(),
        }
    }
}
impl FragmentUniformBind for BuildinModelBind {
    const ID: u32 = 0;
    const SIZE: usize = Self::WORLD_TO_OBJECT_OFFSIZE + Self::WORLD_TO_OBJECT * 4;
}
impl Bind for BuildinModelBind {
    fn index() -> pi_render::rhi::dyn_uniform_buffer::BindIndex {
        pi_render::rhi::dyn_uniform_buffer::BindIndex::new(Self::ID as usize)
    }
    fn min_size() -> usize {
        Self::SIZE
    }
}
impl Uniform for BuildinModelBind {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.matrix.as_slice()), index as usize + BuildinModelBind::OBJECT_TO_WORLD_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.matrix_inv.as_slice()), index as usize + BuildinModelBind::WORLD_TO_OBJECT_OFFSIZE, buffer);
    }
}

pub struct SysModelUniformUpdate;
#[setup]
impl SysModelUniformUpdate {
    #[system]
    pub fn tick(
        meshes: Query<GameObject, &BuildinModelBind, Changed<BuildinModelBind>>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
         println!("DefaultMaterial Uniform TickUpdate");
        meshes.iter().for_each(|model| {
            dynbuffer.as_mut().set_uniform::<BuildinModelBind>(&model.bind_offset, model);
        });
    }
}