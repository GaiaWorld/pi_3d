use pi_assets::asset::{Asset, Handle};
use pi_render::renderer::vertices::EVerticesBufferUsage;

pub struct D3Memory(pub Vec<u8>);
impl pi_assets::asset::Size for D3Memory {
    fn size(&self) -> usize {
        self.0.len()
    }
}
impl Asset for D3Memory {
    type Key = u64;
}

#[derive(Clone)]
pub enum D3Buffer {
    Buf(EVerticesBufferUsage),
    Byte(Handle<D3Memory>),
    Mem(Vec<u8>),
}
impl D3Buffer {
    pub fn combine(&self, other: &Self) -> Option<Self> {
        match (self.bytes(), other.bytes()) {
            (Some(byte1), Some(byte2)) => {
                let bytes = [byte1.clone(), byte2.clone()].concat();
                Some(Self::Mem(bytes))
            },
            _ => {
                None
            }
        }
    }
    pub fn bytes(&self) -> Option<&Vec<u8>> {
        match self {
            D3Buffer::Buf(_) => None,
            D3Buffer::Byte(data) => Some(&data.0),
            D3Buffer::Mem(data) => Some(data),
        }
    }
    pub fn byte_len(&self) -> usize {
        match self {
            D3Buffer::Buf(data) => {
                let range = data.active_range();
                (range.end - range.start) as usize
            },
            D3Buffer::Byte(data) => data.0.len(),
            D3Buffer::Mem(data) => data.len(),
        }
    }
}

#[derive(Clone)]
pub struct IndicesBuffer {
    pub data: D3Buffer,
    pub asu16: bool
}
impl IndicesBuffer {
    pub fn combine(&self, other: &Self, idx: u32) -> Option<Self> {
        match (self.data.bytes(), other.data.bytes()) {
            (Some(byte1), Some(byte2)) => {
                if self.asu16 {
                    if idx < u16::MAX as u32 {
                        let indices2: &[u16] = bytemuck::cast_slice(byte2);
                        let mut temp: Vec<u16> = vec![];
                        let mut max = 0;
                        indices2.iter().for_each(|i| {
                            let v = *i as u32 + idx;
                            max = u32::max(v, max);
                            if v < u16::MAX as u32 {
                                temp.push(v as u16);
                            }
                        });
                        
                        if max < u16::MAX as u32 {
                            let temp = [byte1.as_slice(), bytemuck::cast_slice(&temp)].concat();
                            Some(Self { data: D3Buffer::Mem(temp), asu16: self.asu16 })
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => {
                None
            }
        }
    }
}

#[derive(Clone)]
pub struct VerticesBuffer {
    pub data: D3Buffer,
    pub stripe: usize,
}
impl VerticesBuffer {
    pub fn combine(&self, other: &Self) -> Option<Self> {
        if self.stripe == other.stripe {
            match self.data.combine(&other.data) {
                Some(data) => Some(Self { data, stripe: self.stripe }),
                None => None,
            }
        } else {
            None
        }
    }
}

#[derive(Clone)]
pub struct InstanceBuffer {
    pub data: D3Buffer,
    pub stripe: usize,
}
impl InstanceBuffer {
    pub fn combine(&self, other: &Self) -> Option<Self> {
        if self.stripe == other.stripe {
            match self.data.combine(&other.data) {
                Some(data) => Some(Self { data, stripe: self.stripe }),
                None => None,
            }
        } else {
            None
        }
    }
}

pub struct D3Geometry {
    pub vertices: Vec<VerticesBuffer>,
    pub instance: Option<InstanceBuffer>,
    pub indices: Option<IndicesBuffer>,
}
impl D3Geometry {
    pub fn combine(&self, other: &Self) -> Option<Self> {
        let vlen1 = self.vertices.len();
        let vlen2 = other.vertices.len();

        if vlen1 != vlen2 || vlen1 == 0 { return None; }

        match (&self.instance, &other.instance) {
            (None, None) => {
                let mut flag = true;

                let vertex0 = self.vertices.get(0).unwrap();
                let vertexcount = vertex0.data.byte_len() / vertex0.stripe;

                let mut vertices = vec![];
                for i in 0..vlen1 {
                    if flag {
                        match (self.vertices[i]).combine(&other.vertices[i]) {
                            Some(item) => vertices.push(item),
                            None => { flag = false; },
                        }
                    }
                }

                if flag {
                    match (&self.indices, &other.indices) {
                        (None, None) => Some(Self { vertices, instance: None, indices: None }),
                        (Some(indices1), Some(indices2)) => match indices1.combine(indices2, vertexcount as u32) {
                            Some(indices) => Some(Self { vertices, instance: None, indices: Some(indices) }),
                            None => None,
                        },
                        _ => { None }
                    }
                } else {
                    None
                }
            },
            (Some(ins1), Some(ins2)) => {
                let ins = ins1.combine(ins2);
                if ins.is_some() {
                    Some(Self {
                        vertices: self.vertices.clone(),
                        instance: ins,
                        indices: self.indices.clone(),
                    })
                } else {
                    None
                }
            },
            _ => None,
        }
    }
}

