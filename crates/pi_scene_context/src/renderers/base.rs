use std::{sync::Arc, ops::Range};

use pi_assets::asset::Handle;
use pi_engine_shell::prelude::*;
use pi_map::smallvecmap::SmallVecMap;

use crate::prelude::TransparentSortParam;

#[derive(Debug, Clone)]
pub struct BindGroups3D {
    pub scene: Arc<BindGroupScene>,
    pub model: Arc<BindGroupModel>, 
    pub textures: Option<Arc<BindGroupTextureSamplers>>,
}
impl BindGroups3D {
    pub fn create(
        scene: Arc<BindGroupScene>,
        model: Arc<BindGroupModel>, 
        textures: Option<Arc<BindGroupTextureSamplers>>,
    ) -> Self {
        Self { scene, model, textures }
    }
    pub fn key_set_blocks(&self) -> KeyShaderSetBlocks<4, EKeyShader3DSetBlock> {
        let mut key_set_blocks = [None, None, None, None];

        key_set_blocks[0] = Some(EKeyShader3DSetBlock::Scene(self.scene.key().key_set.clone()));

        key_set_blocks[1] = Some(EKeyShader3DSetBlock::Model(self.model.key().key.clone()));

        if let Some(set_2) = &self.textures {
            key_set_blocks[2] = Some(EKeyShader3DSetBlock::TextureSampler(set_2.key().key.clone()));
        }

        KeyShaderSetBlocks(key_set_blocks)
    }
    pub fn bind_group_layouts(&self) -> [Option<Handle<BindGroupLayout>>; 4] {
        let mut bind_group_layouts = [None, None, None, None];
        
        bind_group_layouts[0] = Some(self.scene.bind_group().layout());

        bind_group_layouts[1] = Some(self.model.bind_group().layout());

        if let Some(set_2) = &self.textures {
            bind_group_layouts[2] = Some(set_2.bind_group().layout());
        }

        bind_group_layouts
    }
    pub fn key_bindgroup_layouts(&self) -> [Option<u64>; 4] {
        let mut key_bindgroup_layouts = [None, None, None, None];
        
        key_bindgroup_layouts[0] = Some(*self.scene.bind_group().layout().key());
        key_bindgroup_layouts[1] = Some(*self.model.bind_group().layout().key());
        if let Some(set_2) = &self.textures {
            key_bindgroup_layouts[2] = Some(*set_2.bind_group().layout().key());
        }

        key_bindgroup_layouts
    }
    pub fn groups(&self) -> DrawBindGroups {
        let mut groups = DrawBindGroups::default();
        
        groups.insert_group(0, DrawBindGroup::GroupUsage(self.scene.bind_group().clone()));
        groups.insert_group(1, DrawBindGroup::GroupUsage(self.model.bind_group().clone()));

        if let Some(set_2) = &self.textures {
            groups.insert_group(2, DrawBindGroup::GroupUsage(set_2.bind_group().clone()));
        }

        groups
    }
}
pub type KeyPipeline3D = KeyRenderPipeline<4, EKeyShader3DSetBlock>;
pub type Pipeline3D = RenderRes<RenderPipeline>;
pub type Pipeline3DUsage = Handle<Pipeline3D>;

// pub type DrawObj3D = DrawObj;
pub type DrawList3D = DrawList;

pub enum DrawObj3D {
    Tmp(DrawObjTmp),
    Draw(Arc<DrawObj>)
}

pub struct DrawObjTmp {
    pub pipeline: Option<Handle<RenderRes<RenderPipeline>>>,
    pub bindgroups: BindGroups3D,
    ///
    /// * MAX_VERTEX_BUFFER : 可能的最大顶点Buffer数目, 本地电脑 16
    pub vertices: SmallVecMap<RenderVertices, 3>,
    pub instances: Range<u32>,
    pub vertex: Range<u32>,
    pub indices: Option<RenderIndices>,
}

pub struct TmpSortDrawOpaque {
    pub idx: u16,
    pub pass: u8,
    pub distance: f32,
    pub pipeline: u64,
}
impl PartialEq for TmpSortDrawOpaque {
    fn eq(&self, other: &Self) -> bool {
        self.pass == other.pass && self.pipeline == other.pipeline && self.distance.eq(&other.distance)
    }
}
impl Eq for TmpSortDrawOpaque {
    fn assert_receiver_is_total_eq(&self) {

    }
}
impl PartialOrd for TmpSortDrawOpaque {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TmpSortDrawOpaque {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.pass.cmp(&other.pass) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                match self.pipeline.cmp(&other.pipeline) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => {
                        match self.distance.partial_cmp(&other.distance) {
                            Some(order) => order,
                            None => std::cmp::Ordering::Equal,
                        }
                    },
                }
            },
        }
    }
}

pub struct TmpSortDrawTransparent {
    pub idx: u16,
    pub pass: u8,
    pub distance: f32,
    pub queue: TransparentSortParam,
    pub pipeline: u64,
}
impl PartialEq for TmpSortDrawTransparent {
    fn eq(&self, other: &Self) -> bool {
        self.pass == other.pass && self.pipeline == other.pipeline && self.queue == other.queue && self.distance == other.distance
    }
}
impl Eq for TmpSortDrawTransparent {
    fn assert_receiver_is_total_eq(&self) {

    }
}
impl PartialOrd for TmpSortDrawTransparent {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for TmpSortDrawTransparent {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.pass.cmp(&other.pass) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                match self.queue.cmp(&other.queue) {
                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                    std::cmp::Ordering::Equal => {
                        match other.distance.partial_cmp(&self.distance) {
                            Some(order) => order,
                            None => {
                                self.pipeline.cmp(&other.pipeline)
                            },
                        }
                    },
                }
            },
        }
    }
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageRenderer {
    RenderStateCommand,
    RendererCommand,
    PassBindGroup,
    // PassBindGroupLoaded,
    PassBindGroups,
    PassShader,
    // PassShaderLoaded,
    PassPipeline,
    // PassPipelineLoaded,
    PassDraw,
    DrawList,
}