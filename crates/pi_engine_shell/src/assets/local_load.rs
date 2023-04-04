use std::sync::Arc;

use pi_async::rt::AsyncRuntime;
use pi_hal::{
    {init_load_cb, on_load},
    runtime::MULTI_MEDIA_RUNTIME, image::load_from_path
};

use crate::prelude::*;


pub struct PluginLocalLoad;
impl Plugin for PluginLocalLoad {
    fn build(&self, app: &mut App) {
        
        init_load_cb(Arc::new(|path: String| {
            MULTI_MEDIA_RUNTIME
                .spawn(MULTI_MEDIA_RUNTIME.alloc(), async move {
                    log::debug!("Load {}", path);
                    let r = std::fs::read(path.clone()).unwrap();
                    on_load(&path, r);
                })
                .unwrap();
        }));
    }
}