use std::{marker::PhantomData};

use pi_ecs::{prelude::{ResMut, Query, Res, Component, Commands}, query::{Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject, ObjectID}, run_stage::TSystemStageInfo};
use pi_render::rhi::{device::RenderDevice, dyn_uniform_buffer::Uniform};
use render_shader::{shader_bind::{ShaderBindEffectValue}, shader_set::ShaderSetEffectAbout, set_bind::ShaderSetBind};

use crate::{
    materials::{value::FromValueUniformStatistics, shader_effect::{AssetKeyShaderEffect, SysShaderEffectCommands, SysAssetShaderEffectLoad}, material::SysEffectValueUniformComand},
    bindgroup::{
        RenderBindGroupKey, RenderBindGroupPool,
        uniform_buffer::{SysDynUnifromBufferUpdate, DynUnifromBufferReBindFlag},
    }
};


use crate::materials::shader_effect::AssetResShaderEffectMeta;

use super::{
    float::{FloatUniform},
    int::{IntUniform},
    uint::{UintUniform},
    mat4::Mat4Uniform,
    mat2::Mat2Uniform,
    vec4::Vec4Uniform,
    vec2::Vec2Uniform, texture::{TextureResSlot01, SamplerSlot01, TextureResSlot02, SamplerSlot02, UniformTexture, UniformSampler, SamplerSlot03, TextureResSlot03, TextureResSlot04, SamplerSlot04}, sys_texture::{SysTextureResReady1, SysTextureReady, SysTextureCommand},
};

pub struct SysMaterialMetaChange;
impl TSystemStageInfo for SysMaterialMetaChange {
}
#[setup]
impl SysMaterialMetaChange {
    #[system]
    pub fn cmd(
        mut materials: Query<
            GameObject,
            (
                ObjectID, &AssetKeyShaderEffect, &AssetResShaderEffectMeta,
            ),
            Changed<AssetResShaderEffectMeta>
        >,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
        mut mat_4_cmd: Commands<GameObject, Mat4Uniform>,
        mut mat_2_cmd: Commands<GameObject, Mat2Uniform>,
        mut vec_4_cmd: Commands<GameObject, Vec4Uniform>,
        mut vec_2_cmd: Commands<GameObject, Vec2Uniform>,
        mut float_cmd: Commands<GameObject, FloatUniform>,
        mut int32_cmd: Commands<GameObject, IntUniform>,
        mut uint_cmd: Commands<GameObject, UintUniform>,
        mut effect_set_cmd: Commands<GameObject, ShaderSetEffectAbout>,
        mut effect_bindoff_cmd: Commands<GameObject, ShaderBindEffectValue>,
    ) {
        log::debug!("SysMaterialMetaChange: ");
        materials.iter_mut().for_each(|(
            matid,
            effect_key, effect,
        )| {

            let effect_val_bind = ShaderBindEffectValue::new(effect, &mut dynbuffer);

            let uniforms = &effect.uniforms;

            if effect_val_bind.mat4_count    > 0 { let mut data = Mat4Uniform::new(&effect_val_bind);     data.init(&uniforms.mat4_list);  mat_4_cmd.insert(matid, data); }
            if effect_val_bind.mat2_count    > 0 { let mut data = Mat2Uniform::new(&effect_val_bind);     data.init(&uniforms.mat2_list);  mat_2_cmd.insert(matid, data); }
            if effect_val_bind.vec4_count    > 0 { let mut data = Vec4Uniform::new(&effect_val_bind);     data.init(&uniforms.vec4_list);  vec_4_cmd.insert(matid, data); log::info!("vec_4_cmd"); }
            if effect_val_bind.vec2_count    > 0 { let mut data = Vec2Uniform::new(&effect_val_bind);     data.init(&uniforms.vec2_list);  vec_2_cmd.insert(matid, data); }
            if effect_val_bind.float_count   > 0 { let mut data = FloatUniform::new(&effect_val_bind);    data.init(&uniforms.float_list); float_cmd.insert(matid, data); }
            if effect_val_bind.int_count     > 0 { let mut data = IntUniform::new(&effect_val_bind);      data.init(&uniforms.int_list);   int32_cmd.insert(matid, data); }
            if effect_val_bind.uint_count    > 0 { let mut data = UintUniform::new(&effect_val_bind);     data.init(&uniforms.uint_list);  uint_cmd.insert(matid, data); }

            let mut layout_entries = vec![];
            effect_val_bind.layout_entries(&mut layout_entries);
            let effect_set = if let Some(textures) = &effect.textures {
                let start_bind = if effect_val_bind.total_size > 0 { 1 } else { 0 };
                let temp = ShaderSetEffectAbout::new(effect_key.0.0.clone(), ShaderSetBind::SET_EFFECT_ABOUT, effect_val_bind.total_size, textures.list.len() as u32);
                textures.layout_entries(&temp, &mut layout_entries);
                temp
            } else {
                ShaderSetEffectAbout::new(effect_key.0.0.clone(), ShaderSetBind::SET_EFFECT_ABOUT, effect_val_bind.total_size, 0)
            };

            if layout_entries.len() > 0 {
                bindgrouppool.creat(&device, RenderBindGroupKey::EffectAbout(matid.clone()), layout_entries.as_slice(), ShaderSetBind::SET_EFFECT_ABOUT);
            }

            effect_set_cmd.insert(matid.clone(), effect_set);
            effect_bindoff_cmd.insert(matid.clone(), effect_val_bind);
        });
    }
}

