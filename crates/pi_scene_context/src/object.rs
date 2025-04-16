pub use pi_scene_shell::prelude::*;

use crate::prelude::*;

pub type OpsDispose = OpsDisposeReady;
pub type ActionListDispose = ActionListDisposeReady;

#[derive(Resource, Default)]
pub struct TmpCommonVec {
    alphaindexarr: Vec<(i32, usize)>,
    infoarr: Vec<(Vec<(Number, usize)>, Vec<(Entity, (Number, Number, Number))>)>,
}
impl TmpCommonVec {
    pub fn size(&self) -> usize {
        self.alphaindexarr.capacity() * 12 + self.infoarr.capacity() * 128
    }
    pub fn push(&mut self, entity: Entity, alphaindex: i32, sortparam: Number, xyz: (Number, Number, Number)) {
        let info = match self.alphaindexarr.binary_search_by(|a| a.0.cmp(&alphaindex)) {
            Ok(idx) => {
                &mut self.infoarr[self.alphaindexarr[idx].1]
            },
            Err(idx) => {
                let i = self.infoarr.len();
                self.infoarr.push((Vec::with_capacity(128), Vec::with_capacity(128)));
                self.alphaindexarr.insert(idx, (alphaindex, i));
                &mut self.infoarr[i]
            },
        };

        let idx = info.1.len();
        info.1.push((entity, xyz));
        info.0.push((sortparam, idx));
    }
    pub fn sort(&mut self) {
        self.infoarr.iter_mut().for_each(|item| {
            item.0.sort_by(|a, b| if let Some(o) = a.0.partial_cmp(&b.0) { o } else { std::cmp::Ordering::Equal });
        });
    }
    pub fn iter<F: FnMut((&Entity, &i32, &(Number, Number, Number)))>(&self, mut f: F) {
        self.alphaindexarr.iter().for_each(|(alphaidex, idx)| {
            let infos = &self.infoarr[*idx];
            infos.0.iter().for_each(|(_, i)| {
                let (entity, xyz) = &infos.1[*i];
                f((entity, alphaidex, xyz));
            });
        });
    }
    pub fn clear(&mut self) {
        self.alphaindexarr.clear();
        self.infoarr.clear();
    }
    pub fn is_empty(&self) -> bool {
        self.infoarr.is_empty()
    }
    pub fn count(&self) -> usize {
        let mut count = 0;
        self.infoarr.iter().for_each(|i| {
            count += i.0.len();
        });
        return count;
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
