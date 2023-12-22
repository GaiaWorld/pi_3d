use std::sync::Arc;

use pi_engine_shell::prelude::*;
use pi_assets::asset::Handle;
use pi_hash::XHashMap;

#[derive(Component)]
pub struct BindEffectReset;


#[derive(Component, Deref, DerefMut)]
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

pub struct AnimatorableUniformOffset {
    vtype: EUniformValueType,
    offset: u16,
    entity: Entity,
}
impl AnimatorableUniformOffset {
    pub fn new(
        vtype: EUniformValueType,
        offset: u16,
        entity: Entity,
    ) -> Self {
        Self { vtype, offset, entity }
    }
    pub fn vtype(&self) -> EUniformValueType { self.vtype }
    pub fn atype(&self) -> EAnimatorableType { self.vtype.animatorable_type() }
    pub fn offset(&self) -> u16 { self.offset }
    pub fn entity(&self) -> Entity { self.entity }
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

pub enum UniformOffset {
    Animatorable(AnimatorableUniformOffset),
    UnAnimatorable(UnAnimatorableUniformOffset),
}
impl UniformOffset {
    pub fn strip_offset(&self) -> (usize, usize, Option<Entity>) {
        match self {
            UniformOffset::Animatorable(val) => {
                (val.vtype.strip(), val.offset as usize, Some(val.entity))
            },
            UniformOffset::UnAnimatorable(val) => {
                (val.vtype.strip(), val.offset as usize, None)
            },
        }
    }
    pub fn entity(&self) -> Option<Entity> {
        match self {
            UniformOffset::Animatorable(val) => {
                Some(val.entity)
            },
            UniformOffset::UnAnimatorable(_) => {
                None
            },
        }
    }
}

pub struct BindEffectValues {
    // offsets: XHashMap<Atom, UniformOffset>,
    offsets: Vec<(Atom, UniformOffset)>,
    bind: Arc<ShaderBindEffectValue>,
}
impl BindEffectValues {
    pub fn new(
        commands: &mut Commands,
        mat: Entity,
        device: &PiRenderDevice,
        key_meta: KeyShaderMeta,
        meta: Handle<ShaderEffectMeta>,
        allocator: &mut BindBufferAllocator,
        cmds: (
            &mut ActionListAnimatorableFloat,
            &mut ActionListAnimatorableVec2,
            &mut ActionListAnimatorableVec3,
            &mut ActionListAnimatorableVec4,
            &mut ActionListAnimatorableUint,
        )
    ) -> Option<Self> {
        
        if let Some(bind) = ShaderBindEffectValue::new(device, key_meta, meta.clone(), allocator) {
            let linked = mat;
            let (animatorablefloats, 
                animatorablevec2s,
                animatorablevec3s, 
                animatorablevec4s, 
                animatorableuints
            ) = cmds;

            let mut bytes: Vec<u8> = vec![];
            let mut offsets: Vec<(Atom, UniformOffset)> = vec![];
            // let mut offsets: XHashMap<Atom, UniformOffset> = XHashMap::default();
    
            meta.uniforms.mat4_list.iter().for_each(|item| {
                if item.2 {
                    // let entity = commands.spawn_empty().id();
                    // let offset = UniformOffset::Animatorable(AnimatorableUniformOffset::new(EAnimatorableType::VEc4, offset, entity))
                } else {
                    let offset = UniformOffset::UnAnimatorable(UnAnimatorableUniformOffset::new(EUniformValueType::Mat4, bytes.len() as u16));
                    bytemuck::cast_slice(&item.1).iter().for_each(|v| { bytes.push(*v); });
                    // keys.push(item.0.clone());
                    offsets.push((item.0.clone(), offset));
                    // offsets.insert(item.0.clone(), offset);
                }
            });
            meta.uniforms.vec4_list.iter().for_each(|item| {
                let offset = if item.2 {
                    let entity = commands.spawn_empty().id();
                    animatorablevec4s.push(OpsAnimatorableVec4::ops(entity, linked, AnimatorableVec4::from(&item.1)));
                    UniformOffset::Animatorable(AnimatorableUniformOffset::new(EUniformValueType::Vec4, bytes.len() as u16, entity))
                } else {
                    UniformOffset::UnAnimatorable(UnAnimatorableUniformOffset::new(EUniformValueType::Vec4, bytes.len() as u16))
                };
                bytemuck::cast_slice(&item.1).iter().for_each(|v| { bytes.push(*v); });
    
                // keys.push(item.0.clone());
                offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            meta.uniforms.vec3_list.iter().for_each(|item| {
                let offset = if item.2 {
                    let entity = commands.spawn_empty().id();
                    animatorablevec3s.push(OpsAnimatorableVec3::ops(entity, linked, AnimatorableVec3::from(&item.1)));
                    UniformOffset::Animatorable(AnimatorableUniformOffset::new(EUniformValueType::Vec3, bytes.len() as u16, entity))
                } else {
                    UniformOffset::UnAnimatorable(UnAnimatorableUniformOffset::new(EUniformValueType::Vec3, bytes.len() as u16))
                };
                bytemuck::cast_slice(&item.1).iter().for_each(|v| { bytes.push(*v); });
                bytemuck::cast_slice(&[0.0f32]).iter().for_each(|v| { bytes.push(*v); });
    
                // keys.push(item.0.clone());
                offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            meta.uniforms.vec2_list.iter().for_each(|item| {
                let offset = if item.2 {
                    let entity = commands.spawn_empty().id();
                    animatorablevec2s.push(OpsAnimatorableVec2::ops(entity, linked, AnimatorableVec2::from(&item.1)));
                    UniformOffset::Animatorable(AnimatorableUniformOffset::new(EUniformValueType::Vec2, bytes.len() as u16, entity))
                } else {
                    UniformOffset::UnAnimatorable(UnAnimatorableUniformOffset::new(EUniformValueType::Vec2, bytes.len() as u16))
                };
                bytemuck::cast_slice(&item.1).iter().for_each(|v| { bytes.push(*v); });
    
                // keys.push(item.0.clone());
                offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            let fill_vec2_count    = meta.uniforms.vec2_list.len() % 2;
            for _ in 0..fill_vec2_count {
                bytemuck::cast_slice(&[0.0f32, 0.0f32]).iter().for_each(|v| { bytes.push(*v); });
            }
            meta.uniforms.float_list.iter().for_each(|item| {
                let offset = if item.2 {
                    let entity = commands.spawn_empty().id();
                    animatorablefloats.push(OpsAnimatorableFloat::ops(entity, linked, AnimatorableFloat(item.1.clone())));
                    UniformOffset::Animatorable(AnimatorableUniformOffset::new(EUniformValueType::Float, bytes.len() as u16, entity))
                } else {
                    UniformOffset::UnAnimatorable(UnAnimatorableUniformOffset::new(EUniformValueType::Float, bytes.len() as u16))
                };
                log::error!("Float {:?}", (&item.0, item.1, bytes.len()));
                bytemuck::cast_slice(&[item.1]).iter().for_each(|v| { bytes.push(*v); });
                // keys.push(item.0.clone());
                offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
            meta.uniforms.uint_list.iter().for_each(|item| {
                let offset = if item.2 {
                    let entity = commands.spawn_empty().id();
                    animatorableuints.push(OpsAnimatorableUint::ops(entity, linked, AnimatorableUint(item.1.clone())));
                    UniformOffset::Animatorable(AnimatorableUniformOffset::new(EUniformValueType::Uint, bytes.len() as u16, entity))
                } else {
                    UniformOffset::UnAnimatorable(UnAnimatorableUniformOffset::new(EUniformValueType::Uint, bytes.len() as u16))
                };
                bytemuck::cast_slice(&[item.1]).iter().for_each(|v| { bytes.push(*v); });
    
                // keys.push(item.0.clone());
                offsets.push((item.0.clone(), offset));
                // offsets.insert(item.0.clone(), offset);
            });
    
            bind.data().write_data(0, &bytes);
            offsets.sort_by(|a, b| a.0.cmp(&b.0) );
    
            Some(
                BindEffectValues {
                    offsets,
                    bind: Arc::new(bind),
                }
            )
        } else {
            None
        }
    }
    pub fn query_animatorable(&self, key: &Atom) -> Option<&AnimatorableUniformOffset> {
        match self.offset(key) {
            Some(offset) => match offset {
                UniformOffset::Animatorable(offset) => Some(offset),
                UniformOffset::UnAnimatorable(_) => None,
            },
            None => None,
        }
    }
    pub fn update(&self, key: &Atom, value: &[u8]) -> Option<Entity> {
        match self.offset(key) {
            Some(offset) => {
                let (strip, offset, entity) = offset.strip_offset();
                if strip <= value.len() {
                    self.bind.data().write_data(offset, &value[0..strip]);
                }
                entity
            },
            None => None,
        }
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
