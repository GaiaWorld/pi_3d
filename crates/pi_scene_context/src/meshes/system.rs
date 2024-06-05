
use pi_scene_shell::prelude::*;
use pi_scene_math::{Matrix, coordiante_system::CoordinateSytem3, vector::TToolMatrix, Vector3};

use crate::{
    geometry::{
        prelude::*,
        instance::types::{ModelInstanceAttributes, InstanceAttributeAnimated}
    },
    transforms::prelude::*,
    prelude::*,
};

use super::{
    model::*,
    abstract_mesh::AbstructMesh,
};

pub fn sys_calc_render_matrix(
    mut meshes: Query<
        (ObjectID, &AbstructMesh, &LocalScaling, &GlobalMatrix, &ScalingMode, &RenderAlignment, &ModelVelocity, &mut AbsoluteTransform),
        (Without<InstanceMesh>, Or<(Changed<GlobalMatrix>, Changed<ScalingMode>, Changed<RenderAlignment>, Changed<ModelVelocity>)>)
    >,
    mut matrixs: Query<(&mut RenderWorldMatrix, &mut RenderWorldMatrixInv)>,
) {
    // let time = pi_time::Instant::now();

    meshes.iter_mut().for_each(|(
        obj, _,
        localscaling, transform, scalingmode, renderalignment, velocity, mut abstransform
    )| {
        if let Ok((mut wm, mut wmi)) = matrixs.get_mut(obj) {

            // log::warn!("calc_render_matrix:");
            // render_wm.0.clone_from(&worldmatrix.0);
            // render_wminv.0.clone_from(&worldmatrix_inv.0);
            
            let pos = transform.position();
            let mut scl = Vector3::new(1., 1., 1.);
            
            let g_rotation;
            match scalingmode.0 {
                crate::prelude::EScalingMode::Hierarchy => {
                    if renderalignment.0 == ERenderAlignment::Local {
                        wm.0.clone_from(transform.matrix());
                        wmi.0.clone_from(&transform.matrix_inv);
                        return;
                    }
                    scl.clone_from(abstransform.scaling(transform.matrix()));
                    g_rotation = abstransform.rotation(transform.matrix());
                },
                crate::prelude::EScalingMode::Local => {
                    scl.clone_from(&localscaling.0);
                    g_rotation = abstransform.rotation(transform.matrix());
                },
                crate::prelude::EScalingMode::Shape => {
                    g_rotation = abstransform.rotation(transform.matrix());
                },
            }

            let mut m = Matrix::identity();
            let rotation = renderalignment.0.calc_rotation(g_rotation, velocity);
            CoordinateSytem3::matrix4_compose_rotation(&scl, &rotation, &pos, &mut m);
            if let Some(local) = renderalignment.0.calc_local(velocity, 1., 0.) {
                m = m * local;
            }

            if let Some(mi) = m.try_inverse() {
                wm.0.clone_from(&m);
                wmi.0.clone_from(&mi);
            }
        }

    });
    
    // let time1 = pi_time::Instant::now();
    // log::debug!("SysRenderMatrixUpdate: {:?}", time1 - time);
}

