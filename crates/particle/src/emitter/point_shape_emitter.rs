use pi_scene_math::{Matrix, Vector3};
use rand::Rng;

use super::{
    ishape_emitter_type::{EShapeEmitterDirectionMode, IShapeEmitterType},
    serializationObject,
};

/**
 * 点发射器
 */
pub struct PointShapeEmitter {
    MAX_Z: f32,
    /**
     * 创建模式
     */
    directionMode: EShapeEmitterDirectionMode,
    /**
     * 第一发射方向
     */
    direction1: Vector3,
    /**
     * 第二发射方向
     */
    direction2: Vector3,
    pub rotation: Vector3,
    pub position: Vector3,
    pub scaling: Vector3,

    localMatrix: Matrix,
    alignDirection: bool,
    randomizeDirection: f32,
    spherizeDirection: f32,
    randomizePosition: f32,
}

impl PointShapeEmitter {
    /**
     * Serializes the particle system to a JSON object.
     * @returns the JSON object
     */
    pub fn serialize(&self) -> serializationObject {
        serializationObject {
            _type: Some(PointShapeEmitter::get_class_name()),
            radius: None,
            angle: None,
            directionRandomizer: None,
            radiusRange: None,
            heightRange: None,
            emitFromSpawnPointOnly: None,
            size: None,
            direction1: Some(self.direction1),
            direction2: Some(self.direction2),
            
        }
    }

    /**
     * Parse properties from a JSON object
     * @param serializationObject defines the JSON object
     */
    pub fn parse(&mut self, arg: serializationObject) {
        self.direction1 = arg.direction1.as_ref().unwrap().clone();
        self.direction2 = arg.direction2.as_ref().unwrap().clone();
    }

    pub fn new() -> Self{
        Self{
            MAX_Z: 999999999.,
            directionMode: EShapeEmitterDirectionMode::Unity,
            direction1: Vector3::new(0.,0.,1.),
            direction2: Vector3::new(0.,0.,1.),
            rotation: Vector3::new(0., 0., 0.),
            position: Vector3::new(0., 0., 0.),
            scaling: Vector3::new(1., 1., 1.),
            localMatrix: Matrix::identity(),
            alignDirection: false,
            randomizeDirection: 0.,
            spherizeDirection: 0.,
            randomizePosition: 0.,
        }
    }
}

impl IShapeEmitterType for PointShapeEmitter {
    fn start_direction_function(
        &self,
        world_matrix: pi_scene_math::Matrix,
        direction_to_update: &mut Vector3,
        position: Vector3,
        local_position: Vector3,
        is_local: bool,
    ) {
        let mut rng = rand::thread_rng();

        let randX = rng.gen::<f32>() * (self.direction2[0] - self.direction1[0]) + self.direction1[0];
        let randZ = rng.gen::<f32>() * (self.direction2[1] - self.direction1[1]) + self.direction1[0];
        let randY = rng.gen::<f32>() * (self.direction2[2] - self.direction1[2]) + self.direction1[0];

        if is_local {
            *direction_to_update = Vector3::new(randX, randY, randZ);
        } else {
            *direction_to_update =
                world_matrix.transform_vector(&Vector3::new(randX, randY, randZ));
        }
    }

    fn start_position_function(
        &self,
        world_matrix: pi_scene_math::Matrix,
        position_to_update: &mut Vector3,
        emission_loop: f32,
        emission_progress: f32,
        emission_index: f32,
        emission_total: f32,
        is_local: bool,
    ) {
        if (is_local) {
            *position_to_update = Vector3::new(0., 0., 0.);
        } else {
            *position_to_update = world_matrix.transform_vector(&Vector3::new(0., 0., 0.));
        }
    }

    fn get_class_name() -> String
    where
        Self: Sized {
            return "PointParticleEmitter".to_string();
    }

    fn dispose()
    where
        Self: Sized {
        
    }

    fn set_postion(&mut self, position: Vector3) {
        self.position = position;
    }

    fn set_rotation(&mut self, rotation: Vector3) {
        self.rotation = rotation;
    }

    fn set_scaling(&mut self, scaling: Vector3) {
        self.scaling = scaling;
    }

