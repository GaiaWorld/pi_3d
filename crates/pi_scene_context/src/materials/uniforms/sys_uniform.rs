

use std::sync::Arc;

use pi_scene_shell::prelude::*;



use crate::materials::shader_effect::AssetResShaderEffectMeta;

use super::{
    uniform::*,
    texture::*,
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
                            Arc::new(
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