use pi_engine_shell::prelude::*;
use pi_scene_math::*;

use crate::{
    emitter::*,
    interpolation::*,
    iparticle_system_config::{IParticleSystemConfig, IShape, TParamType},
    modifier::*,
    tools::TBurstData,
    base::*,
};

// pub fn initial() {

//     // (<any>ModelObj.prototype).animEndCB = fn(this: ModelObj) {
//     //     if (this.animOpt) {
//     //         const func = this.animOpt.endCall;
//     //         this.animOpt = undefined;
//     //         if (func) {
//     //             let maxWaitTime = 0;
//     //             this.particleSysList.forEach((ps: any) => {
//     //                 if ((<MeshParticleSystem>ps).psTool && !(<MeshParticleSystem>ps).psTool.looping && (<MeshParticleSystem>ps).psTool.maxLifetimeOverage) {
//     //                     maxWaitTime = Math.max((<MeshParticleSystem>ps).psTool.maxLifetimeOverage, maxWaitTime);
//     //                 }
//     //             });

//     //             if (maxWaitTime > 5) {
//     //                 setTimer(func, [], maxWaitTime, 1);
//     //             } else {
//     //                 func();
//     //             }
//     //         }
//     //     }
//     // };

//     (<any>BABYLON.Light.prototype)._resyncMeshes = fn() {
//         if (!this.onlyWorkInLocal) {
//             for (var mesh of this.getScene().meshes) {
//                 mesh._resyncLightSource(this);
//             }
//         }
//     };

//     BABYLON.MaterialHelper.BindLights = fn (scene, mesh: BABYLON.AbstractMesh, effect, defines, maxSimultaneousLights, rebuildInParallel) {
//         if (maxSimultaneousLights === void 0) { maxSimultaneousLights = 4; }
//         if (rebuildInParallel === void 0) { rebuildInParallel = false; }
//         var len = Math.min(mesh.lightSources.length, maxSimultaneousLights);
//         for (var i = 0; i < len; i++) {
//             var light = mesh.lightSources[i];
//             this.BindLight(light, i, scene, effect, typeof defines === "boolean" ? defines : defines["SPECULARTERM"], rebuildInParallel);
//         }
//         var len2 = Math.min(mesh.lightSourcesOnlyWorkInLocal.length + len, maxSimultaneousLights) - len;
//         for (var i = 0; i < len2; i++) {
//             var light = mesh.lightSourcesOnlyWorkInLocal[i];
//             this.BindLight(light, i + len, scene, effect, typeof defines === "boolean" ? defines : defines["SPECULARTERM"], rebuildInParallel);
//         }
//     };

//     BABYLON.GLTF2.GLTFLoader.prototype.loadNodeAsync = fn(this: any, context: string, node: any, assign: (babylonTransformNode: BABYLON.TransformNode) => void = () => { }): Promise<BABYLON.TransformNode> {
//         // PI_BEGIN jzy
//         let p = this.meshPromiseMap.get(node.index);
//         if (p) {

//             this.RecordPromise(`loadNodeAsync - jzy - ${this._fileName} - ${context}`, p);

//             p.then((mesh: any) => {
//                 assign(mesh);
//             });
//             return p;
//         }
//         // PI_END

//         const extensionPromise = this._extensionsLoadNodeAsync(context, node, assign);
//         if (extensionPromise) {

//             this.RecordPromise(`loadNodeAsync - extensionPromise - ${this._fileName} - ${context}`, extensionPromise);

//             return extensionPromise;
//         }

//         if (node._babylonTransformNode) {
//             throw new Error(`${context}: Invalid recursive node hierarchy`);
//         }

//         const promises = new Array<Promise<any>>();

//         this.logOpen(`${context} ${node.name || ""}`);

//         const loadNode = (babylonTransformNode: BABYLON.TransformNode) => {
//             BABYLON.GLTF2.GLTFLoader.AddPointerMetadata(babylonTransformNode, context);
//             (<any>BABYLON.GLTF2.GLTFLoader)._LoadTransform(node, babylonTransformNode);

//             if (node.camera != undefined) {
//                 const camera = BABYLON.GLTF2.ArrayItem.Get(`${context}/camera`, this._gltf.cameras, node.camera);
//                 promises.push(this.loadCameraAsync(`/cameras/${(<any>camera).index}`, camera, (babylonCamera) => {
//                     babylonCamera.parent = babylonTransformNode;
//                 }));
//             }

//             if (node.children) {
//                 for (const index of node.children) {
//                     const childNode = BABYLON.GLTF2.ArrayItem.Get(`${context}/children/${index}`, this._gltf.nodes, index);
//                     promises.push(this.loadNodeAsync(`/nodes/${(<any>childNode).index}`, childNode, (childBabylonMesh) => {
//                         childBabylonMesh.parent = babylonTransformNode;
//                     }));
//                 }
//             }

//             assign(babylonTransformNode);
//         };

//         if (node.mesh == undefined) {
//             const nodeName = node.name || `node${node.index}`;
//             this._babylonScene._blockEntityCollection = this._forAssetContainer;
//             node._babylonTransformNode = new BABYLON.TransformNode(nodeName, this._babylonScene);
//             this._babylonScene._blockEntityCollection = false;
//             // PI_BEGIN
//             if (node._boundingbox) {
//                 this._addBoundingBox(node);
//             }
//             // PI_END
//             loadNode(node._babylonTransformNode);

//             ////////////////////////
//             let meshParticle: MeshParticleSystem;
//             const meshParticleConfig = <IParticleSystemConfig>(<any>node).meshParticle;
//             if (meshParticleConfig) {
//                 (<any>this)._babylonScene._blockEntityCollection = this._forAssetContainer;
//                 meshParticle = new MeshParticleSystem(nodeName, (<any>this)._babylonScene);
//                 (<any>this)._babylonScene._blockEntityCollection = false;

//                 formatMeshParticle(meshParticleConfig, meshParticle);
//                 (<any>this)._particles.push(<any>meshParticle);

//                 meshParticle.parent = node._babylonTransformNode!;

