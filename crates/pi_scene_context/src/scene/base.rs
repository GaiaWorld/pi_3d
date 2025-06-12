
use std::{collections::BTreeSet, sync::Arc};

use pi_scene_shell::prelude::*;

#[derive(Component, Default)]
pub struct Scene;

#[derive(Component, Deref, DerefMut)]
pub struct SceneAnimationEnable(pub bool);
impl Default for SceneAnimationEnable {
    fn default() -> Self {
        Self(true)
    }
}


#[derive(Component, Default)]
pub struct SceneLightingInfosDirty;

#[derive(Clone, Component, Default)]
pub struct SceneLightingInfos(pub Option<ShaderBindSceneLightInfos>);
impl SceneLightingInfos {
    pub fn new(allocator: &mut BindBufferAllocator, lightlimit: LightLimitInfo) -> Self {
        Self ( ShaderBindSceneLightInfos::new(allocator, lightlimit.max_direct_light_count, lightlimit.max_point_light_count, lightlimit.max_spot_light_count, lightlimit.max_hemi_light_count) )
    }
}

#[derive(Component, Default, Deref, Clone, Copy)]
pub struct SceneItemIndex(u32);
impl SceneItemIndex {
    pub fn val(&self) -> u32 {
        self.0
    }
}

/// 场景内指定内容的ID的队列分配
#[derive(Default)]
pub struct SceneItemsQueue {
    max_count: u32,
    idxs: Vec<u32>,
    idxcounter: u32,
    items: BTreeSet<Entity>,
}
impl SceneItemsQueue {
    pub fn new(max_count: u32) -> Self {
        Self { max_count, idxs: vec![], idxcounter: 0, items: BTreeSet::default() }
    }
    pub fn max_count(&self) -> u32 {
        self.max_count
    }
    pub fn items(&self) -> std::collections::btree_set::Iter<'_, pi_scene_shell::prelude::Entity> {
        self.items.iter()
    }
    pub fn add(&mut self, entity: Entity) -> SceneItemIndex {
        self.items.insert(entity);
        if let Some(id) = self.idxs.pop() {
            SceneItemIndex(id)
        } else if self.idxcounter < self.max_count {
            let id = self.idxcounter;
            self.idxcounter += 1;
            SceneItemIndex(id)
        } else {
            SceneItemIndex(u32::MAX)
        }
    }
    pub fn recycle(&mut self, id: &SceneItemIndex, entity: &Entity) {
        if self.items.remove(entity) && id.0 < self.max_count {
            self.idxs.push(id.0)
        }
    }
    pub fn count(&self) -> usize {
        self.items.len()
    }
}

#[derive(Component, Default)]
pub struct SceneDirectLightsQueue(pub SceneItemsQueue);

#[derive(Component, Default)]
pub struct SceneOtherLightsQueue {
    pub point: SceneItemsQueue,
    pub spot: SceneItemsQueue,
    pub hemi: SceneItemsQueue,
}

#[derive(Component, Default)]
pub struct SceneShadowQueue(pub SceneItemsQueue);

#[derive(Component, Default)]
pub struct SceneShadowInfosDirty;


#[derive(Component, Default)]
pub struct SceneShadowRenderTarget(pub Option<KeyRenderTarget>);

#[derive(Clone, Component, Default)]
pub struct SceneShadowInfos(pub Option<ShaderBindShadowData>, pub Option<ShareTargetView>, pub Option<BindDataTexture2D>, pub Option<BindDataSampler>);
impl SceneShadowInfos {
    pub fn new(allocator: &mut BindBufferAllocator, lightlimit: LightLimitInfo, shadowlimit: ShadowLimitInfo) -> Self {
        let data = ShaderBindShadowData::new(allocator, lightlimit.max_direct_light_count, lightlimit.max_point_light_count, lightlimit.max_spot_light_count, lightlimit.max_hemi_light_count, shadowlimit.max_count);
        Self(data, None, None, None)
    }
    pub fn binds(&self, target: &CustomRenderTarget) -> (ShaderBindShadowData, ShaderBindShadowTexture, ShaderBindShadowSampler) {
        let tex = ETextureViewUsage::from(&target.rt);
        (
            self.0.as_ref().unwrap().clone(),
            ShaderBindShadowTexture(BindDataTexture2D(tex)),
            ShaderBindShadowSampler(target.sampler.clone())
        )
    }
}