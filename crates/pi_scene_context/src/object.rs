pub use pi_scene_shell::prelude::*;

use crate::prelude::*;

pub type OpsDispose = OpsDisposeReady;
pub type ActionListDispose = ActionListDisposeReady;

type IndexOfItemsListForSameAlphaIndex = usize;
type ItemInfo = (Entity, (Number, Number, Number));
type ItemInInfoList = Vec<ItemInfo>;
type IndexOfItemInInfoList = usize;

#[derive(Resource, Default)]
pub struct TmpCommonVec {
    alphaindexarr: Vec<(i32, IndexOfItemsListForSameAlphaIndex)>,
    same_alphaindex_items: Vec<(Vec<(Number, IndexOfItemInInfoList)>, ItemInInfoList)>,
    pool0: Vec<(Vec<(Number, IndexOfItemInInfoList)>, ItemInInfoList)>,
}
impl TmpCommonVec {
    pub fn size(&self) -> usize {
        self.alphaindexarr.capacity() * 12 + self.same_alphaindex_items.capacity() * 128
    }
    pub fn push(&mut self, entity: Entity, alphaindex: i32, sortparam: Number, xyz: (Number, Number, Number)) {
        let info = match self.alphaindexarr.binary_search_by(|a| a.0.cmp(&alphaindex)) {
            Ok(idx) => {
                &mut self.same_alphaindex_items[self.alphaindexarr[idx].1]
            },
            Err(idx) => {
                let i = self.same_alphaindex_items.len();
                self.alphaindexarr.insert(idx, (alphaindex, i));
                let item = self.pool0.pop().unwrap_or((Vec::with_capacity(128), Vec::with_capacity(128)));
                self.same_alphaindex_items.push(item);
                &mut self.same_alphaindex_items[i]
            },
        };

        let idx = info.1.len();
        info.1.push((entity, xyz));
        info.0.push((sortparam, idx));
    }
    pub fn sort(&mut self) {
        self.same_alphaindex_items.iter_mut().for_each(|item| {
            item.0.sort_by(|a, b| if let Some(o) = a.0.partial_cmp(&b.0) { o } else { std::cmp::Ordering::Equal });
        });
    }
    pub fn iter<F: FnMut((&Entity, &i32, &(Number, Number, Number)))>(&self, mut f: F) {
        self.alphaindexarr.iter().for_each(|(alphaidex, idx)| {
            let infos = &self.same_alphaindex_items[*idx];
            infos.0.iter().for_each(|(_, i)| {
                let (entity, xyz) = &infos.1[*i];
                f((entity, alphaidex, xyz));
            });
        });
    }
    pub fn clear(&mut self) {
        unsafe {
            self.alphaindexarr.set_len(0);
            while let Some(mut item) = self.same_alphaindex_items.pop() {
                item.0.set_len(0);
                item.1.set_len(0);
                self.pool0.push(item);
            }
        }
    }
    pub fn is_empty(&self) -> bool {
        self.same_alphaindex_items.is_empty()
    }
    pub fn count(&self) -> usize {
        let mut count = 0;
        self.same_alphaindex_items.iter().for_each(|i| {
            count += i.0.len();
        });
        return count;
    }
}
