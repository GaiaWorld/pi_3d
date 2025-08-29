use pi_scene_shell::prelude::*;

use crate::prelude::{ActionListInstanceAttr, FlagRenderWorldMatrix, OpsInstanceAttr, RenderPoseMatrix};

use super::{command::{ActionListSpriteCreate, ActionListSpriteModify, OpsSpriteCreate, OpsSpriteModify}, Sprite};

pub fn sys_create_sprite(
    mut cmds: ResMut<ActionListSpriteCreate>,
    mut commands: Commands,
) {
    cmds.drain().for_each(|OpsSpriteCreate(_mesh, sprite, atlas)| {
        if let Some(mut entitycmd) = commands.get_entity(sprite) {
            entitycmd.insert((
                RenderPoseMatrix::default(),
                Sprite {
                    atlas: Some(atlas)
                }
            ));
        }
    })
}

pub fn sys_modify_sprite(
    mut cmds: ResMut<ActionListSpriteModify>,
    mut items: Query<&mut RenderPoseMatrix>,
    // sprites: Query<&Sprite>,
    // atlasmgr: Res<TextureFrameAtlasManager>,
    mut cmdsfloat: ResMut<ActionListInstanceAttr>,
    mut flagrendermatrix: Query<&mut FlagRenderWorldMatrix>,
    sprites: Res<ResSpriteFrames>,
) {
    let rotmat = Rotation3::from_euler_angles(0., 0., -std::f32::consts::PI * 0.5);
    let mut tmp = Matrix::identity();
    let mut tempparent = Matrix::identity();
    let mut tempscaline = Vector3::zeros();
    let mut tempposition = Vector3::zeros();
    let max = u16::MAX as f32;
    cmds.drain().for_each(|OpsSpriteModify(entity, keyframe, tilloffkey)| {
        if let Ok(mut posematrix) = items.get_mut(entity) {
            let frame = match keyframe {
                super::SpriteModify::Idx(keyframe) => {
                    if let Some(spriteinfo) = sprites.0.get(keyframe as usize) {
                        spriteinfo.clone()
                    } else {
                        return;
                    }
                },
                super::SpriteModify::Data(data) => {
                    Some(SpriteFrame::from_data(&data.as_slice()))
                }
            };

            let frame = if let Some(frame) = frame {
                frame
            } else { return };

            let atlaswidth = frame.w;
            let atlasheight = frame.h;
            let mut su = frame.frame_w as f32 / atlaswidth   as f32;
            let mut sv = frame.frame_h as f32 / atlasheight  as f32;
            let ou = frame.frame_x as f32 / atlaswidth   as f32;
            let ov = frame.frame_y as f32 / atlasheight  as f32;
            if frame.rotated {
                su = frame.frame_h as f32 / atlaswidth   as f32;
                sv = frame.frame_w as f32 / atlasheight  as f32;
            }
            let sx = frame.sprite_source_size_w as f32 / frame.source_size_w as f32;
            let sy = frame.sprite_source_size_h as f32 / frame.source_size_h as f32;
            let dx = frame.sprite_source_size_x as f32 / frame.source_size_w as f32;
            let dy = (frame.source_size_h as f32 - frame.sprite_source_size_y as f32 - frame.sprite_source_size_h as f32) / frame.source_size_h as f32;
        
            let su = (su * max).round().min(max).max(0.) as u16;
            let sv = (sv * max).round().min(max).max(0.) as u16;
            let ou = (ou * max).round().min(max).max(0.) as u16;
            let ov = (ov * max).round().min(max).max(0.) as u16;
            cmdsfloat.push(OpsInstanceAttr::ops(entity, crate::prelude::EInstanceAttr::U16x4([su, sv, ou, ov]), tilloffkey));

            tempposition.x = dx - 0.5;
            tempposition.y = -dy + 0.5;
            tempposition.z = 0.;

            tempscaline.x = sx;
            tempscaline.y = sy;
            tempscaline.z = 1.;
            {
                pi_scene_shell::prelude::matrix4_compose_no_rotation(&tempscaline, &tempposition, &mut tempparent);
                // tempparent.fill_with_identity();
                // tempparent.append_nonuniform_scaling_mut(&tempscaline);
                // tempparent.append_translation_mut(&tempposition);
            }
            {
                tmp.fill_with_identity();
                if frame.rotated {
                    tmp.fixed_view_mut::<3, 3>(0, 0).copy_from(rotmat.matrix());
                    // posematrix.0.copy_from(&rotmat.to_homogeneous());
                }
                tempposition.x = 0.5;tempposition.y = -0.5; tempposition.z = 0.;
                tmp.append_translation_mut(&tempposition);
            }
            CoordinateSytem3::mul_to(&tempparent, &tmp, &mut posematrix.0);
            posematrix.1 = true;
            // posematrix.0 = tempparent * posematrix.0;

            if let Ok(mut flag) = flagrendermatrix.get_mut(entity) {
                *flag = FlagRenderWorldMatrix;
            }
        }
    });
}