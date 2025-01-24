use std::{hash::{Hash, Hasher}, sync::Arc};

use crossbeam::queue::SegQueue;
use pi_assets::asset::Handle;
use pi_atom::Atom;
use pi_hash::{DefaultHasher, XHashMap};
use pi_render::{
    asset::TAssetKeyU64, renderer::{
        bind::*, bind_buffer::{BindBufferAllocator, BindBufferRange}, bind_group::*, shader::{TShaderBindCode, TShaderSetBlock}
    }, rhi::{device::RenderDevice, sampler::EAddressMode, RenderQueue}
};
use pi_share::Share;
use pi_world_macros::Resource;
use crate::{binds::*, prelude::{BindModelLightIndexs, EqAsResource, HashAsResource, KeyShaderMeta}, run_stage::EngineCustomPlugins, shader::* };

#[derive(Resource, Default)]
pub struct MaterialDataMgr {
    pub map: XHashMap<Atom, Vec<MaterialData>>,
}
impl MaterialDataMgr {
    pub fn allocate(&mut self, key_meta: &KeyShaderMeta, meta: &Handle<ShaderEffectMeta>, device: &RenderDevice, allocator: &mut BindBufferAllocator, engineopt: &EngineCustomPlugins) -> Option<Arc<RefBindGroupMaterial>> {
        if self.map.contains_key(key_meta) == false {
            self.map.insert(key_meta.clone(), vec![]);
        }
        if let Some(arr) = self.map.get_mut(key_meta) {
            for item in arr.iter_mut() {
                if let Some(result) = item.allocate() {
                    return Some(Arc::new(result));
                }
            }
            if let Some(mut item) = MaterialData::new(device, key_meta, meta, allocator, engineopt) {
                if let Some(result) = item.allocate() {
                    arr.push(item);
                    return Some(Arc::new(result));
                } else {
                    arr.push(item);
                }
            }
        }
        return None;
    }
    pub fn check(&mut self) {
        for (_, arr) in self.map.iter_mut() {
            let len = arr.len();
            for i in 0..len {
                let idx = len - 1 - i;
                let item = &mut arr[idx];
                if item.seq.len() == item.maxcount {
                    arr.remove(idx);
                }
            }
        }
    }
}

pub struct MaterialData {
    pub seq: Share<SegQueue<u32>>,
    pub effect_value: Option<Arc<ShaderBindEffectValueArr>>,
    pub texture_info: Vec<Arc<BindEffectTextureInfo>>,
    key_bindgroup: KeyBindGroup,
    pub hash: u64,
    pub maxcount: usize,
}
impl MaterialData {
    pub fn new(device: &RenderDevice, key_meta: &KeyShaderMeta, meta: &Handle<ShaderEffectMeta>, allocator: &mut BindBufferAllocator, engineopt: &EngineCustomPlugins) -> Option<Self> {
        let mut result = None;
        let mut texture_info = vec![];
        let mut effect_value = None;

        let mut disenable_material_array = engineopt.disenable_material_array;
        let maxcount = if let Some(bind) = ShaderBindEffectValueArr::new(device, key_meta.clone(), meta.clone(), allocator, engineopt) {
            let maxcount = bind.maxcount;
            effect_value = Some(Arc::new(bind));
            maxcount
        } else {
            let limit = device.limits();
            disenable_material_array = true;
            // if engineopt.disenable_material_array == false {
                engineopt.maxlen_material_array.min(limit.max_uniform_buffer_binding_size / BindEffectTextureInfo::ITEM_SIZE as u32)
            // } else { 1 }
        };

        let texlen = meta.textures.len();
        let seq = Share::new(SegQueue::default());
        for i in 0..maxcount {
            seq.push(i as u32);
        }
        for i in 0..texlen {
            if let Some(bind) = BindEffectTextureInfo::new(maxcount, allocator, engineopt) {
                texture_info.push(Arc::new(bind));
            } else {
                return result;
            }
        }
        
        let mut key_binds = Vec::with_capacity(4);

        if let Some(bind) = &effect_value {
            if let Some(key) = bind.key_bind() {
                key_binds.push(key);
            }
        }

        for bind in texture_info.iter() {
            if let Some(key) = bind.key_bind() {
                key_binds.push(key);
            }
        }

        let mut hasher = DefaultHasher::default();
        effect_value.hash(&mut hasher);
        texture_info.hash(&mut hasher);
        let hash = hasher.finish();

        // log::error!("{:?}", (&seq, seq.len()));

        result = Some(Self {
            seq,
            effect_value,
            texture_info,
            hash,
            key_bindgroup: KeyBindGroup::new(key_binds),
            maxcount: maxcount as usize
        });

        return result;
    }
    pub fn allocate(&mut self, ) -> Option<RefBindGroupMaterial> {
        // log::error!("Alloce: {:?}", self.seq.len());
        if let Some(matidx) = self.seq.pop() {
            Some(RefBindGroupMaterial {
                matidx: matidx as u32,
                hash: self.hash,
                seq: self.seq.clone(),
                effect_value: self.effect_value.clone(),
                texture_info: self.texture_info.clone(),
                key_bindgroup: self.key_bindgroup.clone()
            })
        } else {
            None
        }
    }
}

