
use std::mem::replace;

use pi_engine_shell::prelude::*;
use pi_scene_math::Number;

use pi_scene_context::prelude::*;


pub enum DefaultMaterialCommand {
    EmissiveColor(ObjectID, (Number, Number, Number)),
    EmissiveIntensity(ObjectID, Number),
}
#[derive(Default)]
pub struct SingeDefaultMaterialCommandList {
    pub list: Vec<DefaultMaterialCommand>,
}
// pub struct SysDefaultMaterialCommand;
// impl TSystemStageInfo for SysDefaultMaterialCommand {
//     fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
//         vec![
//             SysMaterialMetaChange::<PassID01>::key()
//         ]
//     }
// }
// #[setup]
// impl SysDefaultMaterialCommand {
//     #[system]
//     pub fn cmd(
//         mut cmds: ResMut<SingeDefaultMaterialCommandList>,
//         mut materials: Query<
//             GameObject,
//             &mut BindEffectValues,
//             With<AssetResShaderEffectMeta>
//         >,
//     ) {
//         let mut list = replace(&mut cmds.list, vec![]);

//         list.drain(..).for_each(|cmd| {
//             match cmd {
//                 DefaultMaterialCommand::EmissiveColor(entity, color) => {
//                     match materials.get_mut(entity.clone()) {
//                         Some(mut prop) => {
//                             let a = prop.vec4_.value(0)[3];
//                             prop.vec4(0, &[color.0, color.1, color.2, a]);
//                         },
//                         None => {
//                             cmds.list.push(cmd);
//                         },
//                     }
//                 },
//                 DefaultMaterialCommand::EmissiveIntensity(entity, intensity) => {
//                     match materials.get_mut(entity) {
//                         Some(mut prop) => {
//                             let t = prop.vec4_.value(0);
//                             let r = t[0];
//                             let g = t[1];
//                             let b = t[2];
//                             prop.vec4(0, &[r, g, b, intensity]);
//                         },
//                         None => {
//                             cmds.list.push(cmd);
//                         },
//                     }
//                 },
//             }
//         });
//     }
// }