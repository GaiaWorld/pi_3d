use pi_ecs::{prelude::{ResMut, Query, Res, Commands}, query::{With, Changed, Or, Write}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::rhi::{dyn_uniform_buffer::{BindOffset, Bind, Uniform, DynUniformBuffer}, device::RenderDevice, RenderQueue};
use pi_scene_math::Matrix;
use render_data_container::{VertexBufferPool, VertexBuffer};
use render_geometry::vertex_data::EVertexDataKind;
use render_resource::uniform_buffer::RenderDynUniformBuffer;
use render_shader::{shader_set::{ShaderSetModelAbout, ShaderSetModelAboutBindOffset}, skin_code::ESkinCode, set_bind::ShaderSetBind, instance_code::EInstanceCode};

use crate::{
    shaders::FragmentUniformBind,  bytes_write_to_memory,
    transforms::{transform_node::{WorldMatrix, WorldMatrixInv}, transform_node_sys::SysWorldMatrixCalc},
    geometry::instance::{types::{TInstancedData, InstancedValue, TInstanceFlag}, InstanceSource, instance_world_matrix::InstancedWorldMatrixDirty},
    bindgroup::{RenderBindGroupKey, RenderBindGroupPool}
};

use super::{abstract_mesh::AbstructMesh, command::SysMeshCommand, Mesh};

/// 对应 EModelAboutBind::ModelMatrix
/// 必须,固定使用,bind固定
pub struct BuildinModelBind {
    pub bind_offset: BindOffset,
}
impl BuildinModelBind {
    pub const OBJECT_TO_WORLD: usize = 16;
    pub const WORLD_TO_OBJECT: usize = 16;

    pub const OBJECT_TO_WORLD_OFFSIZE: usize = 0 * 4;
    pub const WORLD_TO_OBJECT_OFFSIZE: usize = Self::OBJECT_TO_WORLD_OFFSIZE + Self::WORLD_TO_OBJECT * 4;

    pub fn new(
        dynbuffer: &mut render_resource::uniform_buffer::RenderDynUniformBuffer,
    ) -> Self {
        Self {
            bind_offset: dynbuffer.alloc_binding::<Self>(),
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

pub struct RenderMatrixDirty(pub bool);
impl TInstanceFlag for RenderMatrixDirty {
    fn dirty(&self) -> bool {
        self.0
    }

    fn reset(&mut self) {
        self.0 = false
    }
}

#[derive(Debug, Clone)]
pub struct RenderWorldMatrix(pub Matrix);
impl RenderWorldMatrix {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}
impl TInstancedData for RenderWorldMatrix {
    fn vertex_kind(&self) -> EVertexDataKind {
        EVertexDataKind::InsWorldRow1
    }

    fn value(&self) -> &InstancedValue {
        todo!()
    }

    fn size() -> usize {
        16
    }

    fn bytes_size() -> usize {
        16 * 4
    }

    fn local_offset(&self) -> usize {
        0
    }

    fn write_instance_buffer(&self, buffer: &mut VertexBuffer, offset: usize) {
        buffer.update_f32(self.0.as_slice(), offset);
    }
}

#[derive(Debug, Clone)]
pub struct RenderWorldMatrixInv(pub Matrix);
impl RenderWorldMatrixInv {
    pub fn new(m: Matrix) -> Self {
        Self(m)
    }
}


pub struct BuildinModelTemp<'a>(pub &'a Matrix, pub &'a Matrix);
impl<'a> Uniform for BuildinModelTemp<'a> {
    fn write_into(&self, index: u32, buffer: &mut [u8]) {
        bytes_write_to_memory(bytemuck::cast_slice(self.0.as_slice()), index as usize + BuildinModelBind::OBJECT_TO_WORLD_OFFSIZE, buffer);
        bytes_write_to_memory(bytemuck::cast_slice(self.1.as_slice()), index as usize + BuildinModelBind::WORLD_TO_OBJECT_OFFSIZE, buffer);
    }
}

pub struct SysModelAboutUpdate;
impl TSystemStageInfo for SysModelAboutUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysMeshCommand::key(), // Skin
        ]
    }
}
#[setup]
impl SysModelAboutUpdate {
    #[system]
    pub fn sys(
        models: Query<
            GameObject,
            (ObjectID, &Mesh, Option<&ESkinCode>),
            Or<(Changed<Mesh>, Changed<ESkinCode>)>,
        >,
        mut about_cmd: Commands<GameObject, ShaderSetModelAbout>,
        mut about_bindoff_cmd: Commands<GameObject, ShaderSetModelAboutBindOffset>,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
    ) {
        log::debug!("SysModelAboutUpdateBySkin: ");
        models.iter().for_each(|(entity, _, skin)| {
            log::debug!("SysModelAboutUpdateBySkin: 0");
            let skin = if let Some(skin) = skin {
                Some(*skin)
            } else {
                None
            };
            let model_set = ShaderSetModelAbout::new(ShaderSetBind::SET_MODEL_ABOUT, skin);
            let model_bindoff = model_set.bind_offset(&mut dynbuffer);
            
            let layout_entries = model_set.layout_entries();
            bindgrouppool.creat(&device, RenderBindGroupKey::ModelAbout(entity.clone()), layout_entries.as_slice(), ShaderSetBind::SET_MODEL_ABOUT);

            about_cmd.insert(entity.clone(), model_set);
            about_bindoff_cmd.insert(entity.clone(), model_bindoff);
        });

    }
}

pub struct SysRenderMatrixUpdate;
impl TSystemStageInfo for SysRenderMatrixUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysWorldMatrixCalc::key()
        ]
    }
}
#[setup]
impl SysRenderMatrixUpdate {
    #[system]
    pub fn tick(
        mut source_mesh: Query<GameObject, &mut InstancedWorldMatrixDirty>,
        mut meshes: Query<
            GameObject,
            (ObjectID, &AbstructMesh, &WorldMatrix, &WorldMatrixInv, &mut RenderWorldMatrix, &mut RenderWorldMatrixInv, &mut RenderMatrixDirty, Option<&InstanceSource>),
            Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>)>
        >,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
    ) {
        meshes.iter_mut().for_each(|(obj, _, worldmatrix, worldmatrix_inv, mut render_wm, mut render_wminv, mut dirty, id_source)| {
            // log::debug!("SysModelUniformUpdate:");
            render_wm.0.clone_from(&worldmatrix.0);
            render_wminv.0.clone_from(&worldmatrix_inv.0);
            dirty.0 = true;

            if let Some(id_source) = id_source {
                if let Some(mut flag) = source_mesh.get(id_source.0) {
                    flag.0 = true;
                }
            }
        });
    }
}

pub struct SysRenderMatrixUniformUpdate;
impl TSystemStageInfo for SysRenderMatrixUniformUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysRenderMatrixUpdate::key()
        ]
    }
}
#[setup]
impl SysRenderMatrixUniformUpdate {
    #[system]
    pub fn tick(
        mut meshes: Query<GameObject, (&ShaderSetModelAboutBindOffset, &RenderWorldMatrix, &RenderWorldMatrixInv, &mut RenderMatrixDirty)>,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
    ) {
        meshes.iter_mut().for_each(|(model, worldmatrix, worldmatrix_inv, mut flag)| {
            log::info!("SysModelUniformUpdate:");

            if flag.0 {
                let temp = BuildinModelTemp(&worldmatrix.0, &worldmatrix_inv.0);
                dynbuffer.as_mut().set_uniform::<BuildinModelTemp>(model.matrix().bind_offset(), &temp);
                flag.0 = false;
            }
        });
    }
}