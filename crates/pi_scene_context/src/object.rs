pub use pi_engine_shell::prelude::*;

use crate::{
    materials::prelude::*,
    geometry::prelude::*,
    renderers::prelude::*,
    skeleton::prelude::*,
    pass::*,
};

pub struct OpsDispose(Entity);
impl OpsDispose {
    pub fn ops(entity: Entity) -> OpsDispose {
        OpsDispose(entity)
    }
}
pub fn ops_dispose(entity: Entity) -> OpsDispose {
    OpsDispose(entity)
}
pub type ActionListDispose = ActionList<OpsDispose>;
pub(crate) fn sys_dispose(
    mut cmds: ResMut<ActionListDispose>,
    mut commands: Commands,
    instancemeshes: Query<&InstanceSourceID>,
    mut instancesources: Query<&mut InstanceSourceRefs>,
    meshes: Query<(&PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08, &GeometryID, &SkeletonID)>,
    mut geometries: Query<&mut GeometryRefs>,
    mut skeletons: Query<(&mut SkeletonRefs, &Skeleton)>,
    passes: Query<&MaterialID>,
    mut materials: Query<&mut MaterialRefs>,
    mut viewers: Query<(&ViewerRenderersInfo)>,
    scenes: Query<&SceneID>,
    groupmaps: Query<&AnimationGroups>,
    mut animeglobal: ResMut<GlobalAnimeAbout>,
    mut scenectxs: ResMut<SceneAnimationContextMap>,
    mut tree: EntityTreeMut,
) {
    cmds.drain().drain(..).for_each(|OpsDispose(entity)| {
        if let (Ok(scene), Ok(groupmap)) = (scenes.get(entity), groupmaps.get(entity)) {
            groupmap.map.iter().for_each(|(k, id_group)| {
                scenectxs.delete_group(&scene.0, *id_group);
                animeglobal.remove(id_group);
            });
        }

        if let Ok(idsource) = instancemeshes.get(entity) {
            if let Ok(mut refs) = instancesources.get_mut(idsource.0) {
                refs.remove(&entity);
            }
        } else if let (
            Ok(
                (
                    pass01, pass02, pass03, pass04, pass05, pass06, pass07, pass08, idgeo, idskin
                )
            ),
            Ok(refs)
        ) = (meshes.get(entity), instancesources.get(entity)) {
            sys_pass_dispose(&mut commands, pass01.id(), &passes, &mut materials);
            sys_pass_dispose(&mut commands, pass02.id(), &passes, &mut materials);
            sys_pass_dispose(&mut commands, pass03.id(), &passes, &mut materials);
            sys_pass_dispose(&mut commands, pass04.id(), &passes, &mut materials);
            sys_pass_dispose(&mut commands, pass05.id(), &passes, &mut materials);
            sys_pass_dispose(&mut commands, pass06.id(), &passes, &mut materials);
            sys_pass_dispose(&mut commands, pass07.id(), &passes, &mut materials);
            sys_pass_dispose(&mut commands, pass08.id(), &passes, &mut materials);

            // Instance
            refs.iter().for_each(|entity| {
                if let (Ok(scene), Ok(groupmap)) = (scenes.get(*entity), groupmaps.get(*entity)) {
                    groupmap.map.iter().for_each(|(k, id_group)| {
                        scenectxs.delete_group(&scene.0, *id_group);
                        animeglobal.remove(id_group);
                    });
                }
                if let Some(mut cmd) = commands.get_entity(*entity) {
                    tree.remove(*entity);
                    cmd.despawn();
                }
            });

            if let Ok(mut refs) = geometries.get_mut(idgeo.0) {
                refs.remove(&entity);
                if refs.len() == 0 && refs.request_dispose {
                    if let Some(mut cmd) = commands.get_entity(idgeo.0) {
                        cmd.despawn();
                    }
                }
            }
            
            // skeleton
            if let Ok((mut refs, skin)) = skeletons.get_mut(idskin.0) {
                refs.remove(&entity);
                if refs.len() == 0 && refs.request_dispose {
                    // bone
                    skin.bones.iter().for_each(|entity| {
                        if let (Ok(scene), Ok(groupmap)) = (scenes.get(*entity), groupmaps.get(*entity)) {
                            groupmap.map.iter().for_each(|(k, id_group)| {
                                scenectxs.delete_group(&scene.0, *id_group);
                                animeglobal.remove(id_group);
                            });
                        }
                        if let Some(mut cmd) = commands.get_entity(*entity) {
                            tree.remove(*entity);
                            cmd.despawn();
                        }
                    });

                    if let Some(mut cmd) = commands.get_entity(idskin.0) {
                        tree.remove(idskin.0);
                        cmd.despawn();
                    }
                }
            }
        } else if let Ok(mut refs) = geometries.get_mut(entity) {
            //
            refs.request_dispose = true;
            if refs.len() == 0 && refs.request_dispose {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.despawn();
                }
            }
            return;
        } else if let Ok(mut refs) = materials.get_mut(entity) {
            // refs.
            refs.request_dispose = true;
            if refs.len() == 0 && refs.request_dispose {
                if let Some(mut cmd) = commands.get_entity(entity) {
                    cmd.despawn();
                }
            }
            return;
        } else if let Ok((mut refs, skin)) = skeletons.get_mut(entity) {
            // refs.
            refs.request_dispose = true;
            if refs.len() == 0 && refs.request_dispose {
                skin.bones.iter().for_each(|entity| {
                    if let (Ok(scene), Ok(groupmap)) = (scenes.get(*entity), groupmaps.get(*entity)) {
                        groupmap.map.iter().for_each(|(k, id_group)| {
                            scenectxs.delete_group(&scene.0, *id_group);
                            animeglobal.remove(id_group);
                        });
                    }
                    if let Some(mut cmd) = commands.get_entity(*entity) {
                        tree.remove(*entity);
                        cmd.despawn();
                    }
                });

                if let Some(mut cmd) = commands.get_entity(entity) {
                    tree.remove(entity);
                    cmd.despawn();
                }
            }
            return;
        } else if let Ok(renderers) = viewers.get_mut(entity) {
            renderers.map.iter().for_each(|(vk, (v0, v1))| {
                if let Some(mut cmd) = commands.get_entity(v1.0) {
                    cmd.despawn();
                }
            });
        } else if let Ok(scene) = scenes.get(entity) {

        }

        if let Some(mut cmd) = commands.get_entity(entity) {
            tree.remove(entity);
            cmd.despawn();
        }
    });
}
fn sys_pass_dispose(
    commands: &mut Commands,
    entity: Entity,
    passes: &Query<&MaterialID>,
    materials: &mut Query<&mut MaterialRefs>,
) {
    if let Ok(idmat) = passes.get(entity) {
        if let Ok(mut refs) = materials.get_mut(idmat.0) {
            refs.remove(&entity);
            if refs.len() == 0 && refs.request_dispose {
                if let Some(mut cmd) = commands.get_entity(idmat.0) {
                    cmd.despawn();
                }
            }
        }
    }
    if let Some(mut cmd) = commands.get_entity(entity) {
        cmd.despawn();
    }
}

pub struct OpsSceneDispose(Entity);
impl OpsSceneDispose {
    pub fn ops(entity: Entity) -> OpsSceneDispose {
        OpsSceneDispose(entity)
    }
}
pub type ActionListSceneDispose = ActionList<OpsSceneDispose>;
pub fn sys_act_scene_dispose(
    mut cmds: ResMut<ActionListSceneDispose>,
    mut cmds2: ResMut<ActionListDispose>,
    items: Query<(ObjectID, &SceneID)>,
) {
    cmds.drain().drain(..).for_each(|OpsSceneDispose(idscene)| {
        items.iter().for_each(|(entity, sceneid)| {
            if sceneid.0 == idscene {
                cmds2.push(OpsDispose::ops(entity));
            }
        });
        
        cmds2.push(OpsDispose::ops(idscene));
    });
}


pub struct PluginDispose;
impl Plugin for PluginDispose {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListSceneDispose::default());
        app.insert_resource(ActionListDispose::default());
        app.add_system(sys_act_scene_dispose.in_set(ERunStageChap::Draw));
        app.add_system(sys_dispose.after(sys_act_scene_dispose).in_set(ERunStageChap::Draw));
    }
}