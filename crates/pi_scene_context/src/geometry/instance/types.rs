
use pi_scene_shell::prelude::*;

fn _strip(val: &ECustomVertexType) -> usize {
    match val {
        ECustomVertexType::Vec4     => 4 * 4,
        ECustomVertexType::Vec3     => 3 * 4,
        ECustomVertexType::Vec2     => 2 * 4,
        ECustomVertexType::Float    => 1 * 4,
        ECustomVertexType::Uint     => 1 * 4,
        ECustomVertexType::Int      => 1 * 4,
        ECustomVertexType::IVec4    => 4 * 4,
        ECustomVertexType::UVec4    => 4 * 4,
        ECustomVertexType::U16x2    => 2 * 2,
        ECustomVertexType::U16x4    => 2 * 4,
        ECustomVertexType::U8x4     => 1 * 4,
        ECustomVertexType::Unorm16x2 => 2 * 2,
        ECustomVertexType::Unorm16x4 => 2 * 4,
        ECustomVertexType::Unorm8x4  => 1 * 4,
    }
}
fn animatorable_type(val: &ECustomVertexType) -> Option<EAnimatorableType> {
    match val {
        ECustomVertexType::Vec4     => Some(EAnimatorableType::Vec4),
        ECustomVertexType::Vec3     => Some(EAnimatorableType::Vec3),
        ECustomVertexType::Vec2     => Some(EAnimatorableType::Vec2),
        ECustomVertexType::Float    => Some(EAnimatorableType::Float),
        ECustomVertexType::Uint     => Some(EAnimatorableType::Uint),
        ECustomVertexType::Int      => Some(EAnimatorableType::Int ),
        ECustomVertexType::IVec4    => None,
        ECustomVertexType::U16x2    => None,
        ECustomVertexType::U16x4    => None,
        ECustomVertexType::U8x4     => None,
        ECustomVertexType::Unorm16x2 => None,
        ECustomVertexType::Unorm16x4 => None,
        ECustomVertexType::Unorm8x4  => None,
        ECustomVertexType::UVec4 => None,
    }
}

/// 记录模型实例动画的属性名
#[derive(Component, Default)]
pub struct InstanceAttributeAnimated(pub Vec<Atom>);
impl InstanceAttributeAnimated {
    pub fn add(&mut self, key: &Atom) {
        match self.0.binary_search(key) {
            Ok(_) => {},
            Err(idx) => { self.0.insert(idx, key.clone()); },
        }
    }
}

#[derive(Clone)]
pub struct InstanceAttributeOffset {
    vtype: ECustomVertexType,
    entity: Option<Entity>,
    offset: u32,
}
impl InstanceAttributeOffset {
    pub fn new(
        vtype: ECustomVertexType,
        offset: u32,
        entity: Option<Entity>,
    ) -> Self {
        Self { vtype, offset, entity }
    }
    /// 值类型
    pub fn vtype(&self) -> ECustomVertexType { self.vtype }
    /// 对应值动画类型
    pub fn atype(&self) -> Option<EAnimatorableType> { animatorable_type(&self.vtype) }
    pub fn offset(&self) -> u32 { self.offset }
    pub fn entity(&self) -> Option<Entity> { self.entity }
}