//                 // Trail
//                 if (meshParticleConfig.trail) {
//                     let material = (<any>this)._gltf.materials[meshParticleConfig.trail.material];
//                     let trailPromisre = this._loadMaterialAsync("/materials/" + meshParticleConfig.trail.material, material, null, 0, fn(babylonMaterial) {
//                         meshParticle.psTool.trail.mesh.material = babylonMaterial;
//                     });
//                     promises.push(trailPromisre);
//                 }
//                 meshParticle.setSourceMesh(null);
//             }
//             ///////////////////////
//         }
//         else {
//             const mesh = BABYLON.GLTF2.ArrayItem.Get(`${context}/mesh`, this._gltf.meshes, node.mesh);
//             promises.push(this._loadMeshAsync(`/meshes/${(<any>mesh).index}`, node, mesh, loadNode));
//         }

//         this.logClose();

//         // PI_BEGIN
//         // return Promise.all(promises).then(() => {
//         //  this._forEachPrimitive(node, (babylonMesh) => {
//         //      if ((babylonMesh as Mesh).geometry && (babylonMesh as Mesh).geometry!.useBoundingInfoFromGeometry) {
//         //          // simply apply the world matrices to the bounding info - the extends are already ok
//         //          babylonMesh._updateBoundingInfo();
//         //      } else {
//         //          babylonMesh.refreshBoundingInfo(true);
//         //      }
//         //  });
//         //
//         //  return node._babylonTransformNode!;
//         // });
//         // //////////////////////////////
//         const promise =  Promise.all(promises).then(() => {
//             this._forEachPrimitive(node, (babylonMesh) => {
//                 let geometry = (babylonMesh as BABYLON.Mesh).geometry;
//                 if ((babylonMesh as BABYLON.Mesh)._boundingInfo && (babylonMesh as BABYLON.Mesh)._boundingInfo?.isImport) {
//                     babylonMesh._updateBoundingInfo();
//                 }
//                 else if (geometry && (geometry!.useBoundingInfoFromGeometry)) {
//                     // simply apply the world matrices to the bounding info - the extends are already ok
//                     babylonMesh._updateBoundingInfo();
//                 } else {
//                     babylonMesh.refreshBoundingInfo(true);
//                 }
//             });

//             return node._babylonTransformNode!;
//         });

//         this.meshPromiseMap.set(node.index, promise);

//         return promise;
//         // PI_END
//     };

//     (<any>BABYLON.GLTF2.GLTFLoader.prototype)._loadMeshAsync = fn(this: any, context: string, node: any, mesh: any, assign: (babylonTransformNode: BABYLON.TransformNode) => void): Promise<BABYLON.TransformNode> {
//         const primitives = mesh.primitives;
//         if (!primitives || !primitives.length) {
//             throw new Error(`${context}: Primitives are missing`);
//         }

//         if (primitives[0].index == undefined) {
//             BABYLON.GLTF2.ArrayItem.Assign(primitives);
//         }

//         const promises = new Array<Promise<any>>();

//         this.logOpen(`${context} ${mesh.name || ""}`);

//         const name = node.name || `node${node.index}`;

//             (<any>this)._babylonScene._blockEntityCollection = this._forAssetContainer;
//             node._babylonTransformNode = new BABYLON.TransformNode(name, (<any>this)._babylonScene);
//             (<any>this)._babylonScene._blockEntityCollection = false;
//             node._primitiveBabylonMeshes = [];
//             // let counter = 0;
//             // for (const primitive of primitives) {
//             //     let index = counter;
//             //     const meshParticleConfig = <IParticleSystemConfig>(<any>node).meshParticle;
//             //     if (meshParticleConfig) {
//             //         (<any>this)._babylonScene._blockEntityCollection = this._forAssetContainer;
//             //         const meshParticle = new MeshParticleSystem(name, (<any>this)._babylonScene);
//             //         (<any>this)._babylonScene._blockEntityCollection = false;

//             //         formatMeshParticle(meshParticleConfig, meshParticle);
//             //         (<any>this)._particles.push(<any>meshParticle);

//             //         (<any>this).geometryForMeshParticle = true;
//             //         mesh.instanceID = undefined;
//             //         let promise = this._loadMeshPrimitiveAsync(`${context}/primitives/${primitive.index}`, name, node, mesh, primitive, (babylonMesh: BABYLON.AbstractMesh) => {
//             //             meshParticle.parent = node._babylonTransformNode!;
//             //             node._primitiveBabylonMeshes![index] = babylonMesh;
//             //             babylonMesh.setEnabled(false);
//             //             //#region PI_BEGIN - SHADOW ABOUT
//             //             if ((<BABYLON.InstancedMesh>babylonMesh).sourceMesh) {
//             //                 babylonMesh = (<BABYLON.InstancedMesh>babylonMesh).sourceMesh;
//             //             }
//             //             babylonMesh.receiveShadows = !!mesh.receiveShadows;
//             //             babylonMesh.castShadows = !!mesh.castShadows || !!mesh.castShadow;
//             //             //#endregion
//             //         }).then(() => {
//             //             let babylonMesh = node._primitiveBabylonMeshes![index];
//             //             meshParticle.setSourceMesh(<any>babylonMesh);
//             //             babylonMesh.setEnabled(true);
//             //         });
//             //         promises.push(promise);

//             //         // Trail
//             //         if (meshParticleConfig.trail) {
//             //             let material = (<any>this)._gltf.materials[meshParticleConfig.trail.material];
//             //             let trailPromisre = this._loadMaterialAsync("/materials/" + meshParticleConfig.trail.material, material, null, 0, fn(babylonMaterial) {
//             //                 meshParticle.psTool.trail.mesh.material = babylonMaterial;
//             //             });
//             //             promises.push(trailPromisre);
//             //         }
//             //     }
//             //     else {
//             //         promises.push(this._loadMeshPrimitiveAsync(`${context}/primitives/${primitive.index}`, name, node, mesh, primitive, (babylonMesh: BABYLON.AbstractMesh) => {
//             //             babylonMesh.parent = node._babylonTransformNode!;
//             //             node._primitiveBabylonMeshes![index] = babylonMesh;
//             //             //#region PI_BEGIN - SHADOW ABOUT
//             //             if ((<BABYLON.InstancedMesh>babylonMesh).sourceMesh) {
//             //                 babylonMesh = (<BABYLON.InstancedMesh>babylonMesh).sourceMesh;
//             //             }
//             //             babylonMesh.receiveShadows = !!mesh.receiveShadows;
//             //             babylonMesh.castShadows = !!mesh.castShadows || !!mesh.castShadow;
//             //             //#endregion
//             //         }));
//             //     }
//             // }
//             if (node.meshParticle) {
//                 (<any>this).geometryForMeshParticle = true;
//                 (<any>mesh).instanceID = undefined;
//             }

