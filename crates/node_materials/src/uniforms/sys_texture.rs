use std::marker::PhantomData;

use material_textures::image_texture_load::CalcImageLoad;
use pi_ecs::{prelude::{Query, Res, ResMut}, query::{Write, Changed, Or}};
use pi_ecs_macros::setup;
use pi_engine_shell::object::GameObject;
use pi_render::rhi::device::RenderDevice;
use pi_scene_context::materials::bind_group::{RenderBindGroupKey, RenderBindGroupPool};
use render_resource::sampler::SamplerPool;

use crate::value::SlotActiveRequire;

use super::{texture_uniform::{UniformTextureStatistics, TextureBindDesc, TextureBindGroup}, texture::{ValueTextureKey, TextureSlot1, TextureResSlot1, TextureResSlot3, TextureResSlot4, TextureResSlot2, TextureSlot2, TextureSlot3, TextureSlot4, UniformSampler, SamplerSlot1, UniformTexture}};

pub struct SysMaterialChangeTextureSlot<K: ValueTextureKey + SlotActiveRequire, S: UniformSampler>(PhantomData<(K, S)>);
#[setup]
impl<K, S> SysMaterialChangeTextureSlot<K, S>
where
    K: ValueTextureKey + SlotActiveRequire,
    S: UniformSampler,
{
    #[system]
    pub fn material_change(
        mut items: Query<
            GameObject,
            (&UniformTextureStatistics, &TextureBindDesc, Write<K>, Write<S>),
            Or<(Changed<UniformTextureStatistics>, Changed<TextureBindDesc>)>,
        >,
        device: Res<RenderDevice>,
        mut samplerpool: ResMut<SamplerPool>,
    ) {
        items.iter_mut().for_each(|(statistics, binddesc, mut keyslot, mut samplerslot)| {
            if statistics.texture_count >= K::ASK_SLOT_COUNT {
                let item = binddesc.texture_desc_list.get(K::ASK_SLOT_COUNT as usize - 1).unwrap();
                let key = K::new(item.path.clone());
                keyslot.write(key);
                samplerslot.write(S::new(&item.sampler, &device, &mut samplerpool));
            } else {
                keyslot.remove();
                samplerslot.remove();
            }
        });
    }
}

pub struct SysTextureBindGroup;
impl SysTextureBindGroup {
    pub fn sys(
        mut items: Query<GameObject, (&TextureBindDesc, Write<TextureBindGroup>), Changed<TextureBindDesc>>,
        device: Res<RenderDevice>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
    ) {
        items.iter_mut().for_each(|(binddesc, mut texbindgroup)| {

            if let Some(old) = texbindgroup.get() {
                bindgrouppool.delete(&old.0);
            }

            let mut key = binddesc.label();
            key += &bindgrouppool.get_counter().to_string();

            let key = RenderBindGroupKey::from(key);
            bindgrouppool.creat(&device, key.clone(), binddesc.layout_entries().as_slice(), TextureBindDesc::BIND_GROUP_SET);
            texbindgroup.write(TextureBindGroup(key));
        });
    }
}

pub struct SysTextureResReady1;
impl SysTextureResReady1 {
    pub fn sys(
        items: Query<GameObject, (&TextureBindGroup, &TextureBindDesc, &TextureResSlot1, &SamplerSlot1), Or<(Changed<TextureResSlot1>, Changed<SamplerSlot1>)>>,
        mut bindgrouppool: ResMut<RenderBindGroupPool>,
        device: Res<RenderDevice>,
    ) {
        items.iter().for_each(|(bindgroup, binddesc, tex1, sampler1)| {
            if binddesc.texture_desc_list.len() == 1 {
                let group = bindgrouppool.get_mut(&bindgroup.0).unwrap();
                binddesc.bind_group(&device, group, &[tex1.texture()], &[sampler1.sampler()]);
            }
        });
    }
}

pub type SysTextureSlot1Load = CalcImageLoad<TextureSlot1, TextureResSlot1>;
pub type SysTextureSlot2Load = CalcImageLoad<TextureSlot2, TextureResSlot2>;
pub type SysTextureSlot3Load = CalcImageLoad<TextureSlot3, TextureResSlot3>;
pub type SysTextureSlot4Load = CalcImageLoad<TextureSlot4, TextureResSlot4>;