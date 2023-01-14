use pi_ecs::prelude::Query;
use pi_engine_shell::{object::{ObjectID, GameObject}, run_stage::TSystemStageInfo};
use pi_render::rhi::device::RenderDevice;
use render_resource::data_texture2d::DataTexture2D;

use crate::{transforms::{transform_node::WorldMatrix, transform_node_sys::SysWorldMatrixCalc}, skeleton::skeleton::Skeleton};



pub struct SkinRowTexture {
    pub tex: DataTexture2D,
}

impl SkinRowTexture {
    pub fn new(device: &RenderDevice, bone_count: u32, data: Option<&[u8]>) -> Self {
        let mut tex = DataTexture2D::new_rgba_f32(device, (bone_count + 1) * 4, 1);

        if let Some(data) = data {
            tex.update_row(0, data);
        }

        Self {
            tex,
        }
    }
}

pub struct SysSkinRowTextureUpdate;
impl TSystemStageInfo for SysSkinRowTextureUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysWorldMatrixCalc::key()
        ]
    }
}
impl SysSkinRowTextureUpdate {
    pub fn sys(
        mut items: Query<GameObject, (&Skeleton, &mut SkinRowTexture)>,
        bones: Query<GameObject, &WorldMatrix>,
    ) {
        items.iter_mut().for_each(|(skel, mut tex)| {
            let mut data = vec![];
            skel.bones.iter().for_each(|bone| {
                if let Some(matrix) = bones.get(bone.clone()) {
                    matrix.0.as_slice().iter().for_each(|v| {
                        data.push(*v);
                    });
                }
            });

            tex.tex.update_row(0, bytemuck::cast_slice(data.as_slice()));
        });
    }
}