//             if (primitives.length === 1) {
//                 let primitive = primitives[0];
//                 promises.push(this._loadMeshPrimitiveAsync(`${context}/primitives/${primitive.index}`, `${name}`, node, mesh, primitive, (babylonMesh) => {
//                     node._primitiveBabylonMeshes!.push(babylonMesh);

//                     if (node.meshParticle) {
//                         babylonMesh.setEnabled(false);
//                     } else {
//                         babylonMesh.parent = node._babylonTransformNode!;
//                     }

//                     //#region PI_BEGIN - SHADOW ABOUT
//                     if ((<BABYLON.InstancedMesh>babylonMesh).sourceMesh) {
//                         babylonMesh = (<BABYLON.InstancedMesh>babylonMesh).sourceMesh;
//                     }
//                     babylonMesh.receiveShadows = !!mesh.receiveShadows;
//                     babylonMesh.castShadows = !!mesh.castShadows || !!mesh.castShadow;
//                     //#endregion
//                 }));
//             } else {
//                 for (const primitive of primitives) {
//                     promises.push(this._loadMeshPrimitiveAsync(`${context}/primitives/${primitive.index}`, `${name}#${primitive.index}`, node, mesh, primitive, (babylonMesh) => {
//                         node._primitiveBabylonMeshes!.push(babylonMesh);

//                         if (node.meshParticle) {
//                             babylonMesh.setEnabled(false);
//                         } else {
//                             babylonMesh.parent = node._babylonTransformNode!;
//                         }

//                         //#region PI_BEGIN - SHADOW ABOUT
//                         if ((<BABYLON.InstancedMesh>babylonMesh).sourceMesh) {
//                             babylonMesh = (<BABYLON.InstancedMesh>babylonMesh).sourceMesh;
//                         }
//                         babylonMesh.receiveShadows = !!mesh.receiveShadows;
//                         babylonMesh.castShadows = !!mesh.castShadows || !!mesh.castShadow;
//                         //#endregion
//                     }));
//                 }
//             }

//         if (node.skin != undefined) {
//             const skin = BABYLON.GLTF2.ArrayItem.Get(`${context}/skin`, <any[]>(<any>this)._gltf.skins, node.skin);
//             promises.push((<any>this)._loadSkinAsync(`/skins/${skin.index}`, node, skin));
//         }

//         assign(node._babylonTransformNode);

//         this.logClose();

//         let meshParticle: MeshParticleSystem;
//         const meshParticleConfig = <IParticleSystemConfig>(<any>node).meshParticle;
//         if (meshParticleConfig) {
//             (<any>this)._babylonScene._blockEntityCollection = this._forAssetContainer;
//             meshParticle = new MeshParticleSystem(name, (<any>this)._babylonScene);
//             (<any>this)._babylonScene._blockEntityCollection = false;

//             formatMeshParticle(meshParticleConfig, meshParticle);
//             (<any>this)._particles.push(<any>meshParticle);

//             meshParticle.parent = node._babylonTransformNode!;

//             // Trail
//             if (meshParticleConfig.trail) {
//                 let material = (<any>this)._gltf.materials[meshParticleConfig.trail.material];
//                 let trailPromisre = this._loadMaterialAsync("/materials/" + meshParticleConfig.trail.material, material, null, 0, fn(babylonMaterial) {
//                     meshParticle.psTool.trail.mesh.material = babylonMaterial;
//                 });
//                 promises.push(trailPromisre);
//             }
//         }

//         return Promise.all(promises).then(() => {
//             node._babylonTransformNode.computeWorldMatrix();

//             if (meshParticle) {
//                 meshParticle.setSourceMesh(<any>node._primitiveBabylonMeshes[0]);

//                 node._primitiveBabylonMeshes[0]!.setEnabled(true);
//             }

//             return node._babylonTransformNode!;
//         });
//     };
// }
// /**
//  * 格式化粒子系统
//  * @param config josn描述
//  * @param mp 目标粒子系统
//  */
// pub fn format_mesh_particle(config: &IParticleSystemConfig, mp: &mut MeshParticleSystem) {
//     let ps = &mut mp.ps_tool;
//     ps.name = config.name.clone();

//     ps.looping = config.looping == 1;
//     ps.duration = config.duration as u64 * 1000;
//     ps.start_delay = config.start_delay as i32 * 1000;
//     ps.max_particles = config.max_particles as usize;
//     ps.prewarm = config.prewarm;
//     ps.rate_over_time = config.emission.0;
//     ps.simulation_space = config.simulation_space_is_world;
//     ps.scaling_space = config.scaling_mode;
//     ps.set_render_alignment(config.render_alignment);
//     ps.set_render_mode(config.render_mode);
//     ps.stretched_length_scale = if config.stretched_length_scale == 0.0 {
//         1.0
//     } else {
//         config.stretched_length_scale
//     };
//     ps.stretched_velocity_scale = config.stretched_velocity_scale;

//     // bursts
//     if let Some(v) = &config.emission.1 {
//         let len = v.len();
//         for i in 0..len {
//             ps.bursts.push(v[i]);
//         }
//     }


//     // limitVelocityOverLifetime
//     if config.limit_velocity_over_lifetime.is_some() {
//         parse_float_interpolation(
//             &mut ps.limit_velocity_over_lifetime_interpolation.interpolation,
//             &config.limit_velocity_over_lifetime,
//             TParamType::TParamLimitVelocityOverLifetime,
//             1.0,
//         );
//         ps.limit_velocity_over_lifetime_interpolation.dampen =
//             if let Some(limit_velocity_over_lifetime_dampen) = config.limit_velocity_over_lifetime_dampen {
//                 limit_velocity_over_lifetime_dampen
//             } else {
//                 0.
//             };
//         ps.enable_limit_velocity_over_life_time = true;
//     }

