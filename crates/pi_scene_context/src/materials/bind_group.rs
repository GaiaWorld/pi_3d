use pi_atom::Atom;
use pi_ecs::{prelude::{ResMut, Query, Res}, query::Write};
use pi_ecs_macros::setup;
use pi_hash::XHashMap;
use pi_render::rhi::{bind_group_layout::BindGroupLayout, bind_group::BindGroup, device::RenderDevice};
use pi_slotmap::{SlotMap, DefaultKey};

use crate::object::{ObjectID, GameObject};

#[derive(Debug)]
pub struct RenderBindGroup {
    pub set: u32,
    pub layout: BindGroupLayout,
    pub bind_group: Option<BindGroup>,
    pub offsets: Vec<wgpu::BufferAddress>,
}

impl RenderBindGroup {
    pub fn new(device: &RenderDevice, layout: BindGroupLayout, set: u32) -> Self {
        Self {
            set,
            layout,
            bind_group: None,
            offsets: vec![],
        }
    }
}

pub type RenderBindGroupKey = Atom;

#[derive(Debug, Default)]
pub struct RenderBindGroupPool {
    bindgroups: XHashMap<RenderBindGroupKey, RenderBindGroup>,
    layouts: XHashMap<RenderBindGroupKey, BindGroupLayout>,
    counter: usize,
}
impl RenderBindGroupPool {
    pub const SCENE_BIND_GROUP_SET:u32 = 0;
    pub const MODEL_BIND_GROUP_SET:u32 = 1;
    pub const TEXTURE_BIND_GROUP_SET:u32 = 2;
    pub fn get_counter(&self) -> usize {
        self.counter
    }
    pub fn creat(
        &mut self,
        device: &RenderDevice,
        key: RenderBindGroupKey,
        layout_entries: &[wgpu::BindGroupLayoutEntry],
        set: u32,
    ) {
        if self.bindgroups.contains_key(&key) == false {
            self.counter += 1;
            let layout = self.create_layout(device, key.clone(), layout_entries);
            self.bindgroups.insert(
                key,
                RenderBindGroup::new(
                    device,
                    layout,
                    set
                )
            );
        }
    }
    pub fn get(
        & self,
        key: &RenderBindGroupKey
    ) -> Option<&RenderBindGroup> {
        self.bindgroups.get(key)
    }
    pub fn get_mut(&mut self,
        key: &RenderBindGroupKey
    ) -> Option<&mut RenderBindGroup> {
        self.bindgroups.get_mut(key)
    }
    pub fn delete(&mut self, key: &RenderBindGroupKey) {
        self.bindgroups.remove(key);
        self.layouts.remove(key);
    }
    fn create_layout(
        &mut self,
        device: &RenderDevice,
        key: RenderBindGroupKey,
        layout_entries: &[wgpu::BindGroupLayoutEntry],
    ) -> BindGroupLayout {
        if self.layouts.contains_key(&key) == false {
            let layout = BindGroupLayout::from(device.create_bind_group_layout(
                &wgpu::BindGroupLayoutDescriptor {
                    label: Some(&key),
                    entries: layout_entries,
                }
            ));
            self.layouts.insert(key.clone(), layout);
        }

        self.layouts.get(&key).unwrap().clone()
    }
    pub fn get_layout(
        &self,
        key: &RenderBindGroupKey,
    ) -> Option<&BindGroupLayout> {
        match self.layouts.get(key) {
            Some(layout) => Some(layout),
            None => None,
        }
    }
}
