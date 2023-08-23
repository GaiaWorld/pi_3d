
use pi_engine_shell::prelude::*;

#[derive(Component, Debug, Clone)]
///
/// Mesh 与 InstancedMesh 共用组件
pub struct InstanceBoneoffset(pub u32);
impl TInstanceData for InstanceBoneoffset {
    fn vertex_kind(&self) -> EVertexDataKind {
        todo!()
    }

    fn collect(list: &Vec<&Self>) -> Vec<u8> {
        let mut temp = vec![];
        list.iter().for_each(|v| {
            temp.push(v.0);
        });

        bytemuck::cast_slice(temp.as_slice()).to_vec()
    }

    // fn size() -> usize {
    //     4
    // }
    // fn bytes_size() -> usize {
    //     4 * 4
    // }

    // fn local_offset(&self) -> usize {
    //     0
    // }

}
impl Default for InstanceBoneoffset {
    fn default() -> Self {
        Self(0)
    }
}
impl pi_curves::curve::frame::FrameDataValue for InstanceBoneoffset {
    fn interpolate(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        if amount < 0.5 {
            self.clone()
        } else {
            rhs.clone()
        }
    }

    fn hermite(value1: &Self, _tangent1: &Self, value2: &Self, _tangent2: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        if amount < 0.5 {
            value1.clone()
        } else {
            value2.clone()
        }
    }

    fn append(&self, rhs: &Self, amount: pi_curves::curve::frame::KeyFrameCurveValue) -> Self {
        if amount < 0.5 {
            self.clone()
        } else {
            rhs.clone()
        }
    }
    fn size() -> usize {
        2 * 4
    }
}
impl TAssetCapacity for InstanceBoneoffset {
    const ASSET_TYPE: &'static str = "AnimeCurveInstanceBoneoffset";
    fn capacity() -> AssetCapacity {
        AssetCapacity { flag: false, min: 500 * 1024 , max: 1024 * 1024, timeout: 1 * 60 * 1000 }
    }
}
impl TAnimatableComp for InstanceBoneoffset {

}

#[derive(Debug, Clone, Component, Default)]
pub struct RecordInstanceBoneoffset(pub InstanceBoneoffset);
impl TAnimatableCompRecord<InstanceBoneoffset> for RecordInstanceBoneoffset {
    fn comp(&self) -> InstanceBoneoffset {
        self.0.clone()
    }
}

#[derive(Component)]
pub struct InstanceBufferBoneOffset {
    pub slot: usize,
    pub index: KeyVertexBuffer,
    // buffer: Handle<EVertexBufferRange>,
}
impl TInstanceBuffer for InstanceBufferBoneOffset {
    fn display_name() -> String {
        String::from("InstanceBoneoffset")
    }

    fn slot(&self) -> EVertexBufferSlot {
        EVertexBufferSlot::from_u8_unsafe(self.slot as u8)
    }

    fn id(&mut self) -> KeyVertexBuffer {
        // self.index += 1;
        // KeyVertexBuffer::from(self.id.clone() + self.index.to_string().as_str())
        // KeyVertexBuffer::from(self.index)
        self.index.clone()
    }
}


#[derive(Component)]
pub struct InstanceBoneOffsetDirty(pub bool);
impl Default for InstanceBoneOffsetDirty {
    fn default() -> Self {
        Self(false)
    }
}
impl TInstanceFlag for InstanceBoneOffsetDirty {
    fn dirty(&self) -> bool {
        self.0
    }
    fn reset(&mut self) {
        self.0 = false;
    }
}