
use pi_scene_shell::prelude::*;
use pi_scene_math::{Matrix, SQuaternion, Vector3};

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
        (&RenderPoseMatrix, &AbstructMesh, &LocalScaling, &GlobalMatrix, &ScalingMode, &ModelVelocity, &mut AbsoluteTransform, &mut RenderWorldMatrix),
    >,
    instances: Query<&InstanceMesh>,
    renderalignments: Query<&RenderAlignment>,
    // mut matrixs: Query<&mut RenderWorldMatrix>,
    entitysets: Res<EntityFilterForComponentChanged>,
) {
    // let time = pi_time::Instant::now();
    let mut rotation = Rotation3::identity();
    let mut quaternion = SQuaternion::<Number>::identity();
    let mut tempmatrix = Matrix::identity();
    let mut tempmatrix2 = Matrix::identity();
    let mut tempmatrix3 = Matrix::identity();
    let mut tempvec3a = Vector3::zeros();
    let mut tempvec3b = Vector3::zeros();
    let mut entities = entitysets.pop();
    changes.iter().for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok((
            pose, _,
            localscaling, transform, scalingmode, velocity, mut abstransform, mut wm
        )) = meshes.get_mut(*entity) {
            let entity = *entity;
            let renderalignment = if let Ok(instance) = instances.get(entity) {
                renderalignments.get(instance.0)
            } else {
                renderalignments.get(entity)
            };
            if let Ok(renderalignment) = renderalignment {
                // if let Ok(mut wm) = matrixs.get_mut(entity) {
        
                    // log::warn!("calc_render_matrix:");
                    // render_wm.0.clone_from(&worldmatrix.0);
                    // render_wminv.0.clone_from(&worldmatrix_inv.0);
    
                    _calc_render_matrix(
                        velocity, localscaling, scalingmode, renderalignment, transform,
                        &mut abstransform, &mut wm, pose,
                        &mut quaternion, &mut tempmatrix, &mut tempmatrix2, &mut tempmatrix3,
                        &mut tempvec3a, &mut tempvec3b, &mut rotation
                    );
                // }
            }
        }
    });
    // meshes.iter_mut().for_each(|(
    //     entity, _,
    //     localscaling, transform, scalingmode, velocity, mut abstransform
    // )| {
    // });
    // let time1 = pi_time::Instant::now();
    // log::error!("sys_calc_render_matrix");
}

pub fn sys_instance_matidxs(
    changes: ComponentChanged<ModelMatIdxs>,
    mut instances: Query<&mut ModelInstanceAttributes>,
    mut meshes: Query<(&mut InstanceSourceRefs, &ModelMatIdxs)>,
) {
    // let time = pi_time::Instant::now();

    changes.iter().for_each(|entity| {
        if let Ok((mut flag, matidxs)) = meshes.get_mut(*entity) {
            flag.dirty = true;
            flag.iter().for_each(|entity| {
                let entity = if let Some(entity) = entity { entity } else { return; };
                if let Ok(mut instanceattributes) = instances.get_mut(*entity) {
                    instanceattributes.update_matidxs(&matidxs.0);
                }
            });
        }
    });
    
    // let time1 = pi_time::Instant::now();
    // log::debug!("SysInstanceRenderMatrixUpdate: {:?}", time1 - time);
}

pub fn sys_render_matrix_dirty(
    changes: ComponentChanged<RenderWorldMatrix>,
    mut instances: Query<(&InstanceMesh, &RenderWorldMatrix, &mut ModelInstanceAttributes)>,
    mut meshes: Query<&mut InstanceSourceRefs>,
    entitysets: Res<EntityFilterForComponentChanged>,
) {
    // let time = pi_time::Instant::now();

    let mut entities = entitysets.pop();
    let mut soureces = entitysets.pop();
    changes.iter().for_each(|entity| {
        if !entities.insert(&entity) { return; }
        if let Ok((instance, wm, mut instanceattributes)) = instances.get_mut(*entity) {
            instanceattributes.update_worldmatrix(&wm.0);
            if !soureces.insert(&instance.0) { return; }
            if let Ok(mut flag) = meshes.get_mut(instance.0) {
                flag.dirty = true;
            }
        }
    });
    entitysets.push(entities);
    entitysets.push(soureces);
    // let time1 = pi_time::Instant::now();
    // log::debug!("SysInstanceRenderMatrixUpdate: {:?}", time1 - time);
}

