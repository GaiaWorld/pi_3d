use pi_hash::XHashMap;
use pi_ecs::{prelude::{Res, Query, ResMut, query}, query::{Changed, Or, Write, WithOut}};
use pi_ecs_macros::setup;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, device::RenderDevice, texture::{TextureView, Sampler}, asset::{RenderRes, TextureRes}};

use pi_scene_context::{materials::{bind_group::{RenderBindGroup, RenderBindGroupPool}, uniform_buffer::SingleDynUnifromBufferReBindFlag}, object::{GameObject, ObjectID}, meshes::model::BuildinModelBind, shaders::{FragmentUniformBind, FragmentUniformBindTexture, FragmentUniformBindTextureSampler}, resources::RenderDynUniformBuffer};
use pi_slotmap::DefaultKey;
use material_textures::main_texture::{MainTextureKey, MainTextureRes, MainTextureSampler};

use crate::{define::{UnlitMaterialMode, UnlitMaterialDefines}};

use super::unlit_material::UnlitMaterialPropertype;

#[derive(Debug, Default)]
pub struct SingleUnlitBindGroupList {
    pub value: Option<DefaultKey>,
    pub texture_map: XHashMap<UnlitMaterialMode, DefaultKey>,
}

pub struct UnlitMaterialBindGroup(pub DefaultKey);
impl UnlitMaterialBindGroup {
    const LABEL: &'static str = "UnlitMaterialBindGroup";
    pub const SET: u32 = 1;

    pub fn layout(
        device: &RenderDevice
    ) -> BindGroupLayout {
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::LABEL),
                    entries: &[
                        BuildinModelBind::ENTRY,
                        UnlitMaterialPropertype::ENTRY,
                    ],
                }
            )
        )
    }

    pub fn bind_group(
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        dynbuffer: &RenderDynUniformBuffer,
    ) {
        group.bind_group = Some(
            BindGroup::from(
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some(Self::LABEL),
                        layout: &group.layout,
                        entries: &[
                            BuildinModelBind::dyn_entry(dynbuffer),
                            UnlitMaterialPropertype::dyn_entry(dynbuffer),
                        ],
                    }
                )
            )
        ); 
    }
}

pub struct UnlitMaterialTextureBindGroup(pub DefaultKey);
impl UnlitMaterialTextureBindGroup {
    pub fn label(
        mode: UnlitMaterialMode,
    ) -> &'static str {
        "UnlitMaterialTextureBindGroup"
    }

    pub fn slot(
        mode: UnlitMaterialMode,
    ) -> u8 {
        2
    }

    pub fn is_ready(
        &self,
    ) -> bool {
        true
    }

    pub fn layout(
        mode: UnlitMaterialMode,
        device: &RenderDevice
    ) -> BindGroupLayout {
        println!("{:?}", MainTextureRes::ENTRY_TEXTURE);
        println!("{:?}", MainTextureSampler::ENTRY_SAMPLER);
        BindGroupLayout::from(
            device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(Self::label(mode)),
                    entries: &[
                        MainTextureRes::ENTRY_TEXTURE,
                        MainTextureSampler::ENTRY_SAMPLER,
                    ],
                }
            )
        )
    }

    pub fn bind_group(
        &self,
        mode: UnlitMaterialMode,
        device: &RenderDevice,
        group: &mut RenderBindGroup,
        emissive: (&TextureRes, &Sampler),
    ) {
        group.bind_group = Some(
            BindGroup::from(
                device.create_bind_group(
                    &wgpu::BindGroupDescriptor {
                        label: Some(Self::label(mode)),
                        layout: &group.layout,
                        entries: &[
                            MainTextureRes::entry_texture(&emissive.0.texture_view),
                            MainTextureSampler::entry_sampler(emissive.1),
                        ],
                    }
                )
            )
        );
    }

}

pub struct SysUnlitMaterialBindGroupUpdate;
#[setup]
impl SysUnlitMaterialBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        unlit_bindgroup: Res<SingleUnlitBindGroupList>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
    ) {
        if dynbuffer_flag.0 {
            UnlitMaterialBindGroup::bind_group(&device, bindgroups.get_mut(unlit_bindgroup.value.unwrap()).unwrap(), &dynbuffer);
        }
    }
}


pub struct SysUnlitMaterialTextureBindGroupUpdate;
#[setup]
impl SysUnlitMaterialTextureBindGroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        mut items: Query<GameObject, (&UnlitMaterialDefines, &MainTextureRes, &MainTextureSampler, Write<UnlitMaterialTextureBindGroup>), Or<(Changed<UnlitMaterialDefines>, Changed<MainTextureSampler>, Changed<MainTextureRes>)>>,
        mut unlit_bindgroup: ResMut<SingleUnlitBindGroupList>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
    ) {
        println!("Sys UnlitMaterial Texture BindGroup Update");
        items.iter_mut().for_each(|(define, tex2d, sampler, mut texbindgroupwrite)| {
            let deffinemode = define.mode();
            match unlit_bindgroup.texture_map.get(&deffinemode) {
                Some(group) => {
                    let texbindgroup = UnlitMaterialTextureBindGroup(*group);
                    match bindgroups.get_mut(*group) {
                        Some(group) => {
                            println!("UnlitMaterialTextureBindGroup bind_group");
                            texbindgroup.bind_group(deffinemode, &device,  group, (&tex2d.0, &sampler.0));
                            texbindgroupwrite.write(texbindgroup);
                        },
                        None => todo!()
                    }
                },
                None => {
                    let group = bindgroups.creat(&device, UnlitMaterialTextureBindGroup::layout(deffinemode, &device), UnlitMaterialTextureBindGroup::slot(deffinemode) as u32);
                    let texbindgroup = UnlitMaterialTextureBindGroup(group);
                    texbindgroup.bind_group(
                        deffinemode,
                        &device,
                        bindgroups.get_mut(group).unwrap(),
                        (&tex2d.0, &sampler.0),
                    ); 
                    unlit_bindgroup.texture_map.insert(define.mode(), group);
                    texbindgroupwrite.write(texbindgroup);
                    println!("UnlitMaterialTextureBindGroup bind_group");
                },
            }
        });
    }
}