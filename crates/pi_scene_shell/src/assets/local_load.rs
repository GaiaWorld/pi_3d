// use std::sync::Arc;

// use pi_hal::{
//     runtime::RENDER_RUNTIME, image::load_from_path
// };

// use crate::prelude::*;


// pub struct PluginLocalLoad;
// impl Plugin for PluginLocalLoad {
//     fn build(&self, app: &mut App) {
        
//         init_load_cb(Arc::new(|path: String| {
//             RENDER_RUNTIME
//                 .spawn(RENDER_RUNTIME.alloc(), async move {
//                     log::debug!("Load {}", path);
//                     let r = std::fs::read(path.clone()).unwrap();
//                     on_load(&path, r);
//                 })
//                 .unwrap();
//         }));
//     }
// }