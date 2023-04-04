use pi_ecs::{prelude::{Query, Commands, Res}, query::{Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::{run_stage::TSystemStageInfo, object::GameObject};
use pi_render::{rhi::RenderQueue, render_3d::shader::skin_code::ESkinCode};

use crate::transforms::{transform_node_sys::SysWorldMatrixCalc, transform_node::WorldMatrix};

use super::{skeleton::Skeleton, SkeletonBonesDirty, SkeletonID};

pub struct SysSkinDirtyByBonesMatrix;
impl TSystemStageInfo for SysSkinDirtyByBonesMatrix {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysWorldMatrixCalc::key()
        ]
    }
}
#[setup]
impl SysSkinDirtyByBonesMatrix {
    #[system]
    fn sys(
        mut skeletons: Commands<GameObject, SkeletonBonesDirty>,
        bones: Query<GameObject, &SkeletonID, Changed<WorldMatrix>>,
    ) {
        bones.iter().for_each(|bone| {
            skeletons.insert(bone.0.clone(), SkeletonBonesDirty(true));
        });
    }
}

pub struct SysSkinTextureUpdate;
impl TSystemStageInfo for SysSkinTextureUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysSkinDirtyByBonesMatrix::key()
        ]
    }
}
#[setup]
impl SysSkinTextureUpdate {
    #[system]
    pub fn sys(
        mut items: Query<
            GameObject, 
            (
                &Skeleton,
                // Option<&SkinTexture>,
                &mut SkeletonBonesDirty
            ),
            Changed<SkeletonBonesDirty>
        >,
        bones: Query<GameObject, &WorldMatrix>,
        queue: Res<RenderQueue>,
    ) {
        items.iter_mut().for_each(|(skel, mut skindirty)| {
            match skel.mode {
                ESkinCode::None => {},
                ESkinCode::UBO(_, _) => {
                    let mut data = vec![];
                    skel.bones.iter().for_each(|bone| {
                        if let Some(matrix) = bones.get(bone.clone()) {
                            matrix.0.as_slice().iter().for_each(|v| {
                                data.push(*v);
                            });
                        }
                    });

                    skel.bind.data().write_data(0, bytemuck::cast_slice(&data));
                },
                ESkinCode::RowTexture(_) => {
                    // if let Some(tex) = tex {
                    //     let mut data = vec![];
                    //     skel.bones.iter().for_each(|bone| {
                    //         if let Some(matrix) = bones.get(bone.clone()) {
                    //             matrix.0.as_slice().iter().for_each(|v| {
                    //                 data.push(*v);
                    //             });
                    //         }
                    //     });
    
                    //     let mut buff_data = tex.tex.create_data();
    
                    //     log::debug!("Skeleton Tex: {:?}, {:?}", tex.tex.size(), buff_data.len());
            
                    //     tex.tex.update_row(0, bytemuck::cast_slice(data.as_slice()), &mut buff_data);
    
                    //     tex.tex.update_texture(&queue, buff_data.as_slice());
                    // }
                },
                ESkinCode::FramesTexture(_) => {},
            }
            

            skindirty.0 = false;
        });
    }
}