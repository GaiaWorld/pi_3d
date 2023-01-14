use pi_ecs::prelude::Query;
use pi_ecs_macros::setup;
use pi_engine_shell::{run_stage::TSystemStageInfo, object::GameObject};

use crate::transforms::{transform_node_sys::SysWorldMatrixCalc, transform_node::WorldMatrix};

use super::{skeleton::Skeleton, skin_texture::SkinTexture};


pub struct SysSkinTextureUpdate;
impl TSystemStageInfo for SysSkinTextureUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysWorldMatrixCalc::key()
        ]
    }
}
#[setup]
impl SysSkinTextureUpdate {
    #[system]
    pub fn sys(
        mut items: Query<GameObject, (&Skeleton, &mut SkinTexture)>,
        bones: Query<GameObject, &WorldMatrix>,
    ) {
        items.iter_mut().for_each(|(skel, mut tex)| {
            
            match skel.mode {
                render_shader::skin_code::ESkinCode::None => {},
                render_shader::skin_code::ESkinCode::RowTexture(_) => {
                    let mut data = vec![];
                    skel.bones.iter().for_each(|bone| {
                        if let Some(matrix) = bones.get(bone.clone()) {
                            matrix.0.as_slice().iter().for_each(|v| {
                                data.push(*v);
                            });
                        }
                    });
        
                    tex.tex.update_row(0, bytemuck::cast_slice(data.as_slice()));
                },
                render_shader::skin_code::ESkinCode::FramesTexture(_) => {},
            }
        });
    }
}