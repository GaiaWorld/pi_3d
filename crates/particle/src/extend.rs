use pi_scene_math::{Matrix, Vector3};

use crate::{
    emitter::{
        box_shape_emitter::BoxShapeEmitter,
        circle_shape_emitter::CircleShapeEmitter,
        cone_shape_emitter::ConeShapeEmitter,
        edge_shape_emitter::EdgeShapeEmitter,
        hemisphere_shape_emitter::HemisphereShapeEmitter,
        ishape_emitter_type::{EBoxShapeMode, IShapeEmitterType},
        point_shape_emitter::PointShapeEmitter,
        rectangle_shape_emitter::RectangleShapeEmitter,
        sphere_shape_emitter::SphereShapeEmitter,
    },
    interpolation::{parseColor4Gradient, parseFloatInterpolation},
    iparticle_system_config::{IParticleSystemConfig, IShape, ParamInfo, TParamType},
    mesh_particle_system::MeshParticleSystem,
    modifier::{
        base::{
            Color4Interpolate, RotationInterpolate, ScalingInterpolate, TranslationInterpolate,
        },
        force_over_lifetime::ForceOverLifetime,
        rotation_over_lifetime::RotationOverLifetime,
        size_over_lifetime::SizeOverLifetime,
        start_color::StartColor,
        start_rotation::StartRotation,
        start_size::StartSize,
        velocity_over_lifetime::VelocityOverLifetime,
    },
    particle_system_tool::EMeshParticleSpaceMode,
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
/**
 * 格式化粒子系统
 * @param config josn描述
 * @param mp 目标粒子系统
 */
pub fn formatMeshParticle(config: &mut IParticleSystemConfig, mp: &mut MeshParticleSystem) {
    let ps = &mut mp.psTool;

    ps.looping = config.looping == 1;
    ps.duration = config.duration as u64 * 1000;
    ps.startDelay = config.startDelay as i32 * 1000;
    ps.maxParticles = config.maxParticles as usize;
    ps.prewarm = config.prewarm;
    ps.rateOverTime = config.emission.0;
    ps.simulationSpace = config.simulationSpaceIsWorld;
    ps.scalingSpace = config.scalingMode;
    ps.set_renderAlignment(config.renderAlignment);
    ps.set_renderMode(config.renderMode);
    ps.stretchedLengthScale = config.stretchedLengthScale;
    ps.stretchedVelocityScale = config.stretchedVelocityScale;

    // bursts
    if let Some(v) = &config.emission.1 {
        let len = v.len();
        for i in 0..len {
            ps.bursts.push(v[i]);
        }
    }

    // emitter Shape
    ps.emitterShape = Some(formatShape(Some(&config.shape)));

    // startLifetime
    parseFloatInterpolation(
        &mut ps.startLifetimeInterpolation,
        &Some(config.lifetime.clone()),
        TParamType::TParamStartLifetime,
        1000.0,
    );

    // startSpeed
    parseFloatInterpolation(
        &mut ps.startSpeedInterpolation,
        &Some(config.startSpeed.clone()),
        TParamType::TParamStartSpeed,
        1.,
    );

    // startSize
    StartSize::format(&config.startSize, &mut ps.startSizeInterpolation);

    // startRotation
    StartRotation::format(&config.startRotation, &mut ps.startRotationInterpolation);

    // startColor
    StartColor::format(&config.startColor, &mut ps.startColorInterpolation);

    // // gravity
    parseFloatInterpolation(
        &mut ps.gravityInterpolation.interpolation,
        &Some(config.gravity.clone()),
        TParamType::TParamGravity,
        1.0,
    );

    // velocityOverLifetime
    if let Some(velocityOverLifetime) = &config.velocityOverLifetime {
        VelocityOverLifetime::format(
            velocityOverLifetime,
            &mut ps.velocityOverLifetimeInterpolation.translationInterpolate,
        );
        let res = if let Some(velocity_over_lifetime_is_local) = config.velocityOverLifetimeIsLocal
        {
            velocity_over_lifetime_is_local != 0
        } else {
            false
        };

        ps.velocityOverLifetimeInterpolation.set_isLocalSpace(res);
        ps.enableVelocityOverLifeTime = true;
    }

    if let Some(orbtialVelocity) = &(config.orbtialVelocity) {
        VelocityOverLifetime::format(
            orbtialVelocity,
            &mut ps.localPositionModifier.orbitalRotateSpeed,
        );
    }
    if let Some(orbitalOffset) = (&config.orbitalOffset) {
        VelocityOverLifetime::format(orbitalOffset, &mut ps.localPositionModifier.orbitalOffset);
    }
    if config.orbitalRadial.is_some() {
        parseFloatInterpolation(
            &mut ps.localPositionModifier.radial,
            &config.orbitalRadial,
            TParamType::TParamStartSpeed,
            1.0,
        );
    }
    if config.speedModifier.is_some() {
        parseFloatInterpolation(
            &mut ps.localPositionModifier.speedModifier,
            &config.speedModifier,
            TParamType::TParamStartSpeed,
            1.0,
        );
    }

    // limitVelocityOverLifetime
    if config.limitVelocityOverLifetime.is_some() {
        parseFloatInterpolation(
            &mut ps.limitVelocityOverLifetimeInterpolation.interpolation,
            &config.limitVelocityOverLifetime,
            TParamType::TParamLimitVelocityOverLifetime,
            1.0,
        );
        ps.limitVelocityOverLifetimeInterpolation.dampen =
            if let Some(limitVelocityOverLifetimeDampen) = config.limitVelocityOverLifetimeDampen {
                limitVelocityOverLifetimeDampen
            } else {
                0.
            };
        ps.enableLimitVelocityOverLifeTime = true;
    }

    // forceOverLifetime
    if let Some(forceOverLifetime) = &config.forceOverLifetime {
        TranslationInterpolate::format(
            &forceOverLifetime,
            &mut ps.forceOverLifetimeInterpolation.translationInterpolate,
        );
        let res = if let Some(forceSpaceIsLocal) = config.forceSpaceIsLocal {
            forceSpaceIsLocal != 0
        } else {
            false
        };

        ps.forceOverLifetimeInterpolation.set_isLocalSpace(res);
        ps.enableForceOverLifeTime = true;
    }

    // colorOverLifetime
    if let Some(colorOverLifetime) = &(config.colorOverLifetime) {
        Color4Interpolate::format(
            colorOverLifetime,
            &mut ps.colorOverLifetimeInterpolation.color4Interpolate,
        );
        ps.enableColorOverLifeTime = true;
    }
    if let Some(colorBySpeed) = &(config.colorBySpeed) {
        Color4Interpolate::format(
            &colorBySpeed.0,
            &mut ps.colorBySpeedInterpolation.color4Interpolate,
        );
        ps.colorBySpeedInterpolation.set_rangeX(colorBySpeed.1);
        ps.colorBySpeedInterpolation.set_rangeY(colorBySpeed.2);
        ps.enableColorBySpeed = true;
    }

    // sizeOverLifetime
    if let Some(sizeOverLifetime) = &(config.sizeOverLifetime) {
        ScalingInterpolate::format(
            &sizeOverLifetime,
            &mut ps.sizeOverLifetimeInterpolation.scalingInterpolate,
        );
        ps.enableSizeOverLifeTime = true;
    }
    if let Some(sizeBySpeed) = &(config.sizeBySpeed) {
        ScalingInterpolate::format(
            &ParamInfo::OneParamInfo(sizeBySpeed.0.clone()),
            &mut ps.sizeBySpeedInterpolation.scalingInterpolate,
        );
        ps.sizeBySpeedInterpolation.set_rangeX(sizeBySpeed.1);
        ps.sizeBySpeedInterpolation.set_rangeY(sizeBySpeed.2);
        ps.enableSizeBySpeed = true;
    }

    // rotationOverLifetime
    if let Some(rotationOverLifetime) = &(config.rotationOverLifetime) {
        RotationInterpolate::format(
            &rotationOverLifetime,
            &mut ps.rotationOverLifetimeInterpolation.rotationInterpolate,
        );
        ps.enableRotationOverLifeTime = true;
    }
    if let Some(rotationBySpeed) = &(config.rotationBySpeed) {
        RotationInterpolate::format(
            &ParamInfo::OneParamInfo(rotationBySpeed.0.clone()),
            &mut ps.rotationBySpeedInterpolation.rotationInterpolate,
        );

        ps.rotationBySpeedInterpolation
            .set_rangeX(rotationBySpeed.1);
        ps.rotationBySpeedInterpolation
            .set_rangeY(rotationBySpeed.2);
        ps.enableRotationBySpeed = true;
    }

    if let Some(custom1) = &(config.custom1) {
        parseFloatInterpolation(
            &mut ps.customDataForMainUV.uScale,
            &Some(custom1[0].clone()),
            TParamType::TParamStartLifetime,
            1.0,
        );
        parseFloatInterpolation(
            &mut ps.customDataForMainUV.vScale,
            &Some(custom1[1].clone()),
            TParamType::TParamStartLifetime,
            1.0,
        );
        parseFloatInterpolation(
            &mut ps.customDataForMainUV.uOffset,
            &Some(custom1[2].clone()),
            TParamType::TParamStartLifetime,
            1.0,
        );
        parseFloatInterpolation(
            &mut ps.customDataForMainUV.vOffset,
            &Some(custom1[3].clone()),
            TParamType::TParamStartLifetime,
            1.0,
        );
        ps.enableCustomDataForMainUV = true;
    }

    /**
     * 导出时 有 textureSheet 必然对应 导出材质名称 为 SHADER_PI_SHADER_PS
     */
    if let Some(textureSheet) = &(config.textureSheet) {
        let tSheet = &mut ps.textureSheetInterpolation;
        parseFloatInterpolation(
            &mut tSheet.frameOverTime,
            &Some(textureSheet.frameOverTime.clone()),
            TParamType::TParamTextureSheet,
            1.0,
        );
        tSheet.animMode = textureSheet.animMode.clone();
        tSheet.customRow = textureSheet.customRow;
        tSheet.cycles = textureSheet.cycles;
        tSheet.rowMode = textureSheet.rowMode.clone();
        parseFloatInterpolation(
            &mut tSheet.startFrame,
            &Some(textureSheet.startFrame.clone()),
            TParamType::TParamTextureSheet,
            1.0,
        );
        tSheet.set_tilesX(textureSheet.tilesX);
        tSheet.set_tilesY(textureSheet.tilesY);
        tSheet.timeMode = textureSheet.timeMode.clone();
        ps.enableTextureSheet = true;
    }

    if let Some(trail) = &(config.trail) {
        ps.set_enableTrail(true);
        let ps_trail = ps.trail.as_mut().unwrap();
        ps_trail.set_mode(trail.mode);
        ps_trail.ratio = trail.ratio;
        parseFloatInterpolation(
            &mut ps_trail.lifetime,
            &Some(trail.lifetime.clone()),
            TParamType::None,
            1.0,
        );
        ps_trail.ribbonCount = trail.ribbonCount;
        ps_trail.attachRibbonsToTransfoem = trail.attachRTT == 1;
        ps_trail.minimunVertexDistance = trail.minDist;
        ps_trail.set_worldSpace(trail.worldSpace == 1);
        ps_trail.dieWithParticle = trail.dieWith == 1;
        ps_trail.sizeAffectsWidth = trail.sizeAWidth == 1;
        ps_trail.set_textureMode(trail.texMode);
        ps_trail.sizeAffectsLifetime = trail.sizeALifetime == 1;
        ps_trail.inheritParticleColor = trail.inheritColor == 1;
        parseColor4Gradient(
            &mut ps_trail.colorOverLifetime.color4Interpolate.gradient,
            Some(&trail.colorOverLife),
            TParamType::None,
        );
        parseFloatInterpolation(
            &mut ps_trail.widthOverTrail,
            &Some(trail.widthOverTrail.clone()),
            TParamType::None,
            1.0,
        );
        parseColor4Gradient(
            &mut ps_trail.colorOverTrail.color4Interpolate.gradient,
            Some(&trail.colorOverTrail),
            TParamType::None,
        );
    }

    if let Some(renderPivot) = (config.renderPivot) {
        ps.renderPivot = Vector3::new(renderPivot[0], renderPivot[1], renderPivot[2]);
    }
}
/**
 *
 * @param shape 形状发射器json描述
 * @returns
 */
fn formatShape(shape: Option<&IShape>) -> Box<dyn IShapeEmitterType> {
    if let Some(shape) = &shape {
        let mut shapeEmitter: Box<dyn IShapeEmitterType> = match shape {
            // 2
            IShape::ShapeBox(shape) => {
                let mut temp = BoxShapeEmitter::new();
                temp.emitMode = if let Some(mode) = &shape.boxEmitMode {
                    *mode
                } else {
                    EBoxShapeMode::Volume
                };
                temp.set_postion(Vector3::new(
                    shape.position[0],
                    shape.position[1],
                    shape.position[2],
                ));
                temp.set_rotation(Vector3::new(
                    shape.rotation[0],
                    shape.rotation[1],
                    shape.rotation[2],
                ));
                temp.set_scaling(Vector3::new(shape.scale[0], shape.scale[1], shape.scale[2]));
                temp.set_alignDirection(shape.alignDir != 0);
                if let Some(randomize) = &shape.randomize {
                    temp.set_randomizeDirection(randomize[0]);
                    temp.set_spherizeDirection(randomize[1]);
                    temp.set_randomizePosition(randomize[2]);
                }
                Box::new(temp)
            }
            // 3
            IShape::ShapeCircle(shape) => {
                let mut temp = CircleShapeEmitter::new(shape.radius, shape.radiusThickness);
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
                temp.arcMode = mode;
                temp.arcValue = value * std::f32::consts::PI / 180.;
                temp.arcSpread = spread;
                temp.arcSpeed = speed;

                temp.set_postion(Vector3::new(
                    shape.position[0],
                    shape.position[1],
                    shape.position[2],
                ));
                temp.set_rotation(Vector3::new(
                    shape.rotation[0],
                    shape.rotation[1],
                    shape.rotation[2],
                ));
                temp.set_scaling(Vector3::new(shape.scale[0], shape.scale[1], shape.scale[2]));
                temp.set_alignDirection(shape.alignDir != 0);
                if let Some(randomize) = &shape.randomize {
                    temp.set_randomizeDirection(randomize[0]);
                    temp.set_spherizeDirection(randomize[1]);
                    temp.set_randomizePosition(randomize[2]);
                }
                Box::new(temp)
            }
            // 0
            IShape::ShapeCone(shape) => {
                let mut temp =
                    ConeShapeEmitter::new(shape.radius, shape.angle * std::f32::consts::PI / 180.);
                temp.radiusRange = shape.radiusThickness;
                temp.set_height(shape.height);
                temp.heightRange = if shape.emitAsVolume { 1.0 } else { 0.0 };
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

                temp.arcMode = mode;
                temp.arcValue = value * std::f32::consts::PI / 180.;
                temp.arcSpread = spread;
                temp.arcSpeed = speed;

                temp.set_postion(Vector3::new(
                    shape.position[0],
                    shape.position[1],
                    shape.position[2],
                ));
                temp.set_rotation(Vector3::new(
                    shape.rotation[0],
                    shape.rotation[1],
                    shape.rotation[2],
                ));
                temp.set_scaling(Vector3::new(shape.scale[0], shape.scale[1], shape.scale[2]));
                temp.set_alignDirection(shape.alignDir != 0);
                if let Some(randomize) = &shape.randomize {
                    temp.set_randomizeDirection(randomize[0]);
                    temp.set_spherizeDirection(randomize[1]);
                    temp.set_randomizePosition(randomize[2]);
                }
                Box::new(temp)
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
                temp.arcMode = mode;
                temp.arcValue = value * std::f32::consts::PI / 180.;
                temp.arcSpread = spread;
                temp.arcSpeed = speed;

                temp.set_postion(Vector3::new(
                    shape.position[0],
                    shape.position[1],
                    shape.position[2],
                ));
                temp.set_rotation(Vector3::new(
                    shape.rotation[0],
                    shape.rotation[1],
                    shape.rotation[2],
                ));
                temp.set_scaling(Vector3::new(shape.scale[0], shape.scale[1], shape.scale[2]));
                temp.set_alignDirection(shape.alignDir != 0);
                if let Some(randomize) = &shape.randomize {
                    temp.set_randomizeDirection(randomize[0]);
                    temp.set_spherizeDirection(randomize[1]);
                    temp.set_randomizePosition(randomize[2]);
                }
                Box::new(temp)
            }
            //4
            IShape::ShapeHemisphere(shape) => {
                let mut temp = HemisphereShapeEmitter::new(shape.radius, shape.radiusThickness);
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
                temp.arcMode = mode;
                temp.arcValue = value * std::f32::consts::PI / 180.;
                temp.arcSpread = spread;
                temp.arcSpeed = speed;

                temp.set_postion(Vector3::new(
                    shape.position[0],
                    shape.position[1],
                    shape.position[2],
                ));
                temp.set_rotation(Vector3::new(
                    shape.rotation[0],
                    shape.rotation[1],
                    shape.rotation[2],
                ));
                temp.set_scaling(Vector3::new(shape.scale[0], shape.scale[1], shape.scale[2]));
                temp.set_alignDirection(shape.alignDir != 0);
                if let Some(randomize) = &shape.randomize {
                    temp.set_randomizeDirection(randomize[0]);
                    temp.set_spherizeDirection(randomize[1]);
                    temp.set_randomizePosition(randomize[2]);
                }
                Box::new(temp)
            }
            // 6
            IShape::ShapeRectangle(shape) => {
                let mut temp = RectangleShapeEmitter::new();
                temp.set_postion(Vector3::new(
                    shape.position[0],
                    shape.position[1],
                    shape.position[2],
                ));
                temp.set_rotation(Vector3::new(
                    shape.rotation[0],
                    shape.rotation[1],
                    shape.rotation[2],
                ));
                temp.set_scaling(Vector3::new(shape.scale[0], shape.scale[1], shape.scale[2]));
                temp.set_alignDirection(shape.alignDir != 0);
                if let Some(randomize) = &shape.randomize {
                    temp.set_randomizeDirection(randomize[0]);
                    temp.set_spherizeDirection(randomize[1]);
                    temp.set_randomizePosition(randomize[2]);
                }

                Box::new(temp)
            }
            // 1
            IShape::ShapeSphere(shape) => {
                let mut temp = SphereShapeEmitter::new(shape.radius, shape.radiusThickness);
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

                temp.arcMode = mode;
                temp.arcValue = value * std::f32::consts::PI / 180.;
                temp.arcSpread = spread;
                temp.arcSpeed = speed;
                temp.set_postion(Vector3::new(
                    shape.position[0],
                    shape.position[1],
                    shape.position[2],
                ));
                temp.set_rotation(Vector3::new(
                    shape.rotation[0],
                    shape.rotation[1],
                    shape.rotation[2],
                ));
                temp.set_scaling(Vector3::new(shape.scale[0], shape.scale[1], shape.scale[2]));
                temp.set_alignDirection(shape.alignDir != 0);

                if let Some(randomize) = &shape.randomize {
                    temp.set_randomizeDirection(randomize[0]);
                    temp.set_spherizeDirection(randomize[1]);
                    temp.set_randomizePosition(randomize[2]);
                }

                Box::new(temp)
            }
        };

        let mat = Matrix::new_nonuniform_scaling(&shapeEmitter.get_scaling())
            * Matrix::from_euler_angles(
                shapeEmitter.get_rotation()[0],
                shapeEmitter.get_rotation()[1],
                shapeEmitter.get_rotation()[2],
            )
            * Matrix::new_translation(&shapeEmitter.get_postion());
        shapeEmitter.set_localMatrix(mat);

        return shapeEmitter;
    } else {
        return Box::new(PointShapeEmitter::new());
    }
}

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
