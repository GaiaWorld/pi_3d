
use pi_engine_shell::prelude::*;
use pi_scene_math::{Matrix, coordiante_system::CoordinateSytem3, vector::TToolMatrix, Vector3};

use crate::{
    geometry::{
        prelude::*,
        instance::instance_boneoffset::*
    },
    transforms::prelude::*,
    prelude::*, commands::*
};

use super::{
    model::*,
    abstract_mesh::AbstructMesh,
};

pub fn sys_calc_render_matrix(
    mut meshes: Query<
        (ObjectID, &AbstructMesh, &LocalScaling, &WorldMatrix, &WorldMatrixInv, &ScalingMode, &RenderAlignment, &ModelVelocity, &mut GlobalTransform),
        (Without<InstanceMesh>, Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>, Changed<ScalingMode>, Changed<RenderAlignment>, Changed<ModelVelocity>)>)
    >,
    mut matrixs: Query<(&mut RenderWorldMatrix, &mut RenderWorldMatrixInv)>,
) {
    // let time = pi_time::Instant::now();

    meshes.iter_mut().for_each(|(
        obj, _,
        localscaling, worldmatrix, worldmatrix_inv, scalingmode, renderalignment, velocity, mut transform
    )| {
        if let Ok((mut wm, mut wmi)) = matrixs.get_mut(obj) {

            // log::warn!("calc_render_matrix:");
            // render_wm.0.clone_from(&worldmatrix.0);
            // render_wminv.0.clone_from(&worldmatrix_inv.0);
            let pos = transform.position().clone();
            let mut scl = Vector3::new(1., 1., 1.);
            match scalingmode.0 {
                crate::prelude::EScalingMode::Hierarchy => {
                    if renderalignment.0 == ERenderAlignment::Local {
                        wm.0.clone_from(&worldmatrix.0);
                        wmi.0.clone_from(&worldmatrix_inv.0);
                        return;
                    }
                    scl.clone_from(transform.scaling());
                },
                crate::prelude::EScalingMode::Local => {
                    scl.clone_from(&localscaling.0);
                },
                crate::prelude::EScalingMode::Shape => {
                    // 1, 1, 1
                },
            }

            let mut m = Matrix::identity();
            let g_rotation = transform.rotation();
            let rotation = renderalignment.0.calc_rotation(g_rotation, velocity);
            CoordinateSytem3::matrix4_compose_rotation(&scl, &rotation, &pos, &mut m);
            if let Some(local) = renderalignment.0.calc_local(velocity, 1., 0.) {
                m = m * local;
            }

            wm.0.clone_from(&m);
            m.try_inverse_mut();
            wmi.0.clone_from(&m);
        }

    });
    
    // let time1 = pi_time::Instant::now();
    // log::debug!("SysRenderMatrixUpdate: {:?}", time1 - time);
}

pub fn sys_calc_render_matrix_instance(
    meshes: Query<&RenderAlignment>,
    mut instances: Query<
        (ObjectID, &AbstructMesh, &LocalScaling, &WorldMatrix, &WorldMatrixInv, &ScalingMode, &ModelVelocity, &mut GlobalTransform, &InstanceMesh),
        Or<(Changed<WorldMatrix>, Changed<WorldMatrixInv>, Changed<ModelVelocity>, Changed<ScalingMode>)>
    >,
    mut matrixs: Query<(&mut RenderWorldMatrix, &mut RenderWorldMatrixInv)>,
    mut inssources: Query<&mut InstanceWorldMatrixDirty>,
) {
    let time = pi_time::Instant::now();

    instances.iter_mut().for_each(|(
        obj, _,
        localscaling, worldmatrix, worldmatrix_inv, scalingmode, velocity, mut transform, id_source
    )| {
        if let (Ok((mut wm, mut wmi)), Ok(renderalignment)) = (matrixs.get_mut(obj), meshes.get(id_source.0)) {
            
            if let Ok(mut dirty) = inssources.get_mut(id_source.0) {
                *dirty = InstanceWorldMatrixDirty(true);
            }

            // let mut flag = true;

            // log::warn!("calc_render_matrix:");
            // render_wm.0.clone_from(&worldmatrix.0);
            // render_wminv.0.clone_from(&worldmatrix_inv.0);
            let pos = transform.position().clone();
            let mut scl = Vector3::new(1., 1., 1.);
            match scalingmode.0 {
                crate::prelude::EScalingMode::Hierarchy => {
                    if renderalignment.0 == ERenderAlignment::Local {
                        wm.0.clone_from(&worldmatrix.0);
                        wmi.0.clone_from(&worldmatrix_inv.0);
                        // log::warn!("Normal Alignment");
                        return;
                    }
                    scl.clone_from(transform.scaling());
                    // flag = false;
                },
                crate::prelude::EScalingMode::Local => {
                    scl.clone_from(&localscaling.0);
                },
                crate::prelude::EScalingMode::Shape => {
                    // 1, 1, 1
                },
            }

            let mut m = Matrix::identity();
            let g_rotation = transform.rotation();
            let rotation = renderalignment.0.calc_rotation(g_rotation, velocity);
            CoordinateSytem3::matrix4_compose_rotation(&scl, &rotation, &pos, &mut m);
            if let Some(local) = renderalignment.0.calc_local(velocity, 1., 0.) {
                m = m * local;
            }

            wm.0.clone_from(&m);
            m.try_inverse_mut();
            wmi.0.clone_from(&m);
        }

    });
    
    let time1 = pi_time::Instant::now();
    log::debug!("SysInstanceRenderMatrixUpdate: {:?}", time1 - time);
}

