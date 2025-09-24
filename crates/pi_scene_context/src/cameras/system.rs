use pi_scene_shell::prelude::*;

use crate::prelude::{AbsoluteTransform, Camera, GlobalMatrix, LocalPosition, ViewerRenderersInfo};

pub fn sys_camera_link_transform(
    mut cameras: Query<(Entity, &Camera)>,
    mut nodes: Query<(&mut GlobalMatrix, &mut AbsoluteTransform)>,
) {
    cameras.iter_mut().for_each(|(camera, mut link)| {
        if camera != link.1 {
            if let Ok((globalmatrix, absolute)) = nodes.get_mut(link.1) {
                let globalmatrix = globalmatrix.clone();
                let absolute = absolute.clone();
                if let Ok((mut cglobalmatrix, mut cabsolute)) = nodes.get_mut(camera) {
                    *cglobalmatrix = globalmatrix;
                    *cabsolute = absolute;
                }
            }
        }
    });
}

pub fn sys_dispose_about_camera(
    items: Query<(Entity, &DisposeReady, &Camera), Changed<DisposeReady>>,
    renderers: Query<&ViewerRenderersInfo>,
    mut disposecan: Query<&mut DisposeCan>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_dispose_about_camera"));
    items.iter().for_each(|(entity, stae, _)| {
        if stae.0 == false { return; }

        if let Ok(renderinfo) = renderers.get(entity) {
            // renderinfo.map.iter().for_each(|(_k, (_, id))| {
            //     disposecanlist.push(OpsDisposeCan::ops(id.0));
            // });
            renderinfo.renderers().for_each(|entity| {
                if let Ok(mut dispose) = disposecan.get_mut(*entity) { dispose.0 = true };
                // disposecanlist.push(OpsDisposeCan::ops(*idrenderer));
            });
        }
        // disposecanlist.push(OpsDisposeCan::ops(entity));
        if let Ok(mut dispose) = disposecan.get_mut(entity) { dispose.0 = true };
    });
}
