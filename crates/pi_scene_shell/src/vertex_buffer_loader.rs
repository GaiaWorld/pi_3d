// use std::{hash::Hash, marker::PhantomData};


// use pi_hash::{XHashMap, XHashSet};
// use pi_render::renderer::{vertex_buffer::KeyVertexBuffer, vertex_buffer_loader::SingleVertexBufferDataMap, vertices::EVerticesBufferUsage};

// pub struct VertexBufferLoader<D: From<EVerticesBufferUsage>> {
//     range_waits: XHashMap<KeyVertexBuffer, XHashSet<Entity>>,
//     p: PhantomData<D>,
// }
// impl<T: Clone + Hash + PartialEq + Eq, D: From<EVerticesBufferUsage>> Default for VertexBufferLoader<T, D> {
//     fn default() -> Self {
//         Self { range_waits: XHashMap::default(), p: PhantomData }
//     }
// }
// impl<T: Clone + Hash + PartialEq + Eq, D: From<EVerticesBufferUsage>> VertexBufferLoader<T, D> {
//     pub fn request(
//         &mut self,
//         id: T,
//         key: &KeyVertexBuffer,
//         data: Option<Vec<u8>>,
//         datamap: &mut SingleVertexBufferDataMap,
//     ) {
//         if let Some(data) = data {
//             datamap.add(key, data);
//         }
//         if !self.range_waits.contains_key(key) {
//             self.range_waits.insert(key.clone(), XHashMap::default());
//         }

//         let list = self.range_waits.get_mut(key).unwrap();
//         // log::info!("request >>>>>>>>>>>>>>>>>>>>> {:?}", key);
//         list.insert(id.clone(), id);
//     }
//     pub fn request_instance(
//         &mut self,
//         id: T,
//         key: &KeyVertexBuffer,
//         data: Option<Vec<u8>>,
//         datamap: &mut SingleVertexBufferDataMap,
//     ) {
//         if let Some(data) = data {
//             datamap.add_instance(key, data);
//         }
//         if !self.range_waits.contains_key(key) {
//             self.range_waits.insert(key.clone(), XHashMap::default());
//         }

//         let list = self.range_waits.get_mut(key).unwrap();
//         // log::info!("request >>>>>>>>>>>>>>>>>>>>> {:?}", key);
//         list.insert(id.clone(), id);
//     }
//     pub fn loaded(
//         &mut self,
//         key: &KeyVertexBuffer,
//         range: &EVerticesBufferUsage,
//     ) -> Vec<(T, D)> {
//         let mut result = vec![];
//         if let Some(list) = self.range_waits.get_mut(&key) {
//             // log::info!(" success  {:?}", list.len());
//             list.drain().for_each(|(_, id)| {
//                 result.push((id, D::from(range.clone())))
//             });
//         }

//         result
//     }
// }