pub fn sys_render_matrix_for_uniform(
    mut meshes: Query<(&RenderWorldMatrix, &RenderWorldMatrixInv, &BindModel), Changed<RenderWorldMatrix>>,
) {
    meshes.iter_mut().for_each(|(worldmatrix, worldmatrix_inv, bind_model)| {
        // log::debug!("SysModelUniformUpdate:");

        bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(worldmatrix.0.as_slice()));
        bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(worldmatrix_inv.0.as_slice()));
    });
}

pub fn sys_velocity_for_uniform(
    mut meshes: Query<(&ModelVelocity, &BindModel), Changed<ModelVelocity>>,
) {
    meshes.iter_mut().for_each(|(velocity, bind_model)| {
        let len = (velocity.x * velocity.x + velocity.y * velocity.y + velocity.z * velocity.z).sqrt();
        bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_VELOCITY as usize, bytemuck::cast_slice(&[velocity.x, velocity.y, velocity.z, len]));
    });
}

pub fn sys_skinoffset_for_uniform(
    mut meshes: Query<
        (
            &InstanceBoneoffset, &BindModel
        ), 
        Changed<InstanceBoneoffset>
    >,
) {
    meshes.iter_mut().for_each(|(skinoffset, bind_model)| {
        // log::debug!("SysModelUniformUpdate:");
        bind_model.0.data().write_data(ShaderBindModelAboutMatrix::OFFSET_U32_A as usize, bytemuck::cast_slice(&[skinoffset.0]));
    });
}

pub fn sys_enable_about_instance(
    instances: Query<&InstanceMesh, Changed<GlobalEnable>>,
    mut meshes: Query<&mut DirtyInstanceSourceRefs>,
) {
    instances.iter().for_each(|instance| {
        if let Ok(mut flag) = meshes.get_mut(instance.0) {
            *flag = DirtyInstanceSourceRefs;
        }
    });
}

pub fn sys_dispose_about_mesh(
    items: Query<
        (
            Entity, &DisposeReady,
            &PassID01, &PassID02, &PassID03, &PassID04, &PassID05, &PassID06, &PassID07, &PassID08,
            &GeometryID, &InstanceSourceRefs, &Mesh, Option<&SkeletonID>
        ),
        Or<(Changed<DisposeReady>, Changed<InstanceSourceRefs>)>,
    >,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    // mut geometries: Query<&mut GeometryRefs>,
    mut skeletons: Query<(&mut SkeletonRefs, &Skeleton)>,
) {
    items.iter().for_each(|(
        entity, state, pass01, pass02, pass03, pass04, pass05, pass06, pass07, pass08,
        idgeo, instancerefs, _, idskin
    )| {
        if state.0 == false { return; }

        disposecanlist.push(OpsDisposeCan::ops(entity));

        instancerefs.iter().for_each(|instance| {
            disposecanlist.push(OpsDisposeCan::ops(*instance));
        });

        disposereadylist.push(OpsDisposeReady::ops(pass01.id()));
        disposereadylist.push(OpsDisposeReady::ops(pass02.id()));
        disposereadylist.push(OpsDisposeReady::ops(pass03.id()));
        disposereadylist.push(OpsDisposeReady::ops(pass04.id()));
        disposereadylist.push(OpsDisposeReady::ops(pass05.id()));
        disposereadylist.push(OpsDisposeReady::ops(pass06.id()));
        disposereadylist.push(OpsDisposeReady::ops(pass07.id()));
        disposereadylist.push(OpsDisposeReady::ops(pass08.id()));

        // // Mesh - Geometry 一对一 直接销毁
        // if let Ok(mut georefs) = geometries.get_mut(idgeo.0) {
        //     georefs.remove(&entity);
        // }
        // log::warn!("Geometry: {:?}", idgeo.0);
        disposecanlist.push(OpsDisposeCan::ops(idgeo.0));

        if let Some(idskin) = idskin {
            if let Ok((mut refs, _skin)) = skeletons.get_mut(idskin.0) {
                refs.remove(&entity);
            }
            disposereadylist.push(OpsDisposeReady::ops(idskin.0));
        }
    });
}

pub fn sys_dispose_about_pass(
    items: Query<(Entity, &DisposeReady, &MaterialID, &ModelPass), Changed<DisposeReady>>,
    mut materials: Query<&mut MaterialRefs>,
    mut disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    empty: Res<SingleEmptyEntity>,
) {
    items.iter().for_each(|(entity, state, matid, _)| {
        if state.0 == false { return; }

        disposecanlist.push(OpsDisposeCan::ops(entity));

        if let Ok(mut refs) = materials.get_mut(matid.0) {
            refs.remove(&entity);
        }
        if empty.id() != matid.0 {
            disposereadylist.push(OpsDisposeReady::ops(matid.0));
        }
    });
}

pub fn sys_dispose_about_instance(
    items: Query<(Entity, &DisposeReady, &InstanceMesh), Changed<DisposeReady>>,
    mut instancesources: Query<(&mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs)>,
    mut _disposereadylist: ResMut<ActionListDisposeReady>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    items.iter().for_each(|(entity, state, sourceid)| {
        if state.0 == false { return; }

        disposecanlist.push(OpsDisposeCan::ops(entity));

        if let Ok((mut refs, mut flag)) = instancesources.get_mut(sourceid.0) {
            // log::warn!("Remove Instance");
            refs.remove(&entity);
            *flag = DirtyInstanceSourceRefs;
        }

        // if empty.id() != sourceid.0 {
        //     disposereadylist.push(OpsDisposeReady::ops(sourceid.0));
        // }
    });
}