#[inline(always)]
fn _calc_render_matrix(
    velocity: &ModelVelocity,
    localscaling: &LocalScaling,
    scalingmode: &ScalingMode,
    renderalignment: &RenderAlignment,
    transform: &GlobalMatrix,
    abstransform: &mut AbsoluteTransform,
    wm: &mut RenderWorldMatrix,
    pose: &RenderPoseMatrix,
    tmpquaternion: &mut SQuaternion<Number>,
    tmpmatrix: &mut Matrix,
    tmpmatrix2: &mut Matrix,
    tmpmatrix3: &mut Matrix,
    tmpvec3a: &mut Vector3,
    tmpvec3b: &mut Vector3,
    tmprotation: &mut Rotation3,
) {
    let pos = tmpvec3a;
    let scl = tmpvec3b;
    let g_rotation;
    match scalingmode.0 {
        crate::prelude::EScalingMode::Hierarchy => {
            if renderalignment.0 == ERenderAlignment::Local {
                // if let Ok(pose) = pose {
                    // if pose.0.is_identity(Number::EPSILON) == false {
                    if pose.1 {
                        CoordinateSytem3::mul_to(&transform.matrix, &pose.0, &mut wm.0);
                    } else {
                        wm.0.clone_from(&transform.matrix);
                    }
                // } else {
                //     wm.0.clone_from(&transform.matrix);
                // }
                // wm.1.clone_from(&transform.matrix_inv);
                // log::error!("{:?}", &wm.0);
                return;
            }
            scl.clone_from(abstransform.scaling(transform.matrix(), pos, tmprotation));
            g_rotation = abstransform.rotation_quaternion(transform.matrix(), scl, tmprotation);
        },
        crate::prelude::EScalingMode::Local => {
            scl.clone_from(&localscaling.0);
            g_rotation = abstransform.rotation_quaternion(transform.matrix(), pos, tmprotation);
        },
        crate::prelude::EScalingMode::Shape => {
            scl.copy_from_slice(&[1., 1., 1.]);
            g_rotation = abstransform.rotation_quaternion(transform.matrix(), pos, tmprotation);
        },
    }

    transform.to_position(pos);

    let m0 = &mut wm.0;
    // let m1 = &mut wm.1;
    m0.fill_with_identity();
    // m1.fill_with_identity();
    tmpmatrix.fill_with_identity();
    tmpmatrix2.fill_with_identity();
    if renderalignment.0.calc_rotation(g_rotation, velocity, tmpquaternion) {
        pi_scene_shell::prelude::matrix4_compose_quaternion(&scl, &tmpquaternion, &pos, m0);
    } else {
        pi_scene_shell::prelude::matrix4_compose_no_rotation(&scl, &pos, m0);
    }
    if renderalignment.0.calc_local(velocity, 1., 0., tmpmatrix, tmpmatrix2, tmpmatrix3) {
        CoordinateSytem3::mul_to(&m0, &tmpmatrix3, tmpmatrix);
        // m0.mul_to(m1, tmpmatrix);
        m0.copy_from(tmpmatrix);
    }

    // if let Ok(pose) = pose {
        if pose.0.is_identity(Number::EPSILON) == false {
            CoordinateSytem3::mul_to(&m0, &pose.0, tmpmatrix);
            // m0.mul_to(&pose.0, tmpmatrix);
            m0.copy_from(tmpmatrix);
        }
    // }

    // m1.clone_from(&m0);
    // CoordinateSytem3::try_inverse_mut(m1);
}