//     if let Some(custom1) = &(config.custom1) {
//         parse_float_interpolation(
//             &mut ps.custom_data_for_main_uv.u_scale,
//             &Some(custom1[0].clone()),
//             TParamType::TParamStartLifetime,
//             1.0,
//         );
//         parse_float_interpolation(
//             &mut ps.custom_data_for_main_uv.v_scale,
//             &Some(custom1[1].clone()),
//             TParamType::TParamStartLifetime,
//             1.0,
//         );
//         parse_float_interpolation(
//             &mut ps.custom_data_for_main_uv.u_offset,
//             &Some(custom1[2].clone()),
//             TParamType::TParamStartLifetime,
//             1.0,
//         );
//         parse_float_interpolation(
//             &mut ps.custom_data_for_main_uv.v_offset,
//             &Some(custom1[3].clone()),
//             TParamType::TParamStartLifetime,
//             1.0,
//         );
//         ps.enable_custom_data_for_main_uv = true;
//     }

//     //导出时 有 textureSheet 必然对应 导出材质名称 为 SHADER_PI_SHADER_PS

//     if let Some(texture_sheet) = &(config.texture_sheet) {
//         let t_sheet = &mut ps.texture_sheet_interpolation;
//         parse_float_interpolation(
//             &mut t_sheet.frame_over_time,
//             &Some(texture_sheet.frame_over_time.clone()),
//             TParamType::TParamTextureSheet,
//             1.0,
//         );
//         t_sheet.anim_mode = texture_sheet.anim_mode.clone();
//         t_sheet.custom_row = texture_sheet.custom_row;
//         t_sheet.cycles = texture_sheet.cycles;
//         t_sheet.row_mode = texture_sheet.row_mode.clone();
//         parse_float_interpolation(
//             &mut t_sheet.start_frame,
//             &Some(texture_sheet.start_frame.clone()),
//             TParamType::TParamTextureSheet,
//             1.0,
//         );
//         t_sheet.set_tiles_x(texture_sheet.tiles_x);
//         t_sheet.set_tiles_y(texture_sheet.tiles_y);
//         t_sheet.time_mode = texture_sheet.time_mode.clone();
//         ps.enable_texture_sheet = true;
//     }

//     if let Some(trail) = &(config.trail) {
//         ps.set_enable_trail(true);
//         let ps_trail = ps.trail.as_mut().unwrap();
//         ps_trail.set_mode(trail.mode);
//         ps_trail.ratio = trail.ratio;
//         parse_float_interpolation(
//             &mut ps_trail.lifetime,
//             &Some(trail.lifetime.clone()),
//             TParamType::None,
//             1.0,
//         );
//         ps_trail.ribbon_count = trail.ribbon_count;
//         ps_trail.attach_ribbons_to_transfoem = trail.attach_rtt == 1;
//         ps_trail.minimun_vertex_distance = trail.min_dist;
//         ps_trail.set_world_space(trail.world_space == 1);
//         ps_trail.die_with_particle = trail.die_with == 1;
//         ps_trail.size_affects_width = trail.size_awidth == 1;
//         ps_trail.set_texture_mode(trail.tex_mode);
//         ps_trail.size_affects_lifetime = trail.size_alifetime == 1;
//         ps_trail.inherit_particle_color = trail.inherit_color == 1;
//         parse_color4_gradient(
//             &mut ps_trail.color_over_lifetime.color4_interpolate.gradient,
//             Some(&trail.color_over_life),
//             TParamType::None,
//         );
//         parse_float_interpolation(
//             &mut ps_trail.width_over_trail,
//             &Some(trail.width_over_trail.clone()),
//             TParamType::None,
//             1.0,
//         );
//         parse_color4_gradient(
//             &mut ps_trail.color_over_trail.color4_interpolate.gradient,
//             Some(&trail.color_over_trail),
//             TParamType::None,
//         );
//     }

//     if let Some(render_pivot) = config.render_pivot {
//         ps.render_pivot = Vector3::new(render_pivot[0], render_pivot[1], render_pivot[2]);
//     }
// }

pub fn format(cmds: &mut EntityCommands, config: &IParticleSystemConfig) {
    let mut base = ParticleCalculatorBase {
        looping: config.looping == 1,
        prewarm: config.prewarm,
        delay: (config.start_delay * 1000.) as u32,
        duration: (config.duration * 1000.) as u32 ,
        maxcount: config.max_particles as usize,
        scaling_space: config.scaling_mode.mode(),
        simulation_space: config.simulation_space_is_world.mode(),
        render_alignment: config.render_alignment,
        render_mode: config.render_mode,
        stretched_length_scale: if config.stretched_length_scale == 0.0 {
            1.0
        } else {
            config.stretched_length_scale
        },
        stretched_velocity_scale: config.stretched_velocity_scale,
        pivot: Vector3::zeros(),
    };
    if let Some(render_pivot) = config.render_pivot {
        base.pivot = Vector3::new(render_pivot[0], render_pivot[1], render_pivot[2]);
    }
    cmds.insert(base);

    format_shape(cmds, Some(&config.shape));
    format_emission(cmds, config);

    // startLifetime
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.lifetime.clone()), TParamType::TParamStartLifetime, 1.0,);
    cmds.insert(ParticleCalculatorStartLifetime(interpolation));

    // startSpeed
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.start_speed.clone()), TParamType::TParamStartSpeed, 1.,);
    cmds.insert(ParticleCalculatorStartSpeed(interpolation));
    
    // startSize
    let mut interpolation = StartSize::default();
    StartSize::format(&config.start_size, &mut interpolation);
    cmds.insert(ParticleCalculatorStartSize(interpolation));

    // startRotation
    let mut interpolation = StartRotation::default();
    StartRotation::format(&config.start_rotation, &mut interpolation);
    cmds.insert(ParticleCalculatorStartRotation(interpolation));

    // startColor
    let mut interpolation = StartColor::default();
    StartColor::format(&config.start_color, &mut interpolation);
    cmds.insert(ParticleCalculatorStartColor(interpolation));

    // gravity
    let mut interpolation = FloatInterpolation::new(0.);
    parse_float_interpolation(&mut interpolation, &Some(config.gravity.clone()), TParamType::TParamGravity, 1.0, );
    cmds.insert(ParticleCalculatorGravity(interpolation));

    // VelocityOverLifetime
    if let Some(velocity_over_lifetime) = &config.velocity_over_lifetime {
        let mut interpolation = VelocityOverLifetime::default();
        TranslationInterpolate::format(velocity_over_lifetime, &mut interpolation.translation_interpolate);
        interpolation.is_local_space = if let Some(velocity_over_lifetime_is_local) = config.velocity_over_lifetime_is_local { velocity_over_lifetime_is_local != 0 } else { false };
        cmds.insert(ParticleCalculatorVelocityOverLifetime(interpolation));
    }
    // Orbit
    if let Some(orbtial_velocity) = &(config.orbtial_velocity) {
        let mut interpolation = TranslationInterpolate::default();
        TranslationInterpolate::format(orbtial_velocity, &mut interpolation);
        cmds.insert(ParticleCalculatorOrbitVelocity(interpolation));
    }
    if let Some(orbital_offset) = &config.orbital_offset {
        let mut interpolation = TranslationInterpolate::default();
        TranslationInterpolate::format(orbital_offset, &mut interpolation);
        cmds.insert(ParticleCalculatorOrbitOffset(interpolation));
    }
    if config.orbital_radial.is_some() {
        let mut interpolation = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut interpolation, &config.orbital_radial, TParamType::TParamStartSpeed, 1.0);
        cmds.insert(ParticleCalculatorOrbitRadial(interpolation));
    }
    if config.speed_modifier.is_some() {
        let mut interpolation = SpeedModifier::default();
        parse_float_interpolation(&mut interpolation.speed_modifier, &config.speed_modifier, TParamType::TParamStartSpeed, 1.0);
        cmds.insert(ParticleCalculatorSpeedModifier(interpolation));
    }
    // ForceOverLifetime
    if let Some(force_over_lifetime) = &config.force_over_lifetime {
        let mut interpolation = ForceOverLifetime::default();
        TranslationInterpolate::format(force_over_lifetime, &mut interpolation.translation_interpolate);
        interpolation.is_local_space = if let Some(force_space_is_local) = config.force_space_is_local { force_space_is_local != 0 } else { false };
        cmds.insert(ParticleCalculatorForceOverLifetime(interpolation));
    }
    // limitVelocityOverLifetime
    if config.limit_velocity_over_lifetime.is_some() {
        let mut interpolation = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut interpolation, &config.limit_velocity_over_lifetime, TParamType::TParamLimitVelocityOverLifetime, 1.0);
        let dampen = if let Some(val) = config.limit_velocity_over_lifetime_dampen { val } else { 0. };
        cmds.insert(ParticleCalculatorLimitVelocityOverLifetime(LimitVelocityOverLifetime { interpolation, dampen }));
    }

    // SizeOverLifetime
    if let Some(size_over_lifetime) = &config.size_over_lifetime {
        let mut interpolation = SizeOverLifetime::default();
        ScalingInterpolate::format(size_over_lifetime, &mut interpolation.scaling_interpolate);
        cmds.insert(ParticleCalculatorSizeOverLifetime(interpolation));
    }
    // SizeBySpeed
    if let Some(size_by_speed) = &config.size_by_speed {
        let mut interpolation = SizeBySpeed::default();
        ScalingInterpolate::format(&ParamInfo::OneParamInfo(size_by_speed.0.clone()), &mut interpolation.scaling_interpolate);
        interpolation.range_x = size_by_speed.1;
        interpolation.range_y = size_by_speed.2;
        cmds.insert(ParticleCalculatorSizeBySpeed(interpolation));
    }

    // RotationOverLifetime
    if let Some(rotation_over_lifetime) = &config.rotation_over_lifetime {
        let mut interpolation = RotationOverLifetime::default();
        RotationInterpolate::format(rotation_over_lifetime, &mut interpolation.rotation_interpolate);
        cmds.insert(ParticleCalculatorRotationOverLifetime(interpolation));
    }
    // RotationBySpeed
    if let Some(rotation_by_speed) = &config.rotation_by_speed {
        let mut interpolation = RotationBySpeed::default();
        RotationInterpolate::format(&ParamInfo::OneParamInfo(rotation_by_speed.0.clone()), &mut interpolation.rotation_interpolate);
        interpolation.range_x = rotation_by_speed.1;
        interpolation.range_y = rotation_by_speed.2;
        cmds.insert(ParticleCalculatorRotationBySpeed(interpolation));
    }
    
    // ColorOverLifetime
    if let Some(color_over_lifetime) = &config.color_over_lifetime {
        let mut interpolation = ColorOverLifetime::default();
        Color4Interpolate::format(color_over_lifetime, &mut interpolation.color4_interpolate);
        cmds.insert(ParticleCalculatorColorOverLifetime(interpolation));
    }
    // ColorBySpeed
    if let Some(color_by_speed) = &config.color_by_speed {
        let mut interpolation = ColorBySpeed::default();
        Color4Interpolate::format(&color_by_speed.0, &mut interpolation.color4_interpolate);
        interpolation.range_x = color_by_speed.1;
        interpolation.range_y = color_by_speed.2;
        cmds.insert(ParticleCalculatorColorBySpeed(interpolation));
    }

    // Texture Sheet
    if let Some(texture_sheet) = &(config.texture_sheet) {
        let mut frame_over_time = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut frame_over_time, &Some(texture_sheet.frame_over_time.clone()), TParamType::TParamTextureSheet, 1.0);
        let mut start_frame = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut start_frame, &Some(texture_sheet.start_frame.clone()), TParamType::TParamTextureSheet, 1.0);
        let mut t_sheet = TextureSheet::new(frame_over_time, start_frame);
        t_sheet.anim_mode = texture_sheet.anim_mode.clone();
        t_sheet.custom_row = texture_sheet.custom_row;
        t_sheet.cycles = texture_sheet.cycles;
        t_sheet.row_mode = texture_sheet.row_mode.clone();
        t_sheet.set_tiles_x(texture_sheet.tiles_x);
        t_sheet.set_tiles_y(texture_sheet.tiles_y);
        t_sheet.time_mode = texture_sheet.time_mode.clone();
        cmds.insert(ParticleCalculatorTextureSheet(t_sheet));
    }
    
    // Custom Data V4
    if let Some(custom1) = &(config.custom1) {
        let mut x = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut x, &Some(custom1[0].clone()), TParamType::TParamStartLifetime, 1.0);
        let mut y = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut y, &Some(custom1[1].clone()), TParamType::TParamStartLifetime, 1.0);
        let mut z = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut z, &Some(custom1[2].clone()), TParamType::TParamStartLifetime, 1.0);
        let mut w = FloatInterpolation::new(0.);
        parse_float_interpolation(&mut w, &Some(custom1[3].clone()), TParamType::TParamStartLifetime, 1.0);

        cmds.insert(ParticleCalculatorCustomV4 { x, y, z, w });
    }
    // Trail
    if let Some(trail) = &(config.trail) {
        let mut ps_trail = TrailModifier::new();
        // ps_trail.set_mode(trail.mode);
        ps_trail.mode = trail.mode;
        ps_trail.ratio = trail.ratio;
        parse_float_interpolation(
            &mut ps_trail.lifetime,
            &Some(trail.lifetime.clone()),
            TParamType::None,
            1.0,
        );
        ps_trail.ribbon_count = trail.ribbon_count;
        ps_trail.attach_ribbons_to_transfoem = trail.attach_rtt == 1;
        ps_trail.minimun_vertex_distance = trail.min_dist;
        ps_trail.use_world_space = trail.world_space == 1;
        ps_trail.die_with_particle = trail.die_with == 1;
        ps_trail.size_affects_width = trail.size_awidth == 1;
        ps_trail.texture_mode = trail.tex_mode;
        ps_trail.size_affects_lifetime = trail.size_alifetime == 1;
        ps_trail.inherit_particle_color = trail.inherit_color == 1;
        parse_color4_gradient(
            &mut ps_trail.color_over_lifetime.color4_interpolate.gradient,
            Some(&trail.color_over_life),
            TParamType::None,
        );
        parse_float_interpolation(
            &mut ps_trail.width_over_trail,
            &Some(trail.width_over_trail.clone()),
            TParamType::None,
            1.0,
        );
        parse_color4_gradient(
            &mut ps_trail.color_over_trail.color4_interpolate.gradient,
            Some(&trail.color_over_trail),
            TParamType::None,
        );
        cmds.insert(ParticleCalculatorTrail(ps_trail));
    }
}

pub fn format_shape(cmds: &mut EntityCommands, shape: Option<&IShape>) {
    let emitter = if let Some(shape) = &shape {
        let mut _pos = None;
        let mut _rotation = None;
        let mut _scale = None;
        let mut _randomize = None;
        let mut _align_dir = 0;
        let mut shape_emitter: ShapeEmitter = match shape {
            // 2
            IShape::ShapeBox(shape) => {
                let mut temp = BoxShapeEmitter::new();
                temp.emit_mode = if let Some(mode) = &shape.box_emit_mode {
                    *mode
                } else {
                    EBoxShapeMode::Volume
                };
                _pos = shape.position.clone();
                _rotation = shape.rotation.clone();
                _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                ShapeEmitter::Box(temp)
            }
            // 3
            IShape::ShapeCircle(shape) => {
                let mut temp = CircleShapeEmitter::new(shape.radius, shape.radius_thickness);
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };
                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                _pos = shape.position.clone();
                _rotation = shape.rotation.clone();
                _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                ShapeEmitter::Circle(temp)
            }
            // 0
            IShape::ShapeCone(shape) => {
                let mut temp =
                    ConeShapeEmitter::new(shape.radius, shape.angle * std::f32::consts::PI / 180.);
                temp.radius_range = shape.radius_thickness;
                temp.set_height(shape.height);
                temp.height_range = if shape.emit_as_volume { 1.0 } else { 0.0 };
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };

                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                _pos = shape.position.clone();
                _rotation = shape.rotation.clone();
                _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;
                ShapeEmitter::Cone(temp)
            }
            //5
            IShape::ShapeEdge(shape) => {
                let mut temp = EdgeShapeEmitter::new();
                temp.size = shape.radius;
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };
                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                _pos = shape.position.clone();
                _rotation = shape.rotation.clone();
                _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;

                ShapeEmitter::Edge(temp)
            }
            //4
            IShape::ShapeHemisphere(shape) => {
                let mut temp = HemisphereShapeEmitter::new(shape.radius, shape.radius_thickness);
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };
                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                _pos = shape.position.clone();
                _rotation = shape.rotation.clone();
                _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;

                ShapeEmitter::Hemisphere(temp)
            }
            // 6
            IShape::ShapeRectangle(shape) => {
                let temp = RectangleShapeEmitter::new();

                _pos = shape.position.clone();
                _rotation = shape.rotation.clone();
                _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;

                ShapeEmitter::Rectangle(temp)
            }
            // 1
            IShape::ShapeSphere(shape) => {
                let mut temp = SphereShapeEmitter::new(shape.radius, shape.radius_thickness);
                let (mode, value, spread, speed) = match &shape.arc {
                    crate::iparticle_system_config::IShapeArc::IShapeArcRandom(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcLoop(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcPingPong(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                    crate::iparticle_system_config::IShapeArc::IShapeArcBurstSpread(v) => {
                        (v.mode, v.value, v.spread, v.speed)
                    }
                };

                temp.arc_mode = mode;
                temp.arc_value = value * std::f32::consts::PI / 180.;
                temp.arc_spread = spread;
                temp.arc_speed = speed;

                _pos = shape.position.clone();
                _rotation = shape.rotation.clone();
                _scale = shape.scale.clone();
                _randomize = shape.randomize.clone();
                _align_dir = shape.align_dir;

                ShapeEmitter::Sphere(temp)
            }
        };

        if let (Some(pos), Some(rotation), Some(scale)) = (_pos, _rotation, _scale) {
            shape_emitter.position(Vector3::new(pos[0], pos[1], pos[2]));
            shape_emitter.rotation(Vector3::new(rotation[0], rotation[1], rotation[2]));
            shape_emitter.scaling(Vector3::new(scale[0], scale[1], scale[2]));
        }

        shape_emitter.align_direction(_align_dir != 0);

        if let Some(randomize) = &_randomize {
            shape_emitter.randomize_direction(randomize[0]);
            shape_emitter.spherize_direction(randomize[1]);
            shape_emitter.randomize_position(randomize[2]);
        }

        shape_emitter
    } else {
        ShapeEmitter::Point(PointShapeEmitter::new())
    };

    cmds.insert(ParticleCalculatorShapeEmitter(emitter));
}

