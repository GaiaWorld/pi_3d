use std::{marker::PhantomData, sync::Arc};

use pi_assets::{asset::Handle, mgr::AssetMgr};
use pi_render::{rhi::{device::RenderDevice}, renderer::{bind_buffer::{BindBufferAllocator}, shader::KeyShaderMeta}, render_3d::binds::effect_value::ShaderBindEffectValue};
use pi_scene_math::Number;
use pi_share::Share;
use crate::{
    materials::{value::FromValueUniformStatistics, shader_effect::{ShaderEffectMeta}},
};

use super::{
    float::{FloatUniform},
    int::{IntUniform},
    uint::{UintUniform},
    mat4::Mat4Uniform,
    mat2::Mat2Uniform,
    vec4::Vec4Uniform,
    vec2::Vec2Uniform,
};

pub struct BindEffectValueDirty(pub bool);

pub struct BindEffectValues {
    pub mat4_: Mat4Uniform,
    pub mat2_: Mat2Uniform,
    pub vec4_: Vec4Uniform,
    pub vec2_: Vec2Uniform,
    pub float: FloatUniform,
    pub int__: IntUniform,
    pub uint_: UintUniform,
    pub bind: Arc<ShaderBindEffectValue>,
    pub dirty: bool,
}
impl BindEffectValues {
    pub fn new(
        device: &RenderDevice,
        key_meta: KeyShaderMeta,
        meta: Handle<ShaderEffectMeta>,
        allocator: &mut BindBufferAllocator,
    ) -> Option<Self> {
        if let Some(effect_val_bind) = ShaderBindEffectValue::new(device, key_meta, meta.clone(), allocator) {
            log::info!("ShaderBindEffectValue: {:?},  {:?}", effect_val_bind.data().offset(), effect_val_bind.data().size());

            let uniforms = &meta.uniforms;
            let mut mat4 = Mat4Uniform::new(&effect_val_bind);     mat4.init(&uniforms.mat4_list);
            let mut mat2 = Mat2Uniform::new(&effect_val_bind);     mat2.init(&uniforms.mat2_list);
            let mut vec4 = Vec4Uniform::new(&effect_val_bind);     vec4.init(&uniforms.vec4_list); 
            let mut vec2 = Vec2Uniform::new(&effect_val_bind);     vec2.init(&uniforms.vec2_list);
            let mut float = FloatUniform::new(&effect_val_bind);    float.init(&uniforms.float_list);
            let mut int = IntUniform::new(&effect_val_bind);      int.init(&uniforms.int_list);
            let mut uint = UintUniform::new(&effect_val_bind);     uint.init(&uniforms.uint_list);

            Some(Self {
                mat4_: mat4,
                mat2_: mat2,
                vec4_: vec4,
                vec2_: vec2,
                float,
                int__: int,
                uint_: uint,
                bind: Arc::new(effect_val_bind),
                dirty: true,
            })
        } else {
            None
        }
    }

    pub fn mat4(&mut self, slot: usize, value: &[Number]) {
        self.dirty = true;
        self.mat4_.set(slot, value);
    }
    
    pub fn mat2(&mut self, slot: usize, value: &[Number]) {
        self.dirty = true;
        self.mat2_.set(slot, value);
    }
    
    pub fn vec4(&mut self, slot: usize, value: &[Number]) {
        self.dirty = true;
        self.vec4_.set(slot, value);
    }
    
    pub fn vec2(&mut self, slot: usize, value: &[Number]) {
        self.dirty = true;
        self.vec2_.set(slot, value);
    }
    
    pub fn float(&mut self, slot: usize, value: Number) {
        self.dirty = true;
        self.float.set(slot, value);
    }
    
    pub fn int(&mut self, slot: usize, value: i32) {
        self.dirty = true;
        self.int__.set(slot, value);
    }
    
    pub fn uint(&mut self, slot: usize, value: u32) {
        self.dirty = true;
        self.uint_.set(slot, value);
    }

    pub fn update(&self) {
        let range = self.bind.data();
        self.mat4_.update(range);
        self.mat2_.update(range);
        self.vec4_.update(range);
        self.vec2_.update(range);
        self.float.update(range);
        self.int__.update(range);
        self.uint_.update(range);
    }
}