pub struct SysEffectValueUniformUpdate;
impl TSystemStageInfo for SysEffectValueUniformUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysEffectValueUniformComand::key(), SysMaterialMetaChange::key(),
        ]
    }
}

pub struct SysUpdateValueUniform<D: Uniform + Component>(PhantomData<D>);
#[setup]
impl<D> SysUpdateValueUniform<D>
where
    D: Uniform + Component,
{
    #[system]
    pub fn update(
        mut items: Query<
            GameObject, 
            (&ShaderBindEffectValue, &D), 
            Changed<D>
        >,
        mut dynbuffer: ResMut<render_resource::uniform_buffer::RenderDynUniformBuffer>,
    ) {
        items.iter_mut().for_each(|(bindoffset, slot)| {
            // log::info!("SysUpdateValueUniform: 1, {:?}", bindoffset.bind_offset());
            if let Some(bindoffset) = bindoffset.bind_offset() {
                dynbuffer.as_mut().set_uniform(bindoffset, slot);
            }
        });
    }
}

pub struct SysEffectValueBindgroupUpdate;
impl TSystemStageInfo for SysEffectValueBindgroupUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysDynUnifromBufferUpdate::key()
        ]
    }
}
#[setup]
impl SysEffectValueBindgroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        items: Query<GameObject, (ObjectID, &ShaderSetEffectAbout, &DynUnifromBufferReBindFlag), Changed<DynUnifromBufferReBindFlag>>,
        mut flag_delete: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        log::debug!("SysEffectValueBindgroupUpdate: ");
        // log::debug!("Sys MainCameraRender BindGroup Update");
        items.iter().for_each(|(
            obj,
            effect_set, flag,
        )| {
            if effect_set.tex_count() == 0 {
                match bindgroups.get_mut(&RenderBindGroupKey::EffectAbout(obj.clone())) {
                    Some(mut group) => {
                        let entries = effect_set.bind_group_entries(
                            &dynbuffer,
                            &[], 
                            &[]
                        );
    
                        group.bind_group = Some(
                            device.create_bind_group(&wgpu::BindGroupDescriptor {
                                label: Some(effect_set.label()),
                                layout: &group.layout,
                                entries: entries.as_slice()
                            })
                        )
                    },
                    None => todo!(),
                }
                flag_delete.delete(obj.clone());
            }
        });
    }
}

pub struct SysEffectTexBindgroupUpdate;
impl TSystemStageInfo for SysEffectTexBindgroupUpdate {
    fn depends() -> Vec<pi_engine_shell::run_stage::KeySystem> {
        vec![
            SysDynUnifromBufferUpdate::key(),
        ]
    }
}

pub struct SysEffectBindgroupUpdateTex01;
#[setup]
impl SysEffectBindgroupUpdateTex01 {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        items: Query<
            GameObject,
            (
                ObjectID,
                &ShaderSetEffectAbout, Option<&DynUnifromBufferReBindFlag>,
                &TextureResSlot01, &SamplerSlot01
            ),
            Or<(
                Changed<TextureResSlot01>, Changed<SamplerSlot01>, Changed<DynUnifromBufferReBindFlag>
            )>
        >,
        mut flag_delete: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        log::trace!("SysEffectBindgroupUpdateTex01:");
        items.iter().for_each(|(
            obj,
            effect_set, flag,
            tex1, samp1
        )| {
            log::debug!("SysEffectBindgroupUpdateTex01: {}", effect_set.tex_count());
            if effect_set.tex_count() == 1 {
                match bindgroups.get_mut(&RenderBindGroupKey::EffectAbout(obj.clone())) {
                    Some(mut group) => {
                        let entries = effect_set.bind_group_entries(
                            &dynbuffer,
                            &[tex1.texture()], 
                            &[samp1.sampler()]
                        );
    
                        group.bind_group = Some(
                            device.create_bind_group(&wgpu::BindGroupDescriptor {
                                label: Some(effect_set.label()),
                                layout: &group.layout,
                                entries: entries.as_slice()
                            })
                        )
                    },
                    None => todo!(),
                }
                flag_delete.delete(obj.clone());
            }
        });
    }
}

