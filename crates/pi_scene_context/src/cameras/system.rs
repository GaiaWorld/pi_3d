use pi_engine_shell::prelude::*;

use crate::{commands::{DisposeReady, ActionListDisposeCan, OpsDisposeCan, ActionListDisposeReady}, prelude::{CameraParam, Camera, ViewerRenderersInfo}};


pub fn sys_dispose_about_camera(
    items: Query<(Entity, &DisposeReady, &Camera), Changed<DisposeReady>>,
    renderers: Query<&ViewerRenderersInfo>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    empty: Res<SingleEmptyEntity>,
) {
    items.iter().for_each(|(entity, stae, _)| {
        if stae.0 == false { return; }

        if let Ok(renderinfo) = renderers.get(entity) {
            renderinfo.map.iter().for_each(|(k, (_, id))| {
                disposecanlist.push(OpsDisposeCan::ops(id.0));
            });
        }
        disposecanlist.push(OpsDisposeCan::ops(entity));
    });
}