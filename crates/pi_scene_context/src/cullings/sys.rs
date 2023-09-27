
use pi_engine_shell::prelude::*;

use crate::prelude::{InstanceMesh, WorldMatrix, DisposeReady, InstanceSourceRefs};

use super::base::{GeometryBounding, SceneBoundingPool, GeometryCullingMode};

pub fn sys_update_culling_by_worldmatrix(
    mut scenes: Query<&mut SceneBoundingPool>,
    items: Query<(Entity, &WorldMatrix, &SceneID, &DisposeReady), Or<(Changed<WorldMatrix>, Changed<DisposeReady>)>>,
    boundings: Query<(&GeometryBounding, &GeometryCullingMode)>,
    instances: Query<&InstanceMesh>,
) {
    items.iter().for_each(|(entity, worldmatrix, idscene, dispose)| {
        if let Ok(mut pool) = scenes.get_mut(idscene.0) {
            if dispose.0 == true {
                pool.remove(entity);
            } else {
                let bounding = if let Ok(instance) = instances.get(entity) {
                    if let Ok(bounding) = boundings.get(instance.0) {
                        Some(bounding)
                    } else {
                        None
                    }
                } else {
                    if let Ok(bounding) = boundings.get(entity) {
                        Some(bounding)
                    } else {
                        None
                    }
                };

                if let Some((info, mode)) = bounding {
                    pool.set(entity, info, mode, &worldmatrix);
                }
            }
        }
    });
}

pub fn sys_update_culling_by_cullinginfo(
    mut scenes: Query<&mut SceneBoundingPool>,
    items: Query<(&WorldMatrix, &DisposeReady), Changed<WorldMatrix>>,
    boundings: Query<(Entity, &SceneID, &GeometryBounding, &GeometryCullingMode, &InstanceSourceRefs), Or<(Changed<GeometryBounding>, Changed<GeometryCullingMode>)>>,
) {
    boundings.iter().for_each(|(entity, idscene, info, mode, instances)| {
        // log::error!("AAAA");
        if let Ok(mut pool) = scenes.get_mut(idscene.0) {
            if let Ok((worldmatrix, disposed)) = items.get(entity) {
                if disposed.0 == true {
                    pool.remove(entity);
                } else {
                    pool.set(entity, info, mode, &worldmatrix);
                }
            }
            instances.iter().for_each(|entity| {
                let entity = *entity;
                if let Ok((worldmatrix, disposed)) = items.get(entity) {
                    if disposed.0 == true {
                        pool.remove(entity);
                    } else {
                        pool.set(entity, info, mode, &worldmatrix);
                    }
                }
            });
        }
    });
}