pub fn sys_calc_render_matrix_instance(
    meshes: Query<&RenderAlignment>,
    mut instances: Query<
        (ObjectID, &AbstructMesh, &LocalScaling, &ScalingMode, &ModelVelocity, &GlobalMatrix, &InstanceMesh, &mut AbsoluteTransform),
        Or<(Changed<GlobalMatrix>, Changed<ModelVelocity>, Changed<ScalingMode>)>
    >,
    mut matrixs: Query<(&mut RenderWorldMatrix, &mut RenderWorldMatrixInv, &mut ModelInstanceAttributes)>,
) {
    let time = pi_time::Instant::now();

    instances.iter_mut().for_each(|(
        obj, _,
        localscaling, scalingmode, velocity, transform, id_source, mut abstransform
    )| {
        // log::warn!("calc_render_matrix:");
        if let (
            Ok((mut wm, mut wmi, mut instanceattributes)),
            Ok(renderalignment)
        ) = (matrixs.get_mut(obj), meshes.get(id_source.0)) {
            // let mut flag = true;

            // render_wm.0.clone_from(&worldmatrix.0);
            // render_wminv.0.clone_from(&worldmatrix_inv.0);
            let pos = transform.position();
            let mut scl = Vector3::new(1., 1., 1.);
            let g_rotation;
            match scalingmode.0 {
                crate::prelude::EScalingMode::Hierarchy => {
                    if renderalignment.0 == ERenderAlignment::Local {
                        wm.0.clone_from(&transform.matrix);
                        wmi.0.clone_from(&transform.matrix_inv);
                        instanceattributes.update_worldmatrix(&wm.0);
                        // log::warn!("Normal Alignment");
                        return;
                    }
                    scl.clone_from(abstransform.scaling(transform.matrix()));
                    g_rotation = abstransform.rotation(transform.matrix());
                },
                crate::prelude::EScalingMode::Local => {
                    scl.clone_from(&localscaling.0);
                    g_rotation = abstransform.rotation(transform.matrix());
                },
                crate::prelude::EScalingMode::Shape => {
                    g_rotation = abstransform.rotation(transform.matrix());
                },
            }

            let mut m = Matrix::identity();
            let rotation = renderalignment.0.calc_rotation(g_rotation, velocity);
            CoordinateSytem3::matrix4_compose_rotation(&scl, &rotation, &pos, &mut m);
            if let Some(local) = renderalignment.0.calc_local(velocity, 1., 0.) {
                m = m * local;
            }

            wm.0.clone_from(&m);
            m.try_inverse_mut();
            wmi.0.clone_from(&m);

            instanceattributes.update_worldmatrix(&wm.0);
        }

    });
    
    let time1 = pi_time::Instant::now();
    // log::debug!("SysInstanceRenderMatrixUpdate: {:?}", time1 - time);
}

pub fn sys_render_matrix_for_uniform(
    mut meshes: Query<(&RenderWorldMatrix, &RenderWorldMatrixInv, &BindModel), (Changed<RenderWorldMatrix>, Without<ModelStatic>)>,
) {
    meshes.iter_mut().for_each(|(worldmatrix, worldmatrix_inv, bind_model)| {
        // log::warn!("SysModelUniformUpdate: {:?}", worldmatrix.0.as_slice());

        bind_model.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(worldmatrix.0.as_slice()));
        bind_model.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(worldmatrix_inv.0.as_slice()));
    });
}

pub fn sys_velocity_for_uniform(
    mut meshes: Query<(&ModelVelocity, &BindModel), (Changed<ModelVelocity>, Without<ModelStatic>)>,
) {
    meshes.iter_mut().for_each(|(velocity, bind_model)| {
        let len = (velocity.x * velocity.x + velocity.y * velocity.y + velocity.z * velocity.z).sqrt();
        bind_model.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_VELOCITY as usize, bytemuck::cast_slice(&[velocity.x, velocity.y, velocity.z, len]));
    });
}

pub fn sys_enable_about_instance(
    instances: Query<&InstanceMesh, Or<(Changed<GlobalEnable>, Changed<GlobalMatrix>, Changed<ModelInstanceAttributes>)>>,
    mut meshes: Query<&mut DirtyInstanceSourceRefs>,
) {
    instances.iter().for_each(|instance| {
        if let Ok(mut flag) = meshes.get_mut(instance.0) {
            *flag = DirtyInstanceSourceRefs;
        }
    });
}

pub fn sys_animator_update_instance_attribute(
    floats: Query<&AnimatorableFloat, (Changed<AnimatorableFloat>, With<AnimatorableAttribute>)>,
    _vec2s: Query<&AnimatorableVec2 , (Changed<AnimatorableVec2>, With<AnimatorableAttribute>)>,
    _vec3s: Query<&AnimatorableVec3 , (Changed<AnimatorableVec3>, With<AnimatorableAttribute>)>,
    _vec4s: Query<&AnimatorableVec4 , (Changed<AnimatorableVec4>, With<AnimatorableAttribute>)>,
    _uints: Query<&AnimatorableUint , (Changed<AnimatorableUint>, With<AnimatorableAttribute>)>,
    _sints: Query<&AnimatorableSint , (Changed<AnimatorableSint>, With<AnimatorableAttribute>)>,
    mut items: Query<(&mut ModelInstanceAttributes, &InstanceAttributeAnimated), Changed<TargetAnimatorableIsRunning>>,
) {
    items.iter_mut().for_each(|(mut attributes, animators)| {
        animators.0.iter().for_each(|key| {
            if let Some(offset) = attributes.offset(key) {
                let mut idx = offset.offset() as usize;
                if let Some(entity) = offset.entity() {
                    match offset.atype() {
                        EAnimatorableType::Vec4 => if let Ok(data) = _vec4s.get(entity) {
                            bytemuck::cast_slice(data.0.as_slice()).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                        },
                        EAnimatorableType::Vec3 => if let Ok(data) = _vec3s.get(entity) {
                            bytemuck::cast_slice(data.0.as_slice()).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                        },
                        EAnimatorableType::Vec2 => if let Ok(data) = _vec2s.get(entity) {
                            bytemuck::cast_slice(data.0.as_slice()).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                        },
                        EAnimatorableType::Float => if let Ok(data) = floats.get(entity) {
                            bytemuck::cast_slice(&[data.0]).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                        },
                        EAnimatorableType::Uint => if let Ok(data) = _uints.get(entity) {
                            bytemuck::cast_slice(&[data.0]).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                        },
                        EAnimatorableType::Int => if let Ok(data) = _sints.get(entity) {
                            bytemuck::cast_slice(&[data.0]).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                        },
                    }
                }
            }
        });
    });
}

