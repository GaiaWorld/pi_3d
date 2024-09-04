
use pi_scene_shell::prelude::*;
use pi_scene_math::{Matrix, Vector3};

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

pub fn sys_calc_render_matrix_pre(
    adds: ComponentAdded<GlobalMatrix>,
    changes: ComponentChanged<GlobalMatrix>,
    mut matrixs: Query<&mut FlagRenderWorldMatrix>,
) {
    let changes = changes.iter().chain(adds.iter());
    changes.for_each(|entity| {
        if let Ok(mut flag) = matrixs.get_mut(*entity) {
            *flag = FlagRenderWorldMatrix;
        }
    });
}

pub fn sys_calc_render_matrix(
    changes: ComponentChanged<FlagRenderWorldMatrix>,
    mut meshes: Query<
        (&AbstructMesh, &LocalScaling, &GlobalMatrix, &ScalingMode, &ModelVelocity, &mut AbsoluteTransform)
    >,
    instances: Query<&InstanceMesh>,
    renderalignments: Query<&RenderAlignment>,
    pose: Query<&RenderPoseMatrix>,
    mut matrixs: Query<(&mut RenderWorldMatrix, &mut RenderWorldMatrixInv)>,
) {
    // let time = pi_time::Instant::now();
    let mut rotation = Rotation3::identity();
    let mut tempmatrix = Matrix::identity();
    let mut tempmatrix2 = Matrix::identity();
    changes.iter().for_each(|entity| {
        if let Ok((
            _,
            localscaling, transform, scalingmode, velocity, mut abstransform
        )) = meshes.get_mut(*entity) {
            let renderalignment = if let Ok(instance) = instances.get(*entity) {
                renderalignments.get(instance.0)
            } else {
                renderalignments.get(*entity)
            };
            if let Ok(renderalignment) = renderalignment {
                if let Ok((mut wm, mut wmi)) = matrixs.get_mut(*entity) {
        
                    // log::warn!("calc_render_matrix:");
                    // render_wm.0.clone_from(&worldmatrix.0);
                    // render_wminv.0.clone_from(&worldmatrix_inv.0);
    
                    _calc_render_matrix(
                        velocity, localscaling, scalingmode, renderalignment, transform,
                        &mut abstransform, &mut wm, &mut wmi, pose.get(*entity),
                        &mut rotation, &mut tempmatrix, &mut tempmatrix2
                    );
                }
            }
        }
    });
    
    // let time1 = pi_time::Instant::now();
    // log::debug!("SysRenderMatrixUpdate: {:?}", time1 - time);
}

pub fn sys_render_matrix_dirty(
    changes: ComponentChanged<RenderWorldMatrix>,
    mut instances: Query<(&InstanceMesh, &RenderWorldMatrix, &RenderWorldMatrixInv, &mut ModelInstanceAttributes)>,
    mut meshes: Query<&mut DirtyInstanceSourceRefs>,
) {
    // let time = pi_time::Instant::now();

    changes.iter().for_each(|entity| {
        if let Ok((instance, wm, _wmi, mut instanceattributes)) = instances.get_mut(*entity) {
            instanceattributes.update_worldmatrix(&wm.0);

            if let Ok(mut flag) = meshes.get_mut(instance.0) {
                *flag = DirtyInstanceSourceRefs;
            }
        }
    });
    
    // let time1 = pi_time::Instant::now();
    // log::debug!("SysInstanceRenderMatrixUpdate: {:?}", time1 - time);
}