pub struct SysEffectBindgroupUpdateTex02;
#[setup]
impl SysEffectBindgroupUpdateTex02 {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        items: Query<
            GameObject,
            (
                ObjectID,
                &ShaderSetEffectAbout, &DynUnifromBufferReBindFlag,
                &TextureResSlot01, &SamplerSlot01,
                &TextureResSlot02, &SamplerSlot02,
            ),
            Or<(
                Changed<DynUnifromBufferReBindFlag>,
                Changed<TextureResSlot01>, Changed<SamplerSlot01>,
                Changed<TextureResSlot02>, Changed<SamplerSlot02>,
            )>
        >,
        mut flag_delete: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        // log::debug!("Sys MainCameraRender BindGroup Update");
        items.iter().for_each(|(
            obj,
            effect_set, flag,
            tex1, samp1,
            tex2, samp2,
        )| {
            if effect_set.tex_count() == 2 {
                match bindgroups.get_mut(&RenderBindGroupKey::EffectAbout(obj.clone())) {
                    Some(mut group) => {
                        let entries = effect_set.bind_group_entries(
                            &dynbuffer,
                            &[
                                tex1.texture(),
                                tex2.texture()
                            ],
                            &[
                                samp1.sampler(),
                                samp2.sampler()
                            ]
                        );
    
                        group.bind_group = Some(
                            device.create_bind_group(&wgpu::BindGroupDescriptor {
                                label: Some(effect_set.label()),
                                layout: &group.layout,
                                entries: entries.as_slice() 
                            })
                        )
                    },
                    None => todo!(),
                }
                flag_delete.delete(obj.clone());
            }
        });
    }
}

pub struct SysEffectBindgroupUpdateTex03;
#[setup]
impl SysEffectBindgroupUpdateTex03 {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        items: Query<
            GameObject,
            (
                ObjectID,
                &ShaderSetEffectAbout, &DynUnifromBufferReBindFlag,
                &TextureResSlot01, &SamplerSlot01,
                &TextureResSlot02, &SamplerSlot02,
                &TextureResSlot03, &SamplerSlot03,
            ),
            Or<(
                Changed<DynUnifromBufferReBindFlag>,
                Changed<TextureResSlot01>, Changed<SamplerSlot01>,
                Changed<TextureResSlot02>, Changed<SamplerSlot02>,
                Changed<TextureResSlot03>, Changed<SamplerSlot03>,
            )>
        >,
        mut flag_delete: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        // log::debug!("Sys MainCameraRender BindGroup Update");
        items.iter().for_each(|(
            obj,
            effect_set, flag,
            tex1, samp1,
            tex2, samp2,
            tex3, samp3,
        )| {
            if effect_set.tex_count() == 3 {
                match bindgroups.get_mut(&RenderBindGroupKey::EffectAbout(obj.clone())) {
                    Some(mut group) => {
                        let entries = effect_set.bind_group_entries(
                            &dynbuffer,
                            &[
                                tex1.texture(), tex2.texture(), tex3.texture()
                            ],
                            &[
                                samp1.sampler(), samp2.sampler(), samp3.sampler()
                            ]
                        );
    
                        group.bind_group = Some(
                            device.create_bind_group(&wgpu::BindGroupDescriptor {
                                label: Some(effect_set.label()),
                                layout: &group.layout,
                                entries: entries.as_slice() 
                            })
                        )
                    },
                    None => todo!(),
                }

                flag_delete.delete(obj.clone());
            }
        });
    }
}

pub struct SysEffectBindgroupUpdateTex04;
#[setup]
impl SysEffectBindgroupUpdateTex04 {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<render_resource::uniform_buffer::RenderDynUniformBuffer>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        items: Query<
            GameObject,
            (
                ObjectID,
                &ShaderSetEffectAbout, &DynUnifromBufferReBindFlag,
                &TextureResSlot01, &SamplerSlot01,
                &TextureResSlot02, &SamplerSlot02,
                &TextureResSlot03, &SamplerSlot03,
                &TextureResSlot04, &SamplerSlot04,
            ),
            Or<(
                Changed<DynUnifromBufferReBindFlag>,
                Changed<TextureResSlot01>, Changed<SamplerSlot01>,
                Changed<TextureResSlot02>, Changed<SamplerSlot02>,
                Changed<TextureResSlot03>, Changed<SamplerSlot03>,
                Changed<TextureResSlot04>, Changed<SamplerSlot04>,
            )>
        >,
        mut flag_delete: Commands<GameObject, DynUnifromBufferReBindFlag>,
    ) {
        // log::debug!("Sys MainCameraRender BindGroup Update");
        items.iter().for_each(|(
            obj,
            effect_set, flag,
            tex1, samp1,
            tex2, samp2,
            tex3, samp3,
            tex4, samp4,
        )| {
            if effect_set.tex_count() == 4 {
                match bindgroups.get_mut(&RenderBindGroupKey::EffectAbout(obj.clone())) {
                    Some(mut group) => {
                        let entries = effect_set.bind_group_entries(
                            &dynbuffer,
                            &[
                                tex1.texture(), tex2.texture(), tex3.texture(), tex4.texture()
                            ],
                            &[
                                samp1.sampler(), samp2.sampler(), samp3.sampler(), samp4.sampler()
                            ]
                        );
    
                        group.bind_group = Some(
                            device.create_bind_group(&wgpu::BindGroupDescriptor {
                                label: Some(effect_set.label()),
                                layout: &group.layout,
                                entries: entries.as_slice() 
                            })
                        );
                    },
                    None => {
                        
                    },
                }
                flag_delete.delete(obj.clone());
            }
        });
    }
}