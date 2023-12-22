
use pi_engine_shell::prelude::*;
use pi_scene_math::Number;

use pi_scene_context::prelude::*;

#[derive(Debug, Clone)]
pub enum EUnlitMaterialCommand {
    EmissiveColor(ObjectID, (Number, Number, Number)),
    EmissiveIntensity(ObjectID, Number),
    EmissiveTexture(ObjectID, UniformTextureWithSamplerParam),
}

pub type ActionListUnlitMaterial = ActionList<EUnlitMaterialCommand>;
pub fn sys_act_unlit_material(
    mut cmds: ResMut<ActionListUnlitMaterial>,
    material_vec4: Query<
        &BindEffect,
        With<AssetResShaderEffectMeta>
    >,
) {
    // cmds.drain().drain(..).for_each(|cmd| {
    //     match cmd {
    //         EUnlitMaterialCommand::EmissiveColor(entity, color) => {
    //             match material_vec4.get_mut(entity) {
    //                 Ok(mut valueuniform) => {
    //                     let valueuniform = if let Some(valueuniform) = &mut valueuniform.0 { valueuniform } else { return; };
    //                     let a = valueuniform.vec4_.value(0)[3];
    //                     valueuniform.vec4(0, &[color.0, color.1, color.2, a]);
    //                 },
    //                 _ => {
    //                     cmds.push(cmd.clone());
    //                 },
    //             }
    //         },
    //         EUnlitMaterialCommand::EmissiveIntensity(entity, intensity) => {
    //             match material_vec4.get_mut(entity) {
    //                 Ok(mut valueuniform) => {
    //                     let valueuniform = if let Some(valueuniform) = &mut valueuniform.0 { valueuniform } else { return; };
    //                     let t = valueuniform.vec4_.value(0);
    //                     let r = t[0];
    //                     let g = t[1];
    //                     let b = t[2];
    //                     valueuniform.vec4(0, &[r, g, b, intensity]);
    //                 },
    //                 _ => {
    //                     cmds.push(cmd.clone());
    //                 },
    //             }
    //         },
    //         EUnlitMaterialCommand::EmissiveTexture(_entity, _imagepath) => {
    //             // commands.entity(entity).insert(TextureSlot01(Arc::new(imagepath)));
    //         },
    //     }
    // });
}
