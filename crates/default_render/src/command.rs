
use std::mem::replace;

use pi_ecs::{prelude::{ResMut, Query, Commands}, query::{Write, With}};
use pi_ecs_macros::setup;
use pi_engine_shell::run_stage::TSystemStageInfo;
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
impl TSystemStageInfo for SysDefaultMaterialCommand {}
#[setup]
impl SysDefaultMaterialCommand {
    #[system]
    pub fn cmd(
        mut cmds: ResMut<SingeDefaultMaterialCommandList>,
        mut materials: Query<
            GameObject,
            &mut Vec4Uniform,
            With<AssetResShaderEffectMeta>
        >,
    ) {
        let mut list = replace(&mut cmds.list, vec![]);

        list.drain(..).for_each(|cmd| {
            match cmd {
                DefaultMaterialCommand::EmissiveColor(entity, color) => {
                    match materials.get_mut(entity.clone()) {
                        Some(mut prop) => {
                            let a = prop.value(0)[3];
                            prop.set(0, &[color.0, color.1, color.2, a]);
                        },
                        None => {
                            cmds.list.push(cmd);
                        },
                    }
                },
                DefaultMaterialCommand::EmissiveIntensity(entity, intensity) => {
                    match materials.get_mut(entity) {
                        Some(mut prop) => {
                            let t = prop.value(0);
                            let r = t[0];
                            let g = t[1];
                            let b = t[2];
                            prop.set(0, &[r, g, b, intensity]);
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