#[inline(never)]
fn _calc_render_matrix<T>(
    velocity: &ModelVelocity,
    localscaling: &LocalScaling,
    scalingmode: &ScalingMode,
    renderalignment: &RenderAlignment,
    transform: &GlobalMatrix,
    abstransform: &mut AbsoluteTransform,
    wm: &mut RenderWorldMatrix,
    wmi: &mut RenderWorldMatrixInv,
    pose: Result<&RenderPoseMatrix, T>,
    tmprotation: &mut Rotation3,
    tmpmatrix: &mut Matrix,
    tmpmatrix2: &mut Matrix,
) {
    let pos = transform.position();
    let mut scl = Vector3::new(1., 1., 1.);
    let g_rotation;
    match scalingmode.0 {
        crate::prelude::EScalingMode::Hierarchy => {
            if renderalignment.0 == ERenderAlignment::Local {
                if let Ok(pose) = pose {
                    // let mut m = Matrix::identity();
                    // m.clone_from(&transform.matrix);
                    // m = m * pose.0;
                    // wm.0.clone_from(&m);
                    CoordinateSytem3::mul_to(&transform.matrix, &pose.0, &mut wm.0);
                    // transform.matrix.mul_to(&pose.0, &mut wm.0);

                    wmi.0.clone_from(&wm.0);
                    wmi.0.try_inverse_mut();
                    // log::warn!("Normal Alignment {:?}", (m, obj));
                    return;
                }
                // log::warn!("Normal Alignment 2 {:?}", (obj));
                wm.0.clone_from(&transform.matrix);
                wmi.0.clone_from(&transform.matrix_inv);
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

    let m0 = &mut wm.0;
    let m1 = &mut wmi.0;
    m0.fill_with_identity();
    m1.fill_with_identity();
    tmpmatrix.fill_with_identity();
    tmpmatrix2.fill_with_identity();
    if renderalignment.0.calc_rotation(g_rotation, velocity, tmprotation) {
        pi_scene_shell::prelude::matrix4_compose_rotation(&scl, &tmprotation, &pos, m0);
    } else {
        pi_scene_shell::prelude::matrix4_compose_no_rotation(&scl, &pos, m0);
    }
    if renderalignment.0.calc_local(velocity, 1., 0., tmpmatrix, tmpmatrix2, m1) {
        CoordinateSytem3::mul_to(&m0, &m1, tmpmatrix);
        // m0.mul_to(m1, tmpmatrix);
        m0.copy_from(tmpmatrix);
    }

    if let Ok(pose) = pose {
        CoordinateSytem3::mul_to(&m0, &pose.0, tmpmatrix);
        // m0.mul_to(&pose.0, tmpmatrix);
        m0.copy_from(tmpmatrix);
    }

    m1.clone_from(&m0);
    m1.try_inverse_mut();
}

pub fn sys_model_for_uniform(
    changes: ComponentChanged<RenderWorldMatrix>,
    meshes: Query<(&RenderWorldMatrix, &RenderWorldMatrixInv, &BindModel, &ModelStatic)>,
    velocitychanges: ComponentChanged<ModelVelocity>,
    velocitymeshes: Query<(&ModelVelocity, &BindModel, &ModelStatic)>,
) {
    changes.iter().for_each(|entity| {
        if let Ok((worldmatrix, worldmatrix_inv, bind_model, meshstatic)) = meshes.get(*entity) {
        // log::warn!("SysModelUniformUpdate: {:?}", worldmatrix.0.as_slice());
            if meshstatic.0 { return; }
            bind_model.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(worldmatrix.0.as_slice()));
            bind_model.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(worldmatrix_inv.0.as_slice()));
        }
    });
    velocitychanges.iter().for_each(|entity| {
        if let Ok((velocity, bind_model, meshstatic)) = velocitymeshes.get(*entity) {
            if meshstatic.0 { return; }
            let len = (velocity.x * velocity.x + velocity.y * velocity.y + velocity.z * velocity.z).sqrt();
            bind_model.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_VELOCITY as usize, bytemuck::cast_slice(&[velocity.x, velocity.y, velocity.z, len]));
        }
    });
}

pub fn sys_enable_about_instance(
    instances: Query<&InstanceMesh>,
    changes: ComponentChanged<GlobalEnable>,
    changes2: ComponentChanged<GlobalMatrix>,
    // changes3: ComponentChanged<ModelInstanceAttributes>,
    mut meshes: Query<&mut DirtyInstanceSourceRefs>,
) {
    changes.iter().for_each(|entity| {
        if let Ok(instance) = instances.get(*entity) {
            if let Ok(mut flag) = meshes.get_mut(instance.0) {
                *flag = DirtyInstanceSourceRefs;
            }
        }
    });
    changes2.iter().for_each(|entity| {
        if let Ok(instance) = instances.get(*entity) {
            if let Ok(mut flag) = meshes.get_mut(instance.0) {
                *flag = DirtyInstanceSourceRefs;
            }
        }
    });
    // changes3.iter().for_each(|entity| {
    //     if let Ok(instance) = instances.get(*entity) {
    //         if let Ok(mut flag) = meshes.get_mut(instance.0) {
    //             *flag = DirtyInstanceSourceRefs;
    //         }
    //     }
    // });
    // instances.iter().for_each(|instance| {
    //     if let Ok(mut flag) = meshes.get_mut(instance.0) {
    //         *flag = DirtyInstanceSourceRefs;
    //     }
    // });
}