pub struct RefBindGroupMaterial {
    pub seq: Share<SegQueue<u32>>,
    pub effect_value: Option<Arc<ShaderBindEffectValueArr>>,
    pub texture_info: Vec<Arc<BindEffectTextureInfo>>,
    matidx: u32,
    key_bindgroup: KeyBindGroup,
    pub hash: u64,
}
impl RefBindGroupMaterial {
    pub fn update_data(&self, offset: usize, data: &[u8]) {
        if let Some(buffer) = &self.effect_value {
            let offset = self.matidx as usize * buffer.item_size + offset;
            buffer.data().write_data(offset, data);
        }
    }
    pub fn update_texture(&self, texidx: usize, tilloff: &[u8], wrap_u: EAddressMode, wrap_v: EAddressMode, wrap_w: EAddressMode, coord: u32) {
        if let Some(bind) = self.texture_info.get(texidx) {
            bind.update(self.matidx as usize, tilloff, wrap_u.to_u8() as u32, wrap_v.to_u8() as u32, wrap_w.to_u8() as u32, coord);
        }
    }
    pub fn key_bind_group(&self) -> KeyBindGroup {
        self.key_bindgroup.clone()
    }
    pub fn key_bind_group_layout(&self) -> KeyBindGroupLayout {
        self.key_bindgroup.key_bind_group_layout()
    }
    pub fn matidx(&self) -> u32 {
        self.matidx
    }
}
impl RefBindGroupMaterial {
    pub fn vs_define_code(&self, set: u32, meta: &ShaderEffectMeta, engineopt: &EngineCustomPlugins) -> String {

        let mut result = String::from("");
        let mut bind = 0;
        let mut texidx = 0;

        if let Some(item) = &self.effect_value {
            result += item.vs_define_code(set, bind, engineopt).as_str();
            bind += 1;
        }

        for item in self.texture_info.iter() {
            let key = &meta.textures[texidx];
            result += item.vs_define_code(set, bind, &key.slotname).as_str();
            bind += 1;
            texidx += 1;
        }

        result
    }

    pub fn fs_define_code(&self, set: u32, meta: &ShaderEffectMeta, engineopt: &EngineCustomPlugins) -> String {
        let mut result = String::from("");
        let mut bind = 0;
        let mut texidx = 0;

        if let Some(item) = &self.effect_value {
            result += item.fs_define_code(set, bind, engineopt).as_str();
            bind += 1;
        }
        for item in self.texture_info.iter() {
            let key = &meta.textures[texidx];
            result += item.vs_define_code(set, bind, &key.slotname).as_str();
            bind += 1;
            texidx += 1;
        }

        result
    }
}
impl TAssetKeyU64 for RefBindGroupMaterial {}
impl Hash for RefBindGroupMaterial {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.hash.hash(state);
    }
}
impl PartialEq for RefBindGroupMaterial {
    fn eq(&self, other: &Self) -> bool {
        self.hash == other.hash
    }
}
impl Eq for RefBindGroupMaterial {}
impl Drop for RefBindGroupMaterial {
    fn drop(&mut self) {
        self.seq.push(self.matidx);
    }
}

#[derive(Clone)]
pub struct BindGroupMaterial {
    pub(crate) bind_group: BindGroupUsage,
    pub(crate) key: Arc<RefBindGroupMaterial>,
}
impl BindGroupMaterial {
    pub fn new(
        bind_group: BindGroupUsage,
        key: Arc<RefBindGroupMaterial>,
    ) -> Self {
        Self { bind_group, key }
    }
    pub fn key(&self) -> &RefBindGroupMaterial { &self.key }
    pub fn bind_group(&self) -> &BindGroupUsage { &self.bind_group }
    pub fn vs_running_model_snippet(&self, meta: &ShaderEffectMeta) -> String {
        let mut result = String::from("");
        if self.key.effect_value.is_some() {
            result += meta.uniforms.vs_running_code().as_str();
        }
        result
    }
}
impl BindGroupMaterial {
    pub fn fs_define_code(&self, set: u32, meta: &ShaderEffectMeta, engineopt: &EngineCustomPlugins) -> String {
        self.key.fs_define_code(set, meta, engineopt)
    }

    pub fn vs_define_code(&self, set: u32, meta: &ShaderEffectMeta, engineopt: &EngineCustomPlugins) -> String {
        self.key.vs_define_code(set, meta, engineopt)
    }
}

impl EqAsResource for BindGroupMaterial {
    fn eq_resource(&self, other: &Self) -> bool {
        self.bind_group.key() == other.bind_group.key() && self.key == other.key
    }
}
impl HashAsResource for BindGroupMaterial {
    fn hash_resource<H: std::hash::Hasher>(&self, state: &mut H) {
        self.bind_group.key().asset_u64().hash(state);
    }
}
