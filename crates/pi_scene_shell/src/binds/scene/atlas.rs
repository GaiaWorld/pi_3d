use pi_hash::XHashMap;
use crate::prelude::*;

#[derive(Default)]
pub struct AtlasConfigs {
    pub map: XHashMap<Atom, [f32;4]>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct KeyAtlasFrame {
    pub key: EKeyTexture,
    pub wrapu: EAddressMode,
    pub wrapv: EAddressMode,
    pub wrapw: EAddressMode,
}
impl KeyAtlasFrame {
    pub fn value(&self, mut sx: f32, mut sy: f32, mut ox: f32, mut oy: f32) -> [u32; 2] {
        sx = sx * (TextureAtlas::ATLAS_MAX_SIZE as f32);
        sy = sy * (TextureAtlas::ATLAS_MAX_SIZE as f32);
        ox = ox * (TextureAtlas::ATLAS_MAX_SIZE as f32);
        oy = oy * (TextureAtlas::ATLAS_MAX_SIZE as f32);
        let mu = Self::address_value(self.wrapu);
        let mv = Self::address_value(self.wrapv);
        let mw = Self::address_value(self.wrapw);
        let mm = 0;
        [
            (((mu << TextureAtlas::ATLAS_VALUE_BYTE) + sx.round() as u32) << TextureAtlas::ATLAS_BYTE) + (((mv << TextureAtlas::ATLAS_VALUE_BYTE) + sy.round() as u32) << 0),
            (((mw << TextureAtlas::ATLAS_VALUE_BYTE) + ox.round() as u32) << TextureAtlas::ATLAS_BYTE) + (((mm << TextureAtlas::ATLAS_VALUE_BYTE) + oy.round() as u32) << 0),
        ]
    }
    fn address_value(mode: EAddressMode) -> u32 {
        match mode {
            EAddressMode::ClampToEdge   => 0,
            EAddressMode::Repeat        => 1,
            EAddressMode::MirrorRepeat  => 2,
            EAddressMode::ClampToBorder => 3,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct TextureAtlasIdx(usize);
impl TextureAtlasIdx {
    pub fn zero() -> Self {
        Self(0)
    }
    pub fn val(&self) -> u32 {
        self.0 as u32
    }
}

pub struct TextureAtlas {
    pub data: BindBufferRange,
    pub maxcount: usize,
    pub idx: usize,
    pub keys: XHashMap<KeyAtlasFrame, usize>,
}
impl TextureAtlas {
    pub const ATLAS_BYTE: usize = 16;
    pub const ATLAS_VALUE_BYTE: usize = 13;
    pub const ATLAS_MODE_BYTE: usize = 2;
    pub const ATLAS_MAX_SIZE: usize = 1 << Self::ATLAS_VALUE_BYTE;
    pub const ATLAS_INFO_SIZE: usize = 4 * 2;
    pub const DEFAULT_IDX: usize = 0;
    pub fn new(device: &RenderDevice, bindbuffer: &mut BindBufferAllocator) -> Option<Self> {
        let maxcount = device.limits().max_uniform_buffer_binding_size as usize / Self::ATLAS_INFO_SIZE;
        if let Some(buffer) = bindbuffer.allocate((maxcount * Self::ATLAS_INFO_SIZE) as u32) {
            let mut data: Vec<u32> = Vec::with_capacity(maxcount * 2);
            let frame = KeyAtlasFrame {
                key: EKeyTexture::SRT(0),
                wrapu: EAddressMode::Repeat,
                wrapv: EAddressMode::Repeat,
                wrapw: EAddressMode::Repeat,
            };
            let temp = frame.value(1., 1., 0., 0.);
            for _ in 0..maxcount {
                temp.iter().for_each(|v| { data.push(*v); });
            }
            buffer.0.write_data(0, bytemuck::cast_slice(&data));
            Some(
                Self {
                    data: buffer,
                    maxcount,
                    idx: 0,
                    keys: XHashMap::default()
                }
            )
        } else {
            None
        }
    }
    pub fn get(&self, key: &KeyAtlasFrame) -> TextureAtlasIdx {
        if let Some(idx) = self.keys.get(key) {
            TextureAtlasIdx(*idx)
        } else {
            TextureAtlasIdx::zero()
        }
    }
    pub fn set(&mut self, key: &KeyAtlasFrame, sx: f32, sy: f32, ox: f32, oy: f32) -> TextureAtlasIdx {
        let idx = if let Some(idx) = self.keys.get(key) {
            *idx
        } else {
            self.idx += 1;
            self.keys.insert(key.clone(), self.idx);
            self.idx
        };

        self.data.write_data(idx * Self::ATLAS_INFO_SIZE, bytemuck::cast_slice(&key.value(sx, sy, ox, oy)));

        TextureAtlasIdx(idx)
    }
}

// pub 

pub struct BindAtlas {
    pub data: BindBufferRange,
    // pub 
}