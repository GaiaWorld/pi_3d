
use pi_engine_shell::prelude::*;

use crate::materials::shader_effect::*;

    pub fn sys_texture_ready07(
        mut items: Query<
            (
                ObjectID,
                &AssetResShaderEffectMeta
                , (&EffectBindTexture2D01Comp, &EffectBindSampler2D01Comp)
                , (&EffectBindTexture2D02Comp, &EffectBindSampler2D02Comp)
                , (&EffectBindTexture2D03Comp, &EffectBindSampler2D03Comp)
                , (&EffectBindTexture2D04Comp, &EffectBindSampler2D04Comp)
                , (&EffectBindTexture2D05Comp, &EffectBindSampler2D05Comp)
                , (&EffectBindTexture2D06Comp, &EffectBindSampler2D06Comp)
                , (&EffectBindTexture2D07Comp, &EffectBindSampler2D07Comp)
                , (&EffectBindTexture2D08Comp, &EffectBindSampler2D08Comp)
                , &mut EffectTextureSamplersComp
            ),
            Or<(
                  Or<(Changed<EffectBindTexture2D01Comp>, Changed<EffectBindSampler2D01Comp>)>
                , Or<(Changed<EffectBindTexture2D02Comp>, Changed<EffectBindSampler2D02Comp>)>
                , Or<(Changed<EffectBindTexture2D03Comp>, Changed<EffectBindSampler2D03Comp>)>
                , Or<(Changed<EffectBindTexture2D04Comp>, Changed<EffectBindSampler2D04Comp>)>
                , Or<(Changed<EffectBindTexture2D05Comp>, Changed<EffectBindSampler2D05Comp>)>
                , Or<(Changed<EffectBindTexture2D06Comp>, Changed<EffectBindSampler2D06Comp>)>
                , Or<(Changed<EffectBindTexture2D07Comp>, Changed<EffectBindSampler2D07Comp>)>
                , Or<(Changed<EffectBindTexture2D08Comp>, Changed<EffectBindSampler2D08Comp>)>
            )>
        >,
    ) {
        items.iter_mut().for_each(|(
            _entity, binddesc
            , (tex00, sampl00)
            , (tex01, sampl01)
            , (tex02, sampl02)
            , (tex03, sampl03)
            , (tex04, sampl04)
            , (tex05, sampl05)
            , (tex06, sampl06)
            , (tex07, sampl07)
            , mut comp
        )| {
            let need = binddesc.textures.len() as u32;
            let mut texsamplerarr =  EffectTextureSamplers::default();

            if let (Some(v1), Some(v2)) = (&tex00.0, &sampl00.0) {
                texsamplerarr.textures.push(v1.clone()); texsamplerarr.samplers.push(v2.clone());
                if 1 == need { *comp = EffectTextureSamplersComp( Some( texsamplerarr ) ); return; }
            } else { comp.0 = None; return; }
            
            if let (Some(v1), Some(v2)) = (&tex01.0, &sampl01.0) {
                texsamplerarr.textures.push(v1.clone()); texsamplerarr.samplers.push(v2.clone());
                if 2 == need { *comp = EffectTextureSamplersComp( Some( texsamplerarr ) ); return; }
            } else { comp.0 = None; return; }
            
            if let (Some(v1), Some(v2)) = (&tex02.0, &sampl02.0) {
                texsamplerarr.textures.push(v1.clone()); texsamplerarr.samplers.push(v2.clone());
                if 3 == need { *comp = EffectTextureSamplersComp( Some( texsamplerarr ) ); return; }
            } else { comp.0 = None; return; }

            if let (Some(v1), Some(v2)) = (&tex03.0, &sampl03.0) {
                texsamplerarr.textures.push(v1.clone()); texsamplerarr.samplers.push(v2.clone());
                if 4 == need { *comp = EffectTextureSamplersComp( Some( texsamplerarr ) ); return; }
            } else { comp.0 = None; return; }
            
            if let (Some(v1), Some(v2)) = (&tex04.0, &sampl04.0) {
                texsamplerarr.textures.push(v1.clone()); texsamplerarr.samplers.push(v2.clone());
                if 5 == need { *comp = EffectTextureSamplersComp( Some( texsamplerarr ) ); return; }
            } else { comp.0 = None; return; }
            
            if let (Some(v1), Some(v2)) = (&tex05.0, &sampl05.0) {
                texsamplerarr.textures.push(v1.clone()); texsamplerarr.samplers.push(v2.clone());
                if 6 == need { *comp = EffectTextureSamplersComp( Some( texsamplerarr ) ); return; }
            } else { comp.0 = None; return; }
            
            if let (Some(v1), Some(v2)) = (&tex06.0, &sampl06.0) {
                texsamplerarr.textures.push(v1.clone()); texsamplerarr.samplers.push(v2.clone());
                if 7 == need { *comp = EffectTextureSamplersComp( Some( texsamplerarr ) ); return; }
            } else { comp.0 = None; return; }
            
            if let (Some(v1), Some(v2)) = (&tex07.0, &sampl07.0) {
                texsamplerarr.textures.push(v1.clone()); texsamplerarr.samplers.push(v2.clone());
                if 8 == need { *comp = EffectTextureSamplersComp( Some( texsamplerarr ) ); return; }
            } else { comp.0 = None; return; }
        });
    }
    