pub fn sys_model_for_uniform(
    changes: ComponentChanged<RenderWorldMatrix>,
    meshes: Query<(&RenderWorldMatrix, &BindModel, &ModelStatic)>,
    velocitychanges: ComponentChanged<ModelVelocity>,
    velocitymeshes: Query<(&ModelVelocity, &BindModel, &ModelStatic)>,
) {
    let matrix = Matrix::identity();
    changes.iter().for_each(|entity| {
        if let Ok((worldmatrix, bind_model, meshstatic)) = meshes.get(*entity) {
        // log::warn!("SysModelUniformUpdate: {:?}", worldmatrix.0.as_slice());
            if meshstatic.0 { return; }
            bind_model.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX as usize, bytemuck::cast_slice(worldmatrix.0.as_slice()));
            bind_model.0.as_ref().unwrap().data().write_data(ShaderBindModelAboutMatrix::OFFSET_WORLD_MATRIX_INV as usize, bytemuck::cast_slice(matrix.as_slice()));
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
    mut meshes: Query<&mut InstanceSourceRefs>,
) {
    changes.iter().for_each(|entity| {
        if let Ok(instance) = instances.get(*entity) {
            if let Ok(mut flag) = meshes.get_mut(instance.0) {
                flag.dirty = true;
            }
        }
    });
    changes2.iter().for_each(|entity| {
        if let Ok(instance) = instances.get(*entity) {
            if let Ok(mut flag) = meshes.get_mut(instance.0) {
                flag.dirty = true;
            }
        }
    });
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
    mut meshes: Query<&mut InstanceSourceRefs>,
    entitysets: Res<EntityFilterForComponentChanged>,
) {
    let mut entities = entitysets.pop();
    // changes.iter().for_each(|entity| {
    //     entities.insert(*entity);
    // });
    changes.iter().for_each(|entity| {
        if !entities.insert(entity) { return; }
        if let Ok((mut attributes, animators)) = items.get_mut(*entity) {
            animators.0.iter().for_each(|key| {
                if let Some(offset) = attributes.offset(key) {
                    let mut idx = offset.offset() as usize;
                    if let Some(entity) = offset.entity() {
                        if let Some(atype) = offset.atype() {
                            match atype {
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
                }
            });
            
            if let Ok(mut flag) = meshes.get_mut(*entity) {
                flag.dirty = true;
            } else if let Ok(instance) = instances.get(*entity) {
                if let Ok(mut flag) = meshes.get_mut(instance.0) {
                    flag.dirty = true;
                }
            }
        }
    });
    entitysets.push(entities);
}

pub fn sys_dispose_about_mesh(
    items: Query<
        (
            Entity, &DisposeReady, &PassIDs, &SceneID, 
            &GeometryID, &InstanceSourceRefs, &Mesh, &SkeletonID, &ModelInstanceAttributes
        ),
        Changed<DisposeReady>,
    >,
    mut viewers: Query<(&mut ModelList, &mut ForceIncludeModelList)>,
    mut scenes: Query<(&mut SceneColliderPool, &mut SceneBoundingPool)>,
    mut skeletons: Query<(&mut SkeletonRefs, &Skeleton)>,
    mut materials: Query<&mut MaterialRefs>,
    passes: Query<&PassMaterialID>,
    empty: Res<SingleEmptyEntity>,
    mut disposecan: Query<&mut DisposeCan>,
    // mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_dispose_about_mesh"));

    items.iter().for_each(|(
        entity, state, passids, sceneid,
        idgeo, instancerefs, _, idskin, animators
    )| {
        if state.0 == false { return; }
        if let Ok(mut dispose) = disposecan.get_mut(entity) {
            dispose.0 = true;
        }
        animators.attributes().iter().for_each(|v| {
            if let Some(entity) = v.1.entity() {
                if let Ok(mut dispose) = disposecan.get_mut(entity) { dispose.0 = true; }
            }
        });
        passids.0.iter().for_each(|entity| {
            if let Ok(mut dispose) = disposecan.get_mut(*entity) { dispose.0 = true; }
            if let Ok(matid) = passes.get(*entity) {
                if let Ok(mut refs) = materials.get_mut(matid.0) {
                    refs.remove(&entity);
                }
            }
        });
        instancerefs.iter().for_each(|entity| {
            let entity = if let Some(entity) = entity { entity } else { return; };
            if let Ok(mut dispose) = disposecan.get_mut(*entity) { dispose.0 = true; }
        });

        // // Mesh - Geometry 一对一 直接销毁
        // if let Ok(mut georefs) = geometries.get_mut(idgeo.0) {
        //     georefs.remove(&entity);
        // }
        if let Ok(mut dispose) = disposecan.get_mut(idgeo.0) { dispose.0 = true; }

        if let Some(idskin) = idskin.0 {
            if let Ok((mut refs, _skin)) = skeletons.get_mut(idskin) {
                refs.remove(&entity);
            }
        }
        viewers.iter_mut().for_each(|(mut list0, mut list1)| {
            list0.0.remove(&entity);
            list1.0.remove(&entity);
        });
        
        if let Ok((mut pool1, mut pool2)) = scenes.get_mut(sceneid.0) {
            pool1.remove(entity);
            pool2.remove(entity);
        }
    });
}

pub fn sys_dispose_about_pass(
) {
}

pub fn sys_dispose_about_instance(
    changes: ComponentChanged<DisposeReady>,
    items: Query<(Entity, &SceneID, &DisposeReady, &InstanceMesh, &ModelInstanceAttributes)>,
    mut viewers: Query<(&mut ModelList, &mut ForceIncludeModelList)>,
    mut scenes: Query<(&mut SceneColliderPool, &mut SceneBoundingPool)>,
    mut instancesources: Query<(&mut InstanceSourceRefs, &mut FlagMeshNeedRecheckForView)>,
    mut disposecan: Query<&mut DisposeCan>,
    mut performance: ResMut<Performance>,
) {
    // performance.systems.push(String::from("sys_dispose_about_instance"));
    changes.iter().for_each(|entity| {
        if let Ok((entity, sceneid, state, sourceid, animators)) = items.get(*entity) {
            if state.0 == false { return; }

            if let Ok(mut dispose) = disposecan.get_mut(entity) {
                dispose.0 = true;
            }
            animators.attributes().iter().for_each(|v| {
                if let Some(entity) = v.1.entity() {
                    if let Ok(mut dispose) = disposecan.get_mut(entity) {
                        dispose.0 = true;
                    }
                }
            });

            if let Ok((mut refs, mut flagview)) = instancesources.get_mut(sourceid.0) {
                // log::warn!("Remove Instance");
                refs.remove(&entity);
                *flagview = FlagMeshNeedRecheckForView;
            }

            viewers.iter_mut().for_each(|(mut list0, mut list1)| {
                list0.0.remove(&entity);
                list1.0.remove(&entity);
            });

            if let Ok((mut pool1, mut pool2)) = scenes.get_mut(sceneid.0) {
                pool1.remove(entity);
                pool2.remove(entity);
            }
        }
    });
}
