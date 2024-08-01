use pi_scene_shell::prelude::*;

use crate::prelude::{ActionListInstanceAttr, OpsInstanceAttr, RenderPoseMatrix};

use super::{command::{ActionListSpriteCreate, ActionListSpriteModify, OpsSpriteCreate, OpsSpriteModify}, Sprite};

pub fn sys_create_sprite(
    mut cmds: ResMut<ActionListSpriteCreate>,
    mut commands: Commands,
    atlasmgr: Res<TextureFrameAtlasManager>,
) {
    cmds.drain().drain(..).for_each(|OpsSpriteCreate(mesh, sprite, atlas)| {
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
    mut sprites: Query<(&Sprite, &mut RenderPoseMatrix)>,
    atlasmgr: Res<TextureFrameAtlasManager>,
    mut cmdsfloat: ResMut<ActionListInstanceAttr>,
) {
    cmds.drain().drain(..).for_each(|OpsSpriteModify(entity, keyframe)| {
        if let Ok((spriteinfo, mut posematrix)) = sprites.get_mut(entity) {
            if let Some(keyatlas) = &spriteinfo.atlas {
                if let Some(atlas) = atlasmgr.get(keyatlas) {
                    if let Some(frame) = atlas.get_frame_by_idx(keyframe) {
                        
                        let su = frame.frame_w as f32 / atlas.width   as f32;
                        let sv = frame.frame_h as f32 / atlas.height  as f32;
                        let ou = frame.frame_x as f32 / atlas.width   as f32;
                        let ov = frame.frame_y as f32 / atlas.height  as f32;
                        let dx = frame.sprite_source_size_x as f32 / frame.source_size_w as f32;
                        let dy = frame.sprite_source_size_y as f32 / frame.source_size_h as f32;
                        let dw = frame.sprite_source_size_w as f32 / frame.source_size_w as f32;
                        let dh = frame.sprite_source_size_h as f32 / frame.source_size_h as f32;
        
                        cmdsfloat.push(OpsInstanceAttr::ops(entity, crate::prelude::EInstanceAttr::Vec4([su, sv, ou, ov]), Atom::from("InsTilloff")));
        
                        let translation = Vector3::new(
                            (dx + dw * 0.5) - 0.5,
                            (-dy - dh * 0.5) + 0.5,
                            0.
                        );
                        let mut scaling = Vector3::new(
                            dw,
                            dh,
                            1.
                        );
        
                        let mut rotation = Vector3::zeros();
                        if frame.rotated {
                            rotation.z = std::f32::consts::PI * 0.5;
                            scaling.x = dh;
                            scaling.y = dw;
                        }
        
                        CoordinateSytem3::matrix4_compose_euler_angle(&scaling, &rotation, &translation, &mut posematrix.0);
                    }
                }
            }
        }
    });
}