pub use pi_scene_shell::prelude::*;

use crate::prelude::*;

pub type OpsDispose = OpsDisposeReady;
pub type ActionListDispose = ActionListDisposeReady;

#[derive(Resource, Default)]
pub struct TmpCommonVec {
    pub instancesort: Vec<TmpInstanceSort>,
    pub instances: Vec<(Entity, (f32, f32, f32))>,
}
impl TmpCommonVec {
    pub fn size(&self) -> usize {
        self.instancesort.capacity() * 12
    }
}
#[derive(Resource, Default)]
pub struct TmpSortDrawOpaqueVec {
    pub opaque_list: Vec<TmpSortDrawOpaque>,
}
impl TmpSortDrawOpaqueVec {
    pub fn size(&self) -> usize {
        self.opaque_list.capacity() * 32
    }
}
#[derive(Resource, Default)]
pub struct TmpSortDrawTransparentVec {
    pub transparent_list: Vec<TmpSortDrawTransparent>,
}
impl TmpSortDrawTransparentVec {
    pub fn size(&self) -> usize {
        self.transparent_list.capacity() * 40
    }
}