/// 记录模型实例的属性数据 及 存储信息
#[derive(Component, Default)]
pub struct ModelInstanceAttributes {
    bytes: Vec<u8>,
    attributes: Vec<(Atom, InstanceAttributeOffset)>,
    worldmatrix: bool,
    matarray: bool,
}
impl ModelInstanceAttributes {
    pub fn new(
        insances: &Vec<CustomVertexAttribute>,
        worldmatrix: bool,
        matarray: bool,
    ) -> Self {

        let tmp: [f32;4] = [0., 0., 0., 0.];

        let mut bytes = vec![];
        let mut attributes = vec![];
        let mut offset = 0;

        if worldmatrix {
            unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(Matrix::identity().as_slice()));
            // bytemuck::cast_slice(Matrix::identity().as_slice()).iter().for_each(|byte| { bytes.push(*byte) });
            offset += 64;
        }
        // matidx
        if matarray && (worldmatrix || insances.len() > 0)  {
            unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u32, 0u32, 0u32, 0u32]));
            offset += 16;
        }

        insances.iter().for_each(|attr| {
            // let entity = command.spawn_empty_id();
            let atype = animatorable_type(&attr.vtype());
            attributes.push((Atom::from(attr.var_code()), InstanceAttributeOffset::new(attr.vtype(), offset, None)));
            match attr.vtype() {
                ECustomVertexType::Vec4     => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&tmp[0..4]));
                    offset += 16;
                },
                ECustomVertexType::UVec4     => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u32, 0u32, 0u32, 0u32]));
                    offset += 16;
                },
                ECustomVertexType::Vec3     => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&tmp[0..3]));
                    offset += 12;
                },
                ECustomVertexType::Vec2     => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&tmp[0..2]));
                    offset += 8;
                },
                ECustomVertexType::Float    => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&tmp[0..1]));
                    offset += 4;
                },
                ECustomVertexType::Uint     => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u32]));
                    offset += 4;
                },
                ECustomVertexType::Int      => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0i32]));
                    offset += 4;
                },
                ECustomVertexType::IVec4    => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0i32, 0i32, 0i32, 0i32]));
                    offset += 16;
                },
                ECustomVertexType::U16x2    => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u16, 0u16]));
                    offset += 4;
                },
                ECustomVertexType::U16x4    => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u16, 0u16, 0u16, 0u16]));
                    offset += 8;
                },
                ECustomVertexType::U8x4     => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u8, 0u8, 0u8, 0u8]));
                    offset += 4;
                },
                ECustomVertexType::Unorm16x2 => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u16, 0u16]));
                    offset += 4;
                },
                ECustomVertexType::Unorm16x4 => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u16, 0u16, 0u16, 0u16]));
                    offset += 8;
                },
                ECustomVertexType::Unorm8x4  => {
                    unsafe_vec_append_slice(&mut bytes, bytemuck::cast_slice(&[0u8, 0u8, 0u8, 0u8]));
                    offset += 4;
                },
            }
        });

        attributes.sort_by(|a, b| a.0.cmp(&b.0) );

        Self {
            bytes, attributes, worldmatrix, matarray
        }
    }
    pub fn worldmatrix(&self) -> bool {
        self.worldmatrix
    }
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
    pub fn update_worldmatrix(&mut self, data: &Matrix) -> bool {
        if self.worldmatrix {
            self.bytes.as_mut_slice()[0..64].copy_from_slice(bytemuck::cast_slice(data.as_slice()));
            // let mut idx = 0;
            // bytemuck::cast_slice(data.as_slice()).iter().for_each(|v| {
            //     self.bytes[idx] = *v;
            //     idx += 1;
            // });
        }
        return self.worldmatrix;
    }
    pub fn update_matidx(&mut self, passidx: usize, data: u16) {
        if self.matarray == false || self.bytes.len() == 0 { return }
        let mut idx = if self.worldmatrix { 64 } else { 0 };
        idx += passidx * 2;
        self.bytes.as_mut_slice()[idx..(idx+2)].copy_from_slice(bytemuck::cast_slice(&[data]));
        // bytemuck::cast_slice(&[data]).iter().for_each(|v| {
        //     self.bytes[idx] = *v;
        //     idx += 1;
        // });
    }
    pub fn update_matidxs(&mut self, data: &[u16]) {
        if self.matarray == false || self.bytes.len() == 0 { return }
        let mut idx = if self.worldmatrix { 64 } else { 0 };
        self.bytes.as_mut_slice()[idx..(idx+16)].copy_from_slice(bytemuck::cast_slice(data));
        // bytemuck::cast_slice(data).iter().for_each(|v| {
        //     self.bytes[idx] = *v;
        //     idx += 1;
        // });
    }
    pub fn offset(&self, key: &Atom) -> Option<&InstanceAttributeOffset> {
        match self.attributes.binary_search_by(|v| v.0.cmp(key) ) {
            Ok(idx) => Some(&self.attributes.get(idx).unwrap().1),
            Err(_) => None,
        }
    }
    pub fn attributes(&self) -> &Vec<(Atom, InstanceAttributeOffset)> {
        &self.attributes
    }
    pub fn clone(
        &self, 
    ) -> Self {
        let bytes = self.bytes.clone();
        let mut attributes = vec![];

        self.attributes.iter().for_each(|(key, offset)| {
            attributes.push((key.clone(), InstanceAttributeOffset::new(offset.vtype, offset.offset, None)));
        });

        Self { bytes, attributes, worldmatrix: self.worldmatrix, matarray: self.matarray }
    }
    pub fn animator(
        &mut self,
        key: &Atom,
        item: Entity,
        command: &mut Commands,
        animatorablefloat: &mut ActionListAnimatorableFloat,
        animatorablevec2s: &mut ActionListAnimatorableVec2,
        animatorablevec3s: &mut ActionListAnimatorableVec3,
        animatorablevec4s: &mut ActionListAnimatorableVec4,
        animatorableuints: &mut ActionListAnimatorableUint,
        animatorablesints: &mut ActionListAnimatorableSint,
    ) -> Option<InstanceAttributeOffset> {
        let linked = item;
        match self.attributes.binary_search_by(|v| v.0.cmp(key) ) {
            Ok(idx) => {
                let offset = &mut self.attributes.get_mut(idx).unwrap().1;
                if offset.entity.is_none() {
                    let entity = command.spawn_empty_id();
                    offset.entity = Some(entity);
                    if let Some(atype) = offset.atype() {
                        match atype {
                            EAnimatorableType::Vec4     => {
                                let start = offset.offset as usize; let end = offset.offset as usize + 16;
                                let data = bytemuck::cast_slice(&self.bytes[start..end]);
                                animatorablevec4s.push(OpsAnimatorableVec4::ops(entity, linked, AnimatorableVec4::from(data), EAnimatorableEntityType::Attribute));
                            },
                            EAnimatorableType::Vec3     => {
                                let start = offset.offset as usize; let end = offset.offset as usize + 12;
                                let data = bytemuck::cast_slice(&self.bytes[start..end]);
                                animatorablevec3s.push(OpsAnimatorableVec3::ops(entity, linked, AnimatorableVec3::from(data), EAnimatorableEntityType::Attribute));
                            },
                            EAnimatorableType::Vec2     => {
                                let start = offset.offset as usize; let end = offset.offset as usize + 8;
                                let data = bytemuck::cast_slice(&self.bytes[start..end]);
                                animatorablevec2s.push(OpsAnimatorableVec2::ops(entity, linked, AnimatorableVec2::from(data), EAnimatorableEntityType::Attribute));
                            },
                            EAnimatorableType::Float    => {
                                let start = offset.offset as usize; let end = offset.offset as usize + 4;
                                let data = bytemuck::cast_slice(&self.bytes[start..end]);
                                animatorablefloat.push(OpsAnimatorableFloat::ops(entity, linked, AnimatorableFloat(data[0]), EAnimatorableEntityType::Attribute));
                            },
                            EAnimatorableType::Uint     => {
                                let start = offset.offset as usize; let end = offset.offset as usize + 4;
                                let data = bytemuck::cast_slice(&self.bytes[start..end]);
                                animatorableuints.push(OpsAnimatorableUint::ops(entity, linked, AnimatorableUint(data[0]), EAnimatorableEntityType::Attribute));
                            },
                            EAnimatorableType::Int      => {
                                let start = offset.offset as usize; let end = offset.offset as usize + 4;
                                let data = bytemuck::cast_slice(&self.bytes[start..end]);
                                animatorablesints.push(OpsAnimatorableSint::ops(entity, linked, AnimatorableSint(data[0]), EAnimatorableEntityType::Attribute));
                            },
                        }
                    }
                }
                Some(offset.clone())
            },
            Err(_) => { None },
        }
    }
}

