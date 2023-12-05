
use pi_engine_shell::prelude::*;

use crate::commands::*;
use super::{
    material::*,
    uniforms::{uniform::*, texture::*},
    shader_effect::*
};

pub fn sys_material_textures_modify(
    mut materials: Query<
        (
            ObjectID, &AssetResShaderEffectMeta, &mut UniformTextureWithSamplerParams,
            (
                &mut TextureSlot01, &mut TextureSlot02, &mut TextureSlot03, &mut TextureSlot04, 
                &mut TextureSlot05, &mut TextureSlot06, &mut TextureSlot07, &mut TextureSlot08, 
            ),
            (
                &mut EffectBindSampler2D01Comp, &mut EffectBindSampler2D02Comp, &mut EffectBindSampler2D03Comp, &mut EffectBindSampler2D04Comp, 
                &mut EffectBindSampler2D05Comp, &mut EffectBindSampler2D06Comp, &mut EffectBindSampler2D07Comp, &mut EffectBindSampler2D08Comp, 
            )
        ),
        Or<(Changed<AssetResShaderEffectMeta>, Changed<UniformTextureWithSamplerParamsDirty>)>
    >,
    device: Res<PiRenderDevice>,
    asset_samp: Res<ShareAssetMgr<SamplerRes>>,
) {
    // log::debug!("SysMaterialMetaChange: ");
    materials.iter_mut().for_each(|(
        matid,
        effect, mut texparams,
        mut slots,
        mut samplers
    )| {
        if effect.textures.len() == 0 {
            //
        } else {
            for index in 0..effect.textures.len() {
                let item = effect.textures.get(index).unwrap();
                let param = if let Some(param) = texparams.0.get(&item.slotname) {
                    param
                } else {
                    texparams.0.insert(
                        item.slotname.clone(), 
                        std::sync::Arc::new(
                                UniformTextureWithSamplerParam {
                                slotname: item.slotname.clone(),
                                filter: true,
                                sample: KeySampler::default(),
                                url: EKeyTexture::Tex(Atom::from(DefaultTexture::path(item.initial, wgpu::TextureDimension::D2))),
                            }
                        )
                    );
                    texparams.0.get(&item.slotname).unwrap()
                };
                // log::error!("Texture {:?} {:?}", index, &param.url);
                
                if index == 0 {
                    
                    if !slots.0.0.eq(&param) { slots.0.0 = param.clone(); }
                    // entitycmd.insert(TextureSlot01(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        *samplers.0 = EffectBindSampler2D01Comp(Some(samp));
                    }
                } else if index == 1 {
                    
                    if !slots.1.0.eq(&param) { slots.1.0 = param.clone(); } 
                    // entitycmd.insert(TextureSlot02(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        *samplers.1 = EffectBindSampler2D02Comp(Some(samp));
                    }
                } else if index == 2 {
                    // log::warn!("Texture 2 {:?}", &param.url);
                    if !slots.2.0.eq(&param) { slots.2.0 = param.clone(); }
                    // entitycmd.insert(TextureSlot03(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        *samplers.2 = EffectBindSampler2D03Comp(Some(samp));
                    }
                } else if index == 3 {
                    // log::warn!("Texture 3 {:?}", &param.url);
                    slots.3.0 = param.clone();
                    // entitycmd.insert(TextureSlot04(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        *samplers.3 = EffectBindSampler2D04Comp(Some(samp));
                    }
                } else if index == 4 {
                    slots.4.0 = param.clone();
                    // entitycmd.insert(TextureSlot05(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        *samplers.4 = EffectBindSampler2D05Comp(Some(samp));
                    }
                } else if index == 5 {
                    slots.5.0 = param.clone();
                    // entitycmd.insert(TextureSlot06(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        *samplers.5 = EffectBindSampler2D06Comp(Some(samp));
                    }
                } else if index == 6 {
                    slots.6.0 = param.clone();
                    // entitycmd.insert(TextureSlot07(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        *samplers.6 = EffectBindSampler2D07Comp(Some(samp));
                    }
                } else if index == 7 {
                    slots.7.0 = param.clone();
                    // entitycmd.insert(TextureSlot08(param.clone()));
                    if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                        *samplers.7 = EffectBindSampler2D08Comp(Some(samp));
                    }
                }
            }
        }

    });
}

pub fn sys_material_uniform_apply(
    mut items: Query<
        &BindEffect,
        Changed<BindEffectValueDirty>,
    >,
    mut performance: ResMut<Performance>,
) {
    let time0 = pi_time::Instant::now();
    items.iter_mut().for_each(|bind| {
        match &bind.0 {
            Some(bind) => bind.update(),
            None => {},
        }
        
    });
    performance.uniformbufferupdate = (pi_time::Instant::now() - time0).as_micros() as u32;
}

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

pub fn sys_dispose_about_material(
    items: Query<(Entity, &DisposeReady, &MaterialRefs), Changed<DisposeReady>>,
    mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
) {
    items.iter().for_each(|(entity, state, refs)| {
        if defaultmat.0 == entity { return; }

        if state.0 == true && refs.len() == 0 {
            disposecanlist.push(OpsDisposeCan::ops(entity));
        }
    });
}