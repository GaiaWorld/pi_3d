
use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query}, query::{Write, With}};
use pi_ecs_macros::setup;
use pi_scene_math::Number;

use pi_scene_context::{object::{ObjectID, GameObject}, materials::{uniforms::vec4::{Vec4Uniform}, shader_effect::AssetResShaderEffectMeta}};


pub enum DefaultMaterialCommand {
    EmissiveColor(ObjectID, (Number, Number, Number)),
    EmissiveIntensity(ObjectID, Number),
}
#[derive(Default)]
pub struct SingeDefaultMaterialCommandList {
    pub list: Vec<DefaultMaterialCommand>,
}
pub struct SysDefaultMaterialCommand;
#[setup]
impl SysDefaultMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingeDefaultMaterialCommandList>,
        mut materials: Query<
            GameObject,
            Write<Vec4Uniform>,
            With<AssetResShaderEffectMeta>
        >,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                DefaultMaterialCommand::EmissiveColor(entity, color) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            if let Some(prop) = mat.get_mut() {
                                let a = prop.value(0)[3];
                                prop.set(0, &[color.0, color.1, color.2, a]);
                                mat.notify_modify();
                            } else {
                                cmds.list.push(cmd);
                            }
                        },
                        None => {
                            cmds.list.push(cmd);
                        },
                    }
                },
                DefaultMaterialCommand::EmissiveIntensity(entity, intensity) => {
                    match materials.get_mut(entity) {
                        Some(mut mat) => {
                            if let Some(prop) = mat.get_mut() {
                                let t = prop.value(0);
                                let r = t[0];
                                let g = t[1];
                                let b = t[2];
                                prop.set(0, &[r, g, b, intensity]);
                                mat.notify_modify();
                            } else {
                                cmds.list.push(cmd);
                            }
                        },
                        None => {
                            cmds.list.push(cmd);
                        },
                    }
                },
            }
        });
    }
}