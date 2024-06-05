
use pi_scene_shell::prelude::*;

fn _strip(val: &ECustomVertexType) -> usize {
    match val {
        ECustomVertexType::Vec4     => 4 * 4,
        ECustomVertexType::Vec3     => 3 * 4,
        ECustomVertexType::Vec2     => 2 * 4,
        ECustomVertexType::Float    => 1 * 4,
        ECustomVertexType::Uint     => 1 * 4,
        ECustomVertexType::Int      => 1 * 4,
        ECustomVertexType::UVec4    => 4 * 4,
    }
}
fn animatorable_type(val: &ECustomVertexType) -> EAnimatorableType {
    match val {
        ECustomVertexType::Vec4     => EAnimatorableType::Vec4,
        ECustomVertexType::Vec3     => EAnimatorableType::Vec3,
        ECustomVertexType::Vec2     => EAnimatorableType::Vec2,
        ECustomVertexType::Float    => EAnimatorableType::Float,
        ECustomVertexType::Uint     => EAnimatorableType::Uint,
        ECustomVertexType::Int      => EAnimatorableType::Int,
        ECustomVertexType::UVec4    => EAnimatorableType::Vec4,
    }
}

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
    pub fn vtype(&self) -> ECustomVertexType { self.vtype }
    pub fn atype(&self) -> EAnimatorableType { animatorable_type(&self.vtype) }
    pub fn offset(&self) -> u32 { self.offset }
    pub fn entity(&self) -> Option<Entity> { self.entity }
}

#[derive(Component, Default)]
pub struct ModelInstanceAttributes {
    bytes: Vec<u8>,
    attributes: Vec<(Atom, InstanceAttributeOffset)>,
    worldmatrix: bool,
}
impl ModelInstanceAttributes {
    pub fn new(
        insances: &Vec<CustomVertexAttribute>,
        worldmatrix: bool,
    ) -> Self {

        let tmp: [f32;4] = [0., 0., 0., 0.];

        let mut bytes = vec![];
        let mut attributes = vec![];
        let mut offset = 0;

        if worldmatrix {
            bytemuck::cast_slice(Matrix::identity().as_slice()).iter().for_each(|byte| { bytes.push(*byte) });
            offset += 64;
        }

        insances.iter().for_each(|attr| {
            // let entity = command.spawn_empty().id();
            let atype = animatorable_type(&attr.vtype());
            attributes.push((Atom::from(attr.var_code()), InstanceAttributeOffset::new(attr.vtype(), offset, None)));
            match atype {
                EAnimatorableType::Vec4     => {
                    // animatorablevec4s.push(OpsAnimatorableVec4::ops(entity, linked, AnimatorableVec4::from(&tmp)));
                    bytemuck::cast_slice(&tmp[0..4]).iter().for_each(|byte| { bytes.push(*byte) });
                    offset += 16;
                },
                EAnimatorableType::Vec3     => {
                    // animatorablevec3s.push(OpsAnimatorableVec3::ops(entity, linked, AnimatorableVec3::from(&[0., 0., 0.])));
                    bytemuck::cast_slice(&tmp[0..3]).iter().for_each(|byte| { bytes.push(*byte) });
                    offset += 12;
                },
                EAnimatorableType::Vec2     => {
                    // animatorablevec2s.push(OpsAnimatorableVec2::ops(entity, linked, AnimatorableVec2::from(&[0., 0.])));
                    bytemuck::cast_slice(&tmp[0..2]).iter().for_each(|byte| { bytes.push(*byte) });
                    offset += 8;
                },
                EAnimatorableType::Float    => {
                    // animatorablefloats.push(OpsAnimatorableFloat::ops(entity, linked, AnimatorableFloat(0.)));
                    bytemuck::cast_slice(&[0.]).iter().for_each(|byte| { bytes.push(*byte) });
                    offset += 4;
                },
                EAnimatorableType::Uint     => {
                    // animatorableuints.push(OpsAnimatorableUint::ops(entity, linked, AnimatorableUint(0)));
                    bytemuck::cast_slice(&[0u32]).iter().for_each(|byte| { bytes.push(*byte) });
                    offset += 4;
                },
                EAnimatorableType::Int      => {
                    // animatorablesints.push(OpsAnimatorableSint::ops(entity, linked, AnimatorableInt(0)));
                    bytemuck::cast_slice(&[0i32]).iter().for_each(|byte| { bytes.push(*byte) });
                    offset += 4;
                },
            }
        });

        attributes.sort_by(|a, b| a.0.cmp(&b.0) );

        Self {
            bytes, attributes, worldmatrix
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
    pub fn update_worldmatrix(&mut self, data: &Matrix) {
        if self.worldmatrix {
            let mut idx = 0;
            bytemuck::cast_slice(data.as_slice()).iter().for_each(|v| {
                self.bytes[idx] = *v;
                idx += 1;
            });
        }
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

        Self { bytes, attributes, worldmatrix: self.worldmatrix }
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
                    let entity = command.spawn_empty().id();
                    offset.entity = Some(entity);
                    match offset.atype() {
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
                Some(offset.clone())
            },
            Err(_) => { None },
        }
    }
}

#[derive(Component, Default)]
pub struct InstanceAttributes {
    bytes: Vec<u8>,
}
impl InstanceAttributes {
    pub fn bytes(&self) -> &Vec<u8> {
        &self.bytes
    }
    pub fn bytes_mut(&mut self) -> &mut Vec<u8> {
        &mut self.bytes
    }
    pub fn update_worldmatrix(&mut self, data: &Matrix) {
        let mut idx = 0;
        bytemuck::cast_slice(data.as_slice()).iter().for_each(|v| {
            self.bytes[idx] = *v;
            idx += 1;
        });
    }
}