    fn get_postion(&self) -> Vector3 {
        self.position.clone()
    }

    fn get_rotation(&self) -> Vector3 {
        self.rotation.clone()
    }

    fn get_scaling(&self) -> Vector3 {
        self.scaling.clone()
    }

    fn set_localMatrix(&mut self, localMatrix: Matrix) {
        self.localMatrix = localMatrix;
    }

    fn set_alignDirection(&mut self, alignDirection: bool) {
        self.alignDirection = alignDirection;
    }
    

    fn set_randomizeDirection(&mut self, randomizeDirection: f32) {
        self.randomizeDirection = randomizeDirection;
    }

    fn set_spherizeDirection(&mut self, spherizeDirection: f32) {
        self.spherizeDirection = spherizeDirection;
    }

    fn set_randomizePosition(&mut self, randomizePosition: f32) {
        self.randomizePosition = randomizePosition;
    }

    fn get_localMatrix(&mut self) -> Matrix {
        self.localMatrix.clone()
    }

    fn get_alignDirection(&mut self) -> bool {
        self.alignDirection.clone()
    }

    fn get_randomizeDirection(&mut self, ) -> f32 {
        self.randomizeDirection.clone()
    }

    fn get_spherizeDirection(&mut self) -> f32 {
        self.spherizeDirection.clone()
    }

    fn get_randomizePosition(&mut self) -> f32 {
        self.randomizePosition.clone()
    }
}

// export class PointShapeEmitter implements IShapeEmitterType {

//     public rotation: BABYLON.Vector3 = new BABYLON.Vector3(0, 0, 0);
//     public position: BABYLON.Vector3 = new BABYLON.Vector3(0, 0, 0);
//     public scaling: BABYLON.Vector3 = new BABYLON.Vector3(1, 1, 1);
//     public localMatrix: BABYLON.Matrix = BABYLON.Matrix.Identity();

//     alignDirection: boolean = false;
//     randomizeDirection: number = 0;
//     spherizeDirection: number = 0;
//     randomizePosition: number = 0;
//     /**
//      *
//      */
//     public startDirectionFunction(worldMatrix: BABYLON.Matrix, directionToUpdate: BABYLON.Vector3, position: BABYLON.Vector3, localPosition: BABYLON.Vector3, isLocal: boolean): void {
//         let randX = rng.gen_range(self.direction1[0], self.direction2[0]);
//         let randY = rng.gen_range(self.direction1[1], self.direction2[1]);
//         let randZ = rng.gen_range(self.direction1[2], self.direction2[2]);

//         if (isLocal) {
//             directionToUpdate.copyFromFloats(randX, randY, randZ);
//         }
//         else {
//             BABYLON.Vector3.TransformNormalFromFloatsToRef(randX, randY, randZ, worldMatrix, directionToUpdate);
//         }
//     }

//     /**
//      * Called by the particle System when the position is computed for the created particle.
//      * @param worldMatrix is the world matrix of the particle system
//      * @param positionToUpdate is the position vector to update with the result
//      * @param particle is the particle we are computed the position for
//      * @param isLocal defines if the position should be set in local space
//      */
//     startPositionFunction(worldMatrix: BABYLON.Matrix, positionToUpdate: BABYLON.Vector3, emissionLoop: number, emissionProgress: number, emissionIndex: number, emissionTotal: number, isLocal: boolean): void {
//         if (isLocal) {
//             positionToUpdate.copyFromFloats(0, 0, 0);
//         }
//         else {
//             BABYLON.Vector3.TransformCoordinatesFromFloatsToRef(0, 0, 0, worldMatrix, positionToUpdate);
//         }
//     }

//     /**
//      * Clones the current emitter and returns a copy of it
//      * @returns the new emitter
//      */
//     public clone(): PointShapeEmitter {
//         let newOne = new PointShapeEmitter();

//         BABYLON.DeepCopier.DeepCopy(self, newOne);

//         return newOne;
//     }

//     /**
//      * @returns a string containing the class name
//      */
//     public getClassName(): string {
//         return "PointParticleEmitter";
//     }

//     public dispose() {
//         // self.direction1 = undefined;
//         // self.direction2 = undefined;
//     }
// }
