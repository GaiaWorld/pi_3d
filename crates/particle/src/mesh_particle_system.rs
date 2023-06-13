use std::time::UNIX_EPOCH;

use pi_scene_math::{Matrix, Vector3};

use crate::particle_system_tool::ParticleSystemTool;

pub struct MeshParticleSystem {
    sqrt3: f32,
    pub psTool: ParticleSystemTool,
    _alwaysSelectAsActive: bool,
    _isMPPlaying: bool,
    // source: BABYLON.Mesh;
}

unsafe impl Send for MeshParticleSystem {}
unsafe impl Sync for MeshParticleSystem {}

impl MeshParticleSystem {
    // pub fn registCompute = (call: Function) => {
    //     getGlobal().setPermanentBefore(call);
    // }
    // public static registRecycle = (call: Function) => {
    //     getGlobal().setPermanent(call);
    // }
    // public static unregistCompute = (call: Function) => {
    //     getGlobal().clearPermanent(call);
    // }
    // public static unregistRecycle = (call: Function) => {
    //     getGlobal().clearPermanent(call);
    // }

    // public psTool: ParticleSystemTool;

    // private source: BABYLON.Mesh;

    // private _alwaysSelectAsActive: boolean = true;
    pub fn get_alwaysSelectAsActive(&self) -> bool {
        return self._alwaysSelectAsActive;
    }
    pub fn set_alwaysSelectAsActive(&mut self, v: bool) {
        self._alwaysSelectAsActive = v;
        // if self.source {
        //     self.source.alwaysSelectAsActiveMesh = v;
        // }
    }

    pub fn new() -> Self {
        Self {
            sqrt3: 0.,
            psTool: ParticleSystemTool::new(),
            _alwaysSelectAsActive: false,
            _isMPPlaying: false,
        }

        // if (self._scene.getEngine().getCaps().instancedArrays) {
        //     self.psTool = new ParticleSystemTool(scene);
        //     self.psTool.getParentWorldMatrix = () => {
        //         return self.parent ? self.parent.computeWorldMatrix() : undefined;
        //     };

        //     self.psTool.getCameraRotationMatrixInvert = () => {
        //         (self._scene).activeCamera.computeWorldMatrix();
        //         return (self._scene).activeCamera.getViewMatrix().getRotationMatrix().invert();
        //     };
        //     self.psTool.getCameraPosition = () => {
        //         (self._scene).activeCamera.computeWorldMatrix();
        //         return (self._scene).activeCamera.globalPosition;
        //     };

        //     self.psTool.getCameraMatrix = () => {
        //         (self._scene).activeCamera.computeWorldMatrix();
        //         return (self._scene).activeCamera.getViewMatrix();
        //     };

        // r.psTool.getWorldMatrix = () => {
        //     self.computeWorldMatrix();
        //     return self.worldMatrixFromCache;
        // };

        //     self.psTool.getLocalMatrix = () => {
        //         self.computeWorldMatrix();
        //         return self._localMatrix;
        //     };
        // } else {
        //     console.error(`Can not use instancedArrays`);
        // }
    }

    pub fn setSourceMesh(&mut self) {
        // self.source = mesh;
        // if (mesh) {
        //     if (!self.psTool) {
        //         mesh.setEnabled(false);
        //         return;
        //     }
        //     mesh.renderAsMeshParticle = true;
        //     mesh.useVertexColors = true;
        //     if (mesh.alphaIndex >= 3000) {
        //         mesh.hasVertexAlpha  = true;
        //     }
        //     mesh.alwaysSelectAsActiveMesh = self.alwaysSelectAsActive;
        // }
        self.build();
    }

    // public dispose(doNotRecurse?: boolean, disposeMaterialAndTextures?: boolean) {
    //     if (self.isDisposed()) {
    //         return;
    //     }
    //     self.stop();

    //     self.psTool.dispose();

    //     self.psTool = undefined;

    //     if (self.source) {
    //         self.source.dispose(false, true);
    //     }

    //     super.dispose(doNotRecurse, disposeMaterialAndTextures);
    // }

    pub fn build(&mut self) {
        self.psTool.build();

        // if (self.source) {
        //     self.source.thinInstanceSetBuffer("matrix", self.psTool.mpMatrixList, 16, false);
        //     self.source.thinInstanceSetBuffer("color", self.psTool.mpColorData, 4, false);
        //     // self.source.thinInstanceSetBuffer(ATTRIBUTE_PS_UV_SHEET, self.psTool.mpUVSheetData, 4, false);
        //     // var colorBuffer = new BABYLON.VertexBuffer(self._scene.getEngine(), self.psTool.mpColorData, BABYLON.VertexBuffer.ColorKind, true, false, 4, true);
        //     // self.source.setVerticesBuffer(colorBuffer);
        //     var uvBuffer = new BABYLON.VertexBuffer(self._scene.getEngine(), self.psTool.mpColorData, BABYLON.VertexBuffer.TangentKind, true, false, 4, true);
        //     self.source.setVerticesBuffer(uvBuffer);
        // }
    }

    // private _isMPPlaying: boolean = false;
    pub fn start(&mut self) {
        if !self._isMPPlaying {
            self.psTool.start();

            // MeshParticleSystem::registCompute(self._computeCall);
            // MeshParticleSystem::registRecycle(self._recycleCall);
            // self._scene.onBeforeRenderObservable.add(self._updateCall);

            self._isMPPlaying = true;
        }
    }

    pub fn computeCall(&mut self, world_matrix: Matrix, local_matrix: Matrix) {
        // self.computeWorldMatrix();
        self.psTool.varCompute(std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64, world_matrix, local_matrix);
        // self._mpDirty = true;

        // self._recycleCall();
    }

    // private _mpDirty = true;
    pub fn updateCall(
        &mut self,
        world_matrix: Matrix,
        local_matrix: Matrix,
        camera_pos: Vector3,
        camera_rotation_matrix_invert: Matrix,
    ) {
        // if (self._mpDirty) {
        let _ = self.psTool.mpUpdate(
            world_matrix,
            local_matrix,
            camera_pos,
            camera_rotation_matrix_invert,
        );

        // if (self.source) {
        //     self.source.thinInstanceBufferUpdated("matrix");
        //     // self.source.updateVerticesData(BABYLON.VertexBuffer.ColorKind, self.psTool.mpColorData);
        //     self.source.thinInstanceBufferUpdated("color");
        //     // self.source.thinInstanceBufferUpdated(ATTRIBUTE_PS_UV_SHEET);
        //     self.source.thinInstanceCount = count;

        //     self.source.setEnabled(count > 0 && self.isEnabled());
        // }

        // self._mpDirty = false;
        // }
    }

    // private _recycleCall = () => {
    //     self.psTool.recycle();
    // }

    pub fn stop(&mut self) {
        if (self._isMPPlaying) {
            self.psTool.stop();

            // MeshParticleSystem.unregistCompute(self._computeCall);
            // MeshParticleSystem.unregistRecycle(self._recycleCall);
            // self._scene.onBeforeRenderObservable.removeCallback(self._updateCall);

            self._isMPPlaying = false;
        }
    }

    // public reset() {
    //     self.stop();
    // }
}
