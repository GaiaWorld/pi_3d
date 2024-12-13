
use pi_scene_shell::prelude::*;

use super::{
    material::*,
    uniforms::{uniform::*, texture::*},
    shader_effect::*
};

pub fn sys_material_textures_modify(
    addeds: ComponentAdded<UniformTextureWithSamplerParamsDirty>,
    changes: ComponentChanged<UniformTextureWithSamplerParamsDirty>,
    mut materials: Query<
        (
            &AssetResShaderEffectMeta, &mut UniformTextureWithSamplerParams,
            &mut TextureKeyList,
            &mut EffectBindSampler2DList
        )
    >,
    device: Res<PiRenderDevice>,
    asset_samp: Res<ShareAssetMgr<SamplerRes>>,
) {
    // log::debug!("SysMaterialMetaChange: ");
    addeds.iter().chain(changes.iter()) .for_each(|entity| {
        if let Ok((
            effect, mut texparams,
            mut slots,
            mut samplers
        )) = materials.get_mut(*entity) {
            let effect = effect.0.as_ref().unwrap().as_ref();
            if effect.textures.len() > 0 {
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
                                    sample: KeySampler::default(),
                                    url: EKeyTexture::Tex(Atom::from(DefaultTexture::path(item.initial, wgpu::TextureDimension::D2))),
                                    wrapu: EAddressMode::ClampToEdge,
                                    wrapv: EAddressMode::ClampToEdge,
                                    wrapw: EAddressMode::ClampToEdge,
                                    texture_sample: wgpu::TextureSampleType::Float { filterable: true },
                                    sampler_bind_type: wgpu::SamplerBindingType::Filtering,
                                }
                            )
                        );
                        texparams.0.get(&item.slotname).unwrap()
                    };
                    // log::error!("Texture {:?} {:?}", index, &param.url);
    
                    if index < TEXTURE_SLOT_COUNT {
                        if !slots.query(index).eq(&param) {
                            slots.modify(index, param.clone());
                        }
                        if let Some(samp) = BindDataSampler::create(param.sample.clone(), &device, &asset_samp) {
                            samplers.0[index] = Some(samp);
                        } else {
                            // log::error!("Sampler Fail: {:?}", (item.initial, &param.sample));
                        }
                    }
                }
            }
        }
    });
}

pub fn sys_material_uniform_apply(
    changes: ComponentChanged<TargetAnimatorableIsRunning>,
    floats: Query<(Ticker<&AnimatorableFloat>, &AnimatorableUniform)>,
    _vec2s: Query<(Ticker<&AnimatorableVec2 >, &AnimatorableUniform)>,
    _vec3s: Query<(Ticker<&AnimatorableVec3 >, &AnimatorableUniform)>,
    _vec4s: Query<(Ticker<&AnimatorableVec4 >, &AnimatorableUniform)>,
    _uints: Query<(Ticker<&AnimatorableUint >, &AnimatorableUniform)>,
    items: Query<(&BindEffect, &UniformAnimated)>,
    mut performance: ResMut<Performance>,
) {
    if performance.debug { performance.t_uniformbufferupdate = pi_time::Instant::now(); }

    changes.iter().for_each(|entity| {
        if let Ok((bind, animated)) = items.get(*entity) {
            if let Some(bind) = &bind.0 {
                animated.0.iter().for_each(|_k| {
                    if let Some(offset) = bind.offset(_k) {
                        match offset.entity() {
                            Some(entity) => {
                                match offset.atype() {
                                    EAnimatorableType::Vec4 => {
                                        if let Ok((value, _)) = _vec4s.get(entity) {
                                            if value.is_changed() == false { return; }
                                            bind.write_data(offset.offset() as usize, bytemuck::cast_slice(value.0.as_slice()));
                                        }
                                    },
                                    EAnimatorableType::Vec3 => {
                                        if let Ok((value, _)) = _vec3s.get(entity) {
                                            if value.is_changed() == false { return; }
                                            bind.write_data(offset.offset() as usize, bytemuck::cast_slice(value.0.as_slice()));
                                        }
                                    },
                                    EAnimatorableType::Vec2 => {
                                        if let Ok((value, _)) = _vec2s.get(entity) {
                                            if value.is_changed() == false { return; }
                                            bind.write_data(offset.offset() as usize, bytemuck::cast_slice(value.0.as_slice()));
                                        }
                                    },
                                    EAnimatorableType::Float => {
                                        if let Ok((value, _)) = floats.get(entity) {
                                            if value.is_changed() == false { return; }
                                            bind.write_data(offset.offset() as usize, bytemuck::cast_slice(&[value.0]));
                                        }
                                    },
                                    EAnimatorableType::Uint => {
                                        if let Ok((value, _)) = _uints.get(entity) {
                                            if value.is_changed() == false { return; }
                                            bind.write_data(offset.offset() as usize, bytemuck::cast_slice(&[value.0]));
                                        }
                                    },
                                    EAnimatorableType::Int => {
                                    },
                                }
                            },
                            _ => {},
                        }
                    }
                });
            }
        }
    });

    if performance.debug { performance.uniformbufferupdate = (pi_time::Instant::now() - performance.t_uniformbufferupdate).as_micros() as u32; }
}

pub fn sys_texture_ready(
    changes: ComponentChanged<EffectBindTexture2DList>,
    mut items: Query<
        (
            ObjectID,
            &AssetResShaderEffectMeta, &TextureKeyList
            , &EffectBindTexture2DList, &EffectBindSampler2DList
            , &mut EffectTextureSamplersComp
        )
    >,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_texture_ready"));
    changes.iter().for_each(|entity| {
        if let Ok((
            _entity, binddesc, keys
            , textures, samplers
            , mut comp
        )) = items.get_mut(*entity) {
            let binddesc = binddesc.0.as_ref().unwrap();
            let need = binddesc.textures.len();
            let mut texsamplerarr =  EffectTextureSamplers::default();
    
            for idx in 0..TEXTURE_SLOT_COUNT {
                let key = &keys.0[idx];
                if let (Some((v1, k1)), Some(v2)) = (&textures.data[idx], &samplers.0[idx]) {
                    texsamplerarr.textures.push(EffectTextureSampler(v1.clone(), v2.clone(), EShaderStage::FRAGMENT, key.texture_sample, v1.view_dimension(), key.sampler_bind_type));
                    
                    // log::error!("{:?}", (&key.url, k1));
                    if idx + 1 == need && k1 == &key.url {
                        *comp = EffectTextureSamplersComp( Some( texsamplerarr ) );
                        return;
                    }
                } else {
                    comp.0 = None; 
                    // log::error!("{:?}", (textures.data[idx].is_some(), samplers.0[idx].is_some()));
                    return;
                }
            }    
        }
    });
}

pub fn sys_dispose_about_material(
    items: Query<(Entity, &DisposeReady, &MaterialRefs, &BindEffect), Changed<DisposeReady>>,
    defaultmat: Res<SingleIDBaseDefaultMaterial>,
    mut disposecan: Query<&mut DisposeCan>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_dispose_about_material"));
    items.iter().for_each(|(entity, state, refs, bind)| {
        if defaultmat.0 == entity || state.0 == false { return; }

        if refs.is_empty() {
            if let Ok(mut dispose) = disposecan.get_mut(entity) { dispose.0 = true; }
            if let Some(bind) = &bind.0 {
                bind.uniforms().iter().for_each(|v| {
                    if let Some(entity) = v.1.entity() {
                        // log::error!("AAAA {:?}", (entity));
                        if let Ok(mut dispose) = disposecan.get_mut(entity) { dispose.0 = true; }
                    }
                });
            }
        }
    });
}