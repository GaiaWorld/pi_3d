use std::{marker::PhantomData};

use pi_ecs::{prelude::{ResMut, Query, Res, Component}, query::{Write, Changed}};
use pi_ecs_macros::setup;
use pi_engine_shell::{object::{GameObject}};
use pi_render::rhi::{device::RenderDevice, dyn_uniform_buffer::Uniform};

use crate::{resources::RenderDynUniformBuffer, materials::{bind_group::{RenderBindGroupPool, RenderBindGroupKey}, uniform_buffer::SingleDynUnifromBufferReBindFlag, value::FromValueUniformStatistics}};


use crate::materials::material_meta::AssetResMaterailMeta;

use super::{
    value_uniform::{MaterialValueBind},
    texture_uniform::MaterialTextureBindGroupID,
    float::{FloatUniform},
    int::{IntUniform},
    uint::{UintUniform},
    mat4::Mat4Uniform,
    mat2::Mat2Uniform,
    vec4::Vec4Uniform,
    vec2::Vec2Uniform,
};

pub struct SysMaterialMetaChange;
#[setup]
impl SysMaterialMetaChange {
    #[system]
    pub fn cmd(
        mut materials: Query<
            GameObject,
            (
                &AssetResMaterailMeta,
                Write<MaterialValueBind>, Write<MaterialTextureBindGroupID>,
                Write<Mat4Uniform>, Write<Mat2Uniform>, Write<Vec4Uniform>, Write<Vec2Uniform>, Write<FloatUniform>, Write<IntUniform>, Write<UintUniform> 
            ),
            Changed<AssetResMaterailMeta>
        >,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
    ) {
        materials.iter_mut().for_each(|(
            material, mut statistics_value, mut texbindgroup,
            mut mat4, mut mat2, mut vec4, mut vec2, mut float, mut int, mut uint
        )| {
            println!("SysMaterialMetaChange:");
            let uniforms = &material.uniforms;
            let mat4_count      = uniforms.mat4_list.len() as u8;
            let mat2_count      = uniforms.mat2_list.len() as u8;
            let vec4_count      = uniforms.vec4_list.len() as u8;
            let vec2_count      = uniforms.vec2_list.len() as u8;
            let float_count     = uniforms.float_list.len() as u8;
            let int_count       = uniforms.int_list.len() as u8;
            let uint_count      = uniforms.uint_list.len() as u8;
            let align_bytes     = 16;

            let statistics = MaterialValueBind::new(&device, &mut dynbuffer, uniforms.bind, mat4_count, mat2_count, vec4_count, vec2_count, float_count, int_count, uint_count, align_bytes);

            if statistics.mat4_count    > 0 { let mut data = Mat4Uniform::new(&statistics);     data.init(&uniforms.mat4_list); mat4.write(data); }
            if statistics.mat2_count    > 0 { let mut data = Mat2Uniform::new(&statistics);     data.init(&uniforms.mat2_list); mat2.write(data); }
            if statistics.vec4_count    > 0 { let mut data = Vec4Uniform::new(&statistics);     data.init(&uniforms.vec4_list); vec4.write(data); }
            if statistics.vec2_count    > 0 { let mut data = Vec2Uniform::new(&statistics);     data.init(&uniforms.vec2_list); vec2.write(data); }
            if statistics.float_count   > 0 { let mut data = FloatUniform::new(&statistics);    data.init(&uniforms.float_list); float.write(data); }
            if statistics.int_count     > 0 { let mut data = IntUniform::new(&statistics);          data.init(&uniforms.int_list); int.write(data); }
            if statistics.uint_count    > 0 { let mut data = UintUniform::new(&statistics);     data.init(&uniforms.uint_list); uint.write(data); }

            bindgrouppool.creat(&device, statistics.bind_group.clone(), MaterialValueBind::layout_entries(statistics.total_size as usize).as_slice(), uniforms.set);

            statistics_value.write(statistics);

            if let Some(textures) = &material.textures {
                let mut key = textures.label();
                key += &bindgrouppool.get_counter().to_string();
                let key = RenderBindGroupKey::from(key);
                bindgrouppool.creat(&device, key.clone(), textures.layout_entries().as_slice(), textures.set);
                texbindgroup.write(MaterialTextureBindGroupID(key));
            }
        });
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
            (&MaterialValueBind, &D), 
            Changed<D>
        >,
        mut dynbuffer: ResMut<RenderDynUniformBuffer>,
    ) {
        items.iter_mut().for_each(|(bindoffset, slot)| {
            if let Some(bind_offset) = &bindoffset.bind_offset {
                dynbuffer.as_mut().set_uniform(bind_offset, slot);
            }
        });
    }
}

pub struct SysValueBindgroupUpdate;
#[setup]
impl SysValueBindgroupUpdate {
    #[system]
    pub fn tick(
        device: Res<RenderDevice>,
        dynbuffer: Res<RenderDynUniformBuffer>,
        dynbuffer_flag: Res<SingleDynUnifromBufferReBindFlag>,
        mut bindgroups: ResMut<RenderBindGroupPool>,
        items: Query<GameObject, &MaterialValueBind>,
    ) {
        // println!("Sys MainCameraRender BindGroup Update");
        if dynbuffer_flag.0 {
            items.iter().for_each(|item| {
                match bindgroups.get_mut(&item.bind_group) {
                    Some(mut group) => {
                        item.bind_group(&device, &mut group, &dynbuffer);
                    },
                    None => todo!(),
                }
            });
        }
    }
}