use futures::io::Write;
use pi_hash::XHashMap;
use pi_ecs::{prelude::{Res, Query, ResMut, query}, query::Changed};
use pi_ecs_macros::setup;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, device::RenderDevice, texture::{TextureView, Sampler}, asset::{RenderRes, TextureRes}};

use pi_scene_context::{materials::{bind_group::RenderBindGroup, uniform_buffer::SingleDynUnifromBufferReBindFlag}, object::{GameObject, ObjectID}, meshes::model::BuildinModelBind, shaders::{FragmentUniformBind, FragmentUniformBindTexture, FragmentUniformBindTextureSampler}, resources::RenderDynUniformBuffer, texture::texture2d::{Texture2DKey, Texture2D}};
use pi_slotmap::DefaultKey;

use crate::{define::{StandardMaterialMode, StandardMaterialDefines}, emissive::EmissiveTexture};

use super::standard_material::StandardMaterialPropertype;

pub struct SingleStandardBindGroupList {
    pub value_map: XHashMap<StandardMaterialMode, DefaultKey>,
    pub texture_map: XHashMap<StandardMaterialMode, DefaultKey>,
}

pub struct StandardMaterialBindGroup {
    pub bind_group: Option<BindGroup>,
}
impl StandardMaterialBindGroup {
    const LABEL: &'static str = "StandardMaterialBindGroup";
    pub const SET: u32 = 1;

    pub fn layout(
        &self,
        device: &RenderDevice
    ) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::LABEL),
                    entries: &[
                        BuildinModelBind::ENTRY,
                        StandardMaterialPropertype::ENTRY,
                    ],
                }
            )
        )
    }

    pub fn bind_group(
        &self,
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        dynbuffer: &RenderDynUniformBuffer,
    ) {
        group.bind_group = Some(
            BindGroup::from(
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some(StandardMaterialBindGroup::LABEL),
                        layout: &group.layout,
                        entries: &[
                            BuildinModelBind::dyn_entry(dynbuffer),
                            StandardMaterialPropertype::dyn_entry(dynbuffer),
                        ],
                    }
                )
            )
        ); 
    }
}

pub struct StandardMaterialTextureBindGroup;
impl StandardMaterialTextureBindGroup {
    pub fn label(
        mode: StandardMaterialMode,
    ) -> &'static str {
        "StandardMaterialTextureBindGroup"
    }

    pub fn slot(
        mode: StandardMaterialMode,
    ) -> u8 {
        2
    }

    pub fn is_ready(
        &self,
    ) -> bool {
        true
    }

    pub fn layout(
        mode: StandardMaterialMode,
        device: &RenderDevice
    ) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::label(mode)),
                    entries: &[
                        EmissiveTexture::ENTRY_TEXTURE,
                        EmissiveTexture::ENTRY_SAMPLER,
                    ],
                }
            )
        )
    }

    pub fn bind_group(
        &self,
        mode: StandardMaterialMode,
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        emissive: (&TextureRes, &Sampler),
    ) {
        if self.is_ready() {
            group.bind_group = None;
        } else {
            group.bind_group = Some(
                BindGroup::from(
                    device.create_bind_group(
                        &wgpu::BindGroupDescriptor {
                            label: Some(Self::label(mode)),
                            layout: &group.layout,
                            entries: &[
                                EmissiveTexture::entry_texture(&emissive.0.texture_view),
                                EmissiveTexture::entry_sampler(emissive.1),
                            ],
                        }
                    )
                )
            ); 
        }
    }

}

pub struct SysDefaultMaterialBindGroupUpdate;
#[setup]
impl SysDefaultMaterialBindGroupUpdate {
    #[system]
    pub fn tick(
        materials: Query<GameObject, (&StandardMaterialDefines, &StandardMaterialBindGroup)>,
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        mut standard_bindgroup: ResMut<SingleStandardBindGroupList>,
        mut bindgroups: Query<GameObject, &mut RenderBindGroup>,
    ) {
        // println!("Sys DefaultMaterial BindGroup Update");
        if dynbuffer_flag.0 {
            materials.iter().for_each(|(define, material)| {
                match standard_bindgroup.value_map.get_mut(&define.mode()) {
                    Some(mut group) => {
                        
                        // println!("IDDefaultMaterialBindGroup bind_group");
                        material.bind_group(&device, group, &dynbuffer);
                    },
                    None => {
                        let mut group = RenderBindGroup::new(&device, material.layout(&device), StandardMaterialBindGroup::SET);
                        material.bind_group(&device, &mut group, &dynbuffer);
                        standard_bindgroup.value_map.insert(define.mode(), group);
                    },
                }
            });
        }
    }
}

pub struct SysStandardMaterialTextureBindGroupUpdate;
#[setup]
impl SysStandardMaterialTextureBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        items: Query<GameObject, (&StandardMaterialDefines, &StandardMaterialTextureBindGroup, &EmissiveTexture), Changed<StandardMaterialTextureBindGroup>>,
        mut bindgroups: ResMut<SingleStandardBindGroupList>,
        textures: Query<GameObject, &Texture2D>,
    ) {
        // println!("Sys DefaultMaterial BindGroup Update");
        items.iter().for_each(|(define, material, emissive)| {
            let deffinemode = define.mode();
            match bindgroups.texture_map.get_mut(&deffinemode) {
                Some(mut group) => {
                    match textures.get(emissive.id) {
                        Some(tex2d) => {
                            material.bind_group(
                                deffinemode,
                                &device,
                                group,
                                (&tex2d.texture, &tex2d.sampler),
                            ); 
                        },
                        None => {

                        }
                    }
                },
                None => {
                    let mut group = RenderBindGroup::new(&device, StandardMaterialTextureBindGroup::layout(deffinemode, &device), StandardMaterialTextureBindGroup::slot(deffinemode) as u32);
                    match textures.get(emissive.id) {
                        Some(tex2d) => {
                            material.bind_group(
                                deffinemode,
                                &device,
                                &mut group,
                                (&tex2d.texture, &tex2d.sampler),
                            ); 
                        },
                        None => {

                        }
                    }
                    bindgroups.texture_map.insert(define.mode(), group);
                },
            }
        });
    }
}