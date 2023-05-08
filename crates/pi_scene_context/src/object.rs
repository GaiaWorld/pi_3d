pub use pi_engine_shell::prelude::*;

use crate::{prelude::*, materials::material::{MaterialID, MaterialRefs}, geometry::{vertex_buffer_useinfo::{GeometryID, GeometryRefs}, instance::{InstanceSourceRefs, InstanceSourceID}}, renderers::ViewerRenderersInfo, flags::SceneID, skeleton::{SkeletonRefs, SkeletonID}};

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
pub fn sys_dispose(
    mut cmds: ResMut<ActionListDispose>,
    mut commands: Commands,
    mut instancemeshes: Query<&InstanceSourceID>,
    mut instancesources: Query<&mut InstanceSourceRefs>,
    mut meshes: Query<(&PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08, &GeometryID, &SkeletonID)>,
    mut geometries: Query<&mut GeometryRefs>,
    mut skeletons: Query<&mut SkeletonRefs>,
    mut passes: Query<&MaterialID>,
    mut materials: Query<&mut MaterialRefs>,
    mut viewers: Query<(&ViewerRenderersInfo)>,
    mut scenes: Query<&SceneID>,
) {
    cmds.drain().drain(..).for_each(|OpsDispose(entity)| {
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

            refs.iter().for_each(|entity| {
                commands.entity(*entity).despawn();
            });

            if let Ok(mut refs) = geometries.get_mut(idgeo.0) {
                refs.remove(&entity);
                if refs.len() == 0 && refs.request_dispose {
                    commands.entity(idgeo.0).despawn();
                }
            }
            
            if let Ok(mut refs) = skeletons.get_mut(idskin.0) {
                refs.remove(&entity);
                if refs.len() == 0 && refs.request_dispose {
                    commands.entity(idskin.0).despawn();
                }
            }
        } else if let Ok(mut refs) = geometries.get_mut(entity) {
            //
            refs.request_dispose = true;
            if refs.len() == 0 {
                commands.entity(entity).despawn();
            }
            return;
        } else if let Ok(mut refs) = materials.get_mut(entity) {
            // refs.
            refs.request_dispose = true;
            if refs.len() == 0 {
                commands.entity(entity).despawn();
            }
            return;
        } else if let Ok(mut refs) = skeletons.get_mut(entity) {
            // refs.
            refs.request_dispose = true;
            return;
        } else if let Ok(renderers) = viewers.get_mut(entity) {
            renderers.map.iter().for_each(|(vk, (v0, v1))| {
                commands.entity(v1.0).despawn();
            });
        } else if let Ok(scene) = scenes.get(entity) {

        }

        commands.entity(entity).despawn();
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
                commands.entity(idmat.0).despawn();
            }
        }
    }
    commands.entity(entity).despawn();
}

pub struct PluginDispose;
impl Plugin for PluginDispose {
    fn build(&self, app: &mut App) {
        app.insert_resource(ActionListDispose::default());
        app.add_system(sys_dispose.in_set(ERunStageChap::Draw));
    }
}