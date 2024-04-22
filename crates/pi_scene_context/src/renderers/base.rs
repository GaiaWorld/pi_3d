
use pi_scene_shell::prelude::*;

use super::render_sort::TransparentSortParam;

// pub type DrawObj3D = DrawObj;
pub type DrawList3D = DrawList;

pub struct TmpSortDrawOpaque {
    pub idx: u16,
    pub pass: u8,
    pub distance: f32,
    pub pipeline: u64,
    pub resourcehash: (u64, u64),
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
                        match self.resourcehash.cmp(&other.resourcehash) {
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
    pub resourcehash: (u64, u64),
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
                                match self.pipeline.cmp(&other.pipeline) {
                                    std::cmp::Ordering::Less => std::cmp::Ordering::Less,
                                    std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
                                    std::cmp::Ordering::Equal => self.resourcehash.cmp(&other.resourcehash)
                                }
                            }
                        }
                    },
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet, PartialOrd, Ord)]
pub enum StageRenderer {
    Create,
    _CreateApply,
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

pub fn create_bind_group(
    key_bind_group: &KeyBindGroup ,
    device: &RenderDevice,
    asset_mgr_bindgroup_layout: &ShareAssetMgr<BindGroupLayout>,
    asset_mgr_bindgroup: &ShareAssetMgr<BindGroup>,
) -> Option<Handle<BindGroup>> {
    let key_u64 = key_bind_group.asset_u64();
    if let Some(bind_group) = asset_mgr_bindgroup.get(&key_u64) {
        Some(bind_group)
    } else {
        let key_bind_group_layout = key_bind_group.key_bind_group_layout();
        let key_layout_u64 = key_bind_group_layout.asset_u64();
        let bind_group_layout = if let Some(layout) = asset_mgr_bindgroup_layout.get(&key_layout_u64) {
            Ok(layout)
        } else {
            let layout = BindGroupLayout::new(device, &key_bind_group_layout);
            asset_mgr_bindgroup_layout.insert(key_layout_u64, layout)
        };
        if let Ok(bind_group_layout) = bind_group_layout {
            let bind_group = BindGroup::new(&device, &key_bind_group, bind_group_layout);
            if let Ok(bind_group) = asset_mgr_bindgroup.insert(key_u64, bind_group) {
                Some(bind_group)
            } else {
                None
            }
        } else {
            None
        }
    }
}
