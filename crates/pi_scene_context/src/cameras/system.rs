use pi_engine_shell::prelude::*;

use crate::{commands::{DisposeReady, ActionListDisposeCan, OpsDisposeCan, ActionListDisposeReady}, prelude::{Camera, ViewerRenderersInfo, OpsDisposeReady}};


pub fn sys_dispose_about_camera(
    items: Query<(Entity, &DisposeReady, &Camera), Changed<DisposeReady>>,
    renderers: Query<&ViewerRenderersInfo>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    items.iter().for_each(|(entity, stae, _)| {
        if stae.0 == false { return; }

        if let Ok(renderinfo) = renderers.get(entity) {
            // renderinfo.map.iter().for_each(|(_k, (_, id))| {
            //     disposecanlist.push(OpsDisposeCan::ops(id.0));
            // });
            renderinfo.renderers().for_each(|idrenderer| {
                disposereadylist.push(OpsDisposeReady::ops(*idrenderer));
            });
        }
        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}
