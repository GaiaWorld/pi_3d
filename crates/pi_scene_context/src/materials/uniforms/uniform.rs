use std::sync::Arc;

use derive_deref::{Deref, DerefMut};
use pi_scene_shell::prelude::*;

#[derive(Component, Default)]
pub struct BindEffectReset;


#[derive(Component, Deref, DerefMut, Default)]
pub struct BindEffect(pub Option<BindEffectValues>);

#[derive(Default, Component)]
pub struct UniformAnimated(pub Vec<Atom>);
impl UniformAnimated {
    pub fn add(&mut self, key: &Atom) {
        match self.0.binary_search(key) {
            Ok(_) => {},
            Err(idx) => { self.0.insert(idx, key.clone()); },
        }
    }
}

#[derive(Debug, Clone)]
pub struct UniformOffset {
    vtype: EUniformValueType,
    offset: u16,
    entity: Option<Entity>,
}
impl UniformOffset {
    pub fn new(
        vtype: EUniformValueType,
        offset: u16,
        entity: Option<Entity>,
    ) -> Self {
        Self { vtype, offset, entity }
    }
    pub fn vtype(&self) -> EUniformValueType { self.vtype }
    pub fn atype(&self) -> EAnimatorableType { self.vtype.animatorable_type() }
    pub fn offset(&self) -> usize { self.offset as usize }
    pub fn entity(&self) -> Option<Entity> { self.entity }
    pub fn strip_offset(&self) -> (usize, usize, Option<Entity>) {
        (self.vtype.strip(), self.offset as usize, self.entity)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum EUniformValueType {
    Mat4,
    Vec4,
    Vec3,
    Vec2,
    Float,
    Uint,
}
impl EUniformValueType {
    pub fn strip(&self) -> usize {
        match self {
            EUniformValueType::Mat4 => 16 * 4,
            EUniformValueType::Vec4 => 4 * 4,
            EUniformValueType::Vec3 => 3 * 4,
            EUniformValueType::Vec2 => 2 * 4,
            EUniformValueType::Float => 1 * 4,
            EUniformValueType::Uint => 1 * 4,
        }
    }
    pub fn animatorable_type(&self) -> EAnimatorableType {
        match self {
            EUniformValueType::Mat4 => { panic!("") },
            EUniformValueType::Vec4 => EAnimatorableType::Vec4,
            EUniformValueType::Vec3 => EAnimatorableType::Vec3,
            EUniformValueType::Vec2 => EAnimatorableType::Vec2,
            EUniformValueType::Float => EAnimatorableType::Float,
            EUniformValueType::Uint => EAnimatorableType::Uint,
        }
    }
}

pub struct UnAnimatorableUniformOffset {
    vtype: EUniformValueType,
    offset: u16,
}
impl UnAnimatorableUniformOffset {
    pub fn new(
        vtype: EUniformValueType,
        offset: u16,
    ) -> Self {
        Self { vtype, offset }
    }
    pub fn vtype(&self) -> EUniformValueType { self.vtype }
    pub fn offset(&self) -> u16 { self.offset }
}

pub struct BindEffectValues {
    // offsets: XHashMap<Atom, UniformOffset>,
    bytes: Vec<u8>,
    offsets: Vec<(Atom, UniformOffset)>,
    bind: Arc<ShaderBindEffectValue>,
}
impl BindEffectValues {
    pub fn new(
        device: &PiRenderDevice,
        key_meta: KeyShaderMeta,
        meta: Handle<ShaderEffectMeta>,
        allocator: &mut BindBufferAllocator,
    ) -> Option<Self> {
        
        if let Some(bind) = ShaderBindEffectValue::new(device, key_meta, meta.clone(), allocator) {
            let mut bytes: Vec<u8> = vec![];
            let mut offsets: Vec<(Atom, UniformOffset)> = vec![];
            // let mut offsets: XHashMap<Atom, UniformOffset> = XHashMap::default();
    
            meta.uniforms.mat4_list.iter().for_each(|item| {
                Self::_new(item.0.clone(), EUniformValueType::Mat4, bytemuck::cast_slice(&item.1), &mut bytes, &mut offsets);
                // let offset = UniformOffset::new(EUniformValueType::Mat4, bytes.len() as u16, None);
                // bytemuck::cast_slice(&item.1).iter().for_each(|v| { bytes.push(*v); });
                // offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            meta.uniforms.vec4_list.iter().for_each(|item| {
                Self::_new(item.0.clone(), EUniformValueType::Vec4, bytemuck::cast_slice(&item.1), &mut bytes, &mut offsets);
                // let offset = UniformOffset::new(EUniformValueType::Vec4, bytes.len() as u16, None);
                // bytemuck::cast_slice(&item.1).iter().for_each(|v| { bytes.push(*v); });
                // offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            meta.uniforms.vec3_list.iter().for_each(|item| {
                Self::_new(item.0.clone(), EUniformValueType::Vec3, bytemuck::cast_slice(&item.1), &mut bytes, &mut offsets);
                // let offset = UniformOffset::new(EUniformValueType::Vec3, bytes.len() as u16, None);
                // bytemuck::cast_slice(&item.1).iter().for_each(|v| { bytes.push(*v); });
                bytemuck::cast_slice(&[0.0f32]).iter().for_each(|v| { bytes.push(*v); });
                // offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            meta.uniforms.vec2_list.iter().for_each(|item| {
                Self::_new(item.0.clone(), EUniformValueType::Vec2, bytemuck::cast_slice(&item.1), &mut bytes, &mut offsets);
                // let offset = UniformOffset::new(EUniformValueType::Vec2, bytes.len() as u16, None);
                // bytemuck::cast_slice(&item.1).iter().for_each(|v| { bytes.push(*v); });
                // offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            let fill_vec2_count    = meta.uniforms.vec2_list.len() % 2;
            for _ in 0..fill_vec2_count {
                bytemuck::cast_slice(&[0.0f32, 0.0f32]).iter().for_each(|v| { bytes.push(*v); });
            }
            meta.uniforms.float_list.iter().for_each(|item| {
                Self::_new(item.0.clone(), EUniformValueType::Float, bytemuck::cast_slice(&[item.1]), &mut bytes, &mut offsets);
                // let offset = UniformOffset::new(EUniformValueType::Float, bytes.len() as u16, None);
                // bytemuck::cast_slice(&[item.1]).iter().for_each(|v| { bytes.push(*v); });
                // offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            meta.uniforms.uint_list.iter().for_each(|item| {
                Self::_new(item.0.clone(), EUniformValueType::Uint, bytemuck::cast_slice(&[item.1]), &mut bytes, &mut offsets);
                // let offset = UniformOffset::new(EUniformValueType::Uint, bytes.len() as u16, None);
                // bytemuck::cast_slice(&[item.1]).iter().for_each(|v| { bytes.push(*v); });
                // offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
    
            bind.data().write_data(0, &bytes);
            offsets.sort_by(|a, b| a.0.cmp(&b.0) );

            // log::error!("MEAT: {:?}", meta.uniforms);
    
            Some(
                BindEffectValues { bytes, offsets, bind: Arc::new(bind), }
            )
        } else {
            None
        }
    }
    fn _new(
        key: Atom,
        vtype: EUniformValueType,
        data: &[u8],
        bytes: &mut Vec<u8>,
        offsets: &mut  Vec<(Atom, UniformOffset)>,
    ) {
        let offset = UniformOffset::new(vtype, bytes.len() as u16, None);
        data.iter().for_each(|v| { bytes.push(*v); });
        offsets.push((key, offset));
    }
    pub fn animator(
        &mut self,
        key: &Atom,
        item: Entity,
        command: &mut Insert<()>,
        animatorablefloat: &mut ActionListAnimatorableFloat,
        animatorablevec2s: &mut ActionListAnimatorableVec2,
        animatorablevec3s: &mut ActionListAnimatorableVec3,
        animatorablevec4s: &mut ActionListAnimatorableVec4,
        animatorableuints: &mut ActionListAnimatorableUint,
    ) -> Option<UniformOffset> {
        let linked = item;
        match self.offsets.binary_search_by(|v| v.0.cmp(key) ) {
            Ok(idx) => {
                let offset = &mut self.offsets.get_mut(idx).unwrap().1;
                if offset.entity.is_none() {
                    let entity = command.insert(());
                    offset.entity = Some(entity);
                    match offset.atype() {
                        EAnimatorableType::Vec4     => {
                            let start = offset.offset as usize; let end = offset.offset as usize + 16;
                            let data = bytemuck::cast_slice(&self.bytes[start..end]);
                            animatorablevec4s.push(OpsAnimatorableVec4::ops(entity, linked, AnimatorableVec4::from(data), EAnimatorableEntityType::Uniform));
                        },
                        EAnimatorableType::Vec3     => {
                            let start = offset.offset as usize; let end = offset.offset as usize + 12;
                            let data = bytemuck::cast_slice(&self.bytes[start..end]);
                            animatorablevec3s.push(OpsAnimatorableVec3::ops(entity, linked, AnimatorableVec3::from(data), EAnimatorableEntityType::Uniform));
                        },
                        EAnimatorableType::Vec2     => {
                            let start = offset.offset as usize; let end = offset.offset as usize + 8;
                            let data = bytemuck::cast_slice(&self.bytes[start..end]);
                            animatorablevec2s.push(OpsAnimatorableVec2::ops(entity, linked, AnimatorableVec2::from(data), EAnimatorableEntityType::Uniform));
                        },
                        EAnimatorableType::Float    => {
                            let start = offset.offset as usize; let end = offset.offset as usize + 4;
                            let data = bytemuck::cast_slice(&self.bytes[start..end]);
                            animatorablefloat.push(OpsAnimatorableFloat::ops(entity, linked, AnimatorableFloat(data[0]), EAnimatorableEntityType::Uniform));
                        },
                        EAnimatorableType::Uint     => {
                            let start = offset.offset as usize; let end = offset.offset as usize + 4;
                            let data = bytemuck::cast_slice(&self.bytes[start..end]);
                            animatorableuints.push(OpsAnimatorableUint::ops(entity, linked, AnimatorableUint(data[0]), EAnimatorableEntityType::Uniform));
                        },
                        EAnimatorableType::Int => {
                        },
                    }
                }
                Some(offset.clone())
            },
            Err(_) => { None },
        }
    }
    pub fn update(&mut self, mut offset: usize, value: &[u8]) {
        value.iter().for_each(|v| { self.bytes[offset] = *v; offset += 1; });
    }
    pub fn bind(&self) -> Arc<ShaderBindEffectValue> {
        self.bind.clone()
    }
    pub fn offset(&self, key: &Atom) -> Option<&UniformOffset> {
        match self.offsets.binary_search_by(|v| v.0.cmp(key) ) {
            Ok(idx) => Some(&self.offsets.get(idx).unwrap().1),
            Err(_) => None,
        }
    }
    pub fn uniforms(&self) -> &Vec<(Atom, UniformOffset)> {
        &self.offsets
    }
    pub fn log(&self) {
        // log::error!("{:?}", &self.offsets);
    }
}



// pub struct BindEffectValues {
//     pub mat4_: Mat4Uniform,
//     // pub mat2_: Mat2Uniform,
//     pub vec4_: Vec4Uniform,
//     pub vec2_: Vec2Uniform,
//     pub float: FloatUniform,
//     // pub int__: IntUniform,
//     pub uint_: UintUniform,
//     pub bind: Arc<ShaderBindEffectValue>,
//     pub dirty: bool,
//     pub keys: XHashMap<Atom, usize>,
// }
// impl BindEffectValues {
//     pub fn new(
//         device: &PiRenderDevice,
//         key_meta: KeyShaderMeta,
//         meta: Handle<ShaderEffectMeta>,
//         allocator: &mut BindBufferAllocator,
//     ) -> Option<Self> {
//         if let Some(effect_val_bind) = ShaderBindEffectValue::new(device, key_meta, meta.clone(), allocator) {

//             let mut keys = XHashMap::default();

//             let uniforms = &meta.uniforms;
//             let mut mat4 = Mat4Uniform::new(&effect_val_bind);     mat4.init(&uniforms.mat4_list);
//             // let mut mat2 = Mat2Uniform::new(&effect_val_bind);     mat2.init(&uniforms.mat2_list);
//             let mut vec4 = Vec4Uniform::new(&effect_val_bind);     vec4.init(&uniforms.vec4_list); 
//             let mut vec2 = Vec2Uniform::new(&effect_val_bind);     vec2.init(&uniforms.vec2_list);
//             let mut float = FloatUniform::new(&effect_val_bind);    float.init(&uniforms.float_list);
//             // let mut int = IntUniform::new(&effect_val_bind);      int.init(&uniforms.int_list);
//             let mut uint = UintUniform::new(&effect_val_bind);     uint.init(&uniforms.uint_list);
            
//             let mut index = 0;
//             uniforms.mat4_list.iter().for_each(|v| {
//                 keys.insert(v.0.clone(), index); index += 1;
//             });
//             // let mut index = 0;
//             // uniforms.mat2_list.iter().for_each(|v| {
//             //     keys.insert(v.0.clone(), index); index += 1;
//             // });
//             let mut index = 0;
//             uniforms.vec4_list.iter().for_each(|v| {
//                 keys.insert(v.0.clone(), index); index += 1;
//             });
//             let mut index = 0;
//             uniforms.vec2_list.iter().for_each(|v| {
//                 keys.insert(v.0.clone(), index); index += 1;
//             });
//             let mut index = 0;
//             uniforms.float_list.iter().for_each(|v| {
//                 keys.insert(v.0.clone(), index); index += 1;
//             });
//             // let mut index = 0;
//             // uniforms.int_list.iter().for_each(|v| {
//             //     keys.insert(v.0.clone(), index); index += 1;
//             // });
//             let mut index = 0;
//             uniforms.uint_list.iter().for_each(|v| {
//                 keys.insert(v.0.clone(), index); index += 1;
//             });

//             Some(Self {
//                 mat4_: mat4,
//                 // mat2_: mat2,
//                 vec4_: vec4,
//                 vec2_: vec2,
//                 float,
//                 // int__: int,
//                 uint_: uint,
//                 bind: Arc::new(effect_val_bind),
//                 dirty: true,
//                 keys
//             })
//         } else {
//             None
//         }
//     }

//     pub fn mat4(&mut self, slot: usize, value: &[Number]) {
//         let item = self; // unsafe { &mut *(self as *const Self as usize as *mut Self) };
//         item.dirty = true;
//         item.mat4_.set(slot, value);
//     }
    
//     // pub fn mat2(& self, slot: usize, value: &[Number]) {
//     //     let item = unsafe { &mut *(self as *const Self as *mut Self) };
//     //     item.dirty = true;
//     //     item.mat2_.set(slot, value);
//     // }
    
//     pub fn vec4(&mut self, slot: usize, value: &[Number]) {
//         let item = self; // unsafe { &mut *(self as *const Self as usize as *mut Self) };
//         item.dirty = true;
//         item.vec4_.set(slot, value);
//     }
    
//     pub fn vec2(&mut self, slot: usize, value: &[Number]) {
//         let item = self; // unsafe { &mut *(self as *const Self as usize as *mut Self) };
//         item.dirty = true;
//         item.vec2_.set(slot, value);
//     }
    
//     pub fn float(&mut self, slot: usize, value: Number) {
//         let item = self; // unsafe { &mut *(self as *const Self as usize as *mut Self) };
//         item.dirty = true;
//         item.float.set(slot, value);
//     }
    
//     // pub fn int(& self, slot: usize, value: i32) {
//     //     let item = unsafe { &mut *(self as *const Self as *mut Self) };
//     //     item.dirty = true;
//     //     item.int__.set(slot, value);
//     // }
    
//     pub fn uint(&mut self, slot: usize, value: u32) {
//         let item = self; // unsafe { &mut *(self as *const Self as usize as *mut Self) };
//         item.dirty = true;
//         item.uint_.set(slot, value);
//     }

//     pub fn update(&self) {
//         let range = self.bind.data();
//         self.mat4_.update(range);
//         // self.mat2_.update(range);
//         self.vec4_.update(range);
//         self.vec2_.update(range);
//         self.float.update(range);
//         // self.int__.update(range);
//         self.uint_.update(range);
//         // log::warn!("{:?}", self.vec4_.data);
//     }

//     // pub fn mat4_one(& self, slot: usize, offset: usize, value: Number) {
//     //     let item = unsafe { &mut *(self as *const Self as *mut Self) };
//     //     item.dirty = true;
//     //     if let Some(data) = item.mat4_.value_mut(slot) {
//     //         data[offset] = value;
//     //     }
//     // }

//     // pub fn mat2_one(& self, slot: usize, offset: usize, value: Number) {
//     //     let item = unsafe { &mut *(self as *const Self as *mut Self) };
//     //     item.dirty = true;
//     //     if let Some(data) = item.mat2_.value_mut(slot) {
//     //         data[offset] = value;
//     //     }
//     // }

//     pub fn vec4_one(&mut self, slot: usize, offset: usize, value: Number) {
//         let item = self; // unsafe { &mut *(self as *const Self as usize as *mut Self) };
//         item.dirty = true;
//         if slot < item.vec4_.slot as usize {
//             // log::warn!("{:?}: {:?}", slot * Vec4Uniform::N + offset, value);
//             item.vec4_.data[slot * Vec4Uniform::N + offset] = value;
//             // log::warn!("{:?}", item.vec4_.data[slot * Vec4Uniform::N + offset]);
//         }
//         // log::warn!("{:?}", item.vec4_.data);
//         // if let Some(data) = item.vec4_.value_mut(slot) {
//         //     data[offset] = value;
//         // }
//     }

//     pub fn vec2_one(&mut self, slot: usize, offset: usize, value: Number) {
//         let item = self; // unsafe { &mut *(self as *const Self as usize as *mut Self) };
//         item.dirty = true;
//         if let Some(data) = item.vec2_.value_mut(slot) {
//             data[offset] = value;
//         }
//     }

//     pub fn slot(&self, key: &Atom) -> Option<usize> {
//         if let Some(slot) = self.keys.get(key) {
//             Some(*slot)
//         } else {
//             None
//         }
//     }
// }