pub fn format_emission(cmds: &mut EntityCommands, config: &IParticleSystemConfig) {
    // bursts
    let mut bursts: Vec<TBurstData> = vec![];
    if let Some(v) = &config.emission.1 {
        let len = v.len();
        for i in 0..len {
            bursts.push(v[i].clone());
        }
    }
    let emission = ParticleCalculatorEmission { 
        bursts,
        rateovertime: FloatInterpolation::new(config.emission.0)
    };

    cmds.insert(emission);
}


// pub struct ParticleCalculatorBursts {
//     pub(crate) bursts: Vec<TBurstData>,
// }

// pub struct ParticleCalculatorRateOverTime {
//     pub(crate) rateovertime: FloatInterpolation,
// }

// pub struct ParticleCalculatorShapeEmitter(pub(crate) ShapeEmitter);

// pub struct ParticleCalculatorStartLifetime(pub(crate) FloatInterpolation);

// pub struct ParticleCalculatorStartColor(pub(crate) Color4Interpolate);

// pub struct ParticleCalculatorStartSpeed(pub(crate) FloatInterpolation);

// pub struct ParticleCalculatorStartScaling(pub(crate) ScalingInterpolate);

// pub struct ParticleCalculatorGravity(pub(crate) FloatInterpolation);

// pub struct ParticleCalculatorStartRotation(pub(crate) RotationInterpolate);

// pub struct ParticleCalculatorVelocityOverLifetime(pub(crate) VelocityOverLifetime);

// pub struct ParticleCalculatorSizeOverLifetime(pub(crate) SizeOverLifetime);

// pub struct ParticleCalculatorLimitVelocityOverLifetime(pub(crate) LimitVelocityOverLifetime);

// pub struct ParticleCalculatorForceOverLifetime(pub(crate) ForceOverLifetime);

// pub struct ParticleCalculatorRotationOverLifetime(pub(crate) RotationOverLifetime);

// pub struct ParticleCalculatorRotationBySpeed(pub(crate) RotationBySpeed);

// pub struct ParticleCalculatorColorOverLifetime(pub(crate) ColorOverLifetime);

// pub struct ParticleCalculatorColorBySpeed(pub(crate) ColorBySpeed);

// pub struct ParticleCalculatorTextureSheet(pub(crate) TextureSheet);

// pub struct ParticleCalculatorRenderPivot{
//     pub(crate) position: Vector3,
// }

// pub struct ParticleCalculatorBase {
//     pub(crate) looping: bool,
//     pub(crate) prewarm: bool,
//     pub(crate) scaling_space: EScalingMode,
//     pub(crate) simulation_space: EParticleSimulationSpace,
//     pub(crate) render_alignment: EParticleRenderAlignment,
//     pub(crate) render_mode: EParticleRenderMode,
// }

// const SHADER_PI_SHADER_PS = 'PI_SHADER_PS';
// // CUSTOM_MATERIAL_SHADER_NAMES[SHADER_PI_SHADER_PS] = SHADER_PI_SHADER_PS;
// pub const SHADER_PI_SHADER_PS_NO_TEXSHEET = 'PI_SHADER_PS_NO_TEXSHEET';
// // CUSTOM_MATERIAL_SHADER_NAMES[SHADER_PI_SHADER_PS_NO_TEXSHEET] = SHADER_PI_SHADER_PS_NO_TEXSHEET;
// fn _createMaterialPiShaderPS(context: string, babylonScene: BABYLON.Scene, materialName: string | undefined, index: number, materialInfo: any, babylonDrawMode: number): any {

//     let name = materialName || "material" + index;
//     let mat: BABYLON.StandardMaterial;

//     mat = new BABYLON.StandardMaterial(name, babylonScene);

//     // mat = <any>new BABYLON.CustomMaterial(name, babylonScene);
//     // mat.name = SHADER_PI_SHADER_PS;

//     // (<any>mat).Vertex_MainEnd(`
//     // vec4 cell = floor(color / 10.0);
//     // vMainUV1 = vMainUV1.xy / cell.xy + cell.zw / cell.xy;
//     // vColor = color - cell * 10.0;
//     // `);

//     // (<any>mat).Fragment_Custom_Alpha(`
//     // alpha *= vColor.a;
//     // `);

//     if (materialInfo.useLightmapAsShadowmap) {
//         (<BABYLON.StandardMaterial>mat).useLightmapAsShadowmap = true;
//     }
//     if (materialInfo.useAlphaFromDiffuseTexture) {
//         (<BABYLON.StandardMaterial>mat).useAlphaFromDiffuseTexture = true;
//     }
//     if (materialInfo.alphaMode) {
//         (<BABYLON.StandardMaterial>mat).alphaMode = materialInfo.alphaMode;
//     }

//     mat.ambientColor = new BABYLON.Color3(1, 1, 1);

//     mat.sideOrientation = babylonScene.useRightHandedSystem ? BABYLON.Material.CounterClockWiseSideOrientation : BABYLON.Material.ClockWiseSideOrientation;

//     return mat;
// }

// fn _createMaterialPiShaderPSNoTexSheet(context: string, babylonScene: BABYLON.Scene, materialName: string | undefined, index: number, materialInfo: any, babylonDrawMode: number): any {

//     let name = materialName || "material" + index;
//     let mat: BABYLON.StandardMaterial;

//     mat = new BABYLON.StandardMaterial(name, babylonScene);

//     // mat = new BABYLON.CustomMaterial(name, babylonScene);
//     // (<any>mat).name = SHADER_PI_SHADER_PS_NO_TEXSHEET;

//     // mat.Vertex_After_WorldPosComputed(`
//     // vec4 cell = floor(color / 10.0);
//     // `);

//     // mat.Vertex_MainEnd(`
//     // vColor = color - cell * 10.0;
//     // `);

//     // mat.Fragment_Custom_Alpha(`
//     // alpha *= vColor.a;
//     // `);