pub fn sys_dispose_about_mesh(
    items: Query<
        (
            Entity, &DisposeReady,
            &PassIDs,
            &GeometryID, &InstanceSourceRefs, &Mesh, &SkeletonID, &ModelInstanceAttributes
        ),
        Or<(Changed<DisposeReady>, Changed<InstanceSourceRefs>)>,
    >,
    mut viewers: Query<(&mut ModelList, &mut ForceIncludeModelList)>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    // mut geometries: Query<&mut GeometryRefs>,
    mut skeletons: Query<(&mut SkeletonRefs, &Skeleton)>,
) {
    items.iter().for_each(|(
        entity, state, passids,
        idgeo, instancerefs, _, idskin, animators
    )| {
        if state.0 == false { return; }

        disposecanlist.push(OpsDisposeCan::ops(entity));
        animators.attributes().iter().for_each(|v| {
            if let Some(entity) = v.1.entity() {
                disposecanlist.push(OpsDisposeCan::ops(entity));
            }
        });

        instancerefs.iter().for_each(|instance| {
            disposereadylist.push(OpsDisposeReadyForRef::ops(*instance));
        });

        passids.0.iter().for_each(|id| {
            disposereadylist.push(OpsDisposeReadyForRef::ops(*id));
        });

        // // Mesh - Geometry 一对一 直接销毁
        // if let Ok(mut georefs) = geometries.get_mut(idgeo.0) {
        //     georefs.remove(&entity);
        // }
        // log::warn!("Geometry: {:?}", idgeo.0);
        disposecanlist.push(OpsDisposeCan::ops(idgeo.0));

        if let Some(idskin) = idskin.0 {
            if let Ok((mut refs, _skin)) = skeletons.get_mut(idskin) {
                refs.remove(&entity);
            }
            disposereadylist.push(OpsDisposeReadyForRef::ops(idskin));
        }
        viewers.iter_mut().for_each(|(mut list0, mut list1)| {
            list0.0.remove(&entity);
            list1.0.remove(&entity);
        });
    });
}

pub fn sys_dispose_about_pass(
    items: Query<(Entity, &DisposeReady, &PassMaterialID, &PassModelID), Changed<DisposeReady>>,
    mut materials: Query<&mut MaterialRefs>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
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
            disposereadylist.push(OpsDisposeReadyForRef::ops(matid.0));
        }
    });
}

pub fn sys_dispose_about_instance(
    items: Query<(Entity, &DisposeReady, &InstanceMesh, &ModelInstanceAttributes), Changed<DisposeReady>>,
    mut viewers: Query<(&mut ModelList, &mut ForceIncludeModelList)>,
    mut instancesources: Query<(&mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs)>,
    mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    items.iter().for_each(|(entity, state, sourceid, animators)| {
        if state.0 == false { return; }

        disposecanlist.push(OpsDisposeCan::ops(entity));
        animators.attributes().iter().for_each(|v| {
            if let Some(entity) = v.1.entity() {
                disposecanlist.push(OpsDisposeCan::ops(entity));
            }
        });

        if let Ok((mut refs, mut flag)) = instancesources.get_mut(sourceid.0) {
            // log::warn!("Remove Instance");
            refs.remove(&entity);
            *flag = DirtyInstanceSourceRefs;
        }

        viewers.iter_mut().for_each(|(mut list0, mut list1)| {
            list0.0.remove(&entity);
            list1.0.remove(&entity);
        });

        // if empty.id() != sourceid.0 {
        //     disposereadylist.push(OpsDisposeReady::ops(sourceid.0));
        // }
    });
}