pub fn sys_animator_update_instance_attribute(
    floats: Query<(Ticker<&AnimatorableFloat>, &AnimatorableAttribute)>,
    _vec2s: Query<(Ticker<&AnimatorableVec2 >, &AnimatorableAttribute)>,
    _vec3s: Query<(Ticker<&AnimatorableVec3 >, &AnimatorableAttribute)>,
    _vec4s: Query<(Ticker<&AnimatorableVec4 >, &AnimatorableAttribute)>,
    _uints: Query<(Ticker<&AnimatorableUint >, &AnimatorableAttribute)>,
    _sints: Query<(Ticker<&AnimatorableSint >, &AnimatorableAttribute)>,
    changes: ComponentChanged<TargetAnimatorableIsRunning>,
    mut items: Query<(&mut ModelInstanceAttributes, &InstanceAttributeAnimated)>,
    instances: Query<&InstanceMesh>,
    mut meshes: Query<&mut DirtyInstanceSourceRefs>,
) {
    changes.iter().for_each(|entity| {
        if let Ok((mut attributes, animators)) = items.get_mut(*entity) {
            animators.0.iter().for_each(|key| {
                if let Some(offset) = attributes.offset(key) {
                    let mut idx = offset.offset() as usize;
                    if let Some(entity) = offset.entity() {
                        match offset.atype() {
                            EAnimatorableType::Vec4 => if let Ok((data, _)) = _vec4s.get(entity) {
                                if data.is_changed() == false { return; }
                                bytemuck::cast_slice(data.0.as_slice()).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                            },
                            EAnimatorableType::Vec3 => if let Ok((data, _)) = _vec3s.get(entity) {
                                if data.is_changed() == false { return; }
                                bytemuck::cast_slice(data.0.as_slice()).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                            },
                            EAnimatorableType::Vec2 => if let Ok((data, _)) = _vec2s.get(entity) {
                                if data.is_changed() == false { return; }
                                bytemuck::cast_slice(data.0.as_slice()).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                            },
                            EAnimatorableType::Float => if let Ok((data, _)) = floats.get(entity) {
                                if data.is_changed() == false { return; }
                                bytemuck::cast_slice(&[data.0]).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                            },
                            EAnimatorableType::Uint => if let Ok((data, _)) = _uints.get(entity) {
                                if data.is_changed() == false { return; }
                                bytemuck::cast_slice(&[data.0]).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                            },
                            EAnimatorableType::Int => if let Ok((data, _)) = _sints.get(entity) {
                                if data.is_changed() == false { return; }
                                bytemuck::cast_slice(&[data.0]).iter().for_each(|v| { attributes.bytes_mut()[idx] = *v; idx += 1; })
                            },
                        }
                    }
                }
            });
            
            if let Ok(mut flag) = meshes.get_mut(*entity) {
                *flag = DirtyInstanceSourceRefs;
            } else if let Ok(instance) = instances.get(*entity) {
                if let Ok(mut flag) = meshes.get_mut(instance.0) {
                    *flag = DirtyInstanceSourceRefs;
                }
            }
        }
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
    changes: ComponentChanged<DisposeReady>,
    items: Query<(Entity, &DisposeReady, &PassMaterialID, &PassModelID)>,
    mut materials: Query<&mut MaterialRefs>,
    mut disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
    empty: Res<SingleEmptyEntity>,
) {
    changes.iter().for_each(|entity| {
        if let Ok((entity, state, matid, _)) = items.get(*entity) {
            if state.0 == false { return; }
    
            disposecanlist.push(OpsDisposeCan::ops(entity));
    
            if let Ok(mut refs) = materials.get_mut(matid.0) {
                refs.remove(&entity);
            }
            if empty.id() != matid.0 {
                disposereadylist.push(OpsDisposeReadyForRef::ops(matid.0));
            }
        }
    });
}

pub fn sys_dispose_about_instance(
    changes: ComponentChanged<DisposeReady>,
    items: Query<(Entity, &DisposeReady, &InstanceMesh, &ModelInstanceAttributes)>,
    mut viewers: Query<(&mut ModelList, &mut ForceIncludeModelList)>,
    mut instancesources: Query<(&mut InstanceSourceRefs, &mut DirtyInstanceSourceRefs, &mut FlagAbstructMeshForView)>,
    mut _disposereadylist: ResMut<ActionListDisposeReadyForRef>,
    mut disposecanlist: ResMut<ActionListDisposeCan>,
) {
    changes.iter().for_each(|entity| {
        if let Ok((entity, state, sourceid, animators)) = items.get(*entity) {
            if state.0 == false { return; }

            disposecanlist.push(OpsDisposeCan::ops(entity));
            animators.attributes().iter().for_each(|v| {
                if let Some(entity) = v.1.entity() {
                    disposecanlist.push(OpsDisposeCan::ops(entity));
                }
            });

            if let Ok((mut refs, mut flag, mut flagview)) = instancesources.get_mut(sourceid.0) {
                // log::warn!("Remove Instance");
                refs.remove(&entity);
                *flag = DirtyInstanceSourceRefs;
                *flagview = FlagAbstructMeshForView;
            }

            viewers.iter_mut().for_each(|(mut list0, mut list1)| {
                list0.0.remove(&entity);
                list1.0.remove(&entity);
            });

            // if empty.id() != sourceid.0 {
            //     disposereadylist.push(OpsDisposeReady::ops(sourceid.0));
            // }
        }
    });
}