//     if (materialInfo.useLightmapAsShadowmap) {
//         (<any>mat).useLightmapAsShadowmap = true;
//     }
//     if (materialInfo.useAlphaFromDiffuseTexture) {
//         (<any>mat).useAlphaFromDiffuseTexture = true;
//     }
//     if (materialInfo.alphaMode) {
//         (<any>mat).alphaMode = materialInfo.alphaMode;
//     }

//     (<any>mat).ambientColor = new BABYLON.Color3(1, 1, 1);

//     (<any>mat).sideOrientation = babylonScene.useRightHandedSystem ? BABYLON.Material.CounterClockWiseSideOrientation : BABYLON.Material.ClockWiseSideOrientation;

//     return mat;
// }

// fn _loadMaterialPropertiesAsyncPiShaderPS(context: string, material: any, mat: BABYLON.StandardMaterial, _loader: any): any {
//     let m = material;

//     var promises = new Array();

//     mat.backFaceCulling = false;

//     if (m.diffuseColor) {
//         if (m.disableLighting !== undefined) {
//             mat.disableLighting = m.disableLighting;
//             (<BABYLON.StandardMaterial>mat).diffuseColor    = new BABYLON.Color3(m.diffuseColor[0], m.diffuseColor[1], m.diffuseColor[2]);
//             mat.emissiveColor   = new BABYLON.Color3(m.diffuseColor[0], m.diffuseColor[1], m.diffuseColor[2]);
//             // 不管是否受光照影响 - 默认都不需要高光
//             (<BABYLON.StandardMaterial>mat).specularColor   = BABYLON.Color3.Black();
//             // 不受光照时设置自发光颜色 - 白色 - 即表现为纹理自身颜色
//             if (m.disableLighting) {
//                 (<BABYLON.StandardMaterial>mat).ambientColor    = BABYLON.Color3.Black();
//             }
//         // 原来的处理 - 没有文档说明为什么这样做
//         } else {
//             (<BABYLON.StandardMaterial>mat).diffuseColor    = new BABYLON.Color3(m.diffuseColor[0], m.diffuseColor[1], m.diffuseColor[2]);
//             mat.emissiveColor   = new BABYLON.Color3(m.diffuseColor[0], m.diffuseColor[1], m.diffuseColor[2]);
//             // 不管是否受光照影响 - 默认都不需要高光
//             (<BABYLON.StandardMaterial>mat).specularColor   = BABYLON.Color3.Black();
//             // 不管是否受光照影响 - 默认都不需要高光
//             (<BABYLON.StandardMaterial>mat).ambientColor   = BABYLON.Color3.White();
//             // (<BABYLON.StandardMaterial>mat).ambientColor = new BABYLON.Color3(m.diffuseColor[0], m.diffuseColor[1], m.diffuseColor[2]);
//         }
//     }
//     if (m.diffuseTexture) {
//         promises.push(_loader.loadTextureInfoAsync(context + "/PI_material/diffuseTexture", m.diffuseTexture, fn(texture: BABYLON.Texture) {
//             (<BABYLON.StandardMaterial>mat).diffuseTexture = texture;

//             if (m.type.indexOf("Particle") >= 0) {
//                 (<BABYLON.Texture>(<BABYLON.StandardMaterial>mat).diffuseTexture).hasAlpha = true;
//             }
//             if (m.diffuseTexture.hasAlpha) {
//                 (<BABYLON.Texture>(<BABYLON.StandardMaterial>mat).diffuseTexture).hasAlpha = true;
//             }
//             if (m.diffuseTexture.offset) {
//                 (<any>texture).uOffset = m.diffuseTexture.offset[0];
//                 (<any>texture).vOffset = m.diffuseTexture.offset[1];
//             }
//             if (m.diffuseTexture.scale) {
//                 (<any>texture).uScale = m.diffuseTexture.scale[0];
//                 (<any>texture).vScale = m.diffuseTexture.scale[1];
//             }
//             if (m.diffusePreAlpha) {
//                 mat.diffuseTexturePremultiplyAlpha = true;
//             }
//         }));
//     }
//     if (m.bumpTexture) {
//         promises.push(_loader.loadTextureInfoAsync(context + "/PI_material/bumpTexture", m.bumpTexture, fn(texture) {
//             (<BABYLON.StandardMaterial>mat).bumpTexture = texture;

//             if (m.bumpTexture.offset) {
//                 (<any>texture).uOffset = m.bumpTexture.offset[0];
//                 (<any>texture).vOffset = m.bumpTexture.offset[1];
//             }
//             if (m.bumpTexture.scale) {
//                 (<any>texture).uScale = m.bumpTexture.scale[0];
//                 (<any>texture).vScale = m.bumpTexture.scale[1];
//             }
//             (<BABYLON.Texture>texture).level = material.bumpTexture.level || material.normalScale || 1.0;
//         }));
//     }
//     if (m.opacityTexture) {
//         promises.push(_loader.loadTextureInfoAsync(context + "/PI_material/opacityTexture", m.opacityTexture, fn(texture) {
//             (<BABYLON.StandardMaterial>mat).opacityTexture = texture;

//             if (m.opacityTexture.offset) {
//                 (<any>texture).uOffset = m.opacityTexture.offset[0];
//                 (<any>texture).vOffset = m.opacityTexture.offset[1];
//             }
//             if (m.opacityTexture.scale) {
//                 (<any>texture).uScale = m.opacityTexture.scale[0];
//                 (<any>texture).vScale = m.opacityTexture.scale[1];
//             }
//             if (m.opacityApplyRGB) {
//                 mat.opacityTextureApplyRGB = true;
//             }
//         }));
//     }

//     return Promise.all(promises).then(fn() { });
// }

// (<any>BABYLON.GLTF2).Loader.Extensions.PI_material.regist(
//     SHADER_PI_SHADER_PS,
//     _createMaterialPiShaderPS,
//     <any>_loadMaterialPropertiesAsyncPiShaderPS
// );

// (<any>BABYLON.GLTF2).Loader.Extensions.PI_material.regist(
//     SHADER_PI_SHADER_PS_NO_TEXSHEET,
//     _createMaterialPiShaderPSNoTexSheet,
//     <any>_loadMaterialPropertiesAsyncPiShaderPS
